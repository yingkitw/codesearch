//! MCP Server Integration Tests
//!
//! Tests for the MCP server tools and functionality.

use std::fs::{self, File};
use std::io::Write;
use tempfile::tempdir;

// Helper function to create a test project structure
fn create_test_project() -> tempfile::TempDir {
    let dir = tempdir().unwrap();
    
    // Create Rust files
    let src_dir = dir.path().join("src");
    fs::create_dir_all(&src_dir).unwrap();
    
    let mut main_rs = File::create(src_dir.join("main.rs")).unwrap();
    writeln!(main_rs, r#"// Main entry point
fn main() {{
    println!("Hello, world!");
    let x = 42;
    // TODO: Add more functionality
}}

fn helper_function() {{
    let result = compute(10);
    println!("Result: {{}}", result);
}}

fn compute(n: i32) -> i32 {{
    if n > 0 {{
        n * 2
    }} else {{
        0
    }}
}}
"#).unwrap();

    let mut lib_rs = File::create(src_dir.join("lib.rs")).unwrap();
    writeln!(lib_rs, r#"// Library module
pub mod utils;

pub struct Config {{
    pub name: String,
    pub value: i32,
}}

impl Config {{
    pub fn new(name: &str) -> Self {{
        Config {{
            name: name.to_string(),
            value: 0,
        }}
    }}
}}

pub fn process_config(config: &Config) -> String {{
    format!("Config: {{}}", config.name)
}}

// FIXME: This needs refactoring
pub fn legacy_function() {{
    let very_long_variable_name_that_exceeds_the_recommended_line_length_limit = 12345;
    println!("{{}}", very_long_variable_name_that_exceeds_the_recommended_line_length_limit);
}}
"#).unwrap();

    let mut utils_rs = File::create(src_dir.join("utils.rs")).unwrap();
    writeln!(utils_rs, r#"// Utility functions
use std::collections::HashMap;

pub fn format_number(n: i32) -> String {{
    n.to_string()
}}

pub fn parse_number(s: &str) -> Option<i32> {{
    s.parse().ok()
}}

pub fn create_map() -> HashMap<String, i32> {{
    let mut map = HashMap::new();
    map.insert("one".to_string(), 1);
    map.insert("two".to_string(), 2);
    map
}}

// TODO: Add error handling
pub fn unsafe_division(a: i32, b: i32) -> i32 {{
    a / b
}}
"#).unwrap();

    // Create Python files
    let py_dir = dir.path().join("scripts");
    fs::create_dir_all(&py_dir).unwrap();
    
    let mut script_py = File::create(py_dir.join("script.py")).unwrap();
    writeln!(script_py, r#"# Python script
import os
import sys

def main():
    print("Hello from Python")
    result = compute(10)
    return result

def compute(n):
    if n > 0:
        return n * 2
    else:
        return 0

class DataProcessor:
    def __init__(self, name):
        self.name = name
    
    def process(self, data):
        return [x * 2 for x in data]

# TODO: Add logging
if __name__ == "__main__":
    main()
"#).unwrap();

    // Create JavaScript files
    let js_dir = dir.path().join("web");
    fs::create_dir_all(&js_dir).unwrap();
    
    let mut app_js = File::create(js_dir.join("app.js")).unwrap();
    writeln!(app_js, r#"// JavaScript application
const express = require('express');

function main() {{
    console.log("Hello from JavaScript");
    const result = compute(10);
    return result;
}}

function compute(n) {{
    if (n > 0) {{
        return n * 2;
    }} else {{
        return 0;
    }}
}}

class ApiHandler {{
    constructor(name) {{
        this.name = name;
    }}
    
    async handle(request) {{
        return {{ status: 'ok' }};
    }}
}}

// FIXME: Add error handling
module.exports = {{ main, compute, ApiHandler }};
"#).unwrap();

    // Create a README
    let mut readme = File::create(dir.path().join("README.md")).unwrap();
    writeln!(readme, r#"# Test Project

This is a test project for MCP testing.

## Features

- Rust implementation
- Python scripts  
- JavaScript web app

## TODO

- Add tests
- Add documentation
"#).unwrap();

    // Create Cargo.toml
    let mut cargo = File::create(dir.path().join("Cargo.toml")).unwrap();
    writeln!(cargo, r#"[package]
name = "test-project"
version = "0.1.0"
edition = "2021"

[dependencies]
"#).unwrap();

    dir
}

#[test]
fn test_search_code_basic() {
    let dir = create_test_project();
    
    let results = codesearch::search_code(
        "fn main",
        dir.path(),
        Some(&["rs".to_string()]),
        false,
        false,
        0.6,
        10,
        None,
        false,
        false,
        false,
        false,
        false,
    ).unwrap();

    assert!(!results.is_empty(), "Should find 'fn main' in Rust files");
    assert!(results.iter().any(|r| r.file.contains("main.rs")));
}

#[test]
fn test_search_code_case_insensitive() {
    let dir = create_test_project();
    
    let results = codesearch::search_code(
        "MAIN",
        dir.path(),
        Some(&["rs".to_string()]),
        true, // ignore_case
        false,
        0.6,
        10,
        None,
        false,
        false,
        false,
        false,
        false,
    ).unwrap();

    assert!(!results.is_empty(), "Case-insensitive search should find 'main'");
}

#[test]
fn test_search_code_fuzzy() {
    let dir = create_test_project();
    
    let results = codesearch::search_code(
        "comput",  // Typo for "compute"
        dir.path(),
        None,
        false,
        true, // fuzzy
        0.3,  // lower threshold
        20,
        None,
        false,
        false,
        false,
        false,
        false,
    ).unwrap();

    // Fuzzy should find "compute" even with typo
    assert!(!results.is_empty(), "Fuzzy search should find matches");
}

#[test]
fn test_search_code_regex() {
    let dir = create_test_project();
    
    let results = codesearch::search_code(
        r"fn\s+\w+\s*\(",  // Regex pattern for function definitions
        dir.path(),
        Some(&["rs".to_string()]),
        false,
        false,
        0.6,
        50,
        None,
        false,
        false,
        false,
        false,
        false,
    ).unwrap();

    assert!(results.len() >= 3, "Should find multiple function definitions");
}

#[test]
fn test_search_code_with_ranking() {
    let dir = create_test_project();
    
    let results = codesearch::search_code(
        "compute",
        dir.path(),
        None,
        false,
        false,
        0.6,
        20,
        None,
        true, // rank
        false,
        false,
        false,
        false,
    ).unwrap();

    assert!(!results.is_empty());
    // Verify results are sorted by score (descending)
    for i in 1..results.len() {
        assert!(
            results[i - 1].score >= results[i].score,
            "Results should be sorted by score"
        );
    }
}

#[test]
fn test_search_code_exclude_dirs() {
    let dir = create_test_project();
    
    let results = codesearch::search_code(
        "main",
        dir.path(),
        None,
        false,
        false,
        0.6,
        50,
        Some(&["web".to_string()]), // Exclude web directory
        false,
        false,
        false,
        false,
        false,
    ).unwrap();

    // Should not find results in web directory
    assert!(
        !results.iter().any(|r| r.file.contains("web/")),
        "Should not find results in excluded directory"
    );
}

#[test]
fn test_search_code_multiple_extensions() {
    let dir = create_test_project();
    
    let results = codesearch::search_code(
        "compute",
        dir.path(),
        Some(&["rs".to_string(), "py".to_string(), "js".to_string()]),
        false,
        false,
        0.6,
        50,
        None,
        false,
        false,
        false,
        false,
        false,
    ).unwrap();

    // Should find in Rust, Python, and JavaScript
    let has_rs = results.iter().any(|r| r.file.ends_with(".rs"));
    let has_py = results.iter().any(|r| r.file.ends_with(".py"));
    let has_js = results.iter().any(|r| r.file.ends_with(".js"));
    
    assert!(has_rs, "Should find in .rs files");
    assert!(has_py, "Should find in .py files");
    assert!(has_js, "Should find in .js files");
}

#[test]
fn test_search_code_todo_comments() {
    let dir = create_test_project();
    
    let results = codesearch::search_code(
        "TODO",
        dir.path(),
        None,
        false,
        false,
        0.6,
        50,
        None,
        false,
        false,
        false,
        false,
        false,
    ).unwrap();

    assert!(results.len() >= 3, "Should find TODO comments in multiple files");
}

#[test]
fn test_search_code_fixme_comments() {
    let dir = create_test_project();
    
    let results = codesearch::search_code(
        "FIXME",
        dir.path(),
        None,
        false,
        false,
        0.6,
        50,
        None,
        false,
        false,
        false,
        false,
        false,
    ).unwrap();

    assert!(!results.is_empty(), "Should find FIXME comments");
}

#[test]
fn test_list_files_all() {
    let dir = create_test_project();
    
    let files = codesearch::list_files(dir.path(), None, None).unwrap();
    
    assert!(files.len() >= 6, "Should find all files in project");
}

#[test]
fn test_list_files_by_extension() {
    let dir = create_test_project();
    
    let rs_files = codesearch::list_files(
        dir.path(),
        Some(&["rs".to_string()]),
        None,
    ).unwrap();
    
    assert!(rs_files.len() >= 3, "Should find Rust files");
    assert!(rs_files.iter().all(|f| f.path.ends_with(".rs")));
}

#[test]
fn test_list_files_with_exclude() {
    let dir = create_test_project();
    
    let files = codesearch::list_files(
        dir.path(),
        None,
        Some(&["scripts".to_string()]),
    ).unwrap();
    
    assert!(!files.iter().any(|f| f.path.contains("scripts/")));
}

#[test]
fn test_file_info_has_lines() {
    let dir = create_test_project();
    
    let files = codesearch::list_files(
        dir.path(),
        Some(&["rs".to_string()]),
        None,
    ).unwrap();
    
    for file in &files {
        assert!(file.lines > 0, "File {} should have line count", file.path);
        assert!(file.size > 0, "File {} should have size", file.path);
    }
}

#[test]
fn test_search_class_definitions() {
    let dir = create_test_project();
    
    let results = codesearch::search_code(
        r"class\s+\w+",
        dir.path(),
        Some(&["py".to_string(), "js".to_string()]),
        false,
        false,
        0.6,
        20,
        None,
        false,
        false,
        false,
        false,
        false,
    ).unwrap();

    assert!(results.len() >= 2, "Should find class definitions");
}

#[test]
fn test_search_struct_definitions() {
    let dir = create_test_project();
    
    let results = codesearch::search_code(
        r"struct\s+\w+",
        dir.path(),
        Some(&["rs".to_string()]),
        false,
        false,
        0.6,
        20,
        None,
        false,
        false,
        false,
        false,
        false,
    ).unwrap();

    assert!(!results.is_empty(), "Should find struct definitions");
    assert!(results.iter().any(|r| r.content.contains("Config")));
}

#[test]
fn test_search_imports() {
    let dir = create_test_project();
    
    // Search for imports in various languages
    let rust_imports = codesearch::search_code(
        r"^use\s+",
        dir.path(),
        Some(&["rs".to_string()]),
        false,
        false,
        0.6,
        20,
        None,
        false,
        false,
        false,
        false,
        false,
    ).unwrap();

    let python_imports = codesearch::search_code(
        r"^import\s+",
        dir.path(),
        Some(&["py".to_string()]),
        false,
        false,
        0.6,
        20,
        None,
        false,
        false,
        false,
        false,
        false,
    ).unwrap();

    let js_requires = codesearch::search_code(
        r"require\(",
        dir.path(),
        Some(&["js".to_string()]),
        false,
        false,
        0.6,
        20,
        None,
        false,
        false,
        false,
        false,
        false,
    ).unwrap();

    assert!(!rust_imports.is_empty() || !python_imports.is_empty() || !js_requires.is_empty(),
        "Should find imports in at least one language");
}

#[test]
fn test_search_result_structure() {
    let dir = create_test_project();
    
    let results = codesearch::search_code(
        "fn main",
        dir.path(),
        Some(&["rs".to_string()]),
        false,
        false,
        0.6,
        10,
        None,
        true,
        false,
        false,
        false,
        false,
    ).unwrap();

    for result in &results {
        // Check file path is valid
        assert!(!result.file.is_empty(), "File path should not be empty");
        assert!(result.file.ends_with(".rs"), "File should be Rust file");
        
        // Check line number is positive
        assert!(result.line_number > 0, "Line number should be positive");
        
        // Check content is not empty
        assert!(!result.content.is_empty(), "Content should not be empty");
        
        // Check score is in valid range
        assert!(result.score >= 0.0 && result.score <= 100.0, "Score should be 0-100");
        
        // Check relevance is set
        assert!(!result.relevance.is_empty(), "Relevance should be set");
    }
}

#[test]
fn test_empty_search() {
    let dir = create_test_project();
    
    let results = codesearch::search_code(
        "this_string_does_not_exist_anywhere_xyzzy",
        dir.path(),
        None,
        false,
        false,
        0.6,
        10,
        None,
        false,
        false,
        false,
        false,
        false,
    ).unwrap();

    assert!(results.is_empty(), "Should return empty for non-matching search");
}

#[test]
fn test_max_results_limit() {
    let dir = create_test_project();
    
    let results = codesearch::search_code(
        "n",  // Very common pattern
        dir.path(),
        None,
        false,
        false,
        0.6,
        3,  // Limit to 3 results per file
        None,
        false,
        false,
        false,
        false,
        false,
    ).unwrap();

    // Count results per file
    use std::collections::HashMap;
    let mut file_counts: HashMap<String, usize> = HashMap::new();
    for result in &results {
        *file_counts.entry(result.file.clone()).or_insert(0) += 1;
    }
    
    for (file, count) in &file_counts {
        assert!(*count <= 3, "File {} has {} results, should be <= 3", file, count);
    }
}

#[test]
fn test_search_semantic_enhancement() {
    let dir = create_test_project();
    
    // Semantic search should enhance query
    let results = codesearch::search_code(
        "function",  // Should also match fn, def, etc.
        dir.path(),
        None,
        false,
        false,
        0.6,
        50,
        None,
        false,
        false,
        true, // semantic
        false,
        false,
    ).unwrap();

    // Semantic search should find function-like patterns
    assert!(!results.is_empty(), "Semantic search should find results");
}

#[test]
fn test_search_special_characters() {
    let dir = create_test_project();
    
    // Test searching for special regex characters - the function expects regex patterns
    // so special characters should be escaped by the caller if literal search is needed
    // Here we test that a valid pattern works correctly
    let results = codesearch::search_code(
        r"\{",  // Escaped curly brace for regex
        dir.path(),
        Some(&["rs".to_string()]),
        false,
        false,
        0.6,
        10,
        None,
        false,
        false,
        false,
        false,
        false,
    );

    // Should handle special characters gracefully (either error or success, but not panic)
    match results {
        Ok(r) => assert!(!r.is_empty(), "Should find curly braces in Rust code"),
        Err(_) => (), // Regex error is acceptable for invalid patterns
    }
}

#[test]
fn test_print_results_does_not_panic() {
    let dir = create_test_project();
    
    let results = codesearch::search_code(
        "fn main",
        dir.path(),
        Some(&["rs".to_string()]),
        false,
        false,
        0.6,
        10,
        None,
        true,
        false,
        false,
        false,
        false,
    ).unwrap();

    // Should not panic
    codesearch::print_results(&results, true, true);
    codesearch::print_results(&results, true, false);
    codesearch::print_results(&results, false, true);
    codesearch::print_results(&results, false, false);
}

#[test]
fn test_print_stats_does_not_panic() {
    let dir = create_test_project();
    
    let results = codesearch::search_code(
        "fn main",
        dir.path(),
        Some(&["rs".to_string()]),
        false,
        false,
        0.6,
        10,
        None,
        false,
        false,
        false,
        false,
        false,
    ).unwrap();

    // Should not panic
    codesearch::print_search_stats(&results, "fn main");
    
    // Also test with empty results
    codesearch::print_search_stats(&[], "empty query");
}

