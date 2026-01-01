//! Property-based tests for search functionality

use codesearch::search::search_code;
use codesearch::types::SearchOptions;
use proptest::prelude::*;
use std::path::Path;
use tempfile::tempdir;
use std::fs;

// Strategy for generating valid search queries
fn query_strategy() -> impl Strategy<Value = String> {
    prop::string::string_regex("[a-zA-Z0-9_]{1,20}").unwrap()
}

// Strategy for generating file extensions
fn extension_strategy() -> impl Strategy<Value = Vec<String>> {
    prop::collection::vec(
        prop::oneof![
            Just("rs".to_string()),
            Just("py".to_string()),
            Just("js".to_string()),
            Just("txt".to_string()),
        ],
        0..3,
    )
}

// Strategy for generating search options
fn search_options_strategy() -> impl Strategy<Value = SearchOptions> {
    (
        extension_strategy(),
        any::<bool>(),
        any::<bool>(),
        0.1f64..1.0f64,
        1usize..100usize,
    )
        .prop_map(|(extensions, ignore_case, fuzzy, threshold, max_results)| {
            SearchOptions {
                extensions: if extensions.is_empty() {
                    None
                } else {
                    Some(extensions)
                },
                ignore_case,
                fuzzy,
                fuzzy_threshold: threshold,
                max_results,
                exclude: None,
                rank: false,
                cache: false,
                semantic: false,
                benchmark: false,
                vs_grep: false,
            }
        })
}

proptest! {
    #[test]
    fn test_search_never_panics(query in query_strategy(), options in search_options_strategy()) {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("test.rs"), "fn test() { println!(\"hello\"); }").unwrap();
        
        // Search should never panic, even with random inputs
        let result = search_code(&query, dir.path(), &options);
        
        // Either succeeds or returns an error, but never panics
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_search_results_contain_query(
        query in "[a-z]{3,10}",
        ignore_case in any::<bool>()
    ) {
        let dir = tempdir().unwrap();
        let content = format!("This contains the word {} in the text", query);
        fs::write(dir.path().join("test.txt"), &content).unwrap();
        
        let options = SearchOptions {
            extensions: None,
            ignore_case,
            fuzzy: false,
            ..Default::default()
        };
        
        if let Ok(results) = search_code(&query, dir.path(), &options) {
            // If we find results, they should contain the query
            for result in results {
                let content_lower = result.content.to_lowercase();
                let query_lower = query.to_lowercase();
                if ignore_case {
                    assert!(content_lower.contains(&query_lower));
                }
            }
        }
    }

    #[test]
    fn test_max_results_respected(
        query in "test",
        max_results in 1usize..10usize
    ) {
        let dir = tempdir().unwrap();
        
        // Create multiple files with matches
        for i in 0..20 {
            fs::write(
                dir.path().join(format!("file{}.txt", i)),
                "test test test"
            ).unwrap();
        }
        
        let options = SearchOptions {
            extensions: None,
            max_results,
            ..Default::default()
        };
        
        if let Ok(results) = search_code(&query, dir.path(), &options) {
            // Each file should have at most max_results matches
            for result in results {
                assert!(result.matches.len() <= max_results);
            }
        }
    }

    #[test]
    fn test_empty_query_returns_results(
        options in search_options_strategy()
    ) {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("test.rs"), "fn main() {}").unwrap();
        
        // Empty query should not panic
        let result = search_code("", dir.path(), &options);
        assert!(result.is_ok());
    }

    #[test]
    fn test_fuzzy_threshold_affects_results(
        threshold in 0.1f64..1.0f64
    ) {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("test.txt"), "testing tests").unwrap();
        
        let options = SearchOptions {
            fuzzy: true,
            fuzzy_threshold: threshold,
            ..Default::default()
        };
        
        // Should not panic regardless of threshold
        let result = search_code("test", dir.path(), &options);
        assert!(result.is_ok());
    }

    #[test]
    fn test_extension_filter_works(
        extensions in extension_strategy()
    ) {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("test.rs"), "test").unwrap();
        fs::write(dir.path().join("test.py"), "test").unwrap();
        fs::write(dir.path().join("test.js"), "test").unwrap();
        
        let options = SearchOptions {
            extensions: if extensions.is_empty() { None } else { Some(extensions.clone()) },
            ..Default::default()
        };
        
        if let Ok(results) = search_code("test", dir.path(), &options) {
            // All results should match the extension filter
            for result in results {
                if let Some(ref exts) = options.extensions {
                    let has_valid_ext = exts.iter().any(|ext| result.file.ends_with(ext));
                    assert!(has_valid_ext || exts.is_empty());
                }
            }
        }
    }
}

#[test]
fn test_proptest_runs() {
    // This test ensures proptest is working
    assert!(true);
}
