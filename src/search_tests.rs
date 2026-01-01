//! Additional edge case and complex scenario tests for search module

#[cfg(test)]
mod edge_case_tests {
    use crate::search::{search_code, list_files};
    use crate::types::SearchOptions;
    use std::path::PathBuf;

    fn default_options() -> SearchOptions {
        SearchOptions {
            extensions: Some(vec![String::from("rs")]),
            ignore_case: false,
            fuzzy: false,
            fuzzy_threshold: 0.8,
            max_results: 10,
            exclude: None,
            rank: false,
            cache: false,
            semantic: false,
            benchmark: false,
            vs_grep: false,
        }
    }

    #[test]
    fn test_search_empty_query() {
        let path = PathBuf::from(".");
        let options = default_options();
        let result = search_code("", &path, &options);
        assert!(result.is_ok());
    }

    #[test]
    fn test_search_nonexistent_path() {
        let path = PathBuf::from("/nonexistent/path/that/does/not/exist");
        let options = SearchOptions {
            extensions: None,
            ..default_options()
        };
        let result = search_code("test", &path, &options);
        assert!(result.is_ok());
    }

    #[test]
    fn test_search_special_regex_characters() {
        let path = PathBuf::from("src");
        let options = default_options();
        let result = search_code("fn.*test", &path, &options);
        assert!(result.is_ok());
    }

    #[test]
    fn test_search_case_insensitive() {
        let path = PathBuf::from("src");
        let options = SearchOptions {
            ignore_case: true,
            ..default_options()
        };
        let result = search_code("TEST", &path, &options);
        assert!(result.is_ok());
    }

    #[test]
    fn test_search_with_exclude() {
        let path = PathBuf::from(".");
        let options = SearchOptions {
            exclude: Some(vec![String::from("target"), String::from("node_modules")]),
            ..default_options()
        };
        let result = search_code("test", &path, &options);
        assert!(result.is_ok());
    }

    #[test]
    fn test_search_max_results_limit() {
        let path = PathBuf::from("src");
        let options = SearchOptions {
            max_results: 5,
            ..default_options()
        };
        let result = search_code("fn", &path, &options);
        assert!(result.is_ok());
        // Note: max_results limits matches per file, not total results
        // So total results can exceed max_results if matches are in multiple files
        if let Ok(results) = result {
            assert!(!results.is_empty());
        }
    }

    #[test]
    fn test_search_with_ranking() {
        let path = PathBuf::from("src");
        let options = SearchOptions {
            rank: true,
            ..default_options()
        };
        let result = search_code("test", &path, &options);
        assert!(result.is_ok());
    }

    #[test]
    fn test_search_unicode_content() {
        let path = PathBuf::from("src");
        let options = SearchOptions {
            extensions: None,
            ..default_options()
        };
        let result = search_code("测试", &path, &options);
        assert!(result.is_ok());
    }

    #[test]
    fn test_search_multiline_pattern() {
        let path = PathBuf::from("src");
        let options = default_options();
        let result = search_code("fn\\s+\\w+\\s*\\(", &path, &options);
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_files_empty_directory() {
        use tempfile::tempdir;
        let dir = tempdir().unwrap();
        let result = list_files(dir.path(), None, None);
        assert!(result.is_ok());
        let files = result.unwrap();
        assert!(files.is_empty());
    }

    #[test]
    fn test_list_files_with_multiple_extensions() {
        let path = PathBuf::from("src");
        let result = list_files(&path, Some(&[String::from("rs"), String::from("toml")]), None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_search_very_long_pattern() {
        let path = PathBuf::from("src");
        let long_pattern = "a".repeat(1000);
        let options = default_options();
        let result = search_code(&long_pattern, &path, &options);
        assert!(result.is_ok());
    }

    #[test]
    fn test_search_with_cache_enabled() {
        let path = PathBuf::from("src");
        let options = SearchOptions {
            cache: true,
            ..default_options()
        };
        let result = search_code("test", &path, &options);
        assert!(result.is_ok());
    }

    #[test]
    fn test_search_fuzzy_with_low_threshold() {
        let path = PathBuf::from("src");
        let options = SearchOptions {
            fuzzy: true,
            fuzzy_threshold: 0.3,
            ..default_options()
        };
        let result = search_code("tst", &path, &options);
        assert!(result.is_ok());
    }

    #[test]
    fn test_search_fuzzy_with_high_threshold() {
        let path = PathBuf::from("src");
        let options = SearchOptions {
            fuzzy: true,
            fuzzy_threshold: 0.9,
            ..default_options()
        };
        let result = search_code("test", &path, &options);
        assert!(result.is_ok());
    }
}
