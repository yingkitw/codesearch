//! Code Duplication Detection Module
//!
//! Detects similar code blocks across files using Jaccard similarity.

use crate::search::list_files;
use crate::types::DuplicateBlock;
use colored::*;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// Detect code duplication in a directory
pub fn detect_duplicates(
    path: &Path,
    extensions: Option<&[String]>,
    exclude: Option<&[String]>,
    min_lines: usize,
    similarity_threshold: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "🔍 Code Duplication Detection".cyan().bold());
    println!("{}", "─".repeat(30).cyan());
    println!();

    let files = list_files(path, extensions, exclude)?;
    let mut all_blocks: Vec<(String, usize, String)> = Vec::new(); // (file, line, block)

    // Extract code blocks from all files
    for file in &files {
        if let Ok(content) = fs::read_to_string(&file.path) {
            let blocks = extract_code_blocks(&content, min_lines);
            for (line_num, block) in blocks {
                all_blocks.push((file.path.clone(), line_num, block));
            }
        }
    }

    let mut duplicates: Vec<DuplicateBlock> = Vec::new();

    // Compare all blocks for similarity
    for i in 0..all_blocks.len() {
        for j in (i + 1)..all_blocks.len() {
            let (file1, line1, block1) = &all_blocks[i];
            let (file2, line2, block2) = &all_blocks[j];

            // Skip comparison within same file if blocks are adjacent
            if file1 == file2 && (line1.abs_diff(*line2) < min_lines) {
                continue;
            }

            let similarity = string_similarity(block1, block2);

            if similarity >= similarity_threshold {
                duplicates.push(DuplicateBlock {
                    file1: file1.clone(),
                    line1: *line1,
                    file2: file2.clone(),
                    line2: *line2,
                    content: block1.chars().take(100).collect::<String>() + "...",
                    similarity,
                });
            }
        }
    }

    // Sort by similarity (highest first)
    duplicates.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap_or(std::cmp::Ordering::Equal));

    // Print results
    if duplicates.is_empty() {
        println!("{}", "✨ No significant code duplication found!".green().italic());
    } else {
        for dup in &duplicates {
            println!(
                "{} {:.0}% similar",
                "🔄".dimmed(),
                dup.similarity * 100.0
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
            println!("   {}", dup.content.dimmed());
            println!();
        }

        println!("{}", "─".repeat(50).dimmed());
        println!(
            "{} {} potential duplicates found",
            "📊".dimmed(),
            duplicates.len().to_string().yellow().bold()
        );
    }

    Ok(())
}

/// Extract meaningful code blocks from content
fn extract_code_blocks(content: &str, min_lines: usize) -> Vec<(usize, String)> {
    let lines: Vec<&str> = content.lines().collect();
    let mut blocks = Vec::new();

    // Sliding window to extract blocks
    for i in 0..lines.len().saturating_sub(min_lines) {
        let block: Vec<&str> = lines[i..i + min_lines]
            .iter()
            .map(|l| *l)
            .map(|l| l.trim())
            .filter(|l| !l.is_empty() && !l.starts_with("//") && !l.starts_with('#'))
            .collect();

        if block.len() >= min_lines {
            blocks.push((i + 1, block.join("\n")));
        }
    }

    blocks
}

/// Calculate similarity between two strings using Jaccard similarity
pub fn string_similarity(s1: &str, s2: &str) -> f64 {
    if s1.is_empty() && s2.is_empty() {
        return 1.0;
    }
    if s1.is_empty() || s2.is_empty() {
        return 0.0;
    }

    // Tokenize into words/tokens
    let tokens1: HashSet<&str> = s1.split_whitespace().collect();
    let tokens2: HashSet<&str> = s2.split_whitespace().collect();

    if tokens1.is_empty() && tokens2.is_empty() {
        return 1.0;
    }

    let intersection = tokens1.intersection(&tokens2).count();
    let union = tokens1.union(&tokens2).count();

    if union == 0 {
        return 0.0;
    }

    intersection as f64 / union as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_similarity_identical() {
        let sim = string_similarity("hello world", "hello world");
        assert!((sim - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_string_similarity_different() {
        let sim = string_similarity("hello world", "foo bar");
        assert!(sim < 0.5);
    }

    #[test]
    fn test_string_similarity_partial() {
        let sim = string_similarity("hello world", "hello there");
        assert!(sim > 0.0 && sim < 1.0);
    }

    #[test]
    fn test_string_similarity_empty() {
        assert!((string_similarity("", "") - 1.0).abs() < 0.001);
        assert!((string_similarity("hello", "") - 0.0).abs() < 0.001);
        assert!((string_similarity("", "hello") - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_extract_code_blocks() {
        let content = r#"
fn main() {
    println!("hello");
    let x = 5;
    let y = 10;
}
"#;
        let blocks = extract_code_blocks(content, 3);
        assert!(!blocks.is_empty());
    }
}

