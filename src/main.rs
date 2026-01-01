//! CodeSearch - A fast CLI tool for searching codebases
//!
//! This is the main entry point that orchestrates all modules.

use clap::Parser;
use colored::*;

// Use library modules
use codesearch::cli::{Cli, Commands, get_default_exclude_dirs};
use codesearch::{analysis, circular, complexity, deadcode, duplicates, export, interactive};
#[cfg(feature = "mcp")]
use codesearch::mcp;
use codesearch::search::{list_files, print_results, print_search_stats, search_code};


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
            
            let options = SearchOptions {
                extensions: cli.extensions,
                ignore_case: cli.ignore_case,
                fuzzy: cli.fuzzy,
                fuzzy_threshold: 0.6,
                max_results: cli.max_results,
                exclude: Some(final_exclude),
                rank: false,
                cache: false,
                semantic: false,
                benchmark: false,
                vs_grep: false,
            };
            
            let results = search_code(&query, &cli.path, &options)?;
            
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
            
            let options = SearchOptions {
                extensions,
                ignore_case,
                fuzzy,
                fuzzy_threshold,
                max_results,
                exclude: final_exclude,
                rank,
                cache,
                semantic,
                benchmark,
                vs_grep,
            };
            
            let results = search_code(&query, &path, &options)?;

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
        Some(Commands::DesignMetrics { path, extensions, exclude, detailed, format }) => {
            use codesearch::designmetrics::{analyze_design_metrics, print_design_metrics};
            
            println!("{}", "Analyzing design metrics...".cyan().bold());
            let metrics = analyze_design_metrics(&path, extensions.as_deref(), exclude.as_deref())?;
            
            if format == "json" {
                println!("{}", serde_json::to_string_pretty(&metrics)?);
            } else {
                print_design_metrics(&metrics, detailed);
            }
        }
        Some(Commands::Metrics { path, extensions, exclude, detailed, format }) => {
            use codesearch::codemetrics::{analyze_project_metrics, print_metrics_report};
            
            println!("{}", "Analyzing comprehensive code metrics...".cyan().bold());
            let metrics = analyze_project_metrics(&path, extensions.as_deref(), exclude.as_deref())?;
            
            if format == "json" {
                println!("{}", serde_json::to_string_pretty(&metrics)?);
            } else {
                print_metrics_report(&metrics, detailed);
            }
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
        Some(Commands::Index { path, extensions, exclude, index_file }) => {
            use codesearch::index::CodeIndex;
            use std::sync::Arc;
            
            println!("{}", "Building code index...".cyan().bold());
            let index = Arc::new(CodeIndex::new(index_file.clone()));
            index.index_directory(&path, extensions.as_deref(), exclude.as_deref())?;
            index.save()?;
            
            let stats = index.get_stats();
            println!("\n{}", "Index Statistics:".green().bold());
            println!("  Total files: {}", stats.total_files);
            println!("  Total lines: {}", stats.total_lines);
            println!("  Total functions: {}", stats.total_functions);
            println!("  Total classes: {}", stats.total_classes);
            println!("\n{}", format!("Index saved to: {}", index_file.display()).green());
        }
        Some(Commands::Watch { path, extensions, index_file }) => {
            use codesearch::index::CodeIndex;
            use codesearch::watcher::start_watching;
            use std::sync::Arc;
            
            println!("{}", "Starting file watcher...".cyan().bold());
            let index = Arc::new(CodeIndex::new(index_file));
            start_watching(path, index, extensions)?;
        }
        Some(Commands::Ast { path, extensions, format }) => {
            use codesearch::ast::analyze_file;
            use walkdir::WalkDir;
            
            println!("{}", "Analyzing code with AST...".cyan().bold());
            
            if path.is_file() {
                let analysis = analyze_file(&path)?;
                
                if format == "json" {
                    println!("{}", serde_json::to_string_pretty(&analysis)?);
                } else {
                    println!("\n{}", "Functions:".green().bold());
                    for func in &analysis.functions {
                        println!("  {} (line {}) - {} params", func.name, func.line, func.parameters.len());
                    }
                    
                    println!("\n{}", "Classes:".green().bold());
                    for class in &analysis.classes {
                        println!("  {} (line {})", class.name, class.line);
                    }
                    
                    println!("\n{}", "Imports:".green().bold());
                    for import in &analysis.imports {
                        println!("  {} (line {})", import.module, import.line);
                    }
                }
            } else {
                let walker = WalkDir::new(&path)
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .filter(|e| e.file_type().is_file());
                
                let mut total_functions = 0;
                let mut total_classes = 0;
                
                for entry in walker {
                    let file_path = entry.path();
                    if let Some(ext) = file_path.extension().and_then(|s| s.to_str()) {
                        if let Some(exts) = &extensions {
                            if !exts.iter().any(|e| e == ext) {
                                continue;
                            }
                        }
                        
                        if let Ok(analysis) = analyze_file(file_path) {
                            total_functions += analysis.functions.len();
                            total_classes += analysis.classes.len();
                        }
                    }
                }
                
                println!("\n{}", "AST Analysis Summary:".green().bold());
                println!("  Total functions: {}", total_functions);
                println!("  Total classes: {}", total_classes);
            }
        }
        Some(Commands::Cfg { path, extensions: _, format, export }) => {
            use codesearch::cfg::analyze_file_cfg;
            
            println!("{}", "Analyzing Control Flow Graph...".cyan().bold());
            
            if path.is_file() {
                let cfgs = analyze_file_cfg(&path)?;
                
                for cfg in &cfgs {
                    if format == "json" {
                        println!("{}", serde_json::to_string_pretty(&cfg)?);
                    } else if format == "dot" {
                        println!("{}", cfg.to_dot());
                    } else {
                        println!("\n{}", format!("Function: {}", cfg.function_name).green().bold());
                        println!("  Basic blocks: {}", cfg.basic_blocks.len());
                        println!("  Edges: {}", cfg.edges.len());
                        println!("  Cyclomatic complexity: {}", cfg.calculate_cyclomatic_complexity());
                        
                        let unreachable = cfg.find_unreachable_blocks();
                        if !unreachable.is_empty() {
                            println!("  {} Unreachable blocks: {:?}", "⚠️".yellow(), unreachable);
                        }
                        
                        let loops = cfg.find_loops();
                        if !loops.is_empty() {
                            println!("  Loops detected: {}", loops.len());
                        }
                    }
                    
                    if let Some(export_path) = &export {
                        let output = if format == "json" {
                            serde_json::to_string_pretty(&cfg)?
                        } else {
                            cfg.to_dot()
                        };
                        std::fs::write(export_path, output)?;
                        println!("\n{}", format!("Exported to: {}", export_path.display()).green());
                    }
                }
            }
        }
        Some(Commands::Dfg { path, extensions: _, format, export }) => {
            use codesearch::dfg::analyze_file_dfg;
            
            println!("{}", "Analyzing Data Flow Graph...".cyan().bold());
            
            if path.is_file() {
                let dfgs = analyze_file_dfg(&path)?;
                
                for dfg in &dfgs {
                    if format == "json" {
                        println!("{}", serde_json::to_string_pretty(&dfg)?);
                    } else if format == "dot" {
                        println!("{}", dfg.to_dot());
                    } else {
                        println!("\n{}", format!("Function: {}", dfg.function_name).green().bold());
                        println!("  Data nodes: {}", dfg.nodes.len());
                        println!("  Data flows: {}", dfg.edges.len());
                        
                        let unused = dfg.find_unused_variables();
                        if !unused.is_empty() {
                            println!("  {} Unused variables: {:?}", "⚠️".yellow(), unused);
                        }
                        
                        let redundant = dfg.find_redundant_computations();
                        if !redundant.is_empty() {
                            println!("  Redundant computations: {}", redundant.len());
                        }
                    }
                    
                    if let Some(export_path) = &export {
                        let output = if format == "json" {
                            serde_json::to_string_pretty(&dfg)?
                        } else {
                            dfg.to_dot()
                        };
                        std::fs::write(export_path, output)?;
                        println!("\n{}", format!("Exported to: {}", export_path.display()).green());
                    }
                }
            }
        }
        Some(Commands::Callgraph { path, extensions, exclude, format, recursive_only, dead_only }) => {
            use codesearch::callgraph::build_call_graph;
            
            println!("{}", "Building Call Graph...".cyan().bold());
            let graph = build_call_graph(&path, extensions.as_deref(), exclude.as_deref())?;
            
            if format == "json" {
                println!("{}", serde_json::to_string_pretty(&graph)?);
            } else if format == "dot" {
                println!("{}", graph.to_dot());
            } else {
                println!("\n{}", "Call Graph Analysis:".green().bold());
                println!("  Functions: {}", graph.nodes.len());
                println!("  Function calls: {}", graph.edges.len());
                
                if recursive_only || !dead_only {
                    let recursive = graph.find_recursive_functions();
                    if !recursive.is_empty() {
                        println!("\n{}", "Recursive Functions:".yellow().bold());
                        for func in &recursive {
                            println!("  - {}", func);
                        }
                    }
                }
                
                if dead_only || !recursive_only {
                    let dead = graph.find_dead_functions();
                    if !dead.is_empty() {
                        println!("\n{}", "Dead Functions (never called):".red().bold());
                        for func in &dead {
                            println!("  - {}", func);
                        }
                    }
                }
            }
        }
        Some(Commands::Pdg { path, extensions: _, format, parallel, export }) => {
            use codesearch::pdg::analyze_file_pdg;
            
            println!("{}", "Analyzing Program Dependency Graph...".cyan().bold());
            
            if path.is_file() {
                let pdgs = analyze_file_pdg(&path)?;
                
                for pdg in &pdgs {
                    if format == "json" {
                        println!("{}", serde_json::to_string_pretty(&pdg)?);
                    } else if format == "dot" {
                        println!("{}", pdg.to_dot());
                    } else {
                        println!("\n{}", format!("Function: {}", pdg.function_name).green().bold());
                        println!("  Nodes: {}", pdg.nodes.len());
                        println!("  Dependencies: {}", pdg.edges.len());
                        
                        let control_deps = pdg.edges.iter().filter(|e| e.dependency_type == codesearch::pdg::DependencyType::ControlDependence).count();
                        let data_deps = pdg.edges.iter().filter(|e| e.dependency_type == codesearch::pdg::DependencyType::DataDependence).count();
                        
                        println!("  Control dependencies: {}", control_deps);
                        println!("  Data dependencies: {}", data_deps);
                        
                        if parallel {
                            let parallel_ops = pdg.find_parallel_opportunities();
                            if !parallel_ops.is_empty() {
                                println!("\n{}", "Parallelization Opportunities:".cyan().bold());
                                for (i, group) in parallel_ops.iter().enumerate() {
                                    println!("  Group {}: {} independent operations", i + 1, group.len());
                                }
                            }
                        }
                    }
                    
                    if let Some(export_path) = &export {
                        let output = if format == "json" {
                            serde_json::to_string_pretty(&pdg)?
                        } else {
                            pdg.to_dot()
                        };
                        std::fs::write(export_path, output)?;
                        println!("\n{}", format!("Exported to: {}", export_path.display()).green());
                    }
                }
            }
        }
        Some(Commands::GraphAll { path, format, export_dir }) => {
            use codesearch::graphs::GraphAnalyzer;
            
            println!("{}", "Analyzing all graph types...".cyan().bold());
            
            let analyzer = GraphAnalyzer::new(path.to_string_lossy().to_string());
            let results = analyzer.analyze_all(&path)?;
            
            for result in &results {
                if format == "json" {
                    println!("{}", serde_json::to_string_pretty(&result)?);
                } else {
                    println!("\n{}", format!("{:?} Analysis:", result.graph_type).green().bold());
                    println!("  Nodes: {}", result.summary.node_count);
                    println!("  Edges: {}", result.summary.edge_count);
                    println!("\n{}", "Key Findings:".cyan());
                    for finding in &result.summary.key_findings {
                        println!("  • {}", finding);
                    }
                }
                
                if let Some(export_dir) = &export_dir {
                    std::fs::create_dir_all(export_dir)?;
                    
                    if let Some(dot) = &result.dot_output {
                        let filename = format!("{:?}_{}.dot", result.graph_type, path.file_stem().unwrap_or_default().to_string_lossy());
                        let export_path = export_dir.join(filename);
                        std::fs::write(&export_path, dot)?;
                        println!("  Exported DOT to: {}", export_path.display());
                    }
                    
                    if let Some(json) = &result.json_output {
                        let filename = format!("{:?}_{}.json", result.graph_type, path.file_stem().unwrap_or_default().to_string_lossy());
                        let export_path = export_dir.join(filename);
                        std::fs::write(&export_path, json)?;
                        println!("  Exported JSON to: {}", export_path.display());
                    }
                }
            }
        }
        Some(Commands::Depgraph { path, extensions, exclude, format, circular_only }) => {
            use codesearch::depgraph::build_dependency_graph;
            
            println!("{}", "Building dependency graph...".cyan().bold());
            let graph = build_dependency_graph(&path, extensions.as_deref(), exclude.as_deref())?;
            
            if circular_only {
                let cycles = graph.find_circular_dependencies();
                if cycles.is_empty() {
                    println!("{}", "No circular dependencies found.".green());
                } else {
                    println!("\n{}", format!("Found {} circular dependencies:", cycles.len()).red().bold());
                    for (i, cycle) in cycles.iter().enumerate() {
                        println!("\nCycle {}:", i + 1);
                        for node in cycle {
                            println!("  -> {}", node);
                        }
                    }
                }
            } else if format == "json" {
                println!("{}", serde_json::to_string_pretty(&graph)?);
            } else if format == "dot" {
                println!("{}", graph.to_dot());
            } else {
                println!("\n{}", "Dependency Graph:".green().bold());
                println!("  Total nodes: {}", graph.nodes.len());
                println!("  Total edges: {}", graph.edges.len());
                
                let root_nodes = graph.get_root_nodes();
                let leaf_nodes = graph.get_leaf_nodes();
                
                println!("\n{}", "Root nodes (no dependencies):".cyan());
                for node in &root_nodes {
                    println!("  {}", node);
                }
                
                println!("\n{}", "Leaf nodes (no dependents):".cyan());
                for node in &leaf_nodes {
                    println!("  {}", node);
                }
            }
        }
        Some(Commands::GitHistory { query, path, max_commits, author, message, file }) => {
            use codesearch::githistory::GitSearcher;
            
            println!("{}", "Searching git history...".cyan().bold());
            let searcher = GitSearcher::new(&path)?;
            
            if let Some(author_name) = author {
                let commits = searcher.search_by_author(&author_name, max_commits)?;
                println!("\n{}", format!("Found {} commits by {}:", commits.len(), author_name).green().bold());
                for commit in commits {
                    println!("\n{} by {} ({})", commit.id[..8].to_string().yellow(), commit.author, commit.timestamp);
                    println!("  {}", commit.message.lines().next().unwrap_or(""));
                }
            } else if message {
                let commits = searcher.search_by_message(&query, max_commits)?;
                println!("\n{}", format!("Found {} commits matching message:", commits.len()).green().bold());
                for commit in commits {
                    println!("\n{} by {}", commit.id[..8].to_string().yellow(), commit.author);
                    println!("  {}", commit.message.lines().next().unwrap_or(""));
                }
            } else if let Some(file_path) = file {
                let results = searcher.search_file_history(&file_path, &query, max_commits)?;
                println!("\n{}", format!("Found {} matches in file history:", results.len()).green().bold());
                for result in results {
                    println!("\n{} - Line {}", result.commit_id[..8].to_string().yellow(), result.line_number);
                    println!("  {}", result.content);
                }
            } else {
                let results = searcher.search_history(&query, max_commits)?;
                println!("\n{}", format!("Found {} matches in history:", results.len()).green().bold());
                for result in results {
                    println!("\n{} - {} (line {})", result.commit_id[..8].to_string().yellow(), result.file_path, result.line_number);
                    println!("  {}", result.content);
                }
            }
        }
        Some(Commands::Remote { query, repo, extensions, token, github, language, max_results }) => {
            use codesearch::remote::RemoteSearcher;
            
            let api_token = token.or_else(|| std::env::var("GITHUB_TOKEN").ok());
            let searcher = RemoteSearcher::new(api_token)?;
            
            if github {
                println!("{}", "Searching GitHub...".cyan().bold());
                let results = searcher.search_github(&query, language.as_deref(), max_results)?;
                println!("\n{}", format!("Found {} results:", results.len()).green().bold());
                for result in results {
                    println!("\n{} - {}", result.repository.yellow(), result.file_path);
                    println!("  {}", result.url);
                }
            } else if let Some(repo_url) = repo {
                println!("{}", format!("Cloning and searching {}...", repo_url).cyan().bold());
                let results = searcher.clone_and_search(&repo_url, &query, extensions.as_deref())?;
                println!("\n{}", format!("Found {} matches:", results.len()).green().bold());
                for result in results {
                    println!("\n{} (line {})", result.file_path.yellow(), result.line_number);
                    println!("  {}", result.content);
                }
            } else {
                eprintln!("{}", "Error: Must specify --repo or --github".red());
                return Ok(());
            }
        }
        Some(Commands::Languages) => {
            analysis::list_supported_languages()?;
        }
        Some(Commands::McpServer) => {
            #[cfg(feature = "mcp")]
            {
                use tokio::runtime::Runtime;
                let rt = Runtime::new()?;
                rt.block_on(mcp::start_mcp_server())?;
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

