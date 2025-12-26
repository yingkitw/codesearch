//! Search Module
//!
//! Core search functionality with parallel processing, fuzzy matching, and semantic search.

use crate::cache::get_search_cache;
use crate::types::{FileInfo, Match, SearchMetrics, SearchResult};
use colored::*;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use rayon::prelude::*;
use regex::Regex;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;
use walkdir::WalkDir;

/// Main search function with support for fuzzy, regex, semantic, and cached searches
pub fn search_code(
    query: &str,
    path: &Path,
    extensions: Option<&[String]>,
    ignore_case: bool,
    fuzzy: bool,
    fuzzy_threshold: f64,
    max_results: usize,
    exclude: Option<&[String]>,
    rank: bool,
    cache: bool,
    semantic: bool,
    benchmark: bool,
    vs_grep: bool,
) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    let mut results = Vec::new();

    // Performance tracking
    let (cache_hits, cache_misses) = if cache {
        let search_cache = get_search_cache();
        let cache_key = search_cache.get_cache_key(query, &path.to_string_lossy(), extensions, fuzzy);
        if let Some(cached_results) = search_cache.get(&cache_key) {
            if benchmark {
                println!("{}", "🚀 Cache hit! Returning cached results instantly.".green().bold());
            }
            return Ok(cached_results);
        } else {
            (0, 1)
        }
    } else {
        (0, 0)
    };

    // Enhanced query for semantic search
    let enhanced_query = if semantic {
        enhance_query_semantically(query)
    } else {
        query.to_string()
    };

    let regex = if fuzzy {
        // For fuzzy search, we'll use a more permissive pattern with escaped query
        if ignore_case {
            Regex::new(&format!("(?i).*{}.*", regex::escape(&enhanced_query)))?
        } else {
            Regex::new(&format!(".*{}.*", regex::escape(&enhanced_query)))?
        }
    } else if ignore_case {
        // For case-insensitive regex search, don't escape the pattern to preserve regex
        Regex::new(&format!("(?i){}", &enhanced_query))?
    } else {
        Regex::new(&enhanced_query)?
    };

    let walker = WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| {
            if let Some(name) = e.file_name().to_str() {
                if let Some(exclude_dirs) = exclude {
                    for exclude_dir in exclude_dirs {
                        if name == exclude_dir {
                            return false;
                        }
                    }
                }
            }
            true
        })
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file());

    // Collect all files first for parallel processing
    let files: Vec<PathBuf> = walker
        .filter(|entry| {
            let file_path = entry.path();
            if let Some(exts) = extensions {
                if let Some(ext) = file_path.extension().and_then(|s| s.to_str()) {
                    exts.iter().any(|e| e == ext)
                } else {
                    false
                }
            } else {
                true
            }
        })
        .map(|entry| entry.path().to_path_buf())
        .collect();

    let files_processed = files.len();
    let show_progress = files_processed > 50 && !benchmark;

    // Parallel search across files with progress indicator
    let regex_arc = Arc::new(regex);
    let query_arc = Arc::new(enhanced_query.clone());

    let parallel_results: Vec<Vec<SearchResult>> = if show_progress {
        use std::sync::atomic::{AtomicUsize, Ordering};
        let processed = Arc::new(AtomicUsize::new(0));
        let processed_clone = processed.clone();

        eprint!("{}", "🔍 Searching... ".cyan());

        files
            .par_iter()
            .map(|file_path| {
                if cache && !get_search_cache().is_file_modified(&file_path.to_string_lossy()) {
                    let count = processed_clone.fetch_add(1, Ordering::Relaxed) + 1;
                    if count % 10 == 0 || count == files_processed {
                        eprint!("\r{}", format!("🔍 Searching... {}/{} files", count, files_processed).cyan());
                    }
                    return Vec::new();
                }

                let result = search_in_file_parallel(
                    file_path,
                    &regex_arc,
                    fuzzy,
                    fuzzy_threshold,
                    &query_arc,
                    max_results,
                ).unwrap_or_else(|_| Vec::new());

                let count = processed_clone.fetch_add(1, Ordering::Relaxed) + 1;
                if count % 10 == 0 || count == files_processed {
                    eprint!("\r{}", format!("🔍 Searching... {}/{} files", count, files_processed).cyan());
                }

                result
            })
            .collect()
    } else {
        files
            .par_iter()
            .map(|file_path| {
                if cache && !get_search_cache().is_file_modified(&file_path.to_string_lossy()) {
                    return Vec::new();
                }

                search_in_file_parallel(
                    file_path,
                    &regex_arc,
                    fuzzy,
                    fuzzy_threshold,
                    &query_arc,
                    max_results,
                ).unwrap_or_else(|_| Vec::new())
            })
            .collect()
    };

    if show_progress {
        eprintln!("\r{}", format!("✅ Searched {} files", files_processed).green());
    }

    // Flatten results
    for file_results in parallel_results {
        results.extend(file_results);
    }

    // Sort results by relevance score if ranking is enabled
    if rank {
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    }

    // Cache results if caching is enabled
    if cache {
        let search_cache = get_search_cache();
        let cache_key = search_cache.get_cache_key(query, &path.to_string_lossy(), extensions, fuzzy);
        search_cache.set(cache_key, results.clone());
    }

    // Print performance metrics
    let search_time = start_time.elapsed();

    if benchmark || files_processed > 100 {
        let metrics = SearchMetrics {
            files_processed,
            total_lines_scanned: results.len(),
            search_time_ms: search_time.as_millis(),
            parallel_workers: rayon::current_num_threads(),
            cache_hits,
            cache_misses,
        };

        println!("{}", format!("⚡ Performance: {} files in {}ms ({} workers, {} cache hits)",
            metrics.files_processed,
            metrics.search_time_ms,
            metrics.parallel_workers,
            metrics.cache_hits
        ).cyan().italic());

        if semantic {
            println!("{}", "🧠 Semantic search enabled - enhanced context matching".blue().italic());
        }

        if vs_grep {
            compare_with_grep(query, &path.to_string_lossy(), extensions, &metrics);
        }
    }

    Ok(results)
}

/// Search within a single file using parallel processing
fn search_in_file_parallel(
    file_path: &Path,
    regex: &Arc<Regex>,
    fuzzy: bool,
    fuzzy_threshold: f64,
    query: &Arc<String>,
    max_results: usize,
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

                    let (relevance_score, relevance) = calculate_relevance_score(
                        &line, query, line_count, file_path, true, Some(score),
                    );

                    results.push(SearchResult {
                        file: file_path.to_string_lossy().to_string(),
                        line_number: line_count,
                        content: line.clone(),
                        matches,
                        score: relevance_score,
                        relevance,
                    });
                }
            }
        } else {
            for mat in regex.find_iter(&line) {
                let matches = vec![Match {
                    start: mat.start(),
                    end: mat.end(),
                    text: mat.as_str().to_string(),
                }];

                let (relevance_score, relevance) = calculate_relevance_score(
                    &line, query, line_count, file_path, false, None,
                );

                results.push(SearchResult {
                    file: file_path.to_string_lossy().to_string(),
                    line_number: line_count,
                    content: line.clone(),
                    matches,
                    score: relevance_score,
                    relevance,
                });
                break; // Only one result per line for regex
            }
        }
    }

    Ok(results)
}

/// Calculate relevance score for a search result
fn calculate_relevance_score(
    line: &str,
    query: &str,
    line_number: usize,
    file_path: &Path,
    _is_fuzzy: bool,
    fuzzy_score: Option<i64>,
) -> (f64, String) {
    let mut score = 50.0; // Base score

    // Exact match bonus
    if line.contains(query) {
        score += 30.0;
    }

    // Fuzzy score contribution
    if let Some(fs) = fuzzy_score {
        score += (fs as f64) / 10.0;
    }

    // Line position bonus (earlier lines slightly preferred)
    if line_number < 100 {
        score += 5.0;
    }

    // File type bonus
    if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
        match ext {
            "rs" | "py" | "js" | "ts" => score += 10.0, // Source files
            "md" | "txt" => score += 5.0, // Documentation
            _ => {}
        }
    }

    // Definition pattern bonus
    let definition_patterns = ["fn ", "def ", "function ", "class ", "struct ", "impl ", "trait "];
    for pattern in &definition_patterns {
        if line.contains(pattern) {
            score += 15.0;
            break;
        }
    }

    // Normalize score to 0-100
    score = score.min(100.0).max(0.0);

    let relevance = if score >= 80.0 {
        "Very High".to_string()
    } else if score >= 60.0 {
        "High".to_string()
    } else if score >= 40.0 {
        "Medium".to_string()
    } else {
        "Low".to_string()
    };

    (score, relevance)
}

/// Enhance a query with semantic patterns
fn enhance_query_semantically(query: &str) -> String {
    let semantic_patterns = [
        ("function", r"(function|def|fn|func|method|procedure)"),
        ("class", r"(class|struct|interface|trait|type)"),
        ("variable", r"(let|var|const|val|mut)"),
        ("loop", r"(for|while|do|foreach|map|filter)"),
        ("condition", r"(if|else|switch|case|when|match)"),
        ("error", r"(error|exception|panic|fail|throw)"),
        ("test", r"(test|spec|it|describe|assert)"),
        ("import", r"(import|use|require|include|from)"),
        ("export", r"(export|module|pub|public)"),
        ("async", r"(async|await|promise|future)"),
    ];

    let mut enhanced = query.to_string();

    for (term, pattern) in &semantic_patterns {
        if query.to_lowercase().contains(term) {
            enhanced = format!("({}|{})", enhanced, pattern);
        }
    }

    // Add context-aware patterns
    if query.contains("get") {
        enhanced = format!("({}|retrieve|fetch|obtain)", enhanced);
    }
    if query.contains("set") {
        enhanced = format!("({}|assign|update|modify)", enhanced);
    }
    if query.contains("create") {
        enhanced = format!("({}|make|build|construct|new)", enhanced);
    }
    if query.contains("delete") {
        enhanced = format!("({}|remove|destroy|clear)", enhanced);
    }

    enhanced
}

/// Compare search performance with grep
fn compare_with_grep(query: &str, path: &str, extensions: Option<&[String]>, metrics: &SearchMetrics) {
    use std::process::Command;

    let start_time = Instant::now();

    let mut grep_cmd = Command::new("grep");
    grep_cmd.arg("-r").arg("-n").arg("--color=never");

    if let Some(exts) = extensions {
        for ext in exts {
            grep_cmd.arg("--include").arg(&format!("*.{}", ext));
        }
    }

    grep_cmd.arg(query).arg(path);

    let grep_result = grep_cmd.output();
    let grep_time = start_time.elapsed();

    match grep_result {
        Ok(output) => {
            let grep_lines = String::from_utf8_lossy(&output.stdout).lines().count();
            let speedup = if grep_time.as_millis() > 0 {
                grep_time.as_millis() as f64 / metrics.search_time_ms as f64
            } else {
                1.0
            };

            println!();
            println!("{}", "📊 Performance Comparison with Grep".yellow().bold());
            println!("{}", "─".repeat(40).yellow());
            println!("  {}: {}ms", "Code Search".green().bold(), metrics.search_time_ms);
            println!("  {}: {}ms", "Grep".blue().bold(), grep_time.as_millis());
            println!("  {}: {:.1}x", "Speedup".cyan().bold(), speedup);
            println!("  {}: {} vs {} lines", "Results".magenta().bold(), metrics.files_processed, grep_lines);

            if speedup > 1.0 {
                println!("  {}: Code Search is {:.1}x faster!", "Winner".green().bold(), speedup);
            } else if speedup < 1.0 {
                println!("  {}: Grep is {:.1}x faster", "Winner".red().bold(), 1.0 / speedup);
            } else {
                println!("  {}: Similar performance", "Tie".yellow().bold());
            }
        }
        Err(_) => {
            println!("{}", "⚠️  Could not run grep comparison (grep not found)".yellow());
        }
    }
}

/// List all searchable files in a directory
pub fn list_files(
    path: &Path,
    extensions: Option<&[String]>,
    exclude: Option<&[String]>,
) -> Result<Vec<FileInfo>, Box<dyn std::error::Error>> {
    let mut files = Vec::new();

    let walker = WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| {
            if let Some(name) = e.file_name().to_str() {
                if let Some(exclude_dirs) = exclude {
                    for exclude_dir in exclude_dirs {
                        if name == exclude_dir {
                            return false;
                        }
                    }
                }
            }
            true
        })
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file());

    for entry in walker {
        let file_path = entry.path();

        // Filter by extension if specified
        if let Some(exts) = extensions {
            if let Some(ext) = file_path.extension().and_then(|s| s.to_str()) {
                if !exts.iter().any(|e| e == ext) {
                    continue;
                }
            } else {
                continue;
            }
        }

        // Get file metadata
        if let Ok(metadata) = fs::metadata(file_path) {
            let lines = if let Ok(content) = fs::read_to_string(file_path) {
                content.lines().count()
            } else {
                0
            };

            files.push(FileInfo {
                path: file_path.to_string_lossy().to_string(),
                size: metadata.len(),
                lines,
            });
        }
    }

    Ok(files)
}

/// Print search results with optional line numbers and ranking
pub fn print_results(results: &[SearchResult], show_line_numbers: bool, show_ranking: bool) {
    for result in results {
        let mut highlighted_line = result.content.clone();

        // Highlight matches in reverse order to maintain positions
        for m in result.matches.iter().rev() {
            if m.end <= highlighted_line.len() && m.start < m.end {
                let before = &highlighted_line[..m.start];
                let matched = &highlighted_line[m.start..m.end];
                let after = &highlighted_line[m.end..];
                highlighted_line = format!("{}{}{}", before, matched.red().bold(), after);
            }
        }

        if show_line_numbers {
            if show_ranking {
                println!(
                    "{}:{} [{}] {}",
                    result.file.cyan(),
                    result.line_number.to_string().yellow(),
                    format!("{:.0}", result.score).green(),
                    highlighted_line
                );
            } else {
                println!(
                    "{}:{} {}",
                    result.file.cyan(),
                    result.line_number.to_string().yellow(),
                    highlighted_line
                );
            }
        } else if show_ranking {
            println!(
                "{} [{}] {}",
                result.file.cyan(),
                format!("{:.0}", result.score).green(),
                highlighted_line
            );
        } else {
            println!("{} {}", result.file.cyan(), highlighted_line);
        }
    }
}

/// Print search statistics
pub fn print_search_stats(results: &[SearchResult], query: &str) {
    if results.is_empty() {
        return;
    }

    let unique_files: std::collections::HashSet<_> = results.iter().map(|r| &r.file).collect();

    println!();
    println!("{}", "─".repeat(40).dimmed());
    println!(
        "{} {} matches in {} files for '{}'",
        "📊".dimmed(),
        results.len().to_string().green().bold(),
        unique_files.len().to_string().cyan().bold(),
        query.yellow()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_search_code_basic() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.rs");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "fn main() {{}}").unwrap();
        writeln!(file, "fn test() {{}}").unwrap();

        let results = search_code(
            "fn", dir.path(), None, false, false, 0.6, 10, None, false, false, false, false, false,
        ).unwrap();

        assert!(!results.is_empty());
    }

    #[test]
    fn test_search_code_with_extension_filter() {
        let dir = tempdir().unwrap();
        let rs_file = dir.path().join("test.rs");
        let py_file = dir.path().join("test.py");
        
        let mut f1 = File::create(&rs_file).unwrap();
        writeln!(f1, "fn main").unwrap();
        
        let mut f2 = File::create(&py_file).unwrap();
        writeln!(f2, "def main").unwrap();

        let results = search_code(
            "main", dir.path(), Some(&["rs".to_string()]), false, false, 0.6, 10, None, false, false, false, false, false,
        ).unwrap();

        assert!(results.iter().all(|r| r.file.ends_with(".rs")));
    }

    #[test]
    fn test_calculate_relevance_score() {
        let (score, relevance) = calculate_relevance_score(
            "fn main() {}", "main", 1, Path::new("test.rs"), false, None,
        );
        assert!(score > 50.0);
        assert!(!relevance.is_empty());
    }

    #[test]
    fn test_enhance_query_semantically() {
        let enhanced = enhance_query_semantically("function");
        assert!(enhanced.contains("def"));
        assert!(enhanced.contains("fn"));
    }

    #[test]
    fn test_list_files() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.rs");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "test content").unwrap();

        let files = list_files(dir.path(), None, None).unwrap();
        assert!(!files.is_empty());
    }
}

