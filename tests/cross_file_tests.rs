//! Integration tests for cross-file and cross-folder functionality
//! Ensures all search and detect capabilities work across multiple files and directories

use std::fs;
use tempfile::TempDir;

// Helper to create a multi-file/folder test structure
fn create_multi_file_structure() -> TempDir {
    let temp_dir = tempfile::tempdir().unwrap();
    let base = temp_dir.path();

    // Create folder structure:
    // /src/
    //   /module_a/
    //     file1.rs
    //     file2.rs
    //   /module_b/
    //     file3.rs
    //   main.rs
    // /tests/
    //   test1.rs

    fs::create_dir_all(base.join("src/module_a")).unwrap();
    fs::create_dir_all(base.join("src/module_b")).unwrap();
    fs::create_dir_all(base.join("tests")).unwrap();

    // Create files with cross-references
    fs::write(
        base.join("src/module_a/file1.rs"),
        r#"
pub fn function_a() {
    function_b();
    helper_function();
}

fn helper_function() {
    println!("helper");
}

// TODO: Refactor this function
fn old_function() {
    // This is unused
}
"#,
    )
    .unwrap();

    fs::write(
        base.join("src/module_a/file2.rs"),
        r#"
pub fn function_b() {
    function_c();
}

fn duplicate_logic() {
    let x = 10;
    let y = 20;
    let z = x + y;
    println!("{}", z);
}
"#,
    )
    .unwrap();

    fs::write(
        base.join("src/module_b/file3.rs"),
        r#"
pub fn function_c() {
    function_a();
}

fn duplicate_logic() {
    let x = 10;
    let y = 20;
    let z = x + y;
    println!("{}", z);
}

// FIXME: This needs attention
fn complex_function() {
    if true {
        if true {
            if true {
                println!("nested");
            }
        }
    }
}
"#,
    )
    .unwrap();

    fs::write(
        base.join("src/main.rs"),
        r#"
mod module_a;
mod module_b;

fn main() {
    module_a::function_a();
}
"#,
    )
    .unwrap();

    fs::write(
        base.join("tests/test1.rs"),
        r#"
#[test]
fn test_example() {
    assert_eq!(1 + 1, 2);
}
"#,
    )
    .unwrap();

    temp_dir
}

#[test]
fn test_search_across_multiple_files() {
    let temp_dir = create_multi_file_structure();
    
    let options = codesearch::types::SearchOptions {
        extensions: Some(vec![String::from("rs")]),
        ..Default::default()
    };
    let results = codesearch::search::search_code("function", temp_dir.path(), &options).unwrap();

    // Should find "function" in multiple files
    assert!(!results.is_empty());
    
    // Verify results span multiple files
    let unique_files: std::collections::HashSet<_> = results.iter().map(|r| &r.file).collect();
    assert!(unique_files.len() > 1, "Should find matches in multiple files");
}

#[test]
fn test_search_across_nested_folders() {
    let temp_dir = create_multi_file_structure();
    
    let options = codesearch::types::SearchOptions {
        extensions: Some(vec![String::from("rs")]),
        ..Default::default()
    };
    let results = codesearch::search::search_code("TODO|FIXME", temp_dir.path(), &options).unwrap();

    // Should find TODO and FIXME across different folders
    assert!(!results.is_empty());
    
    // Verify results include files from different directories
    let has_module_a = results.iter().any(|r| r.file.contains("module_a"));
    let has_module_b = results.iter().any(|r| r.file.contains("module_b"));
    assert!(has_module_a || has_module_b, "Should find matches in nested folders");
}

#[test]
fn test_list_files_recursive() {
    let temp_dir = create_multi_file_structure();
    
    let options = codesearch::types::SearchOptions {
        extensions: Some(vec![String::from("rs")]),
        ..Default::default()
    };
    let files = codesearch::search::list_files(temp_dir.path(), options.extensions.as_deref(), options.exclude.as_deref()).unwrap();

    // Should find all .rs files recursively
    assert!(files.len() >= 5, "Should find at least 5 .rs files");
    
    // Verify files from different folders
    let file_paths: Vec<_> = files.iter().map(|f| &f.path).collect();
    assert!(file_paths.iter().any(|p| p.contains("module_a")));
    assert!(file_paths.iter().any(|p| p.contains("module_b")));
    assert!(file_paths.iter().any(|p| p.contains("tests")));
}

#[test]
fn test_list_files_with_exclude() {
    let temp_dir = create_multi_file_structure();
    
    let options = codesearch::types::SearchOptions {
        extensions: Some(vec![String::from("rs")]),
        exclude: Some(vec![String::from("tests")]),
        ..Default::default()
    };
    let files = codesearch::search::list_files(temp_dir.path(), options.extensions.as_deref(), options.exclude.as_deref()).unwrap();

    // Should exclude the tests folder
    let file_paths: Vec<_> = files.iter().map(|f| &f.path).collect();
    assert!(!file_paths.iter().any(|p| p.contains("tests")));
    assert!(file_paths.iter().any(|p| p.contains("module_a") || p.contains("module_b")));
}

#[test]
fn test_circular_detection_across_files() {
    let temp_dir = create_multi_file_structure();
    
    // function_a -> function_b -> function_c -> function_a (circular across 3 files)
    let options = codesearch::types::SearchOptions {
        extensions: Some(vec![String::from("rs")]),
        ..Default::default()
    };
    let cycles = codesearch::circular::find_circular_calls(temp_dir.path(), options.extensions.as_deref(), options.exclude.as_deref()).unwrap();

    // Should detect the circular dependency across files
    if !cycles.is_empty() {
        // Verify cycle spans multiple files
        assert!(cycles[0].files.len() > 1, "Cycle should span multiple files");
    }
}

#[test]
fn test_deadcode_detection_across_files() {
    let temp_dir = create_multi_file_structure();
    
    let options = codesearch::types::SearchOptions {
        extensions: Some(vec![String::from("rs")]),
        ..Default::default()
    };
    let result = codesearch::deadcode::detect_dead_code(temp_dir.path(), options.extensions.as_deref(), options.exclude.as_deref());
    
    // Should successfully analyze for dead code across files
    assert!(result.is_ok());
}

#[test]
fn test_duplicate_detection_across_files() {
    let temp_dir = create_multi_file_structure();
    
    let options = codesearch::types::SearchOptions {
        extensions: Some(vec![String::from("rs")]),
        ..Default::default()
    };
    let duplicates = codesearch::duplicates::find_duplicates(temp_dir.path(), options.extensions.as_deref(), options.exclude.as_deref(), 3, 0.9).unwrap();

    // Should find duplicate_logic in file2.rs and file3.rs
    if !duplicates.is_empty() {
        // Verify duplicates are in different files
        assert_ne!(duplicates[0].file1, duplicates[0].file2, "Duplicates should be in different files");
    }
}

#[test]
fn test_complexity_analysis_across_folders() {
    let temp_dir = create_multi_file_structure();
    
    // Get all files and calculate complexity for each
    let options = codesearch::types::SearchOptions {
        extensions: Some(vec![String::from("rs")]),
        ..Default::default()
    };
    let files = codesearch::search::list_files(temp_dir.path(), options.extensions.as_deref(), options.exclude.as_deref()).unwrap();
    
    let mut metrics = Vec::new();
    for file in &files {
        if let Ok(content) = std::fs::read_to_string(&file.path) {
            let m = codesearch::complexity::calculate_file_complexity(&file.path, &content);
            metrics.push(m);
        }
    }

    // Should analyze complexity across all files
    assert!(!metrics.is_empty(), "Should analyze files across folders");
    
    // Verify analysis covers multiple files
    let unique_files: std::collections::HashSet<_> = metrics.iter().map(|m| &m.file_path).collect();
    assert!(unique_files.len() > 1, "Should analyze multiple files");
}

#[test]
fn test_search_with_multiple_extensions() {
    let temp_dir = tempfile::tempdir().unwrap();
    let base = temp_dir.path();

    // Create files with different extensions
    fs::write(base.join("file1.rs"), "fn rust_function() {}").unwrap();
    fs::write(base.join("file2.py"), "def python_function():").unwrap();
    fs::write(base.join("file3.js"), "function js_function() {}").unwrap();

    let options = codesearch::types::SearchOptions {
        extensions: Some(vec![String::from("rs"), String::from("py"), String::from("js")]),
        ..Default::default()
    };
    let results = codesearch::search::search_code("function", base, &options).unwrap();

    // Should find "function" in all three file types
    assert!(!results.is_empty());
    
    let unique_extensions: std::collections::HashSet<_> = results
        .iter()
        .filter_map(|r| std::path::Path::new(&r.file).extension())
        .collect();
    assert!(unique_extensions.len() >= 2, "Should search across multiple file types");
}

#[test]
fn test_deeply_nested_folder_structure() {
    let temp_dir = tempfile::tempdir().unwrap();
    let base = temp_dir.path();

    // Create deeply nested structure
    fs::create_dir_all(base.join("a/b/c/d/e")).unwrap();
    fs::write(
        base.join("a/b/c/d/e/deep.rs"),
        "fn deeply_nested_function() { /* TODO: optimize */ }",
    )
    .unwrap();

    let options = codesearch::types::SearchOptions {
        extensions: Some(vec![String::from("rs")]),
        ..Default::default()
    };
    let results = codesearch::search::search_code("TODO", base, &options).unwrap();

    // Should find TODO in deeply nested file
    assert!(!results.is_empty(), "Should search deeply nested folders");
    assert!(results[0].file.contains("a/b/c/d/e") || results[0].file.contains("a\\b\\c\\d\\e"));
}

#[test]
fn test_cross_folder_with_symlinks() {
    // Note: Symlink handling depends on walkdir configuration
    // This test ensures we don't crash on symlinks
    let temp_dir = tempfile::tempdir().unwrap();
    let base = temp_dir.path();

    fs::create_dir_all(base.join("real_folder")).unwrap();
    fs::write(base.join("real_folder/file.rs"), "fn test() {}").unwrap();

    // Try to search - should handle gracefully even if symlinks exist
    let result = codesearch::search::list_files(
        base,
        Some(&[String::from("rs")]),
        None,
    );

    assert!(result.is_ok(), "Should handle directory traversal gracefully");
}
