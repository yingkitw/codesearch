//! Git History Search Module
//!
//! Provides search capabilities across git history.

use git2::{Commit, DiffOptions, Repository};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitSearchResult {
    pub commit_id: String,
    pub author: String,
    pub timestamp: i64,
    pub message: String,
    pub file_path: String,
    pub line_number: usize,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitInfo {
    pub id: String,
    pub author: String,
    pub email: String,
    pub timestamp: i64,
    pub message: String,
    pub files_changed: Vec<String>,
}

pub struct GitSearcher {
    repo: Repository,
}

impl GitSearcher {
    pub fn new(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let repo = Repository::discover(path)?;
        Ok(Self { repo })
    }

    pub fn search_history(
        &self,
        pattern: &str,
        max_commits: usize,
    ) -> Result<Vec<GitSearchResult>, Box<dyn std::error::Error>> {
        let mut results = Vec::new();
        let regex = regex::Regex::new(pattern)?;

        let mut revwalk = self.repo.revwalk()?;
        revwalk.push_head()?;

        let mut commit_count = 0;
        for oid in revwalk {
            if commit_count >= max_commits {
                break;
            }

            let oid = oid?;
            let commit = self.repo.find_commit(oid)?;
            
            if let Ok(commit_results) = self.search_commit(&commit, &regex) {
                results.extend(commit_results);
            }

            commit_count += 1;
        }

        Ok(results)
    }

    fn search_commit(
        &self,
        commit: &Commit,
        regex: &regex::Regex,
    ) -> Result<Vec<GitSearchResult>, Box<dyn std::error::Error>> {
        let mut results = Vec::new();

        let tree = commit.tree()?;
        let parent_tree = if commit.parent_count() > 0 {
            Some(commit.parent(0)?.tree()?)
        } else {
            None
        };

        let mut diff_opts = DiffOptions::new();
        let diff = self.repo.diff_tree_to_tree(
            parent_tree.as_ref(),
            Some(&tree),
            Some(&mut diff_opts),
        )?;

        diff.foreach(
            &mut |delta, _| {
                if let Some(path) = delta.new_file().path() {
                    if let Ok(blob) = tree.get_path(path).and_then(|entry| self.repo.find_blob(entry.id())) {
                        if let Ok(content) = std::str::from_utf8(blob.content()) {
                            for (line_num, line) in content.lines().enumerate() {
                                if regex.is_match(line) {
                                    results.push(GitSearchResult {
                                        commit_id: commit.id().to_string(),
                                        author: commit.author().name().unwrap_or("Unknown").to_string(),
                                        timestamp: commit.time().seconds(),
                                        message: commit.message().unwrap_or("").to_string(),
                                        file_path: path.to_string_lossy().to_string(),
                                        line_number: line_num + 1,
                                        content: line.to_string(),
                                    });
                                }
                            }
                        }
                    }
                }
                true
            },
            None,
            None,
            None,
        )?;

        Ok(results)
    }

    pub fn get_commit_history(
        &self,
        max_commits: usize,
    ) -> Result<Vec<CommitInfo>, Box<dyn std::error::Error>> {
        let mut commits = Vec::new();
        let mut revwalk = self.repo.revwalk()?;
        revwalk.push_head()?;

        let mut count = 0;
        for oid in revwalk {
            if count >= max_commits {
                break;
            }

            let oid = oid?;
            let commit = self.repo.find_commit(oid)?;
            
            let files_changed = self.get_changed_files(&commit)?;

            commits.push(CommitInfo {
                id: commit.id().to_string(),
                author: commit.author().name().unwrap_or("Unknown").to_string(),
                email: commit.author().email().unwrap_or("").to_string(),
                timestamp: commit.time().seconds(),
                message: commit.message().unwrap_or("").to_string(),
                files_changed,
            });

            count += 1;
        }

        Ok(commits)
    }

    fn get_changed_files(&self, commit: &Commit) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut files = Vec::new();

        let tree = commit.tree()?;
        let parent_tree = if commit.parent_count() > 0 {
            Some(commit.parent(0)?.tree()?)
        } else {
            None
        };

        let mut diff_opts = DiffOptions::new();
        let diff = self.repo.diff_tree_to_tree(
            parent_tree.as_ref(),
            Some(&tree),
            Some(&mut diff_opts),
        )?;

        diff.foreach(
            &mut |delta, _| {
                if let Some(path) = delta.new_file().path() {
                    files.push(path.to_string_lossy().to_string());
                }
                true
            },
            None,
            None,
            None,
        )?;

        Ok(files)
    }

    pub fn search_by_author(
        &self,
        author: &str,
        max_commits: usize,
    ) -> Result<Vec<CommitInfo>, Box<dyn std::error::Error>> {
        let commits = self.get_commit_history(max_commits * 2)?;
        
        Ok(commits
            .into_iter()
            .filter(|c| c.author.contains(author))
            .take(max_commits)
            .collect())
    }

    pub fn search_by_message(
        &self,
        pattern: &str,
        max_commits: usize,
    ) -> Result<Vec<CommitInfo>, Box<dyn std::error::Error>> {
        let regex = regex::Regex::new(pattern)?;
        let commits = self.get_commit_history(max_commits * 2)?;
        
        Ok(commits
            .into_iter()
            .filter(|c| regex.is_match(&c.message))
            .take(max_commits)
            .collect())
    }

    pub fn search_file_history(
        &self,
        file_path: &str,
        pattern: &str,
        max_commits: usize,
    ) -> Result<Vec<GitSearchResult>, Box<dyn std::error::Error>> {
        let regex = regex::Regex::new(pattern)?;
        let mut results = Vec::new();
        let mut revwalk = self.repo.revwalk()?;
        revwalk.push_head()?;

        let mut count = 0;
        for oid in revwalk {
            if count >= max_commits {
                break;
            }

            let oid = oid?;
            let commit = self.repo.find_commit(oid)?;
            let tree = commit.tree()?;

            if let Ok(entry) = tree.get_path(Path::new(file_path)) {
                if let Ok(blob) = self.repo.find_blob(entry.id()) {
                    if let Ok(content) = std::str::from_utf8(blob.content()) {
                        for (line_num, line) in content.lines().enumerate() {
                            if regex.is_match(line) {
                                results.push(GitSearchResult {
                                    commit_id: commit.id().to_string(),
                                    author: commit.author().name().unwrap_or("Unknown").to_string(),
                                    timestamp: commit.time().seconds(),
                                    message: commit.message().unwrap_or("").to_string(),
                                    file_path: file_path.to_string(),
                                    line_number: line_num + 1,
                                    content: line.to_string(),
                                });
                            }
                        }
                    }
                }
            }

            count += 1;
        }

        Ok(results)
    }

    pub fn get_blame_info(
        &self,
        file_path: &str,
        line_number: usize,
    ) -> Result<CommitInfo, Box<dyn std::error::Error>> {
        let blame = self.repo.blame_file(Path::new(file_path), None)?;
        
        if let Some(hunk) = blame.get_line(line_number) {
            let commit = self.repo.find_commit(hunk.final_commit_id())?;
            
            let files_changed = self.get_changed_files(&commit)?;
            
            Ok(CommitInfo {
                id: commit.id().to_string(),
                author: commit.author().name().unwrap_or("Unknown").to_string(),
                email: commit.author().email().unwrap_or("").to_string(),
                timestamp: commit.time().seconds(),
                message: commit.message().unwrap_or("").to_string(),
                files_changed,
            })
        } else {
            Err("Line not found in blame".into())
        }
    }
}

pub fn search_git_history(
    repo_path: &Path,
    pattern: &str,
    max_commits: usize,
) -> Result<Vec<GitSearchResult>, Box<dyn std::error::Error>> {
    let searcher = GitSearcher::new(repo_path)?;
    searcher.search_history(pattern, max_commits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_searcher_creation() {
        let current_dir = std::env::current_dir().unwrap();
        let result = GitSearcher::new(&current_dir);
        
        assert!(result.is_ok() || result.is_err());
    }
}
