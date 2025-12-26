//! Interactive Mode Module
//!
//! Provides an interactive REPL for code searching and analysis.

use crate::search::{print_results, print_search_stats, search_code};
use crate::types::SearchResult;
use crate::{analysis, circular, complexity, deadcode, duplicates, export};
use colored::*;
use std::io::{self, Write};
use std::path::Path;

/// Run interactive search mode
pub fn run(
    path: &Path,
    extensions: Option<&[String]>,
    exclude: Option<&[String]>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "🎮 Interactive Search Mode".cyan().bold());
    println!(
        "{}",
        "Type 'help' for commands and shortcuts, 'quit' to exit"
            .dimmed()
            .italic()
    );
    println!();

    let mut last_query: Option<String> = None;
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
                print_help();
            }
            "/f" | ":f" | "fuzzy" => {
                fuzzy_mode = !fuzzy_mode;
                println!(
                    "Fuzzy mode: {}",
                    if fuzzy_mode {
                        "ON".green()
                    } else {
                        "OFF".dimmed()
                    }
                );
            }
            "/i" | ":i" | "case" => {
                case_insensitive = !case_insensitive;
                println!(
                    "Case insensitive: {}",
                    if case_insensitive {
                        "ON".green()
                    } else {
                        "OFF".dimmed()
                    }
                );
            }
            "/r" | ":r" | "rank" => {
                ranking_mode = !ranking_mode;
                println!(
                    "Ranking mode: {}",
                    if ranking_mode {
                        "ON".green()
                    } else {
                        "OFF".dimmed()
                    }
                );
            }
            "/s" | ":s" | "semantic" => {
                semantic_mode = !semantic_mode;
                println!(
                    "Semantic search: {}",
                    if semantic_mode {
                        "ON".green()
                    } else {
                        "OFF".dimmed()
                    }
                );
            }
            "!!" | "repeat" => {
                if let Some(ref query) = last_query {
                    println!("{}", format!("Repeating: {}", query).blue());
                    let results = search_code(
                        query,
                        path,
                        current_extensions.as_deref(),
                        case_insensitive,
                        fuzzy_mode,
                        0.6,
                        20,
                        current_exclude.as_deref(),
                        ranking_mode,
                        false,
                        semantic_mode,
                        false,
                        false,
                    )?;
                    last_results = results.clone();
                    if results.is_empty() {
                        println!("{}", "No matches found.".dimmed());
                    } else {
                        print_results(&results, true, ranking_mode);
                        print_search_stats(&results, query);
                    }
                } else {
                    println!("{}", "No previous search.".dimmed());
                }
            }
            "ext" | "extensions" => {
                if parts.len() > 1 {
                    let new_exts: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();
                    current_extensions = Some(new_exts.clone());
                    println!(
                        "{}",
                        format!("Extensions set: {}", new_exts.join(", ")).green()
                    );
                } else {
                    match &current_extensions {
                        Some(exts) => {
                            println!("Current extensions: {}", exts.join(", ").blue())
                        }
                        None => println!("No extensions filter (searching all files)"),
                    }
                }
            }
            "exclude" => {
                if parts.len() > 1 {
                    let new_excl: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();
                    current_exclude = Some(new_excl.clone());
                    println!(
                        "{}",
                        format!("Exclude directories: {}", new_excl.join(", ")).green()
                    );
                } else {
                    match &current_exclude {
                        Some(excl) => {
                            println!("Excluded directories: {}", excl.join(", ").yellow())
                        }
                        None => println!("No excluded directories"),
                    }
                }
            }
            "clear" | "cls" => {
                print!("\x1B[2J\x1B[1;1H");
                io::stdout().flush()?;
            }
            "status" | "settings" => {
                println!("{}", "Current Settings:".cyan().bold());
                println!(
                    "  Fuzzy mode:       {}",
                    if fuzzy_mode {
                        "ON".green()
                    } else {
                        "OFF".dimmed()
                    }
                );
                println!(
                    "  Case insensitive: {}",
                    if case_insensitive {
                        "ON".green()
                    } else {
                        "OFF".dimmed()
                    }
                );
                println!(
                    "  Ranking mode:     {}",
                    if ranking_mode {
                        "ON".green()
                    } else {
                        "OFF".dimmed()
                    }
                );
                println!(
                    "  Semantic search:  {}",
                    if semantic_mode {
                        "ON".green()
                    } else {
                        "OFF".dimmed()
                    }
                );
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
                        let query = last_query.as_deref().unwrap_or("");
                        export::export_results(&last_results, export_path, query)?;
                        println!(
                            "{}",
                            format!("Results exported to: {}", export_path).green()
                        );
                    }
                } else {
                    println!("{}", "Usage: export <filename.csv|.md>".dimmed());
                }
            }
            "analyze" => {
                analysis::analyze_codebase(
                    path,
                    current_extensions.as_deref(),
                    current_exclude.as_deref(),
                )?;
            }
            "complexity" => {
                complexity::analyze_complexity(
                    path,
                    current_extensions.as_deref(),
                    current_exclude.as_deref(),
                    None,
                    true,
                )?;
            }
            "duplicates" | "dups" => {
                duplicates::detect_duplicates(
                    path,
                    current_extensions.as_deref(),
                    current_exclude.as_deref(),
                    3,
                    0.9,
                )?;
            }
            "deadcode" | "dead" => {
                deadcode::detect_dead_code(
                    path,
                    current_extensions.as_deref(),
                    current_exclude.as_deref(),
                )?;
            }
            "circular" | "cycle" | "cycles" => {
                circular::detect_circular_calls(
                    path,
                    current_extensions.as_deref(),
                    current_exclude.as_deref(),
                )?;
            }
            "languages" | "langs" => {
                analysis::list_supported_languages()?;
            }
            // Default: perform search
            _ => {
                let query = input;
                last_query = Some(query.to_string());

                let results = search_code(
                    query,
                    path,
                    current_extensions.as_deref(),
                    case_insensitive,
                    fuzzy_mode,
                    0.6,
                    20,
                    current_exclude.as_deref(),
                    ranking_mode,
                    false,
                    semantic_mode,
                    false,
                    false,
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

fn print_help() {
    println!("{}", "📖 Interactive Search Commands".cyan().bold());
    println!("{}", "─".repeat(35).cyan());
    println!();
    println!("{}", "Search:".yellow().bold());
    println!("  <pattern>  - Search for text pattern");
    println!("  !!         - Repeat last search");
    println!();
    println!("{}", "⚡ Toggles:".yellow().bold());
    println!("  /f         - Toggle fuzzy search");
    println!("  /i         - Toggle case insensitivity");
    println!("  /r         - Toggle relevance ranking");
    println!("  /s         - Toggle semantic search");
    println!();
    println!("{}", "Config:".yellow().bold());
    println!("  ext <e1 e2>    - Set file extensions");
    println!("  exclude <d1>   - Set exclude directories");
    println!("  status         - Show current settings");
    println!();
    println!("{}", "Analysis:".yellow().bold());
    println!("  analyze    - Codebase metrics");
    println!("  complexity - Code complexity");
    println!("  duplicates - Duplicate detection");
    println!("  deadcode   - Dead code detection");
    println!("  circular   - Circular call detection");
    println!("  languages  - Supported languages");
    println!();
    println!("{}", "Other:".yellow().bold());
    println!("  export     - Export results");
    println!("  clear      - Clear screen");
    println!("  help       - This help");
    println!("  quit       - Exit");
    println!();
}

