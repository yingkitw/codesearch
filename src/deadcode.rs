//! Dead Code Detection Module
//!
//! Identifies potentially unused code: functions, classes, imports, and commented-out code.

use crate::language::get_language_by_extension;
use crate::search::list_files;
use colored::*;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Dead code detection result
#[derive(Debug, Clone)]
pub struct DeadCodeItem {
    pub file: String,
    pub line_number: usize,
    pub item_type: String,
    pub name: String,
    pub reason: String,
}

/// Detect potentially dead/unused code in the codebase
pub fn detect_dead_code(
    path: &Path,
    extensions: Option<&[String]>,
    exclude: Option<&[String]>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "Dead Code Detection".cyan().bold());
    println!("{}", "─".repeat(30).cyan());
    println!();

    let files = list_files(path, extensions, exclude)?;
    
    if files.is_empty() {
        println!("{}", "No files found to analyze.".dimmed());
        return Ok(());
    }

    let mut dead_code_items: Vec<DeadCodeItem> = Vec::new();
    let mut all_definitions: HashMap<String, (String, usize, String)> = HashMap::new();
    let mut all_references: HashMap<String, usize> = HashMap::new();

    // First pass: collect all definitions and references
    for file in &files {
        if let Ok(content) = fs::read_to_string(&file.path) {
            let ext = Path::new(&file.path)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("");

            if let Some(lang) = get_language_by_extension(ext) {
                // Extract function definitions
                for pattern in lang.function_patterns {
                    if let Ok(re) = Regex::new(pattern) {
                        for (line_num, line) in content.lines().enumerate() {
                            if let Some(caps) = re.captures(line) {
                                if let Some(name) = extract_identifier_from_match(&caps) {
                                    if !is_special_function(&name) {
                                        all_definitions.insert(
                                            name.clone(),
                                            (file.path.clone(), line_num + 1, "function".to_string()),
                                        );
                                    }
                                }
                            }
                        }
                    }
                }

                // Extract class definitions
                for pattern in lang.class_patterns {
                    if let Ok(re) = Regex::new(pattern) {
                        for (line_num, line) in content.lines().enumerate() {
                            if let Some(caps) = re.captures(line) {
                                if let Some(name) = extract_identifier_from_match(&caps) {
                                    all_definitions.insert(
                                        name.clone(),
                                        (file.path.clone(), line_num + 1, "class/struct".to_string()),
                                    );
                                }
                            }
                        }
                    }
                }
            }

            // Count all identifier references
            let identifier_re = Regex::new(r"\b([a-zA-Z_][a-zA-Z0-9_]*)\b").unwrap();
            for cap in identifier_re.captures_iter(&content) {
                if let Some(name) = cap.get(1) {
                    *all_references.entry(name.as_str().to_string()).or_insert(0) += 1;
                }
            }
        }
    }

    // Second pass: find definitions with low reference count
    for (name, (file, line, item_type)) in &all_definitions {
        let ref_count = all_references.get(name).copied().unwrap_or(0);
        
        if ref_count <= 1 {
            dead_code_items.push(DeadCodeItem {
                file: file.clone(),
                line_number: *line,
                item_type: item_type.clone(),
                name: name.clone(),
                reason: "Only defined, never used elsewhere".to_string(),
            });
        } else if ref_count == 2 && item_type == "function" {
            dead_code_items.push(DeadCodeItem {
                file: file.clone(),
                line_number: *line,
                item_type: item_type.clone(),
                name: name.clone(),
                reason: "Used only once - consider inlining".to_string(),
            });
        }
    }

    // Third pass: detect other dead code patterns
    for file in &files {
        if let Ok(content) = fs::read_to_string(&file.path) {
            detect_dead_code_patterns(&file.path, &content, &mut dead_code_items);
        }
    }

    // Sort by file and line number
    dead_code_items.sort_by(|a, b| {
        a.file.cmp(&b.file).then(a.line_number.cmp(&b.line_number))
    });

    // Print results
    print_dead_code_results(&dead_code_items);

    Ok(())
}

fn print_dead_code_results(items: &[DeadCodeItem]) {
    if items.is_empty() {
        println!("{}", "No obvious dead code detected!".green().bold());
    } else {
        println!(
            "{}",
            format!("Found {} potential dead code items:", items.len())
                .yellow()
                .bold()
        );
        println!();

        let mut current_file = String::new();
        for item in items {
            if item.file != current_file {
                current_file = item.file.clone();
                println!("{}", format!("[{}]", current_file).cyan());
            }
            println!(
                "   {} L{}: {} '{}' - {}",
                match item.item_type.as_str() {
                    "function" => "[fn]",
                    "class/struct" => "[cls]",
                    "variable" => "[var]",
                    "import" => "[imp]",
                    _ => "[-]",
                },
                format!("{:4}", item.line_number).yellow(),
                item.item_type.blue(),
                item.name.green(),
                item.reason.dimmed()
            );
        }

        println!();
        println!("{}", "Summary:".cyan().bold());
        
        let mut type_counts: HashMap<String, usize> = HashMap::new();
        for item in items {
            *type_counts.entry(item.item_type.clone()).or_insert(0) += 1;
        }
        
        for (item_type, count) in &type_counts {
            println!("   {} {}: {}", "•".dimmed(), item_type, count);
        }
    }
}

fn extract_identifier_from_match(caps: &regex::Captures) -> Option<String> {
    for i in 1..caps.len() {
        if let Some(m) = caps.get(i) {
            let s = m.as_str().trim();
            if !s.is_empty() && !is_keyword(s) && s.chars().next().map(|c| c.is_alphabetic() || c == '_').unwrap_or(false) {
                return Some(s.to_string());
            }
        }
    }
    None
}

fn is_special_function(name: &str) -> bool {
    matches!(
        name,
        "main" | "new" | "default" | "init" | "__init__" | "setup" | "teardown"
            | "test" | "run" | "start" | "stop" | "get" | "set" | "from" | "into"
    ) || name.starts_with("test_")
      || name.starts_with("Test")
}

fn is_keyword(s: &str) -> bool {
    matches!(
        s,
        "fn" | "function" | "def" | "func" | "pub" | "public" | "private" | "protected"
            | "static" | "async" | "await" | "class" | "struct" | "impl" | "trait"
            | "interface" | "enum" | "type" | "let" | "const" | "var" | "mut"
            | "if" | "else" | "for" | "while" | "loop" | "match" | "switch" | "case"
            | "return" | "break" | "continue" | "true" | "false" | "null" | "None"
            | "self" | "this" | "super" | "import" | "export" | "use" | "from"
    )
}

fn detect_dead_code_patterns(file_path: &str, content: &str, items: &mut Vec<DeadCodeItem>) {
    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        
        // Detect commented-out code
        if is_commented_out_code(trimmed) {
            items.push(DeadCodeItem {
                file: file_path.to_string(),
                line_number: line_num + 1,
                item_type: "commented code".to_string(),
                name: truncate_string(trimmed, 40),
                reason: "Commented-out code should be removed".to_string(),
            });
        }
        
        // Detect unused imports
        if (trimmed.starts_with("use ") || trimmed.starts_with("import ") || trimmed.starts_with("from ")) 
            && !trimmed.contains('*') 
        {
            if let Some(imported) = extract_import_name(trimmed) {
                let usage_count = content.matches(&imported).count();
                if usage_count <= 1 {
                    items.push(DeadCodeItem {
                        file: file_path.to_string(),
                        line_number: line_num + 1,
                        item_type: "import".to_string(),
                        name: imported,
                        reason: "Imported but never used".to_string(),
                    });
                }
            }
        }
    }
}

fn is_commented_out_code(line: &str) -> bool {
    let (_prefix, rest) = if line.starts_with("// ") {
        ("// ", line.trim_start_matches("// "))
    } else if line.starts_with("//") && !line.starts_with("///") && !line.starts_with("//!") {
        ("//", line.trim_start_matches("//").trim())
    } else if line.starts_with("# ") && !line.starts_with("#!") && !line.starts_with("#[") {
        ("# ", line.trim_start_matches("# "))
    } else {
        return false;
    };
    
    if rest.len() < 5 {
        return false;
    }
    
    if rest.starts_with("TODO") || rest.starts_with("FIXME") || rest.starts_with("NOTE")
        || rest.starts_with("HACK") || rest.starts_with("XXX") || rest.starts_with("WARN")
        || rest.starts_with("@") || rest.starts_with("*") || rest.starts_with("-")
        || rest.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) && !rest.contains('(')
    {
        return false;
    }
    
    let code_indicators = [
        (";", true),
        ("fn (", false), ("def (", false), ("function (", false),
        ("class (", false), ("struct (", false),
        ("let ", true), ("const ", true), ("var ", true),
        ("if (", false), ("for (", false), ("while (", false),
        ("return ", false),
    ];
    
    for (indicator, requires_semicolon) in &code_indicators {
        if rest.contains(indicator) {
            if *requires_semicolon && rest.ends_with(';') {
                return true;
            } else if !requires_semicolon {
                if rest.contains('{') || rest.contains('}') || rest.ends_with(';') || rest.ends_with(':') {
                    return true;
                }
            }
        }
    }
    
    if rest.contains(" = ") && rest.ends_with(';') && !rest.contains("//") {
        return true;
    }
    
    false
}

fn extract_import_name(line: &str) -> Option<String> {
    let line = line.trim_end_matches(';').trim();
    
    if line.starts_with("use ") {
        let path = line.trim_start_matches("use ").trim();
        if let Some(last) = path.split("::").last() {
            let name = last.trim_matches(|c| c == '{' || c == '}');
            if !name.is_empty() && name != "*" && !name.contains(',') {
                return Some(name.to_string());
            }
        }
    } else if line.starts_with("import ") {
        let name = line.trim_start_matches("import ").trim();
        if let Some(first) = name.split_whitespace().next() {
            return Some(first.to_string());
        }
    } else if line.starts_with("from ") && line.contains(" import ") {
        if let Some(after_import) = line.split(" import ").nth(1) {
            let name = after_import.split(',').next().unwrap_or("").trim();
            if !name.is_empty() && name != "*" {
                return Some(name.to_string());
            }
        }
    }
    
    None
}

fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_special_function() {
        assert!(is_special_function("main"));
        assert!(is_special_function("test_something"));
        assert!(!is_special_function("helper"));
    }

    #[test]
    fn test_is_keyword() {
        assert!(is_keyword("fn"));
        assert!(is_keyword("class"));
        assert!(!is_keyword("myFunction"));
    }

    #[test]
    fn test_extract_import_name() {
        assert_eq!(extract_import_name("use std::io::Write;"), Some("Write".to_string()));
        assert_eq!(extract_import_name("import os"), Some("os".to_string()));
        assert_eq!(extract_import_name("from os import path"), Some("path".to_string()));
    }

    #[test]
    fn test_truncate_string() {
        assert_eq!(truncate_string("short", 10), "short");
        assert_eq!(truncate_string("this is a long string", 10), "this is...");
    }
}

