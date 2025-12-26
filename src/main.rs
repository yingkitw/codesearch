//! CodeSearch - A fast CLI tool for searching codebases
//!
//! This is the main entry point that orchestrates all modules.

use clap::{Parser, Subcommand};
use colored::*;
use std::path::PathBuf;

// Use library modules
use codesearch::{analysis, circular, complexity, deadcode, duplicates, export, interactive};
#[cfg(feature = "mcp")]
use codesearch::mcp_server;
use codesearch::search::{list_files, print_results, print_search_stats, search_code};

#[derive(Parser)]
#[command(name = "codesearch")]
#[command(about = "A fast CLI tool for searching codebases")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    
    /// Search query pattern
    #[arg(index = 1)]
    query: Option<String>,
    
    /// Path to search (file or directory, default: current directory)
    #[arg(index = 2, default_value = ".")]
    path: PathBuf,
    
    /// File extensions to include (e.g., rs,py,js)
    #[arg(short, long, value_delimiter = ',')]
    extensions: Option<Vec<String>>,
    
    /// Enable fuzzy search
    #[arg(short, long)]
    fuzzy: bool,
    
    /// Case-insensitive search (default: true)
    #[arg(short, long, default_value = "true")]
    ignore_case: bool,
    
    /// Maximum results per file
    #[arg(short, long, default_value = "10")]
    max_results: usize,
    
    /// Exclude directories
    #[arg(long, value_delimiter = ',')]
    exclude: Option<Vec<String>>,
}

#[derive(Subcommand)]
enum Commands {
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
}

/// Default directories to exclude from search
fn get_default_exclude_dirs() -> Vec<String> {
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Handle simple search without subcommand: codesearch <query> [path]
    if cli.command.is_none() {
        if let Some(query) = cli.query {
            // Check if query looks like a mistyped command
            let command_suggestions: &[(&str, &str)] = &[
                ("analysis", "analyze"),
                ("analyse", "analyze"),
                ("file", "files"),
                ("interactive", "interactive"),
                ("complex", "complexity"),
                ("duplicate", "duplicates"),
                ("dups", "duplicates"),
                ("dead", "deadcode"),
                ("unused", "deadcode"),
                ("lang", "languages"),
                ("langs", "languages"),
                ("mcp", "mcp-server"),
            ];
            
            for (typo, correct) in command_suggestions {
                if query.eq_ignore_ascii_case(typo) && *typo != *correct {
                    eprintln!("Did you mean: codesearch {} {}", correct, cli.path.display());
                    eprintln!("Run 'codesearch --help' for available commands.");
                    return Ok(());
                }
            }
            
            // Build exclude list
            let mut final_exclude = get_default_exclude_dirs();
            if let Some(user_exclude) = cli.exclude {
                final_exclude.extend(user_exclude);
            }
            
            let results = search_code(
                &query,
                &cli.path,
                cli.extensions.as_deref(),
                cli.ignore_case,
                cli.fuzzy,
                0.6, // fuzzy_threshold
                cli.max_results,
                Some(final_exclude.as_slice()),
                false, // rank
                false, // cache
                false, // semantic
                false, // benchmark
                false, // vs_grep
            )?;
            
            if results.is_empty() {
                println!("{}", "No matches found.".dimmed());
            } else {
                print_results(&results, true, false);
            print_search_stats(&results, &query);
            }
            return Ok(());
        } else {
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
            export: export_path,
        }) => {
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

            if let Some(path) = export_path {
                export::export_results(&results, &path, &query)?;
                println!("{}", format!("Results exported to: {}", path).green());
            } else {
                match format.as_str() {
                "json" => {
                    let json = serde_json::to_string_pretty(&results)?;
                    println!("{}", json);
                }
                _ => {
                        if results.is_empty() {
                            println!("{}", "No matches found.".dimmed());
                        } else {
                            print_results(&results, !no_line_numbers, rank);
                            if stats {
                        print_search_stats(&results, &query);
                    }
                }
            }
        }
        }
        }
        Some(Commands::Files { path, extensions, exclude }) => {
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
        Some(Commands::Interactive { path, extensions, exclude }) => {
            interactive::run(&path, extensions.as_deref(), exclude.as_deref())?;
        }
        Some(Commands::Analyze { path, extensions, exclude }) => {
            analysis::analyze_codebase(&path, extensions.as_deref(), exclude.as_deref())?;
        }
        Some(Commands::Complexity { path, extensions, exclude, threshold, sort }) => {
            complexity::analyze_complexity(&path, extensions.as_deref(), exclude.as_deref(), threshold, sort)?;
        }
        Some(Commands::Duplicates { path, extensions, exclude, min_lines, similarity }) => {
            duplicates::detect_duplicates(&path, extensions.as_deref(), exclude.as_deref(), min_lines, similarity)?;
        }
        Some(Commands::Deadcode { path, extensions, exclude }) => {
            deadcode::detect_dead_code(&path, extensions.as_deref(), exclude.as_deref())?;
        }
        Some(Commands::Circular { path, extensions, exclude }) => {
            circular::detect_circular_calls(&path, extensions.as_deref(), exclude.as_deref())?;
        }
        Some(Commands::Languages) => {
            analysis::list_supported_languages()?;
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
            Cli::parse_from(&["codesearch", "--help"]);
        }
    }

    Ok(())
}

