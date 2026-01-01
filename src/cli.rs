//! CLI argument definitions and parsing
//!
//! This module contains all command-line interface definitions using clap.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "codesearch")]
#[command(about = "A fast CLI tool for searching codebases")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    
    /// Search query pattern
    #[arg(index = 1)]
    pub query: Option<String>,
    
    /// Path to search (file or directory, default: current directory)
    #[arg(index = 2, default_value = ".")]
    pub path: PathBuf,
    
    /// File extensions to include (e.g., rs,py,js)
    #[arg(short, long, value_delimiter = ',')]
    pub extensions: Option<Vec<String>>,
    
    /// Enable fuzzy search
    #[arg(short, long)]
    pub fuzzy: bool,
    
    /// Case-insensitive search (default: true)
    #[arg(short, long, default_value = "true")]
    pub ignore_case: bool,
    
    /// Maximum results per file
    #[arg(short, long, default_value = "10")]
    pub max_results: usize,
    
    /// Exclude directories
    #[arg(long, value_delimiter = ',')]
    pub exclude: Option<Vec<String>>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Search for text patterns in code files
    Search {
        /// The search query (supports regex)
        query: String,
        /// Path to search (file or directory, default: current directory)
        #[arg(default_value = ".")]
        path: PathBuf,
        /// File extensions to include (e.g., rs,py,js)
        #[arg(short, long, value_delimiter = ',')]
        extensions: Option<Vec<String>>,
        /// Case-insensitive search
        #[arg(short, long)]
        ignore_case: bool,
        /// Hide line numbers (line numbers shown by default)
        #[arg(short = 'N', long)]
        no_line_numbers: bool,
        /// Maximum number of results per file
        #[arg(long, default_value = "10")]
        max_results: usize,
        /// Output format (text, json)
        #[arg(long, default_value = "text")]
        format: String,
        /// Show search statistics
        #[arg(long)]
        stats: bool,
        /// Enable fuzzy search (handles typos and variations)
        #[arg(long)]
        fuzzy: bool,
        /// Fuzzy search threshold (0.0 = exact match, 1.0 = very loose)
        #[arg(long, default_value = "0.6")]
        fuzzy_threshold: f64,
        /// Exclude directories (default: auto-excludes common build dirs)
        #[arg(long, value_delimiter = ',')]
        exclude: Option<Vec<String>>,
        /// Don't auto-exclude common build directories
        #[arg(long)]
        no_auto_exclude: bool,
        /// Sort results by relevance score
        #[arg(long)]
        rank: bool,
        /// Enable intelligent caching for faster repeated searches
        #[arg(long)]
        cache: bool,
        /// Enable semantic search (context-aware matching)
        #[arg(long)]
        semantic: bool,
        /// Performance benchmark mode
        #[arg(long)]
        benchmark: bool,
        /// Compare performance with grep
        #[arg(long)]
        vs_grep: bool,
        /// Export results to file (csv, markdown, md)
        #[arg(long)]
        export: Option<String>,
    },
    /// List all searchable files
    Files {
        /// Path to scan (default: current directory)
        #[arg(default_value = ".")]
        path: PathBuf,
        /// File extensions to include (e.g., rs,py,js)
        #[arg(short, long, value_delimiter = ',')]
        extensions: Option<Vec<String>>,
        /// Exclude directories (e.g., target,node_modules)
        #[arg(long, value_delimiter = ',')]
        exclude: Option<Vec<String>>,
    },
    /// Interactive search mode
    Interactive {
        /// Path to search (default: current directory)
        #[arg(default_value = ".")]
        path: PathBuf,
        /// File extensions to include (e.g., rs,py,js)
        #[arg(short, long, value_delimiter = ',')]
        extensions: Option<Vec<String>>,
        /// Exclude directories (e.g., target,node_modules)
        #[arg(long, value_delimiter = ',')]
        exclude: Option<Vec<String>>,
    },
    /// Analyze codebase metrics and statistics
    Analyze {
        /// Path to analyze (default: current directory)
        #[arg(default_value = ".")]
        path: PathBuf,
        /// File extensions to include (e.g., rs,py,js)
        #[arg(short, long, value_delimiter = ',')]
        extensions: Option<Vec<String>>,
        /// Exclude directories (e.g., target,node_modules)
        #[arg(long, value_delimiter = ',')]
        exclude: Option<Vec<String>>,
    },
    /// Analyze code complexity metrics
    Complexity {
        /// Path to analyze (default: current directory)
        #[arg(default_value = ".")]
        path: PathBuf,
        /// File extensions to include (e.g., rs,py,js)
        #[arg(short, long, value_delimiter = ',')]
        extensions: Option<Vec<String>>,
        /// Exclude directories (e.g., target,node_modules)
        #[arg(long, value_delimiter = ',')]
        exclude: Option<Vec<String>>,
        /// Show only files above complexity threshold
        #[arg(long)]
        threshold: Option<u32>,
        /// Sort by complexity (highest first)
        #[arg(long)]
        sort: bool,
    },
    /// Analyze design metrics (coupling, cohesion, instability)
    DesignMetrics {
        /// Path to analyze (default: current directory)
        #[arg(default_value = ".")]
        path: PathBuf,
        /// File extensions to include (e.g., rs,py,js)
        #[arg(short, long, value_delimiter = ',')]
        extensions: Option<Vec<String>>,
        /// Exclude directories (e.g., target,node_modules)
        #[arg(long, value_delimiter = ',')]
        exclude: Option<Vec<String>>,
        /// Show detailed metrics for each module
        #[arg(long)]
        detailed: bool,
        /// Output format (text, json)
        #[arg(long, default_value = "text")]
        format: String,
    },
    /// Comprehensive code metrics (complexity, size, maintainability)
    Metrics {
        /// Path to analyze (default: current directory)
        #[arg(default_value = ".")]
        path: PathBuf,
        /// File extensions to include (e.g., rs,py,js)
        #[arg(short, long, value_delimiter = ',')]
        extensions: Option<Vec<String>>,
        /// Exclude directories (e.g., target,node_modules)
        #[arg(long, value_delimiter = ',')]
        exclude: Option<Vec<String>>,
        /// Show detailed metrics for each file
        #[arg(long)]
        detailed: bool,
        /// Output format (text, json)
        #[arg(long, default_value = "text")]
        format: String,
    },
    /// Detect code duplication in the codebase
    Duplicates {
        /// Path to analyze (default: current directory)
        #[arg(default_value = ".")]
        path: PathBuf,
        /// File extensions to include (e.g., rs,py,js)
        #[arg(short, long, value_delimiter = ',')]
        extensions: Option<Vec<String>>,
        /// Exclude directories (e.g., target,node_modules)
        #[arg(long, value_delimiter = ',')]
        exclude: Option<Vec<String>>,
        /// Minimum lines for a duplicate block
        #[arg(long, default_value = "3")]
        min_lines: usize,
        /// Similarity threshold (0.0 - 1.0)
        #[arg(long, default_value = "0.9")]
        similarity: f64,
    },
    /// Detect potentially dead/unused code
    Deadcode {
        /// Path to analyze (default: current directory)
        #[arg(default_value = ".")]
        path: PathBuf,
        /// File extensions to include (e.g., rs,py,js)
        #[arg(short, long, value_delimiter = ',')]
        extensions: Option<Vec<String>>,
        /// Exclude directories (e.g., target,node_modules)
        #[arg(long, value_delimiter = ',')]
        exclude: Option<Vec<String>>,
    },
    /// Detect circular function calls
    Circular {
        /// Path to analyze (default: current directory)
        #[arg(default_value = ".")]
        path: PathBuf,
        /// File extensions to include (e.g., rs,py,js)
        #[arg(short, long, value_delimiter = ',')]
        extensions: Option<Vec<String>>,
        /// Exclude directories (e.g., target,node_modules)
        #[arg(long, value_delimiter = ',')]
        exclude: Option<Vec<String>>,
    },
    /// List all supported programming languages
    Languages,
    /// Run as MCP server
    McpServer,
    /// Build or update code index for faster searches
    Index {
        /// Path to index (default: current directory)
        #[arg(default_value = ".")]
        path: PathBuf,
        /// File extensions to include (e.g., rs,py,js)
        #[arg(short, long, value_delimiter = ',')]
        extensions: Option<Vec<String>>,
        /// Exclude directories (e.g., target,node_modules)
        #[arg(long, value_delimiter = ',')]
        exclude: Option<Vec<String>>,
        /// Index file path
        #[arg(long, default_value = ".codesearch/index.json")]
        index_file: PathBuf,
    },
    /// Watch directory for changes and update index
    Watch {
        /// Path to watch (default: current directory)
        #[arg(default_value = ".")]
        path: PathBuf,
        /// File extensions to watch (e.g., rs,py,js)
        #[arg(short, long, value_delimiter = ',')]
        extensions: Option<Vec<String>>,
        /// Index file path
        #[arg(long, default_value = ".codesearch/index.json")]
        index_file: PathBuf,
    },
    /// Analyze code using AST (Abstract Syntax Tree)
    Ast {
        /// Path to analyze (file or directory)
        #[arg(default_value = ".")]
        path: PathBuf,
        /// File extensions to analyze (e.g., rs,py,js)
        #[arg(short, long, value_delimiter = ',')]
        extensions: Option<Vec<String>>,
        /// Output format (text, json)
        #[arg(long, default_value = "text")]
        format: String,
    },
    /// Analyze Control Flow Graph (CFG)
    Cfg {
        /// Path to analyze (file or directory)
        #[arg(default_value = ".")]
        path: PathBuf,
        /// File extensions to analyze (e.g., rs,py,js)
        #[arg(short, long, value_delimiter = ',')]
        extensions: Option<Vec<String>>,
        /// Output format (text, json, dot)
        #[arg(long, default_value = "text")]
        format: String,
        /// Export to file
        #[arg(long)]
        export: Option<PathBuf>,
    },
    /// Analyze Data Flow Graph (DFG)
    Dfg {
        /// Path to analyze (file or directory)
        #[arg(default_value = ".")]
        path: PathBuf,
        /// File extensions to analyze (e.g., rs,py,js)
        #[arg(short, long, value_delimiter = ',')]
        extensions: Option<Vec<String>>,
        /// Output format (text, json, dot)
        #[arg(long, default_value = "text")]
        format: String,
        /// Export to file
        #[arg(long)]
        export: Option<PathBuf>,
    },
    /// Analyze Call Graph
    Callgraph {
        /// Path to analyze (default: current directory)
        #[arg(default_value = ".")]
        path: PathBuf,
        /// File extensions to include (e.g., rs,py,js)
        #[arg(short, long, value_delimiter = ',')]
        extensions: Option<Vec<String>>,
        /// Exclude directories (e.g., target,node_modules)
        #[arg(long, value_delimiter = ',')]
        exclude: Option<Vec<String>>,
        /// Output format (text, json, dot)
        #[arg(long, default_value = "text")]
        format: String,
        /// Show only recursive functions
        #[arg(long)]
        recursive_only: bool,
        /// Show only dead functions
        #[arg(long)]
        dead_only: bool,
    },
    /// Analyze Program Dependency Graph (PDG)
    Pdg {
        /// Path to analyze (file or directory)
        #[arg(default_value = ".")]
        path: PathBuf,
        /// File extensions to analyze (e.g., rs,py,js)
        #[arg(short, long, value_delimiter = ',')]
        extensions: Option<Vec<String>>,
        /// Output format (text, json, dot)
        #[arg(long, default_value = "text")]
        format: String,
        /// Show parallelization opportunities
        #[arg(long)]
        parallel: bool,
        /// Export to file
        #[arg(long)]
        export: Option<PathBuf>,
    },
    /// Analyze all graph types for a file
    GraphAll {
        /// Path to analyze (file)
        path: PathBuf,
        /// Output format (text, json)
        #[arg(long, default_value = "text")]
        format: String,
        /// Export directory for graph files
        #[arg(long)]
        export_dir: Option<PathBuf>,
    },
    /// Build and analyze dependency graph
    Depgraph {
        /// Path to analyze (default: current directory)
        #[arg(default_value = ".")]
        path: PathBuf,
        /// File extensions to include (e.g., rs,py,js)
        #[arg(short, long, value_delimiter = ',')]
        extensions: Option<Vec<String>>,
        /// Exclude directories (e.g., target,node_modules)
        #[arg(long, value_delimiter = ',')]
        exclude: Option<Vec<String>>,
        /// Output format (text, json, dot)
        #[arg(long, default_value = "text")]
        format: String,
        /// Show circular dependencies only
        #[arg(long)]
        circular_only: bool,
    },
    /// Search git history
    GitHistory {
        /// Search pattern
        query: String,
        /// Repository path (default: current directory)
        #[arg(default_value = ".")]
        path: PathBuf,
        /// Maximum commits to search
        #[arg(long, default_value = "100")]
        max_commits: usize,
        /// Search by author
        #[arg(long)]
        author: Option<String>,
        /// Search in commit messages
        #[arg(long)]
        message: bool,
        /// Search specific file history
        #[arg(long)]
        file: Option<String>,
    },
    /// Search remote repositories
    Remote {
        /// Search pattern
        query: String,
        /// Repository URL or search on GitHub/GitLab
        #[arg(long)]
        repo: Option<String>,
        /// File extensions to include (e.g., rs,py,js)
        #[arg(short, long, value_delimiter = ',')]
        extensions: Option<Vec<String>>,
        /// GitHub API token (or set GITHUB_TOKEN env var)
        #[arg(long)]
        token: Option<String>,
        /// Search on GitHub
        #[arg(long)]
        github: bool,
        /// Language filter for GitHub search
        #[arg(long)]
        language: Option<String>,
        /// Maximum results
        #[arg(long, default_value = "20")]
        max_results: usize,
    },
}

/// Default directories to exclude from search
pub fn get_default_exclude_dirs() -> Vec<String> {
    vec![
        "target".to_string(),
        "node_modules".to_string(),
        ".git".to_string(),
        "build".to_string(),
        "dist".to_string(),
        "__pycache__".to_string(),
        ".venv".to_string(),
        "vendor".to_string(),
    ]
}
