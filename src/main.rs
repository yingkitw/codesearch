//! CodeSearch - A fast CLI tool for searching codebases
//!
//! This is the main entry point that orchestrates all modules.

use clap::{Parser, Subcommand, ValueEnum};
use colored::*;
use std::path::PathBuf;

// Use library modules
use codesearch::{
    analysis, complexity, config, duplicates, export, favorites,
    mcp_server, SearchResult,
};
use codesearch::search::{list_files, print_results, print_search_stats, search_code};

/// Output theme options
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum ThemeOption {
    #[default]
    Default,
    Dark,
    Light,
    Mono,
    Ocean,
    Forest,
}

impl ThemeOption {
    #[allow(dead_code)]
    fn to_theme_name(&self) -> &'static str {
        match self {
            ThemeOption::Default => "default",
            ThemeOption::Dark => "dark",
            ThemeOption::Light => "light",
            ThemeOption::Mono => "mono",
            ThemeOption::Ocean => "ocean",
            ThemeOption::Forest => "forest",
        }
    }
}

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

    /// Output theme
    #[arg(long, value_enum, global = true)]
    theme: Option<ThemeOption>,
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
        /// Export results to file (csv, markdown, md)
        #[arg(long)]
        export: Option<String>,
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
    /// Analyze code complexity metrics
    Complexity {
        /// Directory to analyze (default: current directory)
        #[arg(short, long, default_value = ".")]
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
        /// Directory to analyze (default: current directory)
        #[arg(short, long, default_value = ".")]
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
    /// List all supported programming languages
    Languages,
    /// Run as MCP server
    McpServer,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let config = config::load_config();

    // Handle simple search without subcommand
    if cli.command.is_none() {
        if let Some(query) = cli.query {
            let default_exclude_dirs = config::get_default_exclude_dirs();
            let final_exclude: Option<&[String]> = if config.search.auto_exclude {
                if let Some(config_exclude) = &config.defaults.exclude_dirs {
                    Some(config_exclude.as_slice())
                } else {
                    Some(default_exclude_dirs.as_slice())
                }
            } else if let Some(config_exclude) = &config.search.exclude {
                Some(config_exclude.as_slice())
            } else {
                None
            };
            
            let results = search_code(
                &query,
                &PathBuf::from("."),
                config.search.extensions.as_deref(),
                config.search.ignore_case,
                false,
                config.search.fuzzy_threshold,
                config.search.max_results,
                final_exclude,
                config.search.rank,
                config.search.cache,
                config.search.semantic,
                false,
                false,
            )?;
            
            print_results(&results, config.search.show_line_numbers, config.search.rank);
            print_search_stats(&results, &query);
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
            let final_extensions = extensions.or_else(|| config.search.extensions.clone());
            let final_ignore_case = if ignore_case { true } else { config.search.ignore_case };
            let final_fuzzy_threshold = if fuzzy_threshold != 0.6 { fuzzy_threshold } else { config.search.fuzzy_threshold };
            let final_max_results = if max_results != 10 { max_results } else { config.search.max_results };
            let final_format = if format != "text" { format } else { config.search.format.clone() };
            let final_rank = rank || config.search.rank;
            let final_cache = cache || config.search.cache;
            let final_semantic = semantic || config.search.semantic;
            let final_show_line_numbers = !no_line_numbers && config.search.show_line_numbers;
            
            let final_exclude = if no_auto_exclude {
                exclude.or_else(|| config.search.exclude.clone())
            } else if config.search.auto_exclude {
                let mut auto_exclude = if let Some(config_exclude) = &config.defaults.exclude_dirs {
                    config_exclude.clone()
                } else {
                    config::get_default_exclude_dirs()
                };
                if let Some(mut user_exclude) = exclude {
                    auto_exclude.append(&mut user_exclude);
                } else if let Some(mut config_exclude) = config.search.exclude.clone() {
                    auto_exclude.append(&mut config_exclude);
                }
                Some(auto_exclude)
            } else {
                exclude.or_else(|| config.search.exclude.clone())
            };
            
            let results = search_code(
                &query,
                &path,
                final_extensions.as_deref(),
                final_ignore_case,
                fuzzy,
                final_fuzzy_threshold,
                final_max_results,
                final_exclude.as_deref(),
                final_rank,
                final_cache,
                final_semantic,
                benchmark,
                vs_grep,
            )?;

            if let Some(path) = export_path {
                export::export_results(&results, &path, &query)?;
                println!("{}", format!("✅ Results exported to: {}", path).green());
            } else {
                match final_format.as_str() {
                "json" => {
                    let json = serde_json::to_string_pretty(&results)?;
                    println!("{}", json);
                }
                _ => {
                        print_results(&results, final_show_line_numbers, final_rank);
                        if stats || !results.is_empty() {
                        print_search_stats(&results, &query);
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
            interactive_search(&path, extensions.as_deref(), exclude.as_deref())?;
        }
        Some(Commands::Analyze { path, extensions, exclude }) => {
            analysis::analyze_codebase(&path, extensions.as_deref(), exclude.as_deref())?;
        }
        Some(Commands::Refactor { path, extensions, exclude, high_priority }) => {
            analysis::suggest_refactoring(&path, extensions.as_deref(), exclude.as_deref(), high_priority)?;
        }
        Some(Commands::Favorites { list, add, remove, clear, history }) => {
            favorites::manage_favorites(list, add, remove, clear, history)?;
        }
        Some(Commands::Complexity { path, extensions, exclude, threshold, sort }) => {
            complexity::analyze_complexity(&path, extensions.as_deref(), exclude.as_deref(), threshold, sort)?;
        }
        Some(Commands::Duplicates { path, extensions, exclude, min_lines, similarity }) => {
            duplicates::detect_duplicates(&path, extensions.as_deref(), exclude.as_deref(), min_lines, similarity)?;
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

/// Interactive search mode
fn interactive_search(
    path: &std::path::Path,
    extensions: Option<&[String]>,
    exclude: Option<&[String]>,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::{self, Write};

    println!("{}", "🎮 Interactive Search Mode".cyan().bold());
    println!("{}", "Type 'help' for commands and shortcuts, 'quit' to exit".dimmed().italic());
    println!();

    let mut search_history: Vec<String> = Vec::new();
    let mut current_extensions = extensions.map(|exts| exts.to_vec());
    let mut current_exclude = exclude.map(|excl| excl.to_vec());
    let mut fuzzy_mode = false;
    let mut case_insensitive = true;
    let mut ranking_mode = false;
    let mut semantic_mode = false;
    let mut last_results: Vec<SearchResult> = Vec::new();

    loop {
        let mode_indicator = format!(
            "[{}{}{}{}]",
            if fuzzy_mode { "F" } else { "-" },
            if case_insensitive { "I" } else { "-" },
            if ranking_mode { "R" } else { "-" },
            if semantic_mode { "S" } else { "-" }
        );

        print!("{} {} ", mode_indicator.blue(), "codesearch>".green().bold());
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
            "quit" | "exit" | "q" | ":q" => {
                println!("{}", "👋 Goodbye!".green().italic());
                break;
            }
            "help" | "h" | "?" => {
                print_interactive_help();
            }
            "/f" | ":f" | "fuzzy" => {
                fuzzy_mode = !fuzzy_mode;
                println!("Fuzzy mode: {}", if fuzzy_mode { "ON".green() } else { "OFF".dimmed() });
            }
            "/i" | ":i" | "case" => {
                case_insensitive = !case_insensitive;
                println!("Case insensitive: {}", if case_insensitive { "ON".green() } else { "OFF".dimmed() });
            }
            "/r" | ":r" | "rank" => {
                ranking_mode = !ranking_mode;
                println!("Ranking mode: {}", if ranking_mode { "ON".green() } else { "OFF".dimmed() });
            }
            "/s" | ":s" | "semantic" => {
                semantic_mode = !semantic_mode;
                println!("Semantic search: {}", if semantic_mode { "ON".green() } else { "OFF".dimmed() });
            }
            "!!" | "repeat" => {
                if let Some(last_query) = search_history.last() {
                    let query = last_query.clone();
                    println!("{}", format!("Repeating: {}", query).blue());
                    let results = search_code(
                        &query, path, current_extensions.as_deref(), case_insensitive,
                        fuzzy_mode, 0.6, 20, current_exclude.as_deref(),
                        ranking_mode, false, semantic_mode, false, false,
                    )?;
                    last_results = results.clone();
                    if results.is_empty() {
                        println!("{}", "No matches found.".dimmed());
            } else {
                        print_results(&results, true, ranking_mode);
                        print_search_stats(&results, &query);
            }
        } else {
                    println!("{}", "No previous search.".dimmed());
                }
            }
            "ext" | "extensions" => {
                if parts.len() > 1 {
                    let new_exts: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();
                    current_extensions = Some(new_exts.clone());
                    println!("{}", format!("Extensions set: {}", new_exts.join(", ")).green());
        } else {
                    match &current_extensions {
                        Some(exts) => println!("Current extensions: {}", exts.join(", ").blue()),
                        None => println!("No extensions filter (searching all files)"),
                    }
                }
            }
            "exclude" => {
                if parts.len() > 1 {
                    let new_excl: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();
                    current_exclude = Some(new_excl.clone());
                    println!("{}", format!("Exclude directories: {}", new_excl.join(", ")).green());
                } else {
                    match &current_exclude {
                        Some(excl) => println!("Excluded directories: {}", excl.join(", ").yellow()),
                        None => println!("No excluded directories"),
                    }
                }
            }
            "history" | "hist" => {
                if search_history.is_empty() {
                    println!("{}", "No search history".dimmed());
    } else {
                    println!("{}", "📚 Search History:".cyan().bold());
                    for (i, query) in search_history.iter().enumerate() {
                        println!("  {} {}", format!("!{}", i + 1).blue(), query.green());
                    }
                }
            }
            "clear-history" => {
                search_history.clear();
                println!("{}", "Search history cleared.".green());
            }
            "clear" | "cls" => {
                print!("\x1B[2J\x1B[1;1H");
                io::stdout().flush()?;
            }
            "status" | "settings" => {
                println!("{}", "⚙️  Current Settings:".cyan().bold());
                println!("  Fuzzy mode:       {}", if fuzzy_mode { "ON".green() } else { "OFF".dimmed() });
                println!("  Case insensitive: {}", if case_insensitive { "ON".green() } else { "OFF".dimmed() });
                println!("  Ranking mode:     {}", if ranking_mode { "ON".green() } else { "OFF".dimmed() });
                println!("  Semantic search:  {}", if semantic_mode { "ON".green() } else { "OFF".dimmed() });
                match &current_extensions {
                    Some(exts) => println!("  Extensions:       {}", exts.join(", ").blue()),
                    None => println!("  Extensions:       {}", "all".blue()),
                }
            }
            "export" => {
                if parts.len() > 1 {
                    let export_path = parts[1];
                    if last_results.is_empty() {
                        println!("{}", "No results to export. Run a search first.".dimmed());
            } else {
                        let query = search_history.last().map(|s| s.as_str()).unwrap_or("");
                        export::export_results(&last_results, export_path, query)?;
                        println!("{}", format!("✅ Results exported to: {}", export_path).green());
            }
        } else {
                    println!("{}", "Usage: export <filename.csv|.md>".dimmed());
                }
            }
            "analyze" => {
                analysis::analyze_codebase(path, current_extensions.as_deref(), current_exclude.as_deref())?;
            }
            "complexity" => {
                complexity::analyze_complexity(path, current_extensions.as_deref(), current_exclude.as_deref(), None, true)?;
            }
            "duplicates" | "dups" => {
                duplicates::detect_duplicates(path, current_extensions.as_deref(), current_exclude.as_deref(), 3, 0.9)?;
            }
            "refactor" => {
                analysis::suggest_refactoring(path, current_extensions.as_deref(), current_exclude.as_deref(), false)?;
            }
            "languages" | "langs" => {
                analysis::list_supported_languages()?;
            }
            // Handle history recall: !n
            cmd if cmd.starts_with('!') && cmd.len() > 1 => {
                if let Ok(n) = cmd[1..].parse::<usize>() {
                    if n > 0 && n <= search_history.len() {
                        let query = search_history[n - 1].clone();
                        println!("{}", format!("Searching: {}", query).blue());
                        let results = search_code(
                            &query, path, current_extensions.as_deref(), case_insensitive,
                            fuzzy_mode, 0.6, 20, current_exclude.as_deref(),
                            ranking_mode, false, semantic_mode, false, false,
                        )?;
                        last_results = results.clone();
                        search_history.push(query.clone());
                        if search_history.len() > 20 {
                            search_history.remove(0);
                        }
    if results.is_empty() {
                            println!("{}", "No matches found.".dimmed());
        } else {
                            print_results(&results, true, ranking_mode);
                            print_search_stats(&results, &query);
                        }
        } else {
                        println!("{}", format!("Invalid history number: {}", n).red());
                    }
            } else {
                    println!("{}", "Usage: !<number> (e.g., !1 for first history item)".dimmed());
                }
            }
            // Default: perform search
            _ => {
                let query = input;
                search_history.push(query.to_string());
                if search_history.len() > 20 {
                    search_history.remove(0);
                }

                let results = search_code(
                    query, path, current_extensions.as_deref(), case_insensitive,
                    fuzzy_mode, 0.6, 20, current_exclude.as_deref(),
                    ranking_mode, false, semantic_mode, false, false,
                )?;

                last_results = results.clone();

                if results.is_empty() {
                    println!("{}", "No matches found.".dimmed());
                } else {
                    print_results(&results, true, ranking_mode);
                    print_search_stats(&results, query);
                }
                println!();
            }
        }
    }

    Ok(())
}

fn print_interactive_help() {
    println!("{}", "📖 Interactive Search Commands".cyan().bold());
    println!("{}", "─".repeat(35).cyan());
    println!();
    println!("{}", "🔍 Search:".yellow().bold());
    println!("  <pattern>  - Search for text pattern");
    println!("  !!         - Repeat last search");
    println!("  !<n>       - Repeat history item #n");
    println!();
    println!("{}", "⚡ Toggles:".yellow().bold());
    println!("  /f         - Toggle fuzzy search");
    println!("  /i         - Toggle case insensitivity");
    println!("  /r         - Toggle relevance ranking");
    println!("  /s         - Toggle semantic search");
    println!();
    println!("{}", "⚙️  Config:".yellow().bold());
    println!("  ext <e1 e2>    - Set file extensions");
    println!("  exclude <d1>   - Set exclude directories");
    println!("  status         - Show current settings");
    println!();
    println!("{}", "📊 Analysis:".yellow().bold());
    println!("  analyze    - Codebase metrics");
    println!("  complexity - Code complexity");
    println!("  duplicates - Duplicate detection");
    println!("  refactor   - Refactoring suggestions");
    println!("  languages  - Supported languages");
    println!();
    println!("{}", "📁 Other:".yellow().bold());
    println!("  history    - Search history");
    println!("  export     - Export results");
    println!("  clear      - Clear screen");
    println!("  help       - This help");
    println!("  quit       - Exit");
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_option_to_name() {
        assert_eq!(ThemeOption::Default.to_theme_name(), "default");
        assert_eq!(ThemeOption::Dark.to_theme_name(), "dark");
        assert_eq!(ThemeOption::Ocean.to_theme_name(), "ocean");
    }
}
