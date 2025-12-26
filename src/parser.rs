//! Code Parser Utilities Module
//!
//! Provides shared utilities for parsing code files, extracting functions, and common operations.
//! This module ensures DRY, KISS, and SOC principles.

use crate::language::get_language_by_extension;
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// Read file content, returning empty string on error
pub fn read_file_content(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap_or_default()
}

/// Extract file extension from path
pub fn get_file_extension(file_path: &str) -> &str {
    Path::new(file_path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
}

/// Check if a name is a keyword or builtin function
pub fn is_keyword_or_builtin(name: &str) -> bool {
    matches!(
        name,
        "if" | "else" | "for" | "while" | "loop" | "match" | "switch" | "case"
            | "return" | "break" | "continue" | "fn" | "function" | "def" | "func"
            | "class" | "struct" | "impl" | "trait" | "interface" | "enum" | "type"
            | "let" | "const" | "var" | "mut" | "pub" | "public" | "private" | "protected"
            | "static" | "async" | "await" | "try" | "catch" | "throw" | "new"
            | "import" | "export" | "use" | "from" | "require" | "include"
            | "true" | "false" | "null" | "None" | "nil" | "undefined"
            | "self" | "this" | "super" | "println" | "print" | "printf" | "console"
            | "String" | "Vec" | "Option" | "Result" | "Ok" | "Err" | "Some"
            | "len" | "append" | "push" | "pop" | "get" | "set" | "map" | "filter"
    )
}

/// Extract identifier from regex captures (first capture group)
pub fn extract_identifier_from_match(caps: &regex::Captures) -> Option<String> {
    caps.get(1)
        .map(|m| m.as_str().to_string())
        .filter(|name| !is_keyword_or_builtin(name))
}

/// Extract function definitions from content using language patterns
pub fn extract_functions(
    content: &str,
    file_path: &str,
) -> Vec<(String, usize)> {
    let ext = get_file_extension(file_path);
    let mut functions = Vec::new();

    if let Some(lang) = get_language_by_extension(ext) {
        for pattern in lang.function_patterns {
            if let Ok(re) = Regex::new(pattern) {
                for (line_num, line) in content.lines().enumerate() {
                    if let Some(caps) = re.captures(line) {
                        if let Some(name) = extract_identifier_from_match(&caps) {
                            functions.push((name, line_num + 1));
                        }
                    }
                }
            }
        }
    }

    functions
}

/// Extract class/struct definitions from content using language patterns
pub fn extract_classes(
    content: &str,
    file_path: &str,
) -> Vec<(String, usize)> {
    let ext = get_file_extension(file_path);
    let mut classes = Vec::new();

    if let Some(lang) = get_language_by_extension(ext) {
        for pattern in lang.class_patterns {
            if let Ok(re) = Regex::new(pattern) {
                for (line_num, line) in content.lines().enumerate() {
                    if let Some(caps) = re.captures(line) {
                        if let Some(name) = extract_identifier_from_match(&caps) {
                            classes.push((name, line_num + 1));
                        }
                    }
                }
            }
        }
    }

    classes
}

/// Extract function calls from a line of code
pub fn extract_function_calls(line: &str) -> HashSet<String> {
    let call_pattern = Regex::new(r"\b([a-zA-Z_][a-zA-Z0-9_]*)\s*\(").unwrap();
    let mut calls = HashSet::new();

    for cap in call_pattern.captures_iter(line) {
        if let Some(name) = cap.get(1) {
            let called = name.as_str().to_string();
            if !is_keyword_or_builtin(&called) {
                calls.insert(called);
            }
        }
    }

    calls
}

/// Extract all identifier references from content
pub fn extract_identifier_references(content: &str) -> HashSet<String> {
    let identifier_re = Regex::new(r"\b([a-zA-Z_][a-zA-Z0-9_]*)\b").unwrap();
    let mut references = HashSet::new();

    for cap in identifier_re.captures_iter(content) {
        if let Some(name) = cap.get(1) {
            let ref_name = name.as_str().to_string();
            if !is_keyword_or_builtin(&ref_name) {
                references.insert(ref_name);
            }
        }
    }

    references
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file_content() {
        let content = read_file_content("nonexistent.rs");
        assert_eq!(content, "");
    }

    #[test]
    fn test_get_file_extension() {
        assert_eq!(get_file_extension("test.rs"), "rs");
        assert_eq!(get_file_extension("test.py"), "py");
        assert_eq!(get_file_extension("test"), "");
    }

    #[test]
    fn test_is_keyword_or_builtin() {
        assert!(is_keyword_or_builtin("if"));
        assert!(is_keyword_or_builtin("function"));
        assert!(!is_keyword_or_builtin("myFunction"));
    }

    #[test]
    fn test_extract_function_calls() {
        let line = "let result = calculate(x) + process(y);";
        let calls = extract_function_calls(line);
        assert!(calls.contains("calculate"));
        assert!(calls.contains("process"));
        assert!(!calls.contains("let"));
    }

    #[test]
    fn test_extract_identifier_references() {
        let content = "let x = 10; let y = x + 5;";
        let refs = extract_identifier_references(content);
        assert!(refs.contains("x"));
        assert!(refs.contains("y"));
        assert!(!refs.contains("let"));
    }
}

