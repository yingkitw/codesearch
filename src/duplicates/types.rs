//! Type definitions for duplicate detection

use serde::Serialize;

/// Clone type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum CloneType {
    /// Type-1: Exact copies (except whitespace/comments)
    Type1,
    /// Type-2: Renamed identifiers (same structure)
    Type2,
    /// Type-3: Modified statements (some additions/deletions)
    Type3,
    /// Type-4: Semantic clones (different syntax, same behavior)
    Type4,
}

/// Enhanced duplicate block with multi-metric similarity
#[derive(Debug, Clone, Serialize)]
pub struct EnhancedDuplicateBlock {
    pub file1: String,
    pub line1: usize,
    pub file2: String,
    pub line2: usize,
    pub content: String,
    pub similarity: f64,
    pub clone_type: CloneType,
    pub token_similarity: f64,
    pub structural_similarity: f64,
    pub line_count: usize,
}

/// Configuration for duplicate detection
#[derive(Debug, Clone)]
pub struct DuplicateConfig {
    pub min_lines: usize,
    pub min_tokens: usize,
    pub similarity_threshold: f64,
    
    // Advanced filters
    pub exclude_tests: bool,
    pub exclude_generated: bool,
    pub exclude_patterns: Vec<String>,
    
    // Clone type preferences
    pub detect_type1: bool,
    pub detect_type2: bool,
    pub detect_type3: bool,
    
    // Performance
    pub use_parallel: bool,
    pub max_file_size: usize,
}

impl Default for DuplicateConfig {
    fn default() -> Self {
        Self {
            min_lines: 5,
            min_tokens: 10,
            similarity_threshold: 0.9,
            exclude_tests: false,
            exclude_generated: true,
            exclude_patterns: vec![],
            detect_type1: true,
            detect_type2: true,
            detect_type3: true,
            use_parallel: true,
            max_file_size: 1_000_000, // 1MB
        }
    }
}

/// Code block with metadata
#[derive(Debug, Clone)]
pub struct CodeBlock {
    pub file: String,
    pub line_start: usize,
    pub line_end: usize,
    pub content: String,
    pub normalized: String,
    pub tokens: Vec<String>,
    pub hash: u64,
    pub normalized_hash: u64,
}
