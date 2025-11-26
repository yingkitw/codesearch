mod mcp_server;

use clap::{Parser, Subcommand};
use colored::*;
use regex::Regex;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;
use walkdir::WalkDir;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use rayon::prelude::*;
use dashmap::DashMap;

#[derive(Parser)]
#[command(name = "codesearch")]
#[command(about = "A fast CLI tool for searching codebases")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    
    /// Search query (simple search without subcommand)
    #[arg(last = true)]
    query: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Search for text patterns in code files
    Search {
        /// The search query (supports regex)
        query: String,
        /// Directory to search in (default: current directory)
        #[arg(short, long, default_value = ".")]
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
    },
    /// List all searchable files
    Files {
        /// Directory to scan (default: current directory)
        #[arg(short, long, default_value = ".")]
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
        /// Directory to search in (default: current directory)
        #[arg(short, long, default_value = ".")]
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
        /// Directory to analyze (default: current directory)
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
        /// File extensions to include (e.g., rs,py,js)
        #[arg(short, long, value_delimiter = ',')]
        extensions: Option<Vec<String>>,
        /// Exclude directories (e.g., target,node_modules)
        #[arg(long, value_delimiter = ',')]
        exclude: Option<Vec<String>>,
    },
    /// Suggest code refactoring improvements
    Refactor {
        /// Directory to analyze (default: current directory)
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
        /// File extensions to include (e.g., rs,py,js)
        #[arg(short, long, value_delimiter = ',')]
        extensions: Option<Vec<String>>,
        /// Exclude directories (e.g., target,node_modules)
        #[arg(long, value_delimiter = ',')]
        exclude: Option<Vec<String>>,
        /// Show only high-priority suggestions
        #[arg(long)]
        high_priority: bool,
    },
    /// Manage search favorites and history
    Favorites {
        /// List all favorites
        #[arg(long)]
        list: bool,
        /// Add current search to favorites
        #[arg(long)]
        add: Option<String>,
        /// Remove a favorite by name
        #[arg(long)]
        remove: Option<String>,
        /// Clear all favorites
        #[arg(long)]
        clear: bool,
        /// Show search history
        #[arg(long)]
        history: bool,
    },
    /// Run as MCP server
    McpServer,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchResult {
    file: String,
    line_number: usize,
    content: String,
    matches: Vec<Match>,
    score: f64,
    relevance: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Match {
    start: usize,
    end: usize,
    text: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileInfo {
    path: String,
    size: u64,
    lines: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Handle simple search without subcommand: codesearch "query"
    if cli.command.is_none() {
        if let Some(query) = cli.query {
            let results = search_code(
                &query,
                &PathBuf::from("."),
                None,
                true, // ignore_case by default for simple usage
                false, // fuzzy
                0.6,
                10,
                Some(&get_default_exclude_dirs()),
                false, // rank
                false, // cache
                false, // semantic
                false, // benchmark
                false, // vs_grep
            )?;
            
            print_results(&results, true, false); // line_numbers=true by default
            print_search_stats(&results, &query);
            return Ok(());
        } else {
            // No command and no query - show help
            Cli::parse_from(&["codesearch", "--help"]);
            return Ok(());
        }
    }

    match cli.command {
        Some(Commands::Search {
            query,
            path,
            extensions,
            ignore_case,
            no_line_numbers,
            max_results,
            format,
            stats,
            fuzzy,
            fuzzy_threshold,
            exclude,
            rank,
            cache,
            semantic,
            benchmark,
            vs_grep,
            no_auto_exclude,
        }) => {
            // Auto-exclude common build directories unless disabled
            let final_exclude = if no_auto_exclude {
                exclude
            } else {
                let mut auto_exclude = get_default_exclude_dirs();
                if let Some(mut user_exclude) = exclude {
                    auto_exclude.append(&mut user_exclude);
                }
                Some(auto_exclude)
            };
            
            let results = search_code(
                &query,
                &path,
                extensions.as_deref(),
                ignore_case,
                fuzzy,
                fuzzy_threshold,
                max_results,
                final_exclude.as_deref(),
                rank,
                cache,
                semantic,
                benchmark,
                vs_grep,
            )?;

            match format.as_str() {
                "json" => {
                    let json = serde_json::to_string_pretty(&results)?;
                    println!("{}", json);
                }
                _ => {
                    // Line numbers shown by default unless --no-line-numbers is used
                    print_results(&results, !no_line_numbers, rank);
                    // Show stats by default for better UX
                    if stats || results.len() > 0 {
                        print_search_stats(&results, &query);
                    }
                }
            }
        }
        Some(Commands::Files {
            path,
            extensions,
            exclude,
        }) => {
            let files = list_files(&path, extensions.as_deref(), exclude.as_deref())?;
            match extensions {
                Some(_) => {
                    for file in files {
                        println!("{}", file.path);
                    }
                }
                None => {
                    let json = serde_json::to_string_pretty(&files)?;
                    println!("{}", json);
                }
            }
        }
        Some(Commands::Interactive {
            path,
            extensions,
            exclude,
        }) => {
            interactive_search(&path, extensions.as_deref(), exclude.as_deref())?;
        }
        Some(Commands::Analyze {
            path,
            extensions,
            exclude,
        }) => {
            analyze_codebase(&path, extensions.as_deref(), exclude.as_deref())?;
        }
        Some(Commands::Refactor {
            path,
            extensions,
            exclude,
            high_priority,
        }) => {
            suggest_refactoring(&path, extensions.as_deref(), exclude.as_deref(), high_priority)?;
        }
        Some(Commands::Favorites {
            list,
            add,
            remove,
            clear,
            history,
        }) => {
            manage_favorites(list, add, remove, clear, history)?;
        }
        Some(Commands::McpServer) => {
            #[cfg(feature = "mcp")]
            {
                use tokio::runtime::Runtime;
                let rt = Runtime::new()?;
                rt.block_on(mcp_server::run_mcp_server())?;
            }
            #[cfg(not(feature = "mcp"))]
            {
                eprintln!("MCP server support not enabled. Build with: cargo build --features mcp");
                eprintln!("Or add to Cargo.toml: [features] default = [\"mcp\"]");
                std::process::exit(1);
            }
        }
        None => {
            // This shouldn't happen as we handle None above
            // But if it does, show help
            Cli::parse_from(&["codesearch", "--help"]);
        }
    }

    Ok(())
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "mcp", derive(schemars::JsonSchema))]
pub struct RefactorSuggestion {
    file: String,
    line_number: usize,
    suggestion_type: String,
    description: String,
    priority: u8, // 1-10, 10 being highest priority
    code_snippet: String,
    improvement: String,
}

pub fn suggest_refactoring(
    path: &Path,
    extensions: Option<&[String]>,
    exclude: Option<&[String]>,
    high_priority_only: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "🔧 Code Refactoring Suggestions".cyan().bold());
    println!("{}", "─".repeat(30).cyan());
    println!();

    let files = list_files(path, extensions, exclude)?;
    let mut suggestions = Vec::new();

    for file in &files {
        if let Ok(content) = fs::read_to_string(&file.path) {
            analyze_file_for_refactoring(&file.path, &content, &mut suggestions);
        }
    }

    // Sort by priority (highest first)
    suggestions.sort_by(|a, b| b.priority.cmp(&a.priority));

    // Filter by priority if requested
    let filtered_suggestions = if high_priority_only {
        suggestions.into_iter().filter(|s| s.priority >= 7).collect::<Vec<_>>()
    } else {
        suggestions
    };

    if filtered_suggestions.is_empty() {
        println!("{}", "✨ No refactoring suggestions found! Your code looks good.".green().italic());
        return Ok(());
    }

    // Group suggestions by type
    let mut grouped: std::collections::HashMap<String, Vec<&RefactorSuggestion>> = std::collections::HashMap::new();
    for suggestion in &filtered_suggestions {
        grouped.entry(suggestion.suggestion_type.clone()).or_insert_with(Vec::new).push(suggestion);
    }

    for (suggestion_type, type_suggestions) in grouped {
        println!("{}", format!("📋 {} ({})", suggestion_type, type_suggestions.len()).yellow().bold());
        println!("{}", "─".repeat(suggestion_type.len() + 15).yellow());
        
        for suggestion in type_suggestions {
            let priority_color = match suggestion.priority {
                8..=10 => "red".to_string(),
                5..=7 => "yellow".to_string(),
                _ => "green".to_string(),
            };
            
            println!("  {} {} {}", 
                format!("[{}]", suggestion.priority).color(priority_color).bold(),
                suggestion.file.blue().bold(),
                format!("line {}", suggestion.line_number).cyan()
            );
            println!("  {}", suggestion.description.italic());
            println!("  {} {}", "Current:".dimmed(), suggestion.code_snippet.dimmed());
            println!("  {} {}", "Better:".green(), suggestion.improvement.green());
            println!();
        }
    }

    println!("{}", format!("💡 Total suggestions: {}", filtered_suggestions.len()).cyan().bold());
    println!("{}", "✨ Refactoring analysis completed!".green().italic());

    Ok(())
}

pub fn analyze_file_for_refactoring(
    file_path: &str,
    content: &str,
    suggestions: &mut Vec<RefactorSuggestion>,
) {
    let lines: Vec<&str> = content.lines().collect();
    
    for (line_num, line) in lines.iter().enumerate() {
        let line_number = line_num + 1;
        let trimmed = line.trim();
        
        // Check for long lines
        if line.len() > 120 {
            suggestions.push(RefactorSuggestion {
                file: file_path.to_string(),
                line_number,
                suggestion_type: "Code Style".to_string(),
                description: "Line is too long (>120 characters)".to_string(),
                priority: 3,
                code_snippet: if line.len() > 50 { format!("{}...", &line[..50]) } else { line.to_string() },
                improvement: "Consider breaking into multiple lines or extracting variables".to_string(),
            });
        }
        
        // Check for deeply nested code
        let indent_level = line.len() - line.trim_start().len();
        if indent_level > 8 {
            suggestions.push(RefactorSuggestion {
                file: file_path.to_string(),
                line_number,
                suggestion_type: "Complexity".to_string(),
                description: "Deeply nested code (indentation > 8 levels)".to_string(),
                priority: 6,
                code_snippet: trimmed.to_string(),
                improvement: "Consider extracting functions or using early returns to reduce nesting".to_string(),
            });
        }
        
        // Check for TODO comments
        if trimmed.to_uppercase().contains("TODO") || trimmed.to_uppercase().contains("FIXME") {
            suggestions.push(RefactorSuggestion {
                file: file_path.to_string(),
                line_number,
                suggestion_type: "Technical Debt".to_string(),
                description: "TODO/FIXME comment found".to_string(),
                priority: 5,
                code_snippet: trimmed.to_string(),
                improvement: "Address the TODO item or remove if no longer needed".to_string(),
            });
        }
        
        // Check for magic numbers
        if regex::Regex::new(r"\b\d{3,}\b").unwrap().is_match(trimmed) {
            suggestions.push(RefactorSuggestion {
                file: file_path.to_string(),
                line_number,
                suggestion_type: "Code Quality".to_string(),
                description: "Magic number detected (3+ digits)".to_string(),
                priority: 4,
                code_snippet: trimmed.to_string(),
                improvement: "Replace with named constants or configuration values".to_string(),
            });
        }
        
        // Check for commented code
        if trimmed.starts_with("//") && !trimmed.starts_with("///") && !trimmed.starts_with("//!") {
            let code_part = trimmed.strip_prefix("//").unwrap().trim();
            if code_part.len() > 10 && !code_part.starts_with(" ") {
                suggestions.push(RefactorSuggestion {
                    file: file_path.to_string(),
                    line_number,
                    suggestion_type: "Code Cleanup".to_string(),
                    description: "Commented code found".to_string(),
                    priority: 2,
                    code_snippet: trimmed.to_string(),
                    improvement: "Remove commented code or uncomment if still needed".to_string(),
                });
            }
        }
        
        // Check for duplicate code patterns
        if trimmed.contains("if") && trimmed.contains("else") && trimmed.len() > 50 {
            suggestions.push(RefactorSuggestion {
                file: file_path.to_string(),
                line_number,
                suggestion_type: "Code Quality".to_string(),
                description: "Complex conditional statement".to_string(),
                priority: 4,
                code_snippet: if trimmed.len() > 50 { format!("{}...", &trimmed[..50]) } else { trimmed.to_string() },
                improvement: "Consider extracting to a separate function or using guard clauses".to_string(),
            });
        }
        
        // Check for long function signatures
        if trimmed.starts_with("fn ") || trimmed.starts_with("def ") || trimmed.starts_with("function ") {
            if trimmed.len() > 80 {
                suggestions.push(RefactorSuggestion {
                    file: file_path.to_string(),
                    line_number,
                    suggestion_type: "Function Design".to_string(),
                    description: "Long function signature".to_string(),
                    priority: 5,
                    code_snippet: if trimmed.len() > 50 { format!("{}...", &trimmed[..50]) } else { trimmed.to_string() },
                    improvement: "Consider reducing parameters or using a configuration object".to_string(),
                });
            }
        }
        
        // Check for hardcoded strings
        if trimmed.contains("\"") && !trimmed.contains("println") && !trimmed.contains("print") {
            let string_count = trimmed.matches("\"").count();
            if string_count >= 2 && trimmed.len() > 30 {
                suggestions.push(RefactorSuggestion {
                    file: file_path.to_string(),
                    line_number,
                    suggestion_type: "Code Quality".to_string(),
                    description: "Hardcoded string detected".to_string(),
                    priority: 3,
                    code_snippet: if trimmed.len() > 50 { format!("{}...", &trimmed[..50]) } else { trimmed.to_string() },
                    improvement: "Consider using constants or configuration files for string values".to_string(),
                });
            }
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct SearchFavorite {
    name: String,
    query: String,
    path: String,
    extensions: Option<Vec<String>>,
    ignore_case: bool,
    fuzzy: bool,
    fuzzy_threshold: f64,
    exclude: Option<Vec<String>>,
    created_at: String,
}

fn manage_favorites(
    list: bool,
    add: Option<String>,
    remove: Option<String>,
    clear: bool,
    history: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let favorites_file = std::env::var("HOME").unwrap_or_else(|_| ".".to_string()) + "/.codesearch-favorites.json";
    
    if list {
        list_favorites(&favorites_file)?;
    } else if let Some(name) = add {
        add_favorite(&favorites_file, &name)?;
    } else if let Some(name) = remove {
        remove_favorite(&favorites_file, &name)?;
    } else if clear {
        clear_favorites(&favorites_file)?;
    } else if history {
        show_search_history()?;
    } else {
        println!("{}", "Use --help to see available options".yellow());
    }
    
    Ok(())
}

fn list_favorites(favorites_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !std::path::Path::new(favorites_file).exists() {
        println!("{}", "No favorites found. Use --add to create your first favorite!".yellow());
        return Ok(());
    }
    
    let content = fs::read_to_string(favorites_file)?;
    let favorites: Vec<SearchFavorite> = serde_json::from_str(&content)?;
    
    if favorites.is_empty() {
        println!("{}", "No favorites found. Use --add to create your first favorite!".yellow());
        return Ok(());
    }
    
    println!("{}", "⭐ Search Favorites".cyan().bold());
    println!("{}", "─".repeat(20).cyan());
    println!();
    
    for (i, favorite) in favorites.iter().enumerate() {
        println!("{} {}", format!("{}.", i + 1).green().bold(), favorite.name.blue().bold());
        println!("  Query: {}", favorite.query.italic());
        println!("  Path: {}", favorite.path.dimmed());
        if let Some(exts) = &favorite.extensions {
            println!("  Extensions: {}", exts.join(", ").cyan());
        }
        if favorite.ignore_case {
            println!("  Case-insensitive: {}", "Yes".green());
        }
        if favorite.fuzzy {
            println!("  Fuzzy search: {} (threshold: {})", "Yes".green(), favorite.fuzzy_threshold);
        }
        if let Some(excl) = &favorite.exclude {
            println!("  Exclude: {}", excl.join(", ").red());
        }
        println!("  Created: {}", favorite.created_at.dimmed());
        println!();
    }
    
    Ok(())
}

fn add_favorite(favorites_file: &str, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "Adding new favorite...".cyan().bold());
    println!("{}", "Enter search details:".yellow());
    
    use std::io::{self, Write};
    
    print!("Query: ");
    io::stdout().flush()?;
    let mut query = String::new();
    io::stdin().read_line(&mut query)?;
    let query = query.trim().to_string();
    
    print!("Path (default: .): ");
    io::stdout().flush()?;
    let mut path = String::new();
    io::stdin().read_line(&mut path)?;
    let path = if path.trim().is_empty() { "." } else { path.trim() }.to_string();
    
    print!("Extensions (comma-separated, optional): ");
    io::stdout().flush()?;
    let mut extensions = String::new();
    io::stdin().read_line(&mut extensions)?;
    let extensions = if extensions.trim().is_empty() {
        None
    } else {
        Some(extensions.trim().split(',').map(|s| s.trim().to_string()).collect())
    };
    
    print!("Case-insensitive? (y/N): ");
    io::stdout().flush()?;
    let mut ignore_case = String::new();
    io::stdin().read_line(&mut ignore_case)?;
    let ignore_case = ignore_case.trim().to_lowercase() == "y";
    
    print!("Fuzzy search? (y/N): ");
    io::stdout().flush()?;
    let mut fuzzy = String::new();
    io::stdin().read_line(&mut fuzzy)?;
    let fuzzy = fuzzy.trim().to_lowercase() == "y";
    
    let fuzzy_threshold = if fuzzy {
        print!("Fuzzy threshold (0.0-1.0, default: 0.6): ");
        io::stdout().flush()?;
        let mut threshold = String::new();
        io::stdin().read_line(&mut threshold)?;
        threshold.trim().parse().unwrap_or(0.6)
    } else {
        0.6
    };
    
    print!("Exclude directories (comma-separated, optional): ");
    io::stdout().flush()?;
    let mut exclude = String::new();
    io::stdin().read_line(&mut exclude)?;
    let exclude = if exclude.trim().is_empty() {
        None
    } else {
        Some(exclude.trim().split(',').map(|s| s.trim().to_string()).collect())
    };
    
    let favorite = SearchFavorite {
        name: name.to_string(),
        query,
        path,
        extensions,
        ignore_case,
        fuzzy,
        fuzzy_threshold,
        exclude,
        created_at: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    };
    
    let mut favorites = if std::path::Path::new(favorites_file).exists() {
        let content = fs::read_to_string(favorites_file)?;
        serde_json::from_str(&content).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    };
    
    // Check if name already exists
    if favorites.iter().any(|f: &SearchFavorite| f.name == name) {
        println!("{}", format!("Favorite '{}' already exists!", name).red());
        return Ok(());
    }
    
    favorites.push(favorite);
    
    // Ensure directory exists
    if let Some(parent) = std::path::Path::new(favorites_file).parent() {
        fs::create_dir_all(parent)?;
    }
    
    let json = serde_json::to_string_pretty(&favorites)?;
    fs::write(favorites_file, json)?;
    
    println!("{}", format!("✅ Favorite '{}' added successfully!", name).green());
    
    Ok(())
}

fn remove_favorite(favorites_file: &str, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !std::path::Path::new(favorites_file).exists() {
        println!("{}", "No favorites file found.".yellow());
        return Ok(());
    }
    
    let content = fs::read_to_string(favorites_file)?;
    let mut favorites: Vec<SearchFavorite> = serde_json::from_str(&content)?;
    
    let original_len = favorites.len();
    favorites.retain(|f| f.name != name);
    
    if favorites.len() == original_len {
        println!("{}", format!("Favorite '{}' not found.", name).yellow());
        return Ok(());
    }
    
    let json = serde_json::to_string_pretty(&favorites)?;
    fs::write(favorites_file, json)?;
    
    println!("{}", format!("✅ Favorite '{}' removed successfully!", name).green());
    
    Ok(())
}

fn clear_favorites(favorites_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    if std::path::Path::new(favorites_file).exists() {
        fs::remove_file(favorites_file)?;
    }
    
    println!("{}", "✅ All favorites cleared!".green());
    
    Ok(())
}

fn show_search_history() -> Result<(), Box<dyn std::error::Error>> {
    let history_file = std::env::var("HOME").unwrap_or_else(|_| ".".to_string()) + "/.codesearch-history.json";
    
    if !std::path::Path::new(&history_file).exists() {
        println!("{}", "No search history found.".yellow());
        return Ok(());
    }
    
    let content = fs::read_to_string(&history_file)?;
    let history: Vec<String> = serde_json::from_str(&content)?;
    
    if history.is_empty() {
        println!("{}", "No search history found.".yellow());
        return Ok(());
    }
    
    println!("{}", "📚 Search History".cyan().bold());
    println!("{}", "─".repeat(20).cyan());
    println!();
    
    for (i, query) in history.iter().enumerate() {
        println!("{} {}", format!("{}.", i + 1).green().bold(), query.blue());
    }
    
    Ok(())
}

#[derive(Debug, Clone)]
struct SearchMetrics {
    files_processed: usize,
    #[allow(dead_code)]
    total_lines_scanned: usize,
    search_time_ms: u128,
    parallel_workers: usize,
    cache_hits: usize,
    #[allow(dead_code)]
    cache_misses: usize,
}

#[derive(Debug, Clone)]
struct SearchCache {
    cache: DashMap<String, Vec<SearchResult>>,
    file_hashes: DashMap<String, u64>,
}

impl SearchCache {
    fn new() -> Self {
        Self {
            cache: DashMap::new(),
            file_hashes: DashMap::new(),
        }
    }
    
    fn get_cache_key(&self, query: &str, path: &str, extensions: Option<&[String]>, fuzzy: bool) -> String {
        let ext_str = extensions.map(|exts| exts.join(",")).unwrap_or_default();
        format!("{}:{}:{}:{}", query, path, ext_str, fuzzy)
    }
    
    fn get(&self, key: &str) -> Option<Vec<SearchResult>> {
        self.cache.get(key).map(|entry| entry.value().clone())
    }
    
    fn set(&self, key: String, results: Vec<SearchResult>) {
        self.cache.insert(key, results);
    }
    
    fn is_file_modified(&self, file_path: &str) -> bool {
        if let Ok(metadata) = std::fs::metadata(file_path) {
            let current_hash = metadata.len() as u64;
            if let Some(cached_hash) = self.file_hashes.get(file_path) {
                *cached_hash != current_hash
            } else {
                self.file_hashes.insert(file_path.to_string(), current_hash);
                true
            }
        } else {
            true
        }
    }
}

// Global cache instance
static SEARCH_CACHE: std::sync::OnceLock<SearchCache> = std::sync::OnceLock::new();

fn get_search_cache() -> &'static SearchCache {
    SEARCH_CACHE.get_or_init(|| SearchCache::new())
}

// Get default directories to exclude (common build artifacts)
fn get_default_exclude_dirs() -> Vec<String> {
    vec![
        "target".to_string(),
        "node_modules".to_string(),
        "dist".to_string(),
        "build".to_string(),
        ".git".to_string(),
        ".cargo".to_string(),
        "__pycache__".to_string(),
        ".venv".to_string(),
        "venv".to_string(),
        ".next".to_string(),
        ".nuxt".to_string(),
        "vendor".to_string(),
        ".gradle".to_string(),
        ".idea".to_string(),
        ".vscode".to_string(),
    ]
}

pub fn search_code(
    query: &str,
    path: &Path,
    extensions: Option<&[String]>,
    ignore_case: bool,
    fuzzy: bool,
    fuzzy_threshold: f64,
    max_results: usize,
    exclude: Option<&[String]>,
    rank: bool,
    cache: bool,
    semantic: bool,
    benchmark: bool,
    vs_grep: bool,
) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    let mut results = Vec::new();
    
    // Performance tracking
    let (cache_hits, cache_misses) = if cache {
        let search_cache = get_search_cache();
        let cache_key = search_cache.get_cache_key(query, &path.to_string_lossy(), extensions, fuzzy);
        if let Some(cached_results) = search_cache.get(&cache_key) {
            if benchmark {
                println!("{}", "🚀 Cache hit! Returning cached results instantly.".green().bold());
            }
            return Ok(cached_results);
        } else {
            (0, 1)
        }
    } else {
        (0, 0)
    };
    
    // Enhanced query for semantic search
    let enhanced_query = if semantic {
        enhance_query_semantically(query)
    } else {
        query.to_string()
    };
    
    let regex = if fuzzy {
        // For fuzzy search, we'll use a more permissive pattern
        if ignore_case {
            Regex::new(&format!("(?i).*{}.*", regex::escape(&enhanced_query)))?
        } else {
            Regex::new(&format!(".*{}.*", regex::escape(&enhanced_query)))?
        }
    } else if ignore_case {
        Regex::new(&format!("(?i){}", regex::escape(&enhanced_query)))?
    } else {
        Regex::new(&enhanced_query)?
    };

    let walker = WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| {
            if let Some(name) = e.file_name().to_str() {
                if let Some(exclude_dirs) = exclude {
                    for exclude_dir in exclude_dirs {
                        if name == exclude_dir {
                            return false;
                        }
                    }
                }
            }
            true
        })
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file());

    // Collect all files first for parallel processing
    let files: Vec<PathBuf> = walker
        .filter(|entry| {
            let file_path = entry.path();
            if let Some(exts) = extensions {
                if let Some(ext) = file_path.extension().and_then(|s| s.to_str()) {
                    exts.iter().any(|e| e == ext)
                } else {
                    false
                }
            } else {
                true
            }
        })
        .map(|entry| entry.path().to_path_buf())
        .collect();

    let files_processed = files.len();
    
    // Parallel search across files
    let regex_arc = Arc::new(regex);
    let query_arc = Arc::new(enhanced_query.clone());
    
    let parallel_results: Vec<Vec<SearchResult>> = files
        .par_iter()
        .map(|file_path| {
            // Check if file was modified (for cache invalidation)
            if cache && !get_search_cache().is_file_modified(&file_path.to_string_lossy()) {
                return Vec::new(); // Skip unchanged files
            }
            
            search_in_file_parallel(
                file_path,
                &regex_arc,
                fuzzy,
                fuzzy_threshold,
                &query_arc,
                max_results,
            ).unwrap_or_else(|_| Vec::new())
        })
        .collect();
    
    // Flatten results
    for file_results in parallel_results {
        results.extend(file_results);
    }

    // Sort results by relevance score if ranking is enabled
    if rank {
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    }

    // Cache results if caching is enabled
    if cache {
        let search_cache = get_search_cache();
        let cache_key = search_cache.get_cache_key(query, &path.to_string_lossy(), extensions, fuzzy);
        search_cache.set(cache_key, results.clone());
    }
    
    // Print performance metrics
    let search_time = start_time.elapsed();
    
    if benchmark || files_processed > 100 {
        let metrics = SearchMetrics {
            files_processed,
            total_lines_scanned: results.len(),
            search_time_ms: search_time.as_millis(),
            parallel_workers: rayon::current_num_threads(),
            cache_hits,
            cache_misses,
        };
        
        println!("{}", format!("⚡ Performance: {} files in {}ms ({} workers, {} cache hits)", 
            metrics.files_processed, 
            metrics.search_time_ms,
            metrics.parallel_workers,
            metrics.cache_hits
        ).cyan().italic());
        
        if semantic {
            println!("{}", "🧠 Semantic search enabled - enhanced context matching".blue().italic());
        }
        
        if vs_grep {
            let metrics = SearchMetrics {
                files_processed,
                total_lines_scanned: results.len(),
                search_time_ms: search_time.as_millis(),
                parallel_workers: rayon::current_num_threads(),
                cache_hits,
                cache_misses,
            };
            compare_with_grep(query, &path.to_string_lossy(), extensions, &metrics);
        }
    }

    Ok(results)
}

fn enhance_query_semantically(query: &str) -> String {
    // Common programming patterns and their semantic equivalents
    let semantic_patterns = [
        ("function", r"(function|def|fn|func|method|procedure)"),
        ("class", r"(class|struct|interface|trait|type)"),
        ("variable", r"(let|var|const|val|mut)"),
        ("loop", r"(for|while|do|foreach|map|filter)"),
        ("condition", r"(if|else|switch|case|when|match)"),
        ("error", r"(error|exception|panic|fail|throw)"),
        ("test", r"(test|spec|it|describe|assert)"),
        ("import", r"(import|use|require|include|from)"),
        ("export", r"(export|module|pub|public)"),
        ("async", r"(async|await|promise|future)"),
        ("database", r"(db|database|sql|query|table)"),
        ("api", r"(api|endpoint|route|handler)"),
        ("config", r"(config|setting|option|parameter)"),
        ("log", r"(log|debug|info|warn|error)"),
        ("util", r"(util|helper|common|shared)"),
    ];
    
    let mut enhanced = query.to_string();
    
    // Add semantic patterns for common programming terms
    for (term, pattern) in &semantic_patterns {
        if query.to_lowercase().contains(term) {
            enhanced = format!("({}|{})", enhanced, pattern);
        }
    }
    
    // Add context-aware patterns
    if query.contains("get") {
        enhanced = format!("({}|retrieve|fetch|obtain)", enhanced);
    }
    if query.contains("set") {
        enhanced = format!("({}|assign|update|modify)", enhanced);
    }
    if query.contains("create") {
        enhanced = format!("({}|make|build|construct|new)", enhanced);
    }
    if query.contains("delete") {
        enhanced = format!("({}|remove|destroy|clear)", enhanced);
    }
    
    enhanced
}

fn compare_with_grep(
    query: &str,
    path: &str,
    extensions: Option<&[String]>,
    metrics: &SearchMetrics,
) {
    use std::process::Command;
    
    let start_time = Instant::now();
    
    // Build grep command
    let mut grep_cmd = Command::new("grep");
    grep_cmd.arg("-r").arg("-n").arg("--color=never");
    
    // Add file extensions if specified
    if let Some(exts) = extensions {
        for ext in exts {
            grep_cmd.arg("--include").arg(&format!("*.{}", ext));
        }
    }
    
    grep_cmd.arg(query).arg(path);
    
    // Execute grep and measure time
    let grep_result = grep_cmd.output();
    let grep_time = start_time.elapsed();
    
    match grep_result {
        Ok(output) => {
            let grep_lines = String::from_utf8_lossy(&output.stdout).lines().count();
            let speedup = if grep_time.as_millis() > 0 {
                metrics.search_time_ms as f64 / grep_time.as_millis() as f64
            } else {
                0.0
            };
            
            println!();
            println!("{}", "📊 Performance Comparison with Grep".yellow().bold());
            println!("{}", "─".repeat(40).yellow());
            println!("  {}: {}ms", "Code Search".green().bold(), metrics.search_time_ms);
            println!("  {}: {}ms", "Grep".blue().bold(), grep_time.as_millis());
            println!("  {}: {:.1}x", "Speedup".cyan().bold(), speedup);
            println!("  {}: {} vs {} lines", "Results".magenta().bold(), metrics.files_processed, grep_lines);
            
            if speedup > 1.0 {
                println!("  {}: Code Search is {:.1}x faster!", "Winner".green().bold(), speedup);
            } else if speedup < 1.0 {
                println!("  {}: Grep is {:.1}x faster", "Winner".red().bold(), 1.0 / speedup);
            } else {
                println!("  {}: Similar performance", "Tie".yellow().bold());
            }
        }
        Err(_) => {
            println!("{}", "⚠️  Could not run grep comparison (grep not found)".yellow());
        }
    }
}

fn search_in_file_parallel(
    file_path: &Path,
    regex: &Arc<Regex>,
    fuzzy: bool,
    fuzzy_threshold: f64,
    query: &Arc<String>,
    max_results: usize,
) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
    let file = fs::File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut results = Vec::new();
    let mut line_count = 0;
    let matcher = SkimMatcherV2::default();

    for line in reader.lines() {
        line_count += 1;
        let line = line?;
        
        if results.len() >= max_results {
            break;
        }

        if fuzzy {
            // Use fuzzy matching
            if let Some((score, indices)) = matcher.fuzzy_indices(&line, query) {
                if score as f64 >= fuzzy_threshold {
                    let mut matches = Vec::new();
                    let mut last_end = 0;
                    
                    for &idx in &indices {
                        if idx >= last_end {
                            let start = idx;
                            let end = idx + 1;
                            matches.push(Match {
                                start,
                                end,
                                text: line.chars().nth(idx).unwrap().to_string(),
                            });
                            last_end = end;
                        }
                    }

                    let (relevance_score, relevance) = calculate_relevance_score(
                        &line,
                        query,
                        line_count,
                        file_path,
                        true,
                        Some(score),
                    );

                    results.push(SearchResult {
                        file: file_path.to_string_lossy().to_string(),
                        line_number: line_count,
                        content: line.clone(),
                        matches,
                        score: relevance_score,
                        relevance,
                    });
                }
            }
        } else {
            // Use regex matching
            for mat in regex.find_iter(&line) {
                let mut matches = Vec::new();
                matches.push(Match {
                    start: mat.start(),
                    end: mat.end(),
                    text: mat.as_str().to_string(),
                });

                let (relevance_score, relevance) = calculate_relevance_score(
                    &line,
                    query,
                    line_count,
                    file_path,
                    false,
                    None,
                );

                results.push(SearchResult {
                    file: file_path.to_string_lossy().to_string(),
                    line_number: line_count,
                    content: line.clone(),
                    matches,
                    score: relevance_score,
                    relevance,
                });
            }
        }
    }

    Ok(results)
}

fn calculate_relevance_score(
    content: &str,
    query: &str,
    line_number: usize,
    file_path: &Path,
    fuzzy: bool,
    fuzzy_score: Option<i64>,
) -> (f64, String) {
    let mut score = 0.0;
    let mut relevance_factors = Vec::new();

    // Base score from fuzzy matching or exact matching
    if fuzzy {
        if let Some(fs) = fuzzy_score {
            score += fs as f64 / 100.0; // Normalize fuzzy score
            relevance_factors.push("fuzzy match".to_string());
        }
    } else {
        score += 1.0; // Exact match
        relevance_factors.push("exact match".to_string());
    }

    // Boost for function/class definitions
    if regex::Regex::new(r"^(fn|def|function|class|interface|struct|enum)\s+").unwrap().is_match(content.trim()) {
        score += 0.3;
        relevance_factors.push("definition".to_string());
    }

    // Boost for comments and documentation
    if regex::Regex::new(r"^\s*(//|#|/\*|\*|///|//!)").unwrap().is_match(content.trim()) {
        score += 0.2;
        relevance_factors.push("documentation".to_string());
    }

    // Boost for early lines in file (likely more important)
    if line_number <= 50 {
        score += 0.1;
        relevance_factors.push("early in file".to_string());
    }

    // Boost for specific file types
    if let Some(ext) = file_path.extension() {
        match ext.to_str().unwrap() {
            "rs" | "py" | "js" | "ts" | "go" | "java" => {
                score += 0.1;
                relevance_factors.push("source code".to_string());
            }
            _ => {}
        }
    }

    // Boost for multiple matches in the same line
    let match_count = content.matches(query).count();
    if match_count > 1 {
        score += 0.1 * match_count as f64;
        relevance_factors.push(format!("{} matches", match_count));
    }

    // Boost for complete word matches
    if regex::Regex::new(&format!(r"\b{}\b", regex::escape(query))).unwrap().is_match(content) {
        score += 0.2;
        relevance_factors.push("whole word".to_string());
    }

    // Normalize score to 0-1 range
    let normalized_score = (score / 2.0).min(1.0);
    let relevance = relevance_factors.join(", ");

    (normalized_score, relevance)
}


pub fn list_files(
    path: &Path,
    extensions: Option<&[String]>,
    exclude: Option<&[String]>,
) -> Result<Vec<FileInfo>, Box<dyn std::error::Error>> {
    let mut files = Vec::new();

    let walker = WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| {
            if let Some(name) = e.file_name().to_str() {
                if let Some(exclude_dirs) = exclude {
                    for exclude_dir in exclude_dirs {
                        if name == exclude_dir {
                            return false;
                        }
                    }
                }
            }
            true
        })
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file());

    for entry in walker {
        let file_path = entry.path();
        
        // Check file extension if specified
        if let Some(exts) = extensions {
            if let Some(ext) = file_path.extension().and_then(|s| s.to_str()) {
                if !exts.iter().any(|e| e == ext) {
                    continue;
                }
            } else {
                continue;
            }
        }

        let metadata = entry.metadata()?;
        let line_count = count_lines(file_path).unwrap_or(0);
        
        files.push(FileInfo {
            path: file_path.to_string_lossy().to_string(),
            size: metadata.len(),
            lines: line_count,
        });
    }

    Ok(files)
}

fn count_lines(file_path: &Path) -> Result<usize, Box<dyn std::error::Error>> {
    let file = fs::File::open(file_path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines().count())
}

fn print_results(results: &[SearchResult], line_numbers: bool, show_ranking: bool) {
    if results.is_empty() {
        println!("{}", "No matches found.".yellow());
        return;
    }

    // Group results by file for better organization
    let mut file_groups: std::collections::HashMap<String, Vec<&SearchResult>> = std::collections::HashMap::new();
    for result in results {
        file_groups.entry(result.file.clone()).or_insert_with(Vec::new).push(result);
    }

    // Print summary
    let total_matches = results.len();
    let file_count = file_groups.len();
    println!("{}", format!("Found {} matches in {} files", total_matches, file_count).cyan().bold());
    println!();

    let mut current_file = String::new();
    
    for result in results {
        if result.file != current_file {
            current_file = result.file.clone();
            let file_matches = file_groups.get(&current_file).unwrap().len();
            println!("{}", format!("📁 {} ({} matches)", result.file, file_matches).blue().bold());
            println!("{}", "─".repeat(result.file.len() + 15).blue());
        }

        let line_info = if line_numbers {
            format!("{}: ", result.line_number.to_string().green())
        } else {
            String::new()
        };

        let mut content = result.content.clone();
        
        // Highlight matches with better formatting
        for mat in &result.matches {
            let highlighted = format!("{}", mat.text.red().bold());
            content = content.replace(&mat.text, &highlighted);
        }

        // Add ranking information if enabled
        let ranking_info = if show_ranking {
            format!(" [{:.2}] {}", result.score, result.relevance.cyan().italic())
        } else {
            String::new()
        };

        // Add indentation for better readability
        let indented_content = if line_numbers {
            format!("  {}{}{}", line_info, content, ranking_info)
        } else {
            format!("  {}{}", content, ranking_info)
        };

        println!("{}", indented_content);
    }
    
    println!();
    println!("{}", "✨ Search completed!".green().italic());
}

fn print_search_stats(results: &[SearchResult], query: &str) {
    if results.is_empty() {
        return;
    }

    println!();
    println!("{}", "📊 Search Statistics".cyan().bold());
    println!("{}", "─".repeat(20).cyan());

    // Basic stats
    let total_matches = results.len();
    let unique_files: std::collections::HashSet<&String> = results.iter().map(|r| &r.file).collect();
    let file_count = unique_files.len();

    println!("Query: {}", query.blue().bold());
    println!("Total matches: {}", total_matches.to_string().green().bold());
    println!("Files searched: {}", file_count.to_string().green().bold());

    // File breakdown
    let mut file_stats: std::collections::HashMap<&String, usize> = std::collections::HashMap::new();
    for result in results {
        *file_stats.entry(&result.file).or_insert(0) += 1;
    }

    let mut file_vec: Vec<_> = file_stats.iter().collect();
    file_vec.sort_by(|a, b| b.1.cmp(a.1));

    println!();
    println!("{}", "📁 Matches per file:".yellow().bold());
    for (file, count) in file_vec.iter().take(5) {
        let percentage = (**count as f64 / total_matches as f64 * 100.0) as u32;
        println!("  {}: {} matches ({}%)", 
                file.blue(), 
                count.to_string().green(), 
                percentage.to_string().cyan());
    }

    if file_vec.len() > 5 {
        println!("  ... and {} more files", file_vec.len() - 5);
    }

    // Line number analysis
    let line_numbers: Vec<usize> = results.iter().map(|r| r.line_number).collect();
    let min_line = line_numbers.iter().min().unwrap();
    let max_line = line_numbers.iter().max().unwrap();
    let avg_line = line_numbers.iter().sum::<usize>() as f64 / line_numbers.len() as f64;

    println!();
    println!("{}", "📏 Line analysis:".yellow().bold());
    println!("  Earliest match: line {}", min_line.to_string().green());
    println!("  Latest match: line {}", max_line.to_string().green());
    println!("  Average line: {:.1}", avg_line.to_string().green());

    // Pattern analysis
    let mut pattern_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for result in results {
        for mat in &result.matches {
            *pattern_counts.entry(mat.text.clone()).or_insert(0) += 1;
        }
    }

    if pattern_counts.len() > 1 {
        println!();
        println!("{}", "🔍 Pattern breakdown:".yellow().bold());
        let mut pattern_vec: Vec<_> = pattern_counts.iter().collect();
        pattern_vec.sort_by(|a, b| b.1.cmp(a.1));
        
        for (pattern, count) in pattern_vec.iter().take(3) {
            println!("  '{}': {} occurrences", 
                    pattern.red().bold(), 
                    count.to_string().green());
        }
    }
}

fn interactive_search(
    path: &Path,
    extensions: Option<&[String]>,
    exclude: Option<&[String]>,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::{self, Write};
    
    println!("{}", "🎮 Interactive Search Mode".cyan().bold());
    println!("{}", "Type 'help' for commands, 'quit' to exit".yellow().italic());
    println!();

    let mut search_history: Vec<String> = Vec::new();
    let mut current_extensions = extensions.map(|exts| exts.to_vec());
    let mut current_exclude = exclude.map(|excl| excl.to_vec());

    loop {
        print!("{}", "codesearch> ".green().bold());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "quit" | "exit" | "q" => {
                println!("{}", "👋 Goodbye!".green().italic());
                break;
            }
            "help" | "h" => {
                print_help();
            }
            "extensions" | "ext" => {
                if parts.len() > 1 {
                    let new_exts: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();
                    current_extensions = Some(new_exts);
                    println!("{}", "Extensions updated!".green());
                } else {
                    match &current_extensions {
                        Some(exts) => println!("Current extensions: {}", exts.join(", ")),
                        None => println!("No extensions filter"),
                    }
                }
            }
            "exclude" => {
                if parts.len() > 1 {
                    let new_excl: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();
                    current_exclude = Some(new_excl);
                    println!("{}", "Exclude directories updated!".green());
                } else {
                    match &current_exclude {
                        Some(excl) => println!("Excluded directories: {}", excl.join(", ")),
                        None => println!("No excluded directories"),
                    }
                }
            }
            "history" => {
                if search_history.is_empty() {
                    println!("No search history");
                } else {
                    println!("{}", "Search History:".cyan().bold());
                    for (i, query) in search_history.iter().enumerate() {
                        println!("  {}: {}", i + 1, query.blue());
                    }
                }
            }
            "clear" => {
                print!("\x1B[2J\x1B[1;1H");
                io::stdout().flush()?;
            }
            _ => {
                // Perform search
                let query = input;
                search_history.push(query.to_string());
                
                // Keep only last 10 searches
                if search_history.len() > 10 {
                    search_history.remove(0);
                }

                let results = search_code(
                    query,
                    path,
                    current_extensions.as_deref(),
                    false, // ignore_case
                    false, // fuzzy
                    0.6,   // fuzzy_threshold
                    20,    // max_results
                    current_exclude.as_deref(),
                    false, // rank
                    false, // cache
                    false, // semantic
                    false, // benchmark
                    false, // vs_grep
                )?;

                if results.is_empty() {
                    println!("{}", "No matches found.".yellow());
                } else {
                    print_results(&results, true, false);
                    print_search_stats(&results, query);
                }
                println!();
            }
        }
    }

    Ok(())
}

fn print_help() {
    println!("{}", "📖 Interactive Search Commands:".cyan().bold());
    println!("  {} - Search for text pattern", "search <pattern>".green());
    println!("  {} - Set file extensions filter", "extensions <ext1,ext2>".green());
    println!("  {} - Set excluded directories", "exclude <dir1,dir2>".green());
    println!("  {} - Show search history", "history".green());
    println!("  {} - Clear screen", "clear".green());
    println!("  {} - Show this help", "help".green());
    println!("  {} - Exit interactive mode", "quit".green());
    println!();
}

pub fn analyze_codebase(
    path: &Path,
    extensions: Option<&[String]>,
    exclude: Option<&[String]>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "📊 Codebase Analysis".cyan().bold());
    println!("{}", "─".repeat(20).cyan());
    println!();

    let files = list_files(path, extensions, exclude)?;
    
    if files.is_empty() {
        println!("{}", "No files found to analyze.".yellow());
        return Ok(());
    }

    // Basic file statistics
    let total_files = files.len();
    let total_lines: usize = files.iter().map(|f| f.lines).sum();
    let total_size: u64 = files.iter().map(|f| f.size).sum();
    
    println!("{}", "📁 File Statistics:".yellow().bold());
    println!("  Total files: {}", total_files.to_string().green().bold());
    println!("  Total lines: {}", total_lines.to_string().green().bold());
    println!("  Total size: {} bytes", total_size.to_string().green().bold());
    println!();

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

    println!("{}", "📋 File Type Breakdown:".yellow().bold());
    let mut ext_vec: Vec<_> = ext_counts.iter().collect();
    ext_vec.sort_by(|a, b| b.1.0.cmp(&a.1.0));
    
    for (ext, (count, lines, size)) in &ext_vec {
        let percentage = (*count as f64 / total_files as f64 * 100.0) as u32;
        println!("  {}: {} files ({}%), {} lines, {} bytes", 
                ext.blue().bold(),
                count.to_string().green(),
                percentage.to_string().cyan(),
                lines.to_string().green(),
                size.to_string().green());
    }
    println!();

    // Largest files
    let mut file_vec = files.clone();
    file_vec.sort_by(|a, b| b.lines.cmp(&a.lines));
    
    println!("{}", "📏 Largest Files (by lines):".yellow().bold());
    for file in file_vec.iter().take(5) {
        let filename = std::path::Path::new(&file.path)
            .file_name()
            .unwrap()
            .to_string_lossy();
        println!("  {}: {} lines", 
                filename.blue().bold(),
                file.lines.to_string().green());
    }
    println!();

    // Code patterns analysis
    println!("{}", "🔍 Code Pattern Analysis:".yellow().bold());
    
    // Search for common patterns
    let patterns = vec![
        ("Functions", "fn\\s+\\w+|function\\s+\\w+|def\\s+\\w+"),
        ("Classes", "class\\s+\\w+"),
        ("Comments", "//|#|/\\*|<!--"),
        ("TODO", "TODO|FIXME|HACK|XXX"),
        ("Imports", "^import|^use|^#include|^require"),
    ];

    for (name, pattern) in patterns {
        let regex = Regex::new(pattern)?;
        let mut total_matches = 0;
        
        for file in &files {
            if let Ok(content) = fs::read_to_string(&file.path) {
                for _line in content.lines() {
                    if regex.is_match(_line) {
                        total_matches += 1;
                    }
                }
            }
        }
        
        println!("  {}: {} occurrences", 
                name.green().bold(),
                total_matches.to_string().cyan());
    }
    println!();

    // Complexity analysis
    let mut complex_files = Vec::new();
    for file in &files {
        if file.lines > 100 {
            complex_files.push((file.path.clone(), file.lines));
        }
    }
    
    if !complex_files.is_empty() {
        println!("{}", "⚠️  Large Files (>100 lines):".yellow().bold());
        complex_files.sort_by(|a, b| b.1.cmp(&a.1));
        for (file_path, lines) in complex_files.iter().take(5) {
            let filename = std::path::Path::new(file_path)
                .file_name()
                .unwrap()
                .to_string_lossy();
            println!("  {}: {} lines", 
                    filename.blue().bold(),
                    lines.to_string().red().bold());
        }
        println!();
    }

    // Summary
    let avg_lines_per_file = total_lines as f64 / total_files as f64;
    let avg_size_per_file = total_size as f64 / total_files as f64;
    
    println!("{}", "📈 Summary:".cyan().bold());
    println!("  Average lines per file: {:.1}", avg_lines_per_file.to_string().green());
    println!("  Average size per file: {:.0} bytes", avg_size_per_file.to_string().green());
    
    if total_files > 0 {
        println!("  Most common file type: {}", ext_vec[0].0.blue().bold());
    }
    
    println!();
    println!("{}", "✨ Analysis completed!".green().italic());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_files() -> TempDir {
        let temp_dir = TempDir::new().unwrap();
        let test_dir = temp_dir.path();

        // Create test files
        fs::write(test_dir.join("test.rs"), "fn main() {\n    println!(\"Hello, world!\");\n}").unwrap();
        fs::write(test_dir.join("test.py"), "def hello():\n    print('Hello, world!')\n").unwrap();
        fs::write(test_dir.join("test.js"), "function hello() {\n    console.log('Hello, world!');\n}\n").unwrap();

        // Create subdirectory
        fs::create_dir(test_dir.join("src")).unwrap();
        fs::write(test_dir.join("src/lib.rs"), "pub fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n").unwrap();

        temp_dir
    }

    #[test]
    fn test_search_in_file() {
        let temp_dir = create_test_files();
        let test_file = temp_dir.path().join("test.rs");
        let regex = Arc::new(Regex::new("Hello").unwrap());
        let query = Arc::new("Hello".to_string());
        
        let results = search_in_file_parallel(&test_file, &regex, false, 0.6, &query, 10).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].line_number, 2);
        assert!(results[0].content.contains("Hello, world!"));
    }

    #[test]
    fn test_search_code_basic() {
        let temp_dir = create_test_files();
        let results = search_code(
            "Hello",
            temp_dir.path(),
            None,
            false,
            false,
            0.6,
            10,
            None,
            false,
            false,
            false,
            false,
            false,
        ).unwrap();
        
        assert!(!results.is_empty());
        assert!(results.iter().any(|r| r.file.contains("test.rs")));
        assert!(results.iter().any(|r| r.file.contains("test.py")));
        assert!(results.iter().any(|r| r.file.contains("test.js")));
    }

    #[test]
    fn test_search_code_with_extensions() {
        let temp_dir = create_test_files();
        let extensions = vec!["rs".to_string()];
        let results = search_code(
            "Hello",
            temp_dir.path(),
            Some(&extensions),
            false,
            false,
            0.6,
            10,
            None,
            false,
            false,
            false,
            false,
            false,
        ).unwrap();
        
        assert!(!results.is_empty());
        assert!(results.iter().all(|r| r.file.ends_with(".rs")));
    }

    #[test]
    fn test_search_code_case_insensitive() {
        let temp_dir = create_test_files();
        let results = search_code(
            "hello",
            temp_dir.path(),
            None,
            true,
            false,
            0.6,
            10,
            None,
            false,
            false,
            false,
            false,
            false,
        ).unwrap();
        
        assert!(!results.is_empty());
        assert!(results.iter().any(|r| r.content.contains("Hello")));
    }

    #[test]
    fn test_search_code_regex() {
        let temp_dir = create_test_files();
        let results = search_code(
            r"fn\s+\w+",
            temp_dir.path(),
            Some(&vec!["rs".to_string()]),
            false,
            false,
            0.6,
            10,
            None,
            false,
            false,
            false,
            false,
            false,
        ).unwrap();
        
        assert!(!results.is_empty());
        assert!(results.iter().any(|r| r.content.contains("fn main")));
        assert!(results.iter().any(|r| r.content.contains("fn add")));
    }

    #[test]
    fn test_list_files() {
        let temp_dir = create_test_files();
        let files = list_files(
            temp_dir.path(),
            Some(&vec!["rs".to_string()]),
            None,
        ).unwrap();
        
        assert_eq!(files.len(), 2);
        assert!(files.iter().any(|f| f.path.ends_with("test.rs")));
        assert!(files.iter().any(|f| f.path.ends_with("src/lib.rs")));
    }

    #[test]
    fn test_count_lines() {
        let temp_dir = create_test_files();
        let test_file = temp_dir.path().join("test.rs");
        let line_count = count_lines(&test_file).unwrap();
        assert_eq!(line_count, 3); // 3 lines in test.rs
    }
}
