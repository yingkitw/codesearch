//! Search Module
//!
//! Core search functionality with parallel processing, fuzzy matching, and semantic search.

pub mod core;
pub mod fuzzy;
pub mod semantic;
pub mod utilities;

pub use core::{search_code, list_files};
pub use fuzzy::{search_in_file_parallel, calculate_relevance_score};
pub use semantic::enhance_query_semantically;
pub use utilities::{compare_with_grep, print_results, print_search_stats};

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_enhance_query_semantically() {
        let enhanced = enhance_query_semantically("function");
        assert!(enhanced.contains("function") || enhanced.contains("def") || enhanced.contains("fn"));
        
        let enhanced = enhance_query_semantically("class");
        assert!(enhanced.contains("class") || enhanced.contains("struct"));
    }

    #[test]
    fn test_calculate_relevance_score() {
        let score = calculate_relevance_score(
            "fn test_function() {",
            "test",
            10,
            Path::new("test.rs"),
            false,
            None,
        );
        assert!(score > 0.0);
        assert!(score <= 100.0);
    }

    #[test]
    fn test_search_code_basic() {
        use std::fs;
        use tempfile::tempdir;

        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.rs");
        fs::write(&file_path, "fn test() {\n    println!(\"hello\");\n}").unwrap();

        let results = search_code(
            "test",
            dir.path(),
            Some(&["rs".to_string()]),
            true,
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

        assert!(results.is_ok());
        let results = results.unwrap();
        assert!(!results.is_empty());
    }

    #[test]
    fn test_list_files() {
        use std::fs;
        use tempfile::tempdir;

        let dir = tempdir().unwrap();
        fs::write(dir.path().join("test1.rs"), "test").unwrap();
        fs::write(dir.path().join("test2.py"), "test").unwrap();

        let files = list_files(dir.path(), Some(&["rs".to_string()]), None);
        assert!(files.is_ok());
        let files = files.unwrap();
        assert_eq!(files.len(), 1);
    }

    #[test]
    fn test_search_code_with_extension_filter() {
        use std::fs;
        use tempfile::tempdir;

        let dir = tempdir().unwrap();
        fs::write(dir.path().join("test.rs"), "fn test() {}").unwrap();
        fs::write(dir.path().join("test.py"), "def test(): pass").unwrap();

        let results = search_code(
            "test",
            dir.path(),
            Some(&["rs".to_string()]),
            true,
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

        assert!(results.is_ok());
        let results = results.unwrap();
        assert!(results.iter().all(|r| r.file.ends_with(".rs")));
    }
}
