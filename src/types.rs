//! Shared Types Module
//!
//! Common data structures used across the codebase search tool.

use serde::{Deserialize, Serialize};

/// A search result containing match information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub file: String,
    pub line_number: usize,
    pub content: String,
    pub matches: Vec<Match>,
    pub score: f64,
    pub relevance: String,
}

/// A single match within a line
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Match {
    pub start: usize,
    pub end: usize,
    pub text: String,
}

/// File information with path, size, and line count
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub path: String,
    pub size: u64,
    pub lines: usize,
}

/// Refactoring suggestion with priority and improvement
#[derive(Debug, Clone)]
pub struct RefactorSuggestion {
    pub file: String,
    pub line_number: usize,
    pub suggestion_type: String,
    pub description: String,
    pub priority: u8, // 1-10, 10 being highest priority
    pub code_snippet: String,
    pub improvement: String,
}

/// Code complexity metrics for a file
#[derive(Debug, Clone, Default)]
pub struct ComplexityMetrics {
    pub file_path: String,
    pub cyclomatic_complexity: u32,
    pub cognitive_complexity: u32,
    pub lines_of_code: usize,
    pub function_count: usize,
    pub max_nesting_depth: u32,
}

/// Search options to bundle related parameters
#[derive(Debug, Clone)]
pub struct SearchOptions {
    pub extensions: Option<Vec<String>>,
    pub ignore_case: bool,
    pub fuzzy: bool,
    pub fuzzy_threshold: f64,
    pub max_results: usize,
    pub exclude: Option<Vec<String>>,
    pub rank: bool,
    pub cache: bool,
    pub semantic: bool,
    pub benchmark: bool,
    pub vs_grep: bool,
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            extensions: None,
            ignore_case: false,
            fuzzy: false,
            fuzzy_threshold: 0.8,
            max_results: 100,
            exclude: None,
            rank: false,
            cache: false,
            semantic: false,
            benchmark: false,
            vs_grep: false,
        }
    }
}

impl SearchOptions {
    /// Builder pattern: set extensions
    pub fn with_extensions(mut self, extensions: Vec<String>) -> Self {
        self.extensions = Some(extensions);
        self
    }

    /// Builder pattern: set ignore_case
    pub fn with_ignore_case(mut self, ignore_case: bool) -> Self {
        self.ignore_case = ignore_case;
        self
    }

    /// Builder pattern: set fuzzy
    pub fn with_fuzzy(mut self, fuzzy: bool) -> Self {
        self.fuzzy = fuzzy;
        self
    }

    /// Builder pattern: set fuzzy_threshold
    pub fn with_fuzzy_threshold(mut self, threshold: f64) -> Self {
        self.fuzzy_threshold = threshold;
        self
    }

    /// Builder pattern: set max_results
    pub fn with_max_results(mut self, max: usize) -> Self {
        self.max_results = max;
        self
    }

    /// Builder pattern: set exclude
    pub fn with_exclude(mut self, exclude: Vec<String>) -> Self {
        self.exclude = Some(exclude);
        self
    }

    /// Builder pattern: set rank
    pub fn with_rank(mut self, rank: bool) -> Self {
        self.rank = rank;
        self
    }

    /// Builder pattern: set cache
    pub fn with_cache(mut self, cache: bool) -> Self {
        self.cache = cache;
        self
    }

    /// Builder pattern: set semantic
    pub fn with_semantic(mut self, semantic: bool) -> Self {
        self.semantic = semantic;
        self
    }

    /// Builder pattern: set benchmark
    pub fn with_benchmark(mut self, benchmark: bool) -> Self {
        self.benchmark = benchmark;
        self
    }

    /// Builder pattern: set vs_grep
    pub fn with_vs_grep(mut self, vs_grep: bool) -> Self {
        self.vs_grep = vs_grep;
        self
    }
}

/// Search performance metrics
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SearchMetrics {
    pub files_processed: usize,
    pub total_lines_scanned: usize,
    pub search_time_ms: u128,
    pub parallel_workers: usize,
    pub cache_hits: usize,
    pub cache_misses: usize,
}

/// Duplicate code block information
#[derive(Debug, Clone, Serialize)]
pub struct DuplicateBlock {
    pub file1: String,
    pub line1: usize,
    pub file2: String,
    pub line2: usize,
    pub content: String,
    pub similarity: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_result() {
        let result = SearchResult {
            file: "test.rs".to_string(),
            line_number: 10,
            content: "fn main()".to_string(),
            matches: vec![Match {
                start: 0,
                end: 2,
                text: "fn".to_string(),
            }],
            score: 0.95,
            relevance: "High".to_string(),
        };
        assert_eq!(result.file, "test.rs");
        assert_eq!(result.matches.len(), 1);
    }

    #[test]
    fn test_complexity_metrics_default() {
        let metrics = ComplexityMetrics::default();
        assert_eq!(metrics.cyclomatic_complexity, 0);
        assert_eq!(metrics.cognitive_complexity, 0);
    }
}

