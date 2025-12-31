//! Parameter structures for MCP tools

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SearchCodeParams {
    /// The search query (supports regex)
    pub query: String,
    /// Directory to search in (default: current directory)
    #[serde(default)]
    pub path: Option<String>,
    /// File extensions to include (e.g., ["rs", "py", "js"])
    #[serde(default)]
    pub extensions: Option<Vec<String>>,
    /// Case-insensitive search
    #[serde(default)]
    pub ignore_case: Option<bool>,
    /// Enable fuzzy search (handles typos and variations)
    #[serde(default)]
    pub fuzzy: Option<bool>,
    /// Fuzzy search threshold (0.0 = exact match, 1.0 = very loose)
    #[serde(default)]
    pub fuzzy_threshold: Option<f64>,
    /// Maximum number of results per file
    #[serde(default)]
    pub max_results: Option<usize>,
    /// Exclude directories (e.g., ["target", "node_modules"])
    #[serde(default)]
    pub exclude: Option<Vec<String>>,
    /// Sort results by relevance score
    #[serde(default)]
    pub rank: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListFilesParams {
    /// Directory to scan (default: current directory)
    #[serde(default)]
    pub path: Option<String>,
    /// File extensions to include (e.g., ["rs", "py", "js"])
    #[serde(default)]
    pub extensions: Option<Vec<String>>,
    /// Exclude directories (e.g., ["target", "node_modules"])
    #[serde(default)]
    pub exclude: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AnalyzeCodebaseParams {
    /// Directory to analyze (default: current directory)
    #[serde(default)]
    pub path: Option<String>,
    /// File extensions to include (e.g., ["rs", "py", "js"])
    #[serde(default)]
    pub extensions: Option<Vec<String>>,
    /// Exclude directories (e.g., ["target", "node_modules"])
    #[serde(default)]
    pub exclude: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ComplexityParams {
    /// Directory to analyze (default: current directory)
    #[serde(default)]
    pub path: Option<String>,
    /// File extensions to include (e.g., ["rs", "py", "js"])
    #[serde(default)]
    pub extensions: Option<Vec<String>>,
    /// Exclude directories (e.g., ["target", "node_modules"])
    #[serde(default)]
    pub exclude: Option<Vec<String>>,
    /// Show only files above complexity threshold
    #[serde(default)]
    pub threshold: Option<u32>,
    /// Sort by complexity (highest first)
    #[serde(default)]
    pub sort: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DuplicatesParams {
    /// Directory to analyze (default: current directory)
    #[serde(default)]
    pub path: Option<String>,
    /// File extensions to include (e.g., ["rs", "py", "js"])
    #[serde(default)]
    pub extensions: Option<Vec<String>>,
    /// Exclude directories (e.g., ["target", "node_modules"])
    #[serde(default)]
    pub exclude: Option<Vec<String>>,
    /// Minimum number of lines to consider as duplicate
    #[serde(default)]
    pub min_lines: Option<usize>,
    /// Similarity threshold (0.0-1.0, default: 0.9)
    #[serde(default)]
    pub similarity: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DeadcodeParams {
    /// Directory to analyze (default: current directory)
    #[serde(default)]
    pub path: Option<String>,
    /// File extensions to include (e.g., ["rs", "py", "js"])
    #[serde(default)]
    pub extensions: Option<Vec<String>>,
    /// Exclude directories (e.g., ["target", "node_modules"])
    #[serde(default)]
    pub exclude: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CircularParams {
    /// Directory to analyze (default: current directory)
    #[serde(default)]
    pub path: Option<String>,
    /// File extensions to include (e.g., ["rs", "py", "js"])
    #[serde(default)]
    pub extensions: Option<Vec<String>>,
    /// Exclude directories (e.g., ["target", "node_modules"])
    #[serde(default)]
    pub exclude: Option<Vec<String>>,
}
