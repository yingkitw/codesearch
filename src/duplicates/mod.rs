//! Code Duplication Detection Module
//!
//! Enhanced duplicate detection with multi-metric similarity, clone type classification,
//! and performance optimizations.
//!
//! This module is organized into sub-modules for better maintainability:
//! - `types`: Data structures and configuration
//! - `normalize`: Code normalization for better comparison
//! - `similarity`: Multi-metric similarity calculation
//! - `detector`: Core detection logic with parallel processing

mod types;
mod normalize;
mod similarity;
mod detector;

pub use types::{CloneType, DuplicateConfig, EnhancedDuplicateBlock};

use crate::types::DuplicateBlock;
use colored::*;
use std::path::Path;

/// Detect code duplication in a directory (legacy interface)
pub fn detect_duplicates(
    path: &Path,
    extensions: Option<&[String]>,
    exclude: Option<&[String]>,
    min_lines: usize,
    similarity_threshold: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "Code Duplication Detection".cyan().bold());
    println!("{}", "─".repeat(30).cyan());
    println!();

    let config = DuplicateConfig {
        min_lines,
        similarity_threshold,
        ..Default::default()
    };

    let duplicates = detector::find_duplicates(path, extensions, exclude, config)?;

    print_enhanced_results(&duplicates);
    Ok(())
}

/// Find duplicates and return the results (legacy interface)
pub fn find_duplicates(
    path: &Path,
    extensions: Option<&[String]>,
    exclude: Option<&[String]>,
    min_lines: usize,
    similarity_threshold: f64,
) -> Result<Vec<DuplicateBlock>, Box<dyn std::error::Error>> {
    let config = DuplicateConfig {
        min_lines,
        similarity_threshold,
        ..Default::default()
    };

    let enhanced = detector::find_duplicates(path, extensions, exclude, config)?;
    
    // Convert to legacy format
    Ok(enhanced.iter().map(|e| DuplicateBlock {
        file1: e.file1.clone(),
        line1: e.line1,
        file2: e.file2.clone(),
        line2: e.line2,
        content: e.content.clone(),
        similarity: e.similarity,
    }).collect())
}

/// Enhanced duplicate detection with full configuration
pub fn detect_duplicates_enhanced(
    path: &Path,
    extensions: Option<&[String]>,
    exclude: Option<&[String]>,
    config: DuplicateConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "Enhanced Code Duplication Detection".cyan().bold());
    println!("{}", "─".repeat(40).cyan());
    println!();

    let duplicates = detector::find_duplicates(path, extensions, exclude, config)?;

    print_enhanced_results(&duplicates);
    Ok(())
}

/// Find duplicates with enhanced configuration
pub fn find_duplicates_enhanced(
    path: &Path,
    extensions: Option<&[String]>,
    exclude: Option<&[String]>,
    config: DuplicateConfig,
) -> Result<Vec<EnhancedDuplicateBlock>, Box<dyn std::error::Error>> {
    detector::find_duplicates(path, extensions, exclude, config)
}

fn print_enhanced_results(duplicates: &[EnhancedDuplicateBlock]) {
    if duplicates.is_empty() {
        println!("{}", "No significant code duplication found!".green().italic());
        return;
    }

    // Group by clone type
    let mut type1_count = 0;
    let mut type2_count = 0;
    let mut type3_count = 0;
    let mut type4_count = 0;

    for dup in duplicates {
        match dup.clone_type {
            CloneType::Type1 => type1_count += 1,
            CloneType::Type2 => type2_count += 1,
            CloneType::Type3 => type3_count += 1,
            CloneType::Type4 => type4_count += 1,
        }

        let clone_badge = match dup.clone_type {
            CloneType::Type1 => "T1".red(),
            CloneType::Type2 => "T2".yellow(),
            CloneType::Type3 => "T3".blue(),
            CloneType::Type4 => "T4".magenta(),
        };

        println!(
            "{} {} {:.0}% similar ({} lines)",
            "🔄".dimmed(),
            clone_badge,
            dup.similarity * 100.0,
            dup.line_count
        );
        println!(
            "   {} {}:{}",
            "→".dimmed(),
            dup.file1.blue(),
            dup.line1.to_string().yellow()
        );
        println!(
            "   {} {}:{}",
            "→".dimmed(),
            dup.file2.blue(),
            dup.line2.to_string().yellow()
        );
        println!(
            "   {} Token: {:.0}% | Structural: {:.0}%",
            "📊".dimmed(),
            dup.token_similarity * 100.0,
            dup.structural_similarity * 100.0
        );
        println!("   {}", dup.content.dimmed());
        println!();
    }

    println!("{}", "─".repeat(60).dimmed());
    println!(
        "{} {} potential duplicates found",
        "📈".dimmed(),
        duplicates.len().to_string().yellow().bold()
    );
    
    println!("\n{}", "Clone Type Breakdown:".cyan());
    if type1_count > 0 {
        println!("  {} Type-1 (Exact): {}", "•".dimmed(), type1_count.to_string().red());
    }
    if type2_count > 0 {
        println!("  {} Type-2 (Renamed): {}", "•".dimmed(), type2_count.to_string().yellow());
    }
    if type3_count > 0 {
        println!("  {} Type-3 (Modified): {}", "•".dimmed(), type3_count.to_string().blue());
    }
    if type4_count > 0 {
        println!("  {} Type-4 (Semantic): {}", "•".dimmed(), type4_count.to_string().magenta());
    }
}

// Re-export for backward compatibility
pub use similarity::token_similarity as string_similarity;
