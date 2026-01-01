//! Remote Repository Search Module
//!
//! Provides search capabilities for remote repositories (GitHub, GitLab, etc.).

use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteSearchResult {
    pub repository: String,
    pub file_path: String,
    pub line_number: usize,
    pub content: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryInfo {
    pub name: String,
    pub owner: String,
    pub url: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub stars: u32,
}

pub struct RemoteSearcher {
    client: Client,
    api_token: Option<String>,
}

impl RemoteSearcher {
    pub fn new(api_token: Option<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("codesearch/0.1")
            .build()?;
        
        Ok(Self { client, api_token })
    }

    pub fn clone_and_search(
        &self,
        repo_url: &str,
        pattern: &str,
        extensions: Option<&[String]>,
    ) -> Result<Vec<RemoteSearchResult>, Box<dyn std::error::Error>> {
        let temp_dir = tempfile::tempdir()?;
        let repo_path = temp_dir.path();

        self.clone_repository(repo_url, repo_path)?;

        let results = self.search_local_clone(repo_path, pattern, extensions)?;

        Ok(results)
    }

    fn clone_repository(&self, url: &str, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        use git2::Repository;
        Repository::clone(url, path)?;
        Ok(())
    }

    fn search_local_clone(
        &self,
        path: &Path,
        pattern: &str,
        extensions: Option<&[String]>,
    ) -> Result<Vec<RemoteSearchResult>, Box<dyn std::error::Error>> {
        use crate::search::search_code;
        use crate::types::SearchOptions;
        
        let options = SearchOptions {
            extensions: extensions.map(|e| e.to_vec()),
            ignore_case: false,
            fuzzy: false,
            fuzzy_threshold: 0.6,
            max_results: 100,
            exclude: Some(vec!["target".to_string(), "node_modules".to_string(), ".git".to_string()]),
            rank: false,
            cache: false,
            semantic: false,
            benchmark: false,
            vs_grep: false,
        };
        
        let search_results = search_code(pattern, path, &options)?;

        let mut results = Vec::new();
        for result in search_results {
            results.push(RemoteSearchResult {
                repository: path.to_string_lossy().to_string(),
                file_path: result.file.clone(),
                line_number: result.line_number,
                content: result.content.clone(),
                url: format!("file://{}", result.file),
            });
        }

        Ok(results)
    }

    pub fn search_github(
        &self,
        query: &str,
        language: Option<&str>,
        max_results: usize,
    ) -> Result<Vec<RemoteSearchResult>, Box<dyn std::error::Error>> {
        let mut url = format!(
            "https://api.github.com/search/code?q={}",
            urlencoding::encode(query)
        );

        if let Some(lang) = language {
            url.push_str(&format!(" language:{}", lang));
        }

        url.push_str(&format!("&per_page={}", max_results.min(100)));

        let mut request = self.client.get(&url);
        
        if let Some(token) = &self.api_token {
            request = request.header("Authorization", format!("token {token}"));
        }

        let response = request.send()?;
        
        if !response.status().is_success() {
            return Err(format!("GitHub API error: {}", response.status()).into());
        }

        let json: serde_json::Value = response.json()?;
        let mut results = Vec::new();

        if let Some(items) = json["items"].as_array() {
            for item in items {
                if let (Some(repo), Some(path), Some(html_url)) = (
                    item["repository"]["full_name"].as_str(),
                    item["path"].as_str(),
                    item["html_url"].as_str(),
                ) {
                    results.push(RemoteSearchResult {
                        repository: repo.to_string(),
                        file_path: path.to_string(),
                        line_number: 0,
                        content: String::new(),
                        url: html_url.to_string(),
                    });
                }
            }
        }

        Ok(results)
    }

    pub fn search_gitlab(
        &self,
        query: &str,
        project_id: Option<u32>,
        max_results: usize,
    ) -> Result<Vec<RemoteSearchResult>, Box<dyn std::error::Error>> {
        let base_url = if let Some(id) = project_id {
            format!("https://gitlab.com/api/v4/projects/{id}/search")
        } else {
            "https://gitlab.com/api/v4/search".to_string()
        };

        let url = format!(
            "{}?scope=blobs&search={}&per_page={}",
            base_url,
            urlencoding::encode(query),
            max_results.min(100)
        );

        let mut request = self.client.get(&url);
        
        if let Some(token) = &self.api_token {
            request = request.header("PRIVATE-TOKEN", token);
        }

        let response = request.send()?;
        
        if !response.status().is_success() {
            return Err(format!("GitLab API error: {}", response.status()).into());
        }

        let items: Vec<serde_json::Value> = response.json()?;
        let mut results = Vec::new();

        for item in items {
            if let (Some(path), Some(data)) = (
                item["path"].as_str(),
                item["data"].as_str(),
            ) {
                results.push(RemoteSearchResult {
                    repository: item["project_id"].to_string(),
                    file_path: path.to_string(),
                    line_number: 0,
                    content: data.to_string(),
                    url: format!("https://gitlab.com/{path}"),
                });
            }
        }

        Ok(results)
    }

    pub fn get_repository_info(&self, owner: &str, repo: &str) -> Result<RepositoryInfo, Box<dyn std::error::Error>> {
        let url = format!("https://api.github.com/repos/{owner}/{repo}");
        
        let mut request = self.client.get(&url);
        
        if let Some(token) = &self.api_token {
            request = request.header("Authorization", format!("token {token}"));
        }

        let response = request.send()?;
        
        if !response.status().is_success() {
            return Err(format!("GitHub API error: {}", response.status()).into());
        }

        let json: serde_json::Value = response.json()?;

        Ok(RepositoryInfo {
            name: json["name"].as_str().unwrap_or("").to_string(),
            owner: json["owner"]["login"].as_str().unwrap_or("").to_string(),
            url: json["html_url"].as_str().unwrap_or("").to_string(),
            description: json["description"].as_str().map(|s| s.to_string()),
            language: json["language"].as_str().map(|s| s.to_string()),
            stars: json["stargazers_count"].as_u64().unwrap_or(0) as u32,
        })
    }

    pub fn search_multiple_repos(
        &self,
        repos: &[String],
        pattern: &str,
        extensions: Option<&[String]>,
    ) -> Result<Vec<RemoteSearchResult>, Box<dyn std::error::Error>> {
        let mut all_results = Vec::new();

        for repo_url in repos {
            match self.clone_and_search(repo_url, pattern, extensions) {
                Ok(results) => all_results.extend(results),
                Err(e) => eprintln!("Error searching {repo_url}: {e}"),
            }
        }

        Ok(all_results)
    }
}

pub fn search_remote_repository(
    repo_url: &str,
    pattern: &str,
    extensions: Option<&[String]>,
    api_token: Option<String>,
) -> Result<Vec<RemoteSearchResult>, Box<dyn std::error::Error>> {
    let searcher = RemoteSearcher::new(api_token)?;
    searcher.clone_and_search(repo_url, pattern, extensions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remote_searcher_creation() {
        let searcher = RemoteSearcher::new(None);
        assert!(searcher.is_ok());
    }

    #[test]
    fn test_remote_searcher_with_token() {
        let searcher = RemoteSearcher::new(Some("test_token".to_string()));
        assert!(searcher.is_ok());
    }
}
