//! End-to-End Integration Tests
//!
//! Tests complete workflows from search to analysis to export.

mod fixtures;

use codesearch::search::search_code;
use codesearch::types::SearchOptions;
use codesearch::{analysis, complexity, deadcode, export};
use fixtures::TestWorkspace;
use std::fs;

#[test]
fn test_search_and_export_workflow() {
    let workspace = TestWorkspace::new();
    let options = SearchOptions::default();

    // Perform search
    let results = search_code("test", workspace.path(), &options).expect("Search failed");
    assert!(!results.is_empty(), "Should find test matches");

    // Export results
    let export_path = workspace.path().join("results.json");
    export::export_results(&results, export_path.to_str().unwrap(), "test")
        .expect("Export failed");

    // Verify export file exists
    assert!(export_path.exists());
    let content = fs::read_to_string(&export_path).expect("Failed to read export");
    assert!(content.contains("test"));
}

#[test]
fn test_search_with_multiple_extensions() {
    let workspace = TestWorkspace::new();
    let options = SearchOptions {
        extensions: Some(vec!["rs".to_string(), "py".to_string()]),
        ..Default::default()
    };

    let results = search_code("main", workspace.path(), &options).expect("Search failed");
    assert!(!results.is_empty());

    // Verify only .rs and .py files
    for result in results {
        assert!(result.file.ends_with(".rs") || result.file.ends_with(".py"));
    }
}

#[test]
fn test_search_with_fuzzy_matching() {
    let workspace = TestWorkspace::new();
    let options = SearchOptions {
        fuzzy: true,
        fuzzy_threshold: 0.5,
        ..Default::default()
    };

    let results = search_code("tst", workspace.path(), &options).expect("Search failed");
    // Fuzzy search should find "test" even with typo
    assert!(!results.is_empty());
}

#[test]
fn test_analyze_then_search() {
    let workspace = TestWorkspace::new();

    // First analyze the codebase
    let analyze_result = analysis::analyze_codebase(workspace.path(), None, None);
    assert!(analyze_result.is_ok());

    // Then search for specific patterns
    let options = SearchOptions::default();
    let results = search_code("fn", workspace.path(), &options).expect("Search failed");
    assert!(!results.is_empty());
}

#[test]
fn test_complexity_analysis_workflow() {
    let mut workspace = TestWorkspace::new();
    workspace.add_file(
        "complex.rs",
        r#"
fn complex_function(x: i32) -> i32 {
    if x > 0 {
        if x > 10 {
            if x > 20 {
                return x * 2;
            }
            return x + 10;
        }
        return x + 5;
    }
    return 0;
}
"#,
    );

    let result = complexity::analyze_complexity(workspace.path(), Some(&[String::from("rs")]), None, Some(1), false);
    assert!(result.is_ok());
}

#[test]
fn test_deadcode_detection_workflow() {
    let mut workspace = TestWorkspace::new();
    workspace.add_file(
        "deadcode.rs",
        r#"
fn used_function() {
    println!("Used");
}

fn unused_function() {
    // TODO: implement this
    let unused_var = 42;
}
"#,
    );

    let result = deadcode::detect_dead_code(workspace.path(), Some(&[String::from("rs")]), None);
    assert!(result.is_ok());
}

#[test]
fn test_search_ranking() {
    let workspace = TestWorkspace::new();
    let options = SearchOptions {
        rank: true,
        ..Default::default()
    };

    let results = search_code("test", workspace.path(), &options).expect("Search failed");
    
    if results.len() > 1 {
        // Verify results are sorted by score
        for i in 0..results.len() - 1 {
            assert!(results[i].score >= results[i + 1].score);
        }
    }
}

#[test]
fn test_search_with_exclusions() {
    let workspace = TestWorkspace::new();
    let subdir = workspace.create_subdir("excluded");
    fs::write(subdir.join("test.rs"), "fn excluded_test() {}").expect("Failed to write");

    let options = SearchOptions {
        exclude: Some(vec!["excluded".to_string()]),
        ..Default::default()
    };

    let results = search_code("excluded", workspace.path(), &options).expect("Search failed");
    
    // Should not find matches in excluded directory
    for result in results {
        assert!(!result.file.contains("excluded"));
    }
}

#[test]
fn test_max_results_limit() {
    let mut workspace = TestWorkspace::new();
    
    // Create multiple files with many matches
    for i in 0..10 {
        workspace.add_file(
            &format!("file{i}.txt"),
            "test test test test test",
        );
    }

    let options = SearchOptions {
        max_results: 2,
        ..Default::default()
    };

    let results = search_code("test", workspace.path(), &options).expect("Search failed");
    
    // Each file should have at most 2 matches
    for result in results {
        assert!(result.matches.len() <= 2);
    }
}

#[test]
fn test_case_sensitive_search() {
    let mut workspace = TestWorkspace::new();
    workspace.add_file("case.txt", "Test TEST test TeSt");

    let options_sensitive = SearchOptions {
        ignore_case: false,
        ..Default::default()
    };

    let results = search_code("Test", workspace.path(), &options_sensitive).expect("Search failed");
    
    // Should only match exact case
    for result in results {
        assert!(result.content.contains("Test"));
    }
}

#[test]
fn test_case_insensitive_search() {
    let mut workspace = TestWorkspace::new();
    workspace.add_file("case.txt", "Test TEST test TeSt");

    let options_insensitive = SearchOptions {
        ignore_case: true,
        ..Default::default()
    };

    let results = search_code("test", workspace.path(), &options_insensitive).expect("Search failed");
    assert!(!results.is_empty());
}

#[test]
fn test_empty_directory() {
    let workspace = TestWorkspace::with_files(&[]);
    let options = SearchOptions::default();

    let results = search_code("test", workspace.path(), &options).expect("Search failed");
    assert!(results.is_empty());
}

#[test]
fn test_nested_directories() {
    let workspace = TestWorkspace::new();
    let subdir1 = workspace.create_subdir("level1");
    let subdir2 = subdir1.join("level2");
    fs::create_dir_all(&subdir2).expect("Failed to create nested dir");
    fs::write(subdir2.join("nested.rs"), "fn nested_test() {}").expect("Failed to write");

    let options = SearchOptions::default();
    let results = search_code("nested", workspace.path(), &options).expect("Search failed");
    
    assert!(!results.is_empty());
    assert!(results[0].file.contains("level1"));
    assert!(results[0].file.contains("level2"));
}
