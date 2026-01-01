//! MCP tool implementations

use super::params::*;
use crate::search::{list_files, search_code};
use crate::types::{FileInfo, SearchOptions, SearchResult};
use crate::{circular, complexity, deadcode, duplicates};
use rmcp::handler::server::wrapper::{Json, Parameters};
use std::path::PathBuf;

/// Search for text patterns in code files
pub async fn search_code_tool(params: Parameters<SearchCodeParams>) -> Json<Vec<SearchResult>> {
    let params = params.0;
    let path_buf = PathBuf::from(params.path.as_deref().unwrap_or("."));
    
    let options = SearchOptions {
        extensions: params.extensions,
        ignore_case: params.ignore_case.unwrap_or(false),
        fuzzy: params.fuzzy.unwrap_or(false),
        fuzzy_threshold: params.fuzzy_threshold.unwrap_or(0.6),
        max_results: params.max_results.unwrap_or(10),
        exclude: params.exclude,
        rank: params.rank.unwrap_or(false),
        cache: false,
        semantic: false,
        benchmark: false,
        vs_grep: false,
    };
    
    Json(search_code(&params.query, &path_buf, &options).unwrap_or_default())
}

/// List all searchable files in a directory
pub async fn list_files_tool(params: Parameters<ListFilesParams>) -> Json<Vec<FileInfo>> {
    let params = params.0;
    let path_buf = PathBuf::from(params.path.as_deref().unwrap_or("."));
    
    Json(list_files(
        &path_buf,
        params.extensions.as_deref(),
        params.exclude.as_deref(),
    ).unwrap_or_default())
}

/// Analyze codebase metrics and statistics
pub async fn analyze_codebase_tool(params: Parameters<AnalyzeCodebaseParams>) -> Json<serde_json::Value> {
    let params = params.0;
    let path_buf = PathBuf::from(params.path.as_deref().unwrap_or("."));
    
    let files = list_files(
        &path_buf,
        params.extensions.as_deref(),
        params.exclude.as_deref(),
    ).unwrap_or_default();
    
    if files.is_empty() {
        return Json(serde_json::json!({
            "message": "No files found to analyze",
            "files": []
        }));
    }

    // Calculate statistics
    let total_files = files.len();
    let total_lines: usize = files.iter().map(|f| f.lines).sum();
    let total_size: u64 = files.iter().map(|f| f.size).sum();
    
    // File type breakdown
    let mut ext_counts: std::collections::HashMap<String, (usize, usize, u64)> = std::collections::HashMap::new();
    for file in &files {
        let ext = if let Some(ext) = std::path::Path::new(&file.path).extension() {
            ext.to_string_lossy().to_string()
        } else {
            "no extension".to_string()
        };
        
        let entry = ext_counts.entry(ext).or_insert((0, 0, 0));
        entry.0 += 1;
        entry.1 += file.lines;
        entry.2 += file.size;
    }

    // Code patterns analysis
    let patterns = vec![
        ("functions", r"fn\s+\w+|function\s+\w+|def\s+\w+"),
        ("classes", r"class\s+\w+"),
        ("comments", r"//|#|/\*|<!--"),
        ("todo", r"TODO|FIXME|HACK|XXX"),
        ("imports", r"^import|^use|^#include|^require"),
    ];

    let mut pattern_counts = std::collections::HashMap::new();
    for (name, pattern) in patterns {
        if let Ok(regex) = regex::Regex::new(pattern) {
            let mut total_matches = 0;
            
            for file in &files {
                if let Ok(content) = std::fs::read_to_string(&file.path) {
                    for line in content.lines() {
                        if regex.is_match(line) {
                            total_matches += 1;
                        }
                    }
                }
            }
            pattern_counts.insert(name, total_matches);
        }
    }

    // Largest files
    let mut file_vec = files.clone();
    file_vec.sort_by(|a, b| b.lines.cmp(&a.lines));
    let largest_files: Vec<_> = file_vec.iter().take(5).map(|f| {
        serde_json::json!({
            "path": f.path,
            "lines": f.lines,
            "size": f.size
        })
    }).collect();

    Json(serde_json::json!({
        "total_files": total_files,
        "total_lines": total_lines,
        "total_size": total_size,
        "file_types": ext_counts.iter().map(|(ext, (count, lines, size))| {
            serde_json::json!({
                "extension": ext,
                "files": count,
                "lines": lines,
                "size": size
            })
        }).collect::<Vec<_>>(),
        "patterns": pattern_counts,
        "largest_files": largest_files
    }))
}

/// Detect code complexity issues
pub async fn detect_complexity_tool(params: Parameters<ComplexityParams>) -> Json<serde_json::Value> {
    let params = params.0;
    let path_buf = PathBuf::from(params.path.as_deref().unwrap_or("."));
    
    let metrics = complexity::calculate_complexity(
        &path_buf,
        params.extensions.as_deref(),
        params.exclude.as_deref(),
    ).unwrap_or_default();
    
    let mut filtered_metrics = metrics;
    if let Some(threshold) = params.threshold {
        filtered_metrics.retain(|m| m.cyclomatic_complexity >= threshold || m.cognitive_complexity >= threshold);
    }
    
    if params.sort.unwrap_or(false) {
        filtered_metrics.sort_by(|a, b| {
            b.cyclomatic_complexity.cmp(&a.cyclomatic_complexity)
                .then(b.cognitive_complexity.cmp(&a.cognitive_complexity))
        });
    }
    
    Json(serde_json::json!({
        "metrics": filtered_metrics,
        "total_files": filtered_metrics.len()
    }))
}

/// Detect duplicate code blocks
pub async fn detect_duplicates_tool(params: Parameters<DuplicatesParams>) -> Json<serde_json::Value> {
    let params = params.0;
    let path_buf = PathBuf::from(params.path.as_deref().unwrap_or("."));
    
    let duplicates = duplicates::find_duplicates(
        &path_buf,
        params.extensions.as_deref(),
        params.exclude.as_deref(),
        params.min_lines.unwrap_or(5),
        params.similarity.unwrap_or(0.9),
    ).unwrap_or_default();
    
    Json(serde_json::json!({
        "duplicates": duplicates,
        "total_duplicates": duplicates.len()
    }))
}

/// Detect dead code
pub async fn detect_deadcode_tool(params: Parameters<DeadcodeParams>) -> Json<serde_json::Value> {
    let params = params.0;
    let path_buf = PathBuf::from(params.path.as_deref().unwrap_or("."));
    
    let dead_code = deadcode::find_dead_code(
        &path_buf,
        params.extensions.as_deref(),
        params.exclude.as_deref(),
    ).unwrap_or_default();
    
    Json(serde_json::json!({
        "dead_code": dead_code,
        "total_items": dead_code.len()
    }))
}

/// Detect circular dependencies
pub async fn detect_circular_tool(params: Parameters<CircularParams>) -> Json<serde_json::Value> {
    let params = params.0;
    let path_buf = PathBuf::from(params.path.as_deref().unwrap_or("."));
    
    let cycles = circular::find_circular_dependencies(
        &path_buf,
        params.extensions.as_deref(),
        params.exclude.as_deref(),
    ).unwrap_or_default();
    
    Json(serde_json::json!({
        "cycles": cycles,
        "total_cycles": cycles.len()
    }))
}
