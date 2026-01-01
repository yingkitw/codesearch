//! Default SearchEngine Implementation
//!
//! Provides the default implementation of the SearchEngine trait using the existing search_code logic.

use crate::search::core::search_code as search_code_impl;
use crate::traits::SearchEngine;
use crate::types::{SearchOptions, SearchResult};
use std::path::Path;

/// Default search engine implementation
///
/// This wraps the existing search_code function to implement the SearchEngine trait.
/// It provides the standard parallel search with fuzzy matching, caching, and semantic search.
///
/// # Examples
///
/// ```
/// use codesearch::search::DefaultSearchEngine;
/// use codesearch::traits::SearchEngine;
/// use codesearch::types::SearchOptions;
/// use std::path::Path;
///
/// let engine = DefaultSearchEngine::new();
/// let options = SearchOptions::default();
/// let results = engine.search("fn main", Path::new("src"), &options);
/// ```
#[derive(Debug, Clone, Default)]
pub struct DefaultSearchEngine;

impl DefaultSearchEngine {
    /// Create a new default search engine
    pub fn new() -> Self {
        Self
    }
}

impl SearchEngine for DefaultSearchEngine {
    fn search(
        &self,
        query: &str,
        path: &Path,
        options: &SearchOptions,
    ) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
        search_code_impl(query, path, options)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    #[test]
    fn test_default_search_engine() {
        let engine = DefaultSearchEngine::new();
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("test.rs"), "fn test() {}").unwrap();

        let options = SearchOptions {
            extensions: Some(vec!["rs".to_string()]),
            ignore_case: true,
            ..Default::default()
        };

        let result = engine.search("test", dir.path(), &options);
        assert!(result.is_ok());
        let results = result.unwrap();
        assert!(!results.is_empty());
    }

    #[test]
    fn test_search_engine_trait_object() {
        let engine: Box<dyn SearchEngine> = Box::new(DefaultSearchEngine::new());
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("test.rs"), "fn main() {}").unwrap();

        let options = SearchOptions::default();
        let result = engine.search("main", dir.path(), &options);
        assert!(result.is_ok());
    }
}
