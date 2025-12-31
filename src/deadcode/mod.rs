//! Dead Code Detection Module
//!
//! Identifies potentially unused code: functions, classes, imports, and commented-out code.
//! 
//! This module is organized into sub-modules for better maintainability:
//! - `types`: Data structures for dead code items
//! - `helpers`: Utility functions for detection
//! - `detectors`: Individual detection functions for different code patterns

mod types;
mod helpers;
mod detectors;

pub use types::DeadCodeItem;

use crate::parser::{extract_classes, extract_functions, extract_identifier_references, read_file_content};
use crate::search::list_files;
use colored::*;
use std::collections::HashMap;
use std::path::Path;

use helpers::is_special_function;
use detectors::{
    detect_unused_variables,
    detect_unreachable_code,
    detect_empty_functions,
    detect_todo_fixme,
    detect_dead_code_patterns,
};

/// Detect potentially dead/unused code in the codebase
pub fn detect_dead_code(
    path: &Path,
    extensions: Option<&[String]>,
    exclude: Option<&[String]>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "Dead Code Detection".cyan().bold());
    println!("{}", "─".repeat(30).cyan());
    println!();

    let dead_code_items = find_dead_code(path, extensions, exclude)?;
    
    if dead_code_items.is_empty() {
        println!("{}", "No files found to analyze.".dimmed());
        return Ok(());
    }

    print_dead_code_results(&dead_code_items);

    Ok(())
}

/// Find dead code and return the results (shared implementation)
pub fn find_dead_code(
    path: &Path,
    extensions: Option<&[String]>,
    exclude: Option<&[String]>,
) -> Result<Vec<DeadCodeItem>, Box<dyn std::error::Error>> {
    let files = list_files(path, extensions, exclude)?;
    
    if files.is_empty() {
        return Ok(Vec::new());
    }

    let mut dead_code_items: Vec<DeadCodeItem> = Vec::new();
    let mut all_definitions: HashMap<String, (String, usize, String)> = HashMap::new();
    let mut all_references: HashMap<String, usize> = HashMap::new();

    // First pass: collect all definitions and references
    for file in &files {
        let content = read_file_content(&file.path);

        // Extract function definitions
        for (name, line_num) in extract_functions(&content, &file.path) {
            if !is_special_function(&name) {
                all_definitions.insert(
                    name.clone(),
                    (file.path.clone(), line_num, "function".to_string()),
                );
            }
        }

        // Extract class definitions
        for (name, line_num) in extract_classes(&content, &file.path) {
            all_definitions.insert(
                name.clone(),
                (file.path.clone(), line_num, "class/struct".to_string()),
            );
        }

        // Count all identifier references
        for ref_name in extract_identifier_references(&content) {
            *all_references.entry(ref_name).or_insert(0) += 1;
        }
    }

    // Second pass: find definitions with low reference count
    for (name, (file, line, item_type)) in &all_definitions {
        let ref_count = all_references.get(name).copied().unwrap_or(0);
        
        if ref_count <= 1 {
            dead_code_items.push(DeadCodeItem {
                file: file.clone(),
                line_number: *line,
                item_type: item_type.clone(),
                name: name.clone(),
                reason: "Only defined, never used elsewhere".to_string(),
            });
        } else if ref_count == 2 && item_type == "function" {
            dead_code_items.push(DeadCodeItem {
                file: file.clone(),
                line_number: *line,
                item_type: item_type.clone(),
                name: name.clone(),
                reason: "Used only once - consider inlining".to_string(),
            });
        }
    }

    // Third pass: detect other dead code patterns
    for file in &files {
        let content = read_file_content(&file.path);
        detect_dead_code_patterns(&file.path, &content, &mut dead_code_items);
        detect_unused_variables(&file.path, &content, &mut dead_code_items);
        detect_unreachable_code(&file.path, &content, &mut dead_code_items);
        detect_empty_functions(&file.path, &content, &mut dead_code_items);
        detect_todo_fixme(&file.path, &content, &mut dead_code_items);
    }

    // Sort by file and line number
    dead_code_items.sort_by(|a, b| {
        a.file.cmp(&b.file).then(a.line_number.cmp(&b.line_number))
    });

    Ok(dead_code_items)
}

fn print_dead_code_results(items: &[DeadCodeItem]) {
    if items.is_empty() {
        println!("{}", "No obvious dead code detected!".green().bold());
    } else {
        println!(
            "{}",
            format!("Found {} potential dead code items:", items.len())
                .yellow()
                .bold()
        );
        println!();

        let mut current_file = String::new();
        for item in items {
            if item.file != current_file {
                current_file = item.file.clone();
                println!("{}", format!("[{}]", current_file).cyan());
            }
            println!(
                "   {} L{}: {} '{}' - {}",
                match item.item_type.as_str() {
                    "function" => "[fn]",
                    "class/struct" => "[cls]",
                    "variable" => "[var]",
                    "import" => "[imp]",
                    "unreachable" => "[!]",
                    "empty" => "[∅]",
                    "todo" => "[?]",
                    "parameter" => "[prm]",
                    _ => "[-]",
                },
                format!("{:4}", item.line_number).yellow(),
                item.item_type.blue(),
                item.name.green(),
                item.reason.dimmed()
            );
        }

        println!();
        println!("{}", "Summary:".cyan().bold());
        
        let mut type_counts: HashMap<String, usize> = HashMap::new();
        for item in items {
            *type_counts.entry(item.item_type.clone()).or_insert(0) += 1;
        }
        
        for (item_type, count) in &type_counts {
            println!("   {} {}: {}", "•".dimmed(), item_type, count);
        }
    }
}
