//! Codebase Analysis Module
//!
//! Provides metrics and statistics about the codebase.

use crate::language::{get_language_by_extension, get_supported_languages};
use crate::search::list_files;
use crate::types::RefactorSuggestion;
use colored::*;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Analyze codebase and print metrics
pub fn analyze_codebase(
    path: &Path,
    extensions: Option<&[String]>,
    exclude: Option<&[String]>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "📊 Codebase Analysis".cyan().bold());
    println!("{}", "─".repeat(30).cyan());
    println!();

    let files = list_files(path, extensions, exclude)?;

    let mut total_lines = 0;
    let mut total_size = 0;
    let mut language_stats: HashMap<String, (usize, usize, u64)> = HashMap::new(); // ext -> (files, lines, bytes)
    let mut function_count = 0;
    let mut class_count = 0;
    let mut comment_lines = 0;

    for file in &files {
        total_lines += file.lines;
        total_size += file.size;

        // Get extension
        let ext = Path::new(&file.path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("unknown")
            .to_string();

        let entry = language_stats.entry(ext.clone()).or_insert((0, 0, 0));
        entry.0 += 1;
        entry.1 += file.lines;
        entry.2 += file.size;

        // Count patterns
        if let Ok(content) = fs::read_to_string(&file.path) {
            if let Some(lang_info) = get_language_by_extension(&ext) {
                // Count functions
                for pattern in lang_info.function_patterns {
                    if let Ok(regex) = Regex::new(pattern) {
                        function_count += regex.find_iter(&content).count();
                    }
                }

                // Count classes/structures
                for pattern in lang_info.class_patterns {
                    if let Ok(regex) = Regex::new(pattern) {
                        class_count += regex.find_iter(&content).count();
                    }
                }

                // Count comment lines
                for pattern in lang_info.comment_patterns {
                    if let Ok(regex) = Regex::new(pattern) {
                        comment_lines += regex.find_iter(&content).count();
                    }
                }
            } else {
                // Fallback for unknown languages
                function_count += count_generic_functions(&content);
                class_count += count_generic_classes(&content);
                comment_lines += count_generic_comments(&content);
            }
        }
    }

    // Print overall stats
    println!("{}", "📁 Overview".yellow().bold());
    println!("  Total files: {}", files.len().to_string().green());
    println!("  Total lines: {}", total_lines.to_string().green());
    println!("  Total size: {}", format_size(total_size).green());
    println!();

    // Print language breakdown
    println!("{}", "🗂️  Languages".yellow().bold());
    let mut lang_vec: Vec<_> = language_stats.iter().collect();
    lang_vec.sort_by(|a, b| b.1 .1.cmp(&a.1 .1));

    for (ext, (file_count, lines, bytes)) in lang_vec.iter().take(10) {
        let lang_name = get_language_by_extension(ext)
            .map(|l| l.name)
            .unwrap_or(*ext);
        println!(
            "  {} {}: {} files, {} lines ({})",
            "•".dimmed(),
            lang_name.cyan(),
            file_count.to_string().yellow(),
            lines.to_string().green(),
            format_size(*bytes).dimmed()
        );
    }
    println!();

    // Print code patterns
    println!("{}", "📝 Code Patterns".yellow().bold());
    println!("  Functions/Methods: {}", function_count.to_string().green());
    println!("  Classes/Structs: {}", class_count.to_string().green());
    println!("  Comment lines: {}", comment_lines.to_string().green());

    if total_lines > 0 {
        let comment_ratio = (comment_lines as f64 / total_lines as f64) * 100.0;
        println!("  Comment ratio: {:.1}%", comment_ratio);
    }

    println!();
    println!("{}", "✨ Analysis complete!".green().italic());

    Ok(())
}

fn count_generic_functions(content: &str) -> usize {
    let patterns = [
        r"fn\s+\w+",
        r"def\s+\w+",
        r"function\s+\w+",
        r"func\s+\w+",
    ];

    let mut count = 0;
    for pattern in &patterns {
        if let Ok(regex) = Regex::new(pattern) {
            count += regex.find_iter(content).count();
        }
    }
    count
}

fn count_generic_classes(content: &str) -> usize {
    let patterns = [
        r"class\s+\w+",
        r"struct\s+\w+",
        r"interface\s+\w+",
        r"trait\s+\w+",
    ];

    let mut count = 0;
    for pattern in &patterns {
        if let Ok(regex) = Regex::new(pattern) {
            count += regex.find_iter(content).count();
        }
    }
    count
}

fn count_generic_comments(content: &str) -> usize {
    content
        .lines()
        .filter(|l| {
            let trimmed = l.trim();
            trimmed.starts_with("//")
                || trimmed.starts_with('#')
                || trimmed.starts_with("/*")
                || trimmed.starts_with("*")
        })
        .count()
}

fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}

/// Suggest refactoring improvements for the codebase
pub fn suggest_refactoring(
    path: &Path,
    extensions: Option<&[String]>,
    exclude: Option<&[String]>,
    high_priority_only: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "🔧 Code Refactoring Suggestions".cyan().bold());
    println!("{}", "─".repeat(30).cyan());
    println!();

    let files = list_files(path, extensions, exclude)?;
    let mut suggestions = Vec::new();

    for file in &files {
        if let Ok(content) = fs::read_to_string(&file.path) {
            analyze_file_for_refactoring(&file.path, &content, &mut suggestions);
        }
    }

    // Sort by priority (highest first)
    suggestions.sort_by(|a, b| b.priority.cmp(&a.priority));

    // Filter by priority if requested
    let filtered_suggestions = if high_priority_only {
        suggestions.into_iter().filter(|s| s.priority >= 7).collect::<Vec<_>>()
    } else {
        suggestions
    };

    if filtered_suggestions.is_empty() {
        println!("{}", "✨ No refactoring suggestions found! Your code looks good.".green().italic());
        return Ok(());
    }

    // Group suggestions by type
    let mut grouped: HashMap<String, Vec<&RefactorSuggestion>> = HashMap::new();
    for suggestion in &filtered_suggestions {
        grouped.entry(suggestion.suggestion_type.clone()).or_default().push(suggestion);
    }

    for (suggestion_type, type_suggestions) in grouped {
        println!("{}", format!("📋 {} ({})", suggestion_type, type_suggestions.len()).yellow().bold());
        println!("{}", "─".repeat(suggestion_type.len() + 15).yellow());

        for suggestion in type_suggestions {
            let priority_color = match suggestion.priority {
                8..=10 => "red",
                5..=7 => "yellow",
                _ => "green",
            };

            println!(
                "  {} {} {}",
                format!("[{}]", suggestion.priority).color(priority_color).bold(),
                suggestion.file.blue().bold(),
                format!("line {}", suggestion.line_number).cyan()
            );
            println!("  {}", suggestion.description.italic());
            println!("  {} {}", "Current:".dimmed(), suggestion.code_snippet.dimmed());
            println!("  {} {}", "Better:".green(), suggestion.improvement.green());
            println!();
        }
    }

    println!("{}", format!("💡 Total suggestions: {}", filtered_suggestions.len()).cyan().bold());
    println!("{}", "✨ Refactoring analysis completed!".green().italic());

    Ok(())
}

/// Analyze a file for refactoring opportunities
pub fn analyze_file_for_refactoring(
    file_path: &str,
    content: &str,
    suggestions: &mut Vec<RefactorSuggestion>,
) {
    let lines: Vec<&str> = content.lines().collect();

    for (i, line) in lines.iter().enumerate() {
        let line_num = i + 1;
        let trimmed = line.trim();

        // Long lines (>100 characters)
        if line.len() > 100 {
            suggestions.push(RefactorSuggestion {
                file: file_path.to_string(),
                line_number: line_num,
                suggestion_type: "Long Line".to_string(),
                description: "Line exceeds 100 characters".to_string(),
                priority: 3,
                code_snippet: trimmed.chars().take(50).collect::<String>() + "...",
                improvement: "Break into multiple lines".to_string(),
            });
        }

        // TODO comments
        if trimmed.contains("TODO") || trimmed.contains("FIXME") || trimmed.contains("HACK") {
            suggestions.push(RefactorSuggestion {
                file: file_path.to_string(),
                line_number: line_num,
                suggestion_type: "Todo Comment".to_string(),
                description: "Unfinished work marker found".to_string(),
                priority: 5,
                code_snippet: trimmed.to_string(),
                improvement: "Complete or remove the TODO".to_string(),
            });
        }

        // Magic numbers
        let magic_regex = Regex::new(r"[^\w](\d{2,})[^\w]").unwrap();
        if magic_regex.is_match(trimmed) && !trimmed.contains("//") {
            suggestions.push(RefactorSuggestion {
                file: file_path.to_string(),
                line_number: line_num,
                suggestion_type: "Magic Number".to_string(),
                description: "Consider extracting to a named constant".to_string(),
                priority: 4,
                code_snippet: trimmed.to_string(),
                improvement: "Use a named constant".to_string(),
            });
        }

        // Deep nesting (multiple levels of indentation)
        let indent_level = line.len() - line.trim_start().len();
        if indent_level > 16 {
            suggestions.push(RefactorSuggestion {
                file: file_path.to_string(),
                line_number: line_num,
                suggestion_type: "Deep Nesting".to_string(),
                description: "Code is deeply nested (>4 levels)".to_string(),
                priority: 7,
                code_snippet: trimmed.chars().take(50).collect::<String>(),
                improvement: "Extract to separate functions or use early returns".to_string(),
            });
        }

        // Empty catch blocks
        if trimmed.contains("catch") && i + 1 < lines.len() {
            let next_line = lines[i + 1].trim();
            if next_line == "}" || next_line.is_empty() {
                suggestions.push(RefactorSuggestion {
                    file: file_path.to_string(),
                    line_number: line_num,
                    suggestion_type: "Empty Catch Block".to_string(),
                    description: "Empty catch block swallows errors".to_string(),
                    priority: 8,
                    code_snippet: trimmed.to_string(),
                    improvement: "Handle or log the error".to_string(),
                });
            }
        }
    }
}

/// List all supported programming languages
pub fn list_supported_languages() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "🌐 Supported Programming Languages".cyan().bold());
    println!("{}", "─".repeat(35).cyan());
    println!();

    let languages = get_supported_languages();

    // Categorize languages
    let categories: Vec<(&str, Vec<&str>)> = vec![
        ("Systems", vec!["Rust", "C", "C++", "Zig", "V", "Nim"]),
        ("Web", vec!["JavaScript", "TypeScript", "PHP", "CSS", "XML/HTML"]),
        ("Backend", vec!["Python", "Java", "Go", "Kotlin", "C#", "Ruby", "Scala"]),
        ("Mobile", vec!["Swift", "Objective-C", "Dart"]),
        ("Functional", vec!["Haskell", "Elixir", "Erlang", "Clojure", "OCaml", "F#"]),
        ("Scripting", vec!["Shell", "PowerShell", "Lua", "Perl", "R", "Julia"]),
        ("Data/Config", vec!["SQL", "YAML", "TOML", "JSON", "GraphQL", "Protobuf"]),
        ("Infrastructure", vec!["Dockerfile", "Terraform", "Makefile"]),
        ("Other", vec!["Assembly", "Groovy", "Crystal", "Solidity", "WebAssembly", "Markdown"]),
    ];

    for (category, category_langs) in &categories {
        println!("{}", format!("📁 {}", category).yellow().bold());
        for lang_name in category_langs {
            if let Some(lang) = languages.iter().find(|l| l.name == *lang_name) {
                let exts = lang.extensions.join(", ");
                println!(
                    "   {} {} ({})",
                    "•".dimmed(),
                    lang.name.green(),
                    exts.dimmed()
                );
            }
        }
        println!();
    }

    // Count total
    let total = categories.iter().flat_map(|(_, langs)| langs.iter()).count();
    println!("{}", format!("📊 Total: {} languages supported", total).cyan().bold());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size() {
        assert!(format_size(500).contains("bytes"));
        assert!(format_size(1500).contains("KB"));
        assert!(format_size(1_500_000).contains("MB"));
    }

    #[test]
    fn test_count_generic_functions() {
        let code = "fn main() {} def test(): pass function helper() {}";
        assert!(count_generic_functions(code) >= 3);
    }

    #[test]
    fn test_count_generic_classes() {
        let code = "class Foo {} struct Bar {} interface Baz {}";
        assert!(count_generic_classes(code) >= 3);
    }

    #[test]
    fn test_analyze_file_for_refactoring() {
        let content = "// TODO: fix this\nlet x = 12345;";
        let mut suggestions = Vec::new();
        analyze_file_for_refactoring("test.rs", content, &mut suggestions);
        assert!(!suggestions.is_empty());
    }
}

