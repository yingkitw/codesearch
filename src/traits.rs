//! Trait Abstractions Module
//!
//! Defines core traits for better testability and extensibility.

use crate::types::{SearchOptions, SearchResult};
use std::path::Path;

/// Trait for search engine implementations
/// 
/// This trait allows different search strategies to be implemented and tested independently.
/// It enables dependency injection and makes the code more testable by allowing mock implementations.
///
/// # Examples
///
/// ```
/// use codesearch::traits::SearchEngine;
/// use codesearch::types::{SearchOptions, SearchResult};
/// use std::path::Path;
///
/// struct MockSearchEngine;
///
/// impl SearchEngine for MockSearchEngine {
///     fn search(&self, query: &str, path: &Path, options: &SearchOptions) 
///         -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
///         // Return mock results for testing
///         Ok(vec![])
///     }
/// }
/// ```
pub trait SearchEngine: Send + Sync {
    /// Search for patterns in code files
    ///
    /// # Arguments
    ///
    /// * `query` - The search pattern (regex or literal)
    /// * `path` - The directory path to search in
    /// * `options` - Search configuration options
    ///
    /// # Returns
    ///
    /// A vector of search results or an error
    fn search(
        &self,
        query: &str,
        path: &Path,
        options: &SearchOptions,
    ) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>>;
}

/// Trait for code analysis implementations
///
/// This trait allows different analysis strategies to be implemented and tested independently.
/// Examples include complexity analysis, dead code detection, and duplication detection.
///
/// # Examples
///
/// ```
/// use codesearch::traits::Analyzer;
/// use std::path::Path;
///
/// struct MockAnalyzer;
///
/// impl Analyzer for MockAnalyzer {
///     type Output = String;
///     
///     fn analyze(&self, path: &Path, extensions: Option<&[String]>) 
///         -> Result<Self::Output, Box<dyn std::error::Error>> {
///         Ok("Mock analysis".to_string())
///     }
/// }
/// ```
pub trait Analyzer: Send + Sync {
    /// The output type of the analysis
    type Output;

    /// Analyze code at the given path
    ///
    /// # Arguments
    ///
    /// * `path` - The directory or file path to analyze
    /// * `extensions` - Optional file extensions to filter by
    ///
    /// # Returns
    ///
    /// Analysis results or an error
    fn analyze(
        &self,
        path: &Path,
        extensions: Option<&[String]>,
    ) -> Result<Self::Output, Box<dyn std::error::Error>>;
}

/// Trait for graph builder implementations
///
/// This trait allows different graph construction strategies to be implemented.
/// Examples include AST, CFG, DFG, call graphs, and dependency graphs.
pub trait GraphBuilder: Send + Sync {
    /// The graph type produced by this builder
    type Graph;

    /// Build a graph from the given source
    ///
    /// # Arguments
    ///
    /// * `source` - The source code or file path
    /// * `name` - Optional name for the graph
    ///
    /// # Returns
    ///
    /// The constructed graph or an error
    fn build(
        &self,
        source: &str,
        name: Option<&str>,
    ) -> Result<Self::Graph, Box<dyn std::error::Error>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock implementations for testing

    struct MockSearchEngine {
        results: Vec<SearchResult>,
    }

    impl SearchEngine for MockSearchEngine {
        fn search(
            &self,
            _query: &str,
            _path: &Path,
            _options: &SearchOptions,
        ) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
            Ok(self.results.clone())
        }
    }

    struct MockAnalyzer {
        output: String,
    }

    impl Analyzer for MockAnalyzer {
        type Output = String;

        fn analyze(
            &self,
            _path: &Path,
            _extensions: Option<&[String]>,
        ) -> Result<Self::Output, Box<dyn std::error::Error>> {
            Ok(self.output.clone())
        }
    }

    #[test]
    fn test_mock_search_engine() {
        let engine = MockSearchEngine {
            results: vec![],
        };
        let options = SearchOptions::default();
        let result = engine.search("test", Path::new("."), &options);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_mock_analyzer() {
        let analyzer = MockAnalyzer {
            output: "test output".to_string(),
        };
        let result = analyzer.analyze(Path::new("."), None);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test output");
    }
}
