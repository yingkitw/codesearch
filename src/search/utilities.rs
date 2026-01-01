//! Search Utilities
//!
//! Helper functions for search operations including grep comparison and output formatting.

use crate::types::{SearchMetrics, SearchResult};
use colored::*;
use std::time::Instant;

/// Compare search performance with grep
pub fn compare_with_grep(query: &str, path: &str, extensions: Option<&[String]>, metrics: &SearchMetrics) {
    use std::process::Command;

    let start_time = Instant::now();
    
    let mut grep_cmd = Command::new("grep");
    grep_cmd.arg("-r").arg("-n").arg(query).arg(path);
    
    if let Some(exts) = extensions {
        for ext in exts {
            grep_cmd.arg("--include").arg(format!("*.{ext}"));
        }
    }
    
    let output = grep_cmd.output();
    let grep_time = start_time.elapsed();
    
    println!("\n{}", "Grep Comparison:".cyan().bold());
    println!("  codesearch time: {}ms", metrics.search_time_ms);
    println!("  grep time: {}ms", grep_time.as_millis());
    
    if let Ok(result) = output {
        let grep_matches = String::from_utf8_lossy(&result.stdout).lines().count();
        println!("  codesearch files: {}", metrics.files_processed);
        println!("  grep matches: {grep_matches}");
        
        let speedup = grep_time.as_millis() as f64 / metrics.search_time_ms as f64;
        if speedup > 1.0 {
            println!("  {} {:.2}x faster than grep", "codesearch is".green().bold(), speedup);
        } else if speedup < 1.0 {
            println!("  {} {:.2}x slower than grep", "codesearch is".yellow(), 1.0 / speedup);
        } else {
            println!("  Similar performance");
        }
    }
}

/// Print search results with optional line numbers and ranking
pub fn print_results(results: &[SearchResult], show_line_numbers: bool, show_ranking: bool) {
    if results.is_empty() {
        return;
    }

    let mut current_file = String::new();
    
    for result in results {
        if result.file != current_file {
            current_file = result.file.clone();
            println!("\n{}", current_file.green().bold());
        }
        
        let line_prefix = if show_line_numbers {
            format!("{}:", result.line_number).blue().to_string()
        } else {
            String::new()
        };
        
        let ranking_suffix = if show_ranking {
            format!(" [score: {:.1}]", result.score).yellow().to_string()
        } else {
            String::new()
        };
        
        let mut highlighted_content = result.content.clone();
        for mat in result.matches.iter().rev() {
            if mat.end <= highlighted_content.len() {
                let before = &highlighted_content[..mat.start];
                let matched = &highlighted_content[mat.start..mat.end];
                let after = &highlighted_content[mat.end..];
                highlighted_content = format!("{}{}{}", before, matched.red().bold(), after);
            }
        }
        
        println!("  {line_prefix}{highlighted_content}{ranking_suffix}");
    }
}

/// Print search statistics
pub fn print_search_stats(results: &[SearchResult], query: &str) {
    if results.is_empty() {
        return;
    }

    let total_files = results.iter().map(|r| &r.file).collect::<std::collections::HashSet<_>>().len();
    let total_matches: usize = results.iter().map(|r| r.matches.len()).sum();
    
    println!("\n{}", "Search Statistics:".cyan().bold());
    println!("  Query: {}", query.yellow());
    println!("  Files with matches: {total_files}");
    println!("  Total matches: {total_matches}");
    
    let avg_score: f64 = results.iter().map(|r| r.score).sum::<f64>() / results.len() as f64;
    println!("  Average relevance score: {avg_score:.1}");
}
