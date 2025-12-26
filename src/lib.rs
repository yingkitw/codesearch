//! CodeSearch - A fast CLI tool for searching codebases
//!
//! This library provides code search functionality with support for:
//! - Regex and fuzzy search
//! - Multi-language support
//! - Codebase analysis
//! - Complexity metrics
//! - Duplicate detection
//! - MCP server integration

pub mod analysis;
pub mod cache;
pub mod complexity;
pub mod config;
pub mod duplicates;
pub mod export;
pub mod favorites;
pub mod language;
pub mod mcp_server;
pub mod search;
pub mod theme;
pub mod types;

// Re-export commonly used items at the crate root
pub use search::{list_files, print_results, print_search_stats, search_code};
pub use types::{ComplexityMetrics, FileInfo, Match, RefactorSuggestion, SearchResult};
pub use analysis::{analyze_codebase, analyze_file_for_refactoring, suggest_refactoring};
pub use complexity::{calculate_file_complexity, calculate_cyclomatic_complexity, calculate_cognitive_complexity};
pub use duplicates::detect_duplicates;
pub use config::{load_config, Config};
pub use language::{get_supported_languages, LanguageInfo};
pub use theme::{get_theme, list_theme_names, Theme};

