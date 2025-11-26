// MCP Server implementation using rmcp 0.8
// This module provides MCP server functionality for code search operations

#[cfg(feature = "mcp")]
use crate::*;
#[cfg(feature = "mcp")]
use std::path::PathBuf;

#[cfg(feature = "mcp")]
use rmcp::{
    tool, tool_router, tool_handler, ServerHandler,
    handler::server::{
        tool::ToolRouter,
        wrapper::{Parameters, Json},
    },
    transport::io::stdio,
    service::{RoleServer, serve_server},
};
#[cfg(feature = "mcp")]
use schemars::JsonSchema;
#[cfg(feature = "mcp")]
use serde::{Deserialize, Serialize};

// Implement JsonSchema for types that need it for MCP
#[cfg(feature = "mcp")]
impl JsonSchema for SearchResult {
    fn schema_name() -> String {
        "SearchResult".to_string()
    }
    fn json_schema(_generator: &mut schemars::r#gen::SchemaGenerator) -> schemars::Schema {
        schemars::Schema::Object(schemars::schema::SchemaObject::default())
    }
}

#[cfg(feature = "mcp")]
impl JsonSchema for FileInfo {
    fn schema_name() -> String {
        "FileInfo".to_string()
    }
    fn json_schema(_generator: &mut schemars::r#gen::SchemaGenerator) -> schemars::Schema {
        schemars::Schema::Object(schemars::schema::SchemaObject::default())
    }
}

#[cfg(feature = "mcp")]
impl JsonSchema for Match {
    fn schema_name() -> String {
        "Match".to_string()
    }
    fn json_schema(_generator: &mut schemars::r#gen::SchemaGenerator) -> schemars::Schema {
        schemars::Schema::Object(schemars::schema::SchemaObject::default())
    }
}

// Parameter structs for MCP tools
#[cfg(feature = "mcp")]
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

#[cfg(feature = "mcp")]
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

#[cfg(feature = "mcp")]
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

#[cfg(feature = "mcp")]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SuggestRefactoringParams {
    /// Directory to analyze (default: current directory)
    #[serde(default)]
    pub path: Option<String>,
    /// File extensions to include (e.g., ["rs", "py", "js"])
    #[serde(default)]
    pub extensions: Option<Vec<String>>,
    /// Exclude directories (e.g., ["target", "node_modules"])
    #[serde(default)]
    pub exclude: Option<Vec<String>>,
    /// Show only high-priority suggestions
    #[serde(default)]
    pub high_priority: Option<bool>,
}

#[cfg(feature = "mcp")]
#[derive(Debug, Clone)]
pub struct CodeSearchMcpService {
    tool_router: ToolRouter<Self>,
}

#[cfg(feature = "mcp")]
#[tool_router]
impl CodeSearchMcpService {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    /// Search for text patterns in code files with advanced options like fuzzy matching, regex, and filtering
    #[tool(description = "Search for text patterns in code files with advanced options like fuzzy matching, regex, and filtering")]
    pub async fn search_code(
        &self,
        params: Parameters<SearchCodeParams>,
    ) -> Json<Vec<SearchResult>> {
        let params = params.0;
        let path_buf = PathBuf::from(params.path.as_deref().unwrap_or("."));
        
        Json(search_code(
            &params.query,
            &path_buf,
            params.extensions.as_deref(),
            params.ignore_case.unwrap_or(false),
            params.fuzzy.unwrap_or(false),
            params.fuzzy_threshold.unwrap_or(0.6),
            params.max_results.unwrap_or(10),
            params.exclude.as_deref(),
            params.rank.unwrap_or(false),
            false, // cache
            false, // semantic
            false, // benchmark
            false, // vs_grep
        ).unwrap_or_default())
    }

    /// List all searchable files in a directory with optional filtering by extensions
    #[tool(description = "List all searchable files in a directory with optional filtering by extensions")]
    pub async fn list_files(
        &self,
        params: Parameters<ListFilesParams>,
    ) -> Json<Vec<FileInfo>> {
        let params = params.0;
        let path_buf = PathBuf::from(params.path.as_deref().unwrap_or("."));
        
        Json(list_files(
            &path_buf,
            params.extensions.as_deref(),
            params.exclude.as_deref(),
        ).unwrap_or_default())
    }

    /// Analyze codebase metrics and statistics. Returns JSON with file counts, line counts, and code patterns
    #[tool(description = "Analyze codebase metrics and statistics. Returns JSON with file counts, line counts, and code patterns")]
    pub async fn analyze_codebase(
        &self,
        params: Parameters<AnalyzeCodebaseParams>,
    ) -> Json<serde_json::Value> {
        let params = params.0;
        let path_buf = PathBuf::from(params.path.as_deref().unwrap_or("."));
        
        let files = list_files(
            &path_buf,
            params.extensions.as_deref(),
            params.exclude.as_deref(),
        ).unwrap_or_default();
        
        if files.is_empty() {
            return Json(serde_json::json!({
                "message": "No files found to analyze",
                "files": []
            }));
        }

        // Calculate statistics
        let total_files = files.len();
        let total_lines: usize = files.iter().map(|f| f.lines).sum();
        let total_size: u64 = files.iter().map(|f| f.size).sum();
        
        // File type breakdown
        let mut ext_counts: std::collections::HashMap<String, (usize, usize, u64)> = std::collections::HashMap::new();
        for file in &files {
            let ext = if let Some(ext) = std::path::Path::new(&file.path).extension() {
                ext.to_string_lossy().to_string()
            } else {
                "no extension".to_string()
            };
            
            let entry = ext_counts.entry(ext).or_insert((0, 0, 0));
            entry.0 += 1;
            entry.1 += file.lines;
            entry.2 += file.size;
        }

        // Code patterns analysis
        let patterns = vec![
            ("functions", r"fn\s+\w+|function\s+\w+|def\s+\w+"),
            ("classes", r"class\s+\w+"),
            ("comments", r"//|#|/\*|<!--"),
            ("todo", r"TODO|FIXME|HACK|XXX"),
            ("imports", r"^import|^use|^#include|^require"),
        ];

        let mut pattern_counts = std::collections::HashMap::new();
        for (name, pattern) in patterns {
            if let Ok(regex) = regex::Regex::new(pattern) {
                let mut total_matches = 0;
                
                for file in &files {
                    if let Ok(content) = std::fs::read_to_string(&file.path) {
                        for line in content.lines() {
                            if regex.is_match(line) {
                                total_matches += 1;
                            }
                        }
                    }
                }
                pattern_counts.insert(name, total_matches);
            }
        }

        // Largest files
        let mut file_vec = files.clone();
        file_vec.sort_by(|a, b| b.lines.cmp(&a.lines));
        let largest_files: Vec<_> = file_vec.iter().take(5).map(|f| {
            serde_json::json!({
                "path": f.path,
                "lines": f.lines,
                "size": f.size
            })
        }).collect();

        Json(serde_json::json!({
            "total_files": total_files,
            "total_lines": total_lines,
            "total_size": total_size,
            "file_types": ext_counts.iter().map(|(k, v)| {
                serde_json::json!({
                    "extension": k,
                    "count": v.0,
                    "lines": v.1,
                    "size": v.2,
                    "percentage": (v.0 as f64 / total_files as f64 * 100.0) as u32
                })
            }).collect::<Vec<_>>(),
            "patterns": pattern_counts,
            "largest_files": largest_files,
            "average_lines_per_file": total_lines as f64 / total_files as f64,
            "average_size_per_file": total_size as f64 / total_files as f64
        }))
    }

    /// Suggest code refactoring improvements with priority levels. Returns JSON with suggestions
    #[tool(description = "Suggest code refactoring improvements with priority levels. Returns JSON with suggestions")]
    pub async fn suggest_refactoring(
        &self,
        params: Parameters<SuggestRefactoringParams>,
    ) -> Json<serde_json::Value> {
        let params = params.0;
        let path_buf = PathBuf::from(params.path.as_deref().unwrap_or("."));
        
        let files = list_files(
            &path_buf,
            params.extensions.as_deref(),
            params.exclude.as_deref(),
        ).unwrap_or_default();
        
        let mut suggestions = Vec::new();

        for file in &files {
            if let Ok(content) = std::fs::read_to_string(&file.path) {
                analyze_file_for_refactoring(&file.path, &content, &mut suggestions);
            }
        }

        // Sort by priority (highest first)
        suggestions.sort_by(|a, b| b.priority.cmp(&a.priority));

        // Filter by priority if requested
        let filtered_suggestions = if params.high_priority.unwrap_or(false) {
            suggestions.into_iter().filter(|s| s.priority >= 7).collect::<Vec<_>>()
        } else {
            suggestions
        };

        if filtered_suggestions.is_empty() {
            return Json(serde_json::json!({
                "message": "No refactoring suggestions found! Your code looks good.",
                "suggestions": []
            }));
        }

        // Group suggestions by type
        let mut grouped: std::collections::HashMap<String, Vec<&RefactorSuggestion>> = std::collections::HashMap::new();
        for suggestion in &filtered_suggestions {
            grouped.entry(suggestion.suggestion_type.clone()).or_insert_with(Vec::new).push(suggestion);
        }

        let grouped_json: std::collections::HashMap<String, Vec<serde_json::Value>> = grouped.iter().map(|(k, v)| {
            (k.clone(), v.iter().map(|s| {
                serde_json::json!({
                    "file": s.file,
                    "line_number": s.line_number,
                    "description": s.description,
                    "priority": s.priority,
                    "code_snippet": s.code_snippet,
                    "improvement": s.improvement
                })
            }).collect())
        }).collect();

        Json(serde_json::json!({
            "total_suggestions": filtered_suggestions.len(),
            "suggestions_by_type": grouped_json
        }))
    }
}

#[cfg(feature = "mcp")]
#[tool_handler(router = self.tool_router)]
impl ServerHandler for CodeSearchMcpService {}

#[cfg(feature = "mcp")]
pub async fn run_mcp_server() -> Result<(), Box<dyn std::error::Error>> {
    use tokio::io::{stdin, stdout};
    
    let service = CodeSearchMcpService::new();
    let transport = stdio((stdin(), stdout()));
    
    serve_server::<RoleServer, _>(service, transport).await?;
    
    Ok(())
}

#[cfg(not(feature = "mcp"))]
pub async fn run_mcp_server() -> Result<(), Box<dyn std::error::Error>> {
    Err("MCP server support not enabled. Build with --features mcp".into())
}
