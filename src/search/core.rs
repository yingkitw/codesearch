//! Core Search Functionality
//!
//! Main search implementation with parallel processing and caching.

use crate::cache::get_search_cache;
use crate::types::{SearchMetrics, SearchOptions, SearchResult};
use super::fuzzy::search_in_file_parallel;
use super::semantic::enhance_query_semantically;
use super::utilities::compare_with_grep;
use regex::Regex;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;
use walkdir::WalkDir;

/// Main search function with support for fuzzy, regex, semantic, and cached searches
pub fn search_code(
    query: &str,
    path: &Path,
    options: &SearchOptions,
) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    let mut results = Vec::new();

    let (cache_hits, cache_misses) = if options.cache {
        let search_cache = get_search_cache();
        let extensions_slice = options.extensions.as_ref().map(|v| v.as_slice());
        let cache_key = search_cache.get_cache_key(query, &path.to_string_lossy(), extensions_slice, options.fuzzy);
        if let Some(cached_results) = search_cache.get(&cache_key) {
            if options.benchmark {
                use colored::*;
                println!("{}", "Cache hit! Returning cached results instantly.".green().bold());
            }
            return Ok(cached_results);
        } else {
            (0, 1)
        }
    } else {
        (0, 0)
    };

    let enhanced_query = if options.semantic {
        enhance_query_semantically(query)
    } else {
        query.to_string()
    };

    let regex = if options.fuzzy {
        if options.ignore_case {
            Regex::new(&format!("(?i).*{}.*", regex::escape(&enhanced_query)))?
        } else {
            Regex::new(&format!(".*{}.*", regex::escape(&enhanced_query)))?
        }
    } else if options.ignore_case {
        Regex::new(&format!("(?i){}", &enhanced_query))?
    } else {
        Regex::new(&enhanced_query)?
    };

    let walker = WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| {
            if let Some(name) = e.file_name().to_str() {
                if let Some(ref exclude_dirs) = options.exclude {
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

    let files: Vec<PathBuf> = walker
        .filter(|entry| {
            let file_path = entry.path();
            if let Some(ref exts) = options.extensions {
                if let Some(ext) = file_path.extension().and_then(|s| s.to_str()) {
                    exts.iter().any(|e| e == ext)
                } else {
                    false
                }
            } else {
                true
            }
        })
        .map(|e| e.path().to_path_buf())
        .collect();

    let total_files = files.len();
    let regex = Arc::new(regex);

    use rayon::prelude::*;
    let file_results: Vec<SearchResult> = files
        .par_iter()
        .filter_map(|file_path| {
            search_in_file_parallel(file_path, &regex, options.fuzzy, options.fuzzy_threshold, query, options.max_results, options.rank).ok()
        })
        .flatten()
        .collect();

    results.extend(file_results);

    if options.rank {
        results.sort_by(|a, b| {
            b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    let elapsed = start_time.elapsed();
    let total_matches: usize = results.iter().map(|r| r.matches.len()).sum();
    let metrics = SearchMetrics {
        files_processed: total_files,
        total_lines_scanned: 0,
        search_time_ms: elapsed.as_millis(),
        parallel_workers: rayon::current_num_threads(),
        cache_hits,
        cache_misses,
    };

    if options.benchmark {
        use colored::*;
        println!("\n{}", "Performance Metrics:".cyan().bold());
        println!("  Files searched: {}", metrics.files_processed);
        println!("  Total matches: {total_matches}");
        println!("  Search time: {}ms", metrics.search_time_ms);
        println!("  Parallel workers: {}", metrics.parallel_workers);
        if options.cache {
            println!("  Cache hits: {}", metrics.cache_hits);
            println!("  Cache misses: {}", metrics.cache_misses);
        }
    }

    if options.vs_grep {
        let extensions_slice = options.extensions.as_ref().map(|v| v.as_slice());
        compare_with_grep(query, &path.to_string_lossy(), extensions_slice, &metrics);
    }

    if options.cache && !results.is_empty() {
        let search_cache = get_search_cache();
        let extensions_slice = options.extensions.as_ref().map(|v| v.as_slice());
        let cache_key = search_cache.get_cache_key(query, &path.to_string_lossy(), extensions_slice, options.fuzzy);
        search_cache.set(cache_key, results.clone());
    }

    Ok(results)
}

/// List all searchable files in a directory
pub fn list_files(
    path: &Path,
    extensions: Option<&[String]>,
    exclude: Option<&[String]>,
) -> Result<Vec<crate::types::FileInfo>, Box<dyn std::error::Error>> {
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

    let files: Vec<crate::types::FileInfo> = walker
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
        .map(|entry| {
            let path = entry.path();
            let metadata = std::fs::metadata(path).ok();
            let size = metadata.as_ref().map(|m| m.len()).unwrap_or(0);
            let lines = if let Ok(content) = std::fs::read_to_string(path) {
                content.lines().count()
            } else {
                0
            };

            crate::types::FileInfo {
                path: path.to_string_lossy().to_string(),
                size,
                lines,
            }
        })
        .collect();

    Ok(files)
}
