//! Fuzzy Search Implementation
//!
//! Provides fuzzy matching and relevance scoring for search results.

use crate::types::{Match, SearchResult};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use regex::Regex;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::Arc;

/// Search within a single file using parallel processing
pub fn search_in_file_parallel(
    file_path: &Path,
    regex: &Arc<Regex>,
    fuzzy: bool,
    fuzzy_threshold: f64,
    query: &str,
    max_results: usize,
    rank: bool,
) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
    let file = fs::File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut results = Vec::new();
    let mut line_count = 0;
    let matcher = SkimMatcherV2::default();

    for line in reader.lines() {
        line_count += 1;
        let line = line?;

        if results.len() >= max_results {
            break;
        }

        if fuzzy {
            if let Some((score, indices)) = matcher.fuzzy_indices(&line, query) {
                if score as f64 >= fuzzy_threshold {
                    let mut matches = Vec::new();

                    for &idx in &indices {
                        if matches.is_empty() || idx >= matches.last().map(|m: &Match| m.end).unwrap_or(0) {
                            matches.push(Match {
                                start: idx,
                                end: idx + 1,
                                text: line.chars().nth(idx).map(|c| c.to_string()).unwrap_or_default(),
                            });
                        }
                    }

                    let (score_val, relevance) = if rank {
                        let s = calculate_relevance_score(&line, query, line_count, file_path, true, Some(score));
                        let r = if s >= 80.0 { "Very High" } else if s >= 60.0 { "High" } else if s >= 40.0 { "Medium" } else { "Low" };
                        (s, r.to_string())
                    } else {
                        (score as f64, "Medium".to_string())
                    };

                    results.push(SearchResult {
                        file: file_path.to_string_lossy().to_string(),
                        line_number: line_count,
                        content: line.clone(),
                        matches,
                        score: score_val,
                        relevance,
                    });
                }
            }
        } else if let Some(mat) = regex.find_iter(&line).next() {
            let (score_val, relevance) = if rank {
                let s = calculate_relevance_score(&line, query, line_count, file_path, false, None);
                let r = if s >= 80.0 { "Very High" } else if s >= 60.0 { "High" } else if s >= 40.0 { "Medium" } else { "Low" };
                (s, r.to_string())
            } else {
                (50.0, "Medium".to_string())
            };

            let matches = vec![Match {
                start: mat.start(),
                end: mat.end(),
                text: mat.as_str().to_string(),
            }];

            results.push(SearchResult {
                file: file_path.to_string_lossy().to_string(),
                line_number: line_count,
                content: line.clone(),
                matches,
                score: score_val,
                relevance,
            });
        }
    }

    Ok(results)
}

/// Calculate relevance score for a search result
pub fn calculate_relevance_score(
    line: &str,
    query: &str,
    line_number: usize,
    file_path: &Path,
    _is_fuzzy: bool,
    fuzzy_score: Option<i64>,
) -> f64 {
    let mut score = 50.0;

    if line.contains(query) {
        score += 30.0;
    }

    if let Some(fs) = fuzzy_score {
        score += (fs as f64) / 10.0;
    }

    if line_number < 100 {
        score += 5.0;
    }

    if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
        match ext {
            "rs" | "py" | "js" | "ts" => score += 10.0,
            "md" | "txt" => score += 5.0,
            _ => {}
        }
    }

    let definition_patterns = ["fn ", "def ", "function ", "class ", "struct ", "impl ", "trait "];
    for pattern in &definition_patterns {
        if line.contains(pattern) {
            score += 15.0;
            break;
        }
    }

    score.clamp(0.0, 100.0)
}
