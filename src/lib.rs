//! CodeSearch - A fast CLI tool for searching codebases
//!
//! This library provides code search functionality with support for:
//! - Regex and fuzzy search
//! - Multi-language support
//! - Codebase analysis
//! - Complexity metrics
//! - Duplicate detection
//! - Dead code detection
//! - Interactive mode
//! - MCP server integration

pub mod analysis;
pub mod cache;
pub mod circular;
pub mod complexity;
#[cfg(test)]
mod complexity_tests;
pub mod deadcode;
pub mod duplicates;
pub mod export;
pub mod interactive;
pub mod language;
#[cfg(feature = "mcp")]
pub mod mcp;
pub mod parser;
pub mod search;
#[cfg(test)]
mod search_tests;
pub mod types;

// Re-export commonly used items at the crate root
pub use search::{list_files, print_results, print_search_stats, search_code};
pub use types::{ComplexityMetrics, DuplicateBlock, FileInfo, Match, RefactorSuggestion, SearchResult};
pub use analysis::analyze_codebase;
pub use complexity::{calculate_file_complexity, calculate_cyclomatic_complexity, calculate_cognitive_complexity};
pub use circular::{detect_circular_calls, find_circular_calls, CircularCall};
pub use deadcode::{detect_dead_code, find_dead_code, DeadCodeItem};
pub use duplicates::{detect_duplicates, find_duplicates};
pub use language::{get_supported_languages, LanguageInfo};

