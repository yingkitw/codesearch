//! Error Handling Examples
//!
//! Demonstrates how to use custom error types for better error handling.

use codesearch::errors::{AnalysisError, SearchError};
use codesearch::search::{search_code, DefaultSearchEngine};
use codesearch::traits::SearchEngine;
use codesearch::types::SearchOptions;
use std::path::{Path, PathBuf};

/// Example 1: Handling specific search errors
fn example_search_with_error_handling() -> Result<(), SearchError> {
    let path = Path::new("src");
    let options = SearchOptions::default();
    
    // Using the function directly (returns Box<dyn Error>)
    match search_code("test", path, &options) {
        Ok(results) => {
            println!("Found {} results", results.len());
            Ok(())
        }
        Err(e) => {
            // Convert generic error to specific SearchError
            eprintln!("Search failed: {}", e);
            Err(SearchError::InvalidOptions {
                message: e.to_string(),
            })
        }
    }
}

/// Example 2: Using custom error types with trait objects
fn example_trait_based_search() -> Result<(), SearchError> {
    let engine: Box<dyn SearchEngine> = Box::new(DefaultSearchEngine::new());
    let options = SearchOptions::default();
    
    match engine.search("fn main", Path::new("src"), &options) {
        Ok(results) => {
            println!("Found {} matches", results.len());
            Ok(())
        }
        Err(e) => {
            eprintln!("Search error: {}", e);
            Err(SearchError::InvalidOptions {
                message: e.to_string(),
            })
        }
    }
}

/// Example 3: Creating and handling specific error types
fn example_specific_errors() {
    // File not found error
    let err = SearchError::FileNotFound {
        path: PathBuf::from("/nonexistent/file.rs"),
    };
    println!("Error: {}", err);
    
    // Invalid pattern error
    let regex_err = regex::Regex::new("[").unwrap_err();
    let err = SearchError::InvalidPattern {
        pattern: "[".to_string(),
        source: regex_err,
    };
    println!("Error: {}", err);
    if let Some(source) = std::error::Error::source(&err) {
        println!("Caused by: {}", source);
    }
    
    // Analysis error
    let err = AnalysisError::UnsupportedFileType {
        extension: "xyz".to_string(),
    };
    println!("Error: {}", err);
}

/// Example 4: Error context with anyhow
fn example_with_context() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("src");
    let options = SearchOptions::default();
    
    search_code("test", path, &options)?;
    
    Ok(())
}

/// Example 5: Converting between error types
fn example_error_conversion() {
    // IO error to SearchError
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let search_err: SearchError = io_err.into();
    println!("Converted error: {}", search_err);
    
    // Regex error to SearchError
    let regex_err = regex::Regex::new("(").unwrap_err();
    let search_err: SearchError = regex_err.into();
    println!("Converted error: {}", search_err);
}

fn main() {
    println!("=== Error Handling Examples ===\n");
    
    println!("Example 1: Search with error handling");
    if let Err(e) = example_search_with_error_handling() {
        eprintln!("Error: {}", e);
    }
    
    println!("\nExample 2: Trait-based search");
    if let Err(e) = example_trait_based_search() {
        eprintln!("Error: {}", e);
    }
    
    println!("\nExample 3: Specific error types");
    example_specific_errors();
    
    println!("\nExample 4: Error context with anyhow");
    if let Err(e) = example_with_context() {
        eprintln!("Error: {:?}", e);
    }
    
    println!("\nExample 5: Error conversion");
    example_error_conversion();
}
