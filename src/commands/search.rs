//! Search Command Handlers
//!
//! Handles all search-related CLI commands.

use crate::search::{print_results, print_search_stats, search_code};
use crate::types::SearchOptions;
use crate::export;
use colored::*;
use std::path::Path;

/// Handle the search command
///
/// # Arguments
///
/// * `query` - The search pattern
/// * `path` - The directory to search in
/// * `options` - Search configuration options
/// * `no_line_numbers` - Whether to hide line numbers
/// * `format` - Output format (text/json)
/// * `stats` - Whether to show statistics
/// * `export_path` - Optional path to export results
///
/// # Returns
///
/// Result indicating success or error
///
/// # Examples
///
/// ```no_run
/// use codesearch::commands::search::handle_search_command;
/// use codesearch::types::SearchOptions;
/// use std::path::Path;
///
/// let options = SearchOptions::default();
/// handle_search_command(
///     "test",
///     Path::new("src"),
///     options,
///     false,
///     "text",
///     true,
///     None
/// ).unwrap();
/// ```
pub fn handle_search_command(
    query: &str,
    path: &Path,
    options: SearchOptions,
    no_line_numbers: bool,
    format: &str,
    stats: bool,
    export_path: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let results = search_code(query, path, &options)?;

    if let Some(path) = export_path {
        export::export_results(&results, &path, query)?;
        println!("{}", format!("Results exported to: {}", path).green());
    } else {
        match format {
            "json" => {
                let json = serde_json::to_string_pretty(&results)?;
                println!("{}", json);
            }
            _ => {
                print_results(&results, !no_line_numbers, options.rank);
                if stats {
                    print_search_stats(&results, query);
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    #[test]
    fn test_handle_search_command() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("test.rs"), "fn test() {}").unwrap();

        let options = SearchOptions::default();
        let result = handle_search_command(
            "test",
            dir.path(),
            options,
            false,
            "text",
            false,
            None,
        );

        assert!(result.is_ok());
    }
}
