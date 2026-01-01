//! Analysis Command Handlers
//!
//! Handles all code analysis CLI commands.

use crate::{analysis, complexity, deadcode};
use std::path::Path;

/// Handle the analyze command
///
/// Performs comprehensive codebase analysis including file counts,
/// language detection, and code patterns.
///
/// # Arguments
///
/// * `path` - The directory to analyze
/// * `extensions` - Optional file extensions to filter by
/// * `exclude` - Optional directories to exclude
///
/// # Examples
///
/// ```no_run
/// use codesearch::commands::analysis::handle_analyze_command;
/// use std::path::Path;
///
/// handle_analyze_command(Path::new("src"), None, None).unwrap();
/// ```
pub fn handle_analyze_command(
    path: &Path,
    extensions: Option<&[String]>,
    exclude: Option<&[String]>,
) -> Result<(), Box<dyn std::error::Error>> {
    analysis::analyze_codebase(path, extensions, exclude)?;
    Ok(())
}

/// Handle the complexity command
///
/// Analyzes code complexity metrics including cyclomatic and cognitive complexity.
///
/// # Arguments
///
/// * `path` - The directory to analyze
/// * `extensions` - Optional file extensions to filter by
/// * `threshold` - Minimum complexity threshold to report
/// * `sort` - Whether to sort results by complexity
///
/// # Examples
///
/// ```no_run
/// use codesearch::commands::analysis::handle_complexity_command;
/// use std::path::Path;
///
/// handle_complexity_command(Path::new("src"), None, 10, true).unwrap();
/// ```
pub fn handle_complexity_command(
    path: &Path,
    extensions: Option<&[String]>,
    threshold: u32,
    sort: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    complexity::analyze_complexity(path, extensions, threshold, sort)?;
    Ok(())
}

/// Handle the deadcode command
///
/// Detects potential dead code including unused variables, unreachable code,
/// empty functions, and TODO markers.
///
/// # Arguments
///
/// * `path` - The directory to analyze
/// * `extensions` - Optional file extensions to filter by
///
/// # Examples
///
/// ```no_run
/// use codesearch::commands::analysis::handle_deadcode_command;
/// use std::path::Path;
///
/// handle_deadcode_command(Path::new("src"), Some(&[String::from("rs")])).unwrap();
/// ```
pub fn handle_deadcode_command(
    path: &Path,
    extensions: Option<&[String]>,
) -> Result<(), Box<dyn std::error::Error>> {
    deadcode::detect_dead_code(path, extensions)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    #[test]
    fn test_handle_analyze_command() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("test.rs"), "fn test() {}").unwrap();

        let result = handle_analyze_command(dir.path(), None, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_complexity_command() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("test.rs"), "fn test() { if true { } }").unwrap();

        let result = handle_complexity_command(dir.path(), None, 1, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_deadcode_command() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("test.rs"), "fn test() { let x = 1; }").unwrap();

        let result = handle_deadcode_command(dir.path(), None);
        assert!(result.is_ok());
    }
}
