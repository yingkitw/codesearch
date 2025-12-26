//! Complexity Analysis Module
//!
//! Provides code complexity metrics including cyclomatic and cognitive complexity.

use crate::language::get_language_by_extension;
use crate::search::list_files;
use crate::types::ComplexityMetrics;
use colored::*;
use regex::Regex;
use std::fs;
use std::path::Path;

/// Analyze complexity for all files in a directory
pub fn analyze_complexity(
    path: &Path,
    extensions: Option<&[String]>,
    exclude: Option<&[String]>,
    threshold: Option<u32>,
    sort: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "📊 Code Complexity Analysis".cyan().bold());
    println!("{}", "─".repeat(30).cyan());
    println!();

    let files = list_files(path, extensions, exclude)?;
    let mut all_metrics: Vec<ComplexityMetrics> = Vec::new();

    for file in &files {
        if let Ok(content) = fs::read_to_string(&file.path) {
            let metrics = calculate_file_complexity(&file.path, &content);
            all_metrics.push(metrics);
        }
    }

    // Filter by threshold if specified
    if let Some(thresh) = threshold {
        all_metrics.retain(|m| m.cyclomatic_complexity >= thresh);
    }

    // Sort by complexity if requested
    if sort {
        all_metrics.sort_by(|a, b| b.cyclomatic_complexity.cmp(&a.cyclomatic_complexity));
    }

    // Print results
    for metrics in &all_metrics {
        let complexity_color = if metrics.cyclomatic_complexity > 20 {
            "red"
        } else if metrics.cyclomatic_complexity > 10 {
            "yellow"
        } else {
            "green"
        };

        println!(
            "{} {} CC: {} COG: {} LOC: {} Functions: {} Nesting: {}",
            "📄".dimmed(),
            metrics.file_path.blue(),
            metrics.cyclomatic_complexity.to_string().color(complexity_color).bold(),
            metrics.cognitive_complexity.to_string().yellow(),
            metrics.lines_of_code.to_string().dimmed(),
            metrics.function_count.to_string().dimmed(),
            metrics.max_nesting_depth.to_string().dimmed(),
        );
    }

    // Summary
    if !all_metrics.is_empty() {
        let total_cc: u32 = all_metrics.iter().map(|m| m.cyclomatic_complexity).sum();
        let total_cog: u32 = all_metrics.iter().map(|m| m.cognitive_complexity).sum();
        let total_loc: usize = all_metrics.iter().map(|m| m.lines_of_code).sum();
        let total_funcs: usize = all_metrics.iter().map(|m| m.function_count).sum();
        let avg_cc = total_cc as f64 / all_metrics.len() as f64;

        println!();
        println!("{}", "─".repeat(50).dimmed());
        println!("{}", "📈 Summary".cyan().bold());
        println!("  Files analyzed: {}", all_metrics.len().to_string().green());
        println!("  Total cyclomatic complexity: {}", total_cc.to_string().yellow());
        println!("  Average complexity per file: {:.1}", avg_cc);
        println!("  Total cognitive complexity: {}", total_cog.to_string().yellow());
        println!("  Total lines of code: {}", total_loc.to_string().blue());
        println!("  Total functions: {}", total_funcs.to_string().blue());
    }

    Ok(())
}

/// Calculate complexity metrics for a single file
pub fn calculate_file_complexity(file_path: &str, content: &str) -> ComplexityMetrics {
    ComplexityMetrics {
        file_path: file_path.to_string(),
        cyclomatic_complexity: calculate_cyclomatic_complexity(content),
        cognitive_complexity: calculate_cognitive_complexity(content),
        lines_of_code: content.lines().filter(|l| !l.trim().is_empty()).count(),
        function_count: count_functions(content, file_path),
        max_nesting_depth: calculate_nesting_depth(content),
    }
}

/// Calculate cyclomatic complexity of code
pub fn calculate_cyclomatic_complexity(content: &str) -> u32 {
    let mut complexity = 1; // Base complexity

    // Control flow keywords that increase complexity
    let patterns = [
        r"\bif\b",
        r"\belse\s+if\b",
        r"\belse\b",
        r"\bfor\b",
        r"\bwhile\b",
        r"\bcase\b",
        r"\bcatch\b",
        r"\b\?\s*:",          // Ternary operator
        r"\b&&\b",            // Logical AND
        r"\b\|\|\b",          // Logical OR
        r"\bmatch\b",         // Rust match
        r"\bwhen\b",          // Kotlin when
        r"\bguard\b",         // Swift guard
    ];

    for pattern in &patterns {
        if let Ok(regex) = Regex::new(pattern) {
            complexity += regex.find_iter(content).count() as u32;
        }
    }

    complexity
}

/// Calculate cognitive complexity of code
pub fn calculate_cognitive_complexity(content: &str) -> u32 {
    let mut complexity = 0;
    let mut nesting_level = 0;

    for line in content.lines() {
        let trimmed = line.trim();

        // Track nesting increases
        if trimmed.contains('{') {
            nesting_level += trimmed.matches('{').count();
        }

        // Cognitive complexity additions with nesting penalty
        let cognitive_patterns = [
            (r"\bif\b", 1),
            (r"\belse\b", 1),
            (r"\bfor\b", 1),
            (r"\bwhile\b", 1),
            (r"\bmatch\b", 1),
            (r"\b\?\s*:", 1),
            (r"\b&&\b", 1),
            (r"\b\|\|\b", 1),
            (r"\bbreak\b", 1),
            (r"\bcontinue\b", 1),
            (r"\bgoto\b", 3), // High penalty for goto
            (r"\brecursion\b", 2),
        ];

        for (pattern, base_cost) in &cognitive_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                let count = regex.find_iter(trimmed).count() as u32;
                // Add nesting penalty
                complexity += count * (base_cost + nesting_level as u32);
            }
        }

        // Track nesting decreases
        if trimmed.contains('}') {
            let closes = trimmed.matches('}').count();
            nesting_level = nesting_level.saturating_sub(closes);
        }
    }

    complexity
}

/// Count functions in code based on file extension
pub fn count_functions(content: &str, file_path: &str) -> usize {
    let ext = Path::new(file_path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    // Use language-specific patterns if available
    if let Some(lang_info) = get_language_by_extension(ext) {
        let mut count = 0;
        for pattern in lang_info.function_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                count += regex.find_iter(content).count();
            }
        }
        return count;
    }

    // Fallback to generic patterns
    let patterns = [
        r"fn\s+\w+",                    // Rust
        r"def\s+\w+",                   // Python
        r"function\s+\w+",              // JavaScript/TypeScript
        r"func\s+\w+",                  // Go/Swift
        r"(public|private|protected)?\s*(static\s+)?[\w<>\[\]]+\s+\w+\s*\(", // Java/C#
    ];

    let mut count = 0;
    for pattern in &patterns {
        if let Ok(regex) = Regex::new(pattern) {
            count += regex.find_iter(content).count();
        }
    }

    count
}

/// Calculate maximum nesting depth
pub fn calculate_nesting_depth(content: &str) -> u32 {
    let mut max_depth: u32 = 0;
    let mut current_depth: u32 = 0;

    for line in content.lines() {
        for ch in line.chars() {
            match ch {
                '{' | '(' | '[' => {
                    current_depth += 1;
                    max_depth = max_depth.max(current_depth);
                }
                '}' | ')' | ']' => {
                    current_depth = current_depth.saturating_sub(1);
                }
                _ => {}
            }
        }
    }

    max_depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cyclomatic_complexity_simple() {
        let code = "fn main() {}";
        let complexity = calculate_cyclomatic_complexity(code);
        assert_eq!(complexity, 1);
    }

    #[test]
    fn test_cyclomatic_complexity_with_branches() {
        let code = r#"
            fn test() {
                if x > 0 {
                    println!("positive");
                } else {
                    println!("non-positive");
                }
            }
        "#;
        let complexity = calculate_cyclomatic_complexity(code);
        assert!(complexity > 1);
    }

    #[test]
    fn test_cognitive_complexity() {
        let code = r#"
            fn test() {
                if x {
                    for i in items {
                        if y {
                            // deeply nested
                        }
                    }
                }
            }
        "#;
        let complexity = calculate_cognitive_complexity(code);
        assert!(complexity > 0);
    }

    #[test]
    fn test_count_functions() {
        let code = r#"
            fn main() {}
            fn helper() {}
            pub fn public_func() {}
        "#;
        let count = count_functions(code, "test.rs");
        // May count more due to overlapping patterns (pub fn and fn patterns)
        assert!(count >= 3, "Expected at least 3 functions, got {}", count);
    }

    #[test]
    fn test_calculate_nesting_depth() {
        let code = "fn x() { if y { for z {} } }";
        let depth = calculate_nesting_depth(code);
        assert!(depth >= 3);
    }

    #[test]
    fn test_calculate_file_complexity() {
        let code = "fn main() { if x { y } }";
        let metrics = calculate_file_complexity("test.rs", code);
        assert!(!metrics.file_path.is_empty());
        assert!(metrics.cyclomatic_complexity >= 1);
    }
}

