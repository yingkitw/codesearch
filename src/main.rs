use clap::{Parser, Subcommand};
use colored::*;
use regex::Regex;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

#[derive(Parser)]
#[command(name = "code-search")]
#[command(about = "A fast CLI tool for searching codebases")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
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
        /// Show line numbers
        #[arg(short, long)]
        line_numbers: bool,
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
        /// Exclude directories (e.g., target,node_modules)
        #[arg(long, value_delimiter = ',')]
        exclude: Option<Vec<String>>,
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
}

#[derive(Debug, serde::Serialize)]
struct SearchResult {
    file: String,
    line_number: usize,
    content: String,
    matches: Vec<Match>,
}

#[derive(Debug, serde::Serialize)]
struct Match {
    start: usize,
    end: usize,
    text: String,
}

#[derive(Debug, Clone, serde::Serialize)]
struct FileInfo {
    path: String,
    size: u64,
    lines: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Search {
            query,
            path,
            extensions,
            ignore_case,
            line_numbers,
            max_results,
            format,
            stats,
            fuzzy,
            fuzzy_threshold,
            exclude,
        } => {
            let results = search_code(
                &query,
                &path,
                extensions.as_deref(),
                ignore_case,
                fuzzy,
                fuzzy_threshold,
                max_results,
                exclude.as_deref(),
            )?;

            match format.as_str() {
                "json" => {
                    let json = serde_json::to_string_pretty(&results)?;
                    println!("{}", json);
                }
                _ => {
                    print_results(&results, line_numbers);
                    if stats {
                        print_search_stats(&results, &query);
                    }
                }
            }
        }
        Commands::Files {
            path,
            extensions,
            exclude,
        } => {
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
        Commands::Interactive {
            path,
            extensions,
            exclude,
        } => {
            interactive_search(&path, extensions.as_deref(), exclude.as_deref())?;
        }
        Commands::Analyze {
            path,
            extensions,
            exclude,
        } => {
            analyze_codebase(&path, extensions.as_deref(), exclude.as_deref())?;
        }
    }

    Ok(())
}

fn search_code(
    query: &str,
    path: &Path,
    extensions: Option<&[String]>,
    ignore_case: bool,
    fuzzy: bool,
    fuzzy_threshold: f64,
    max_results: usize,
    exclude: Option<&[String]>,
) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
    let mut results = Vec::new();
    
    let regex = if fuzzy {
        // For fuzzy search, we'll use a more permissive pattern
        if ignore_case {
            Regex::new(&format!("(?i).*{}.*", regex::escape(query)))?
        } else {
            Regex::new(&format!(".*{}.*", regex::escape(query)))?
        }
    } else if ignore_case {
        Regex::new(&format!("(?i){}", regex::escape(query)))?
    } else {
        Regex::new(query)?
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

        if let Ok(matches) = search_in_file(file_path, &regex, fuzzy, fuzzy_threshold, query, max_results) {
            results.extend(matches);
        }
    }

    Ok(results)
}

fn search_in_file(
    file_path: &Path,
    regex: &Regex,
    fuzzy: bool,
    fuzzy_threshold: f64,
    query: &str,
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

                    results.push(SearchResult {
                        file: file_path.to_string_lossy().to_string(),
                        line_number: line_count,
                        content: line.clone(),
                        matches,
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

                results.push(SearchResult {
                    file: file_path.to_string_lossy().to_string(),
                    line_number: line_count,
                    content: line.clone(),
                    matches,
                });
            }
        }
    }

    Ok(results)
}

fn list_files(
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

fn print_results(results: &[SearchResult], line_numbers: bool) {
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

        // Add indentation for better readability
        let indented_content = if line_numbers {
            format!("  {}{}", line_info, content)
        } else {
            format!("  {}", content)
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
        print!("{}", "code-search> ".green().bold());
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
                )?;

                if results.is_empty() {
                    println!("{}", "No matches found.".yellow());
                } else {
                    print_results(&results, true);
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

fn analyze_codebase(
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
        let regex = Regex::new("Hello").unwrap();
        
        let results = search_in_file(&test_file, &regex, false, 0.6, "Hello", 10).unwrap();
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
