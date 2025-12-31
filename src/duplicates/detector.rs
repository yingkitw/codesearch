//! Core duplicate detection logic with parallel processing

use super::normalize::{calculate_hash, normalize_code, normalize_with_variables};
use super::similarity::calculate_similarity;
use super::types::{CodeBlock, DuplicateConfig, EnhancedDuplicateBlock};
use crate::parser::read_file_content;
use crate::search::list_files;
use rayon::prelude::*;
use std::collections::HashMap;
use std::path::Path;

/// Extract code blocks from content with variable-length windows
pub fn extract_code_blocks(
    file: &str,
    content: &str,
    config: &DuplicateConfig,
) -> Vec<CodeBlock> {
    let lines: Vec<&str> = content.lines().collect();
    let mut blocks = Vec::new();

    // Variable-length sliding window
    for window_size in config.min_lines..=(config.min_lines * 3).min(lines.len()) {
        for i in 0..lines.len().saturating_sub(window_size) {
            let block_lines: Vec<&str> = lines[i..i + window_size]
                .iter()
                .map(|l| l.trim())
                .filter(|l| !l.is_empty())
                .collect();

            if block_lines.len() < config.min_lines {
                continue;
            }

            let content = block_lines.join("\n");
            
            // Skip if too many comments
            if is_mostly_comments(&content) {
                continue;
            }

            let normalized = normalize_code(&content);
            let normalized_with_vars = normalize_with_variables(&content);
            
            // Tokenize
            let tokens: Vec<String> = normalized
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();

            if tokens.len() < config.min_tokens {
                continue;
            }

            blocks.push(CodeBlock {
                file: file.to_string(),
                line_start: i + 1,
                line_end: i + window_size,
                content: content.clone(),
                normalized,
                tokens,
                hash: calculate_hash(&content),
                normalized_hash: calculate_hash(&normalized_with_vars),
            });
        }
    }

    blocks
}

/// Check if content is mostly comments
fn is_mostly_comments(content: &str) -> bool {
    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() {
        return false;
    }

    let comment_lines = lines
        .iter()
        .filter(|line| {
            let trimmed = line.trim();
            trimmed.starts_with("//") || trimmed.starts_with('#') || trimmed.starts_with("/*")
        })
        .count();

    comment_lines as f64 / lines.len() as f64 > 0.5
}

/// Filter files based on configuration
pub fn should_process_file(file_path: &str, config: &DuplicateConfig) -> bool {
    // Exclude test files
    if config.exclude_tests {
        let lower = file_path.to_lowercase();
        if lower.contains("test") || lower.contains("spec") || lower.contains("__tests__") {
            return false;
        }
    }

    // Exclude generated files
    if config.exclude_generated {
        let lower = file_path.to_lowercase();
        if lower.contains("generated") 
            || lower.contains(".gen.") 
            || lower.contains("_pb.") 
            || lower.contains(".pb.") {
            return false;
        }
    }

    // Exclude custom patterns
    for pattern in &config.exclude_patterns {
        if file_path.contains(pattern) {
            return false;
        }
    }

    true
}

/// Find duplicates using hash-based indexing for performance
pub fn find_duplicates_with_index(
    blocks: Vec<CodeBlock>,
    config: &DuplicateConfig,
) -> Vec<EnhancedDuplicateBlock> {
    // Build hash index for quick lookup
    let mut hash_index: HashMap<u64, Vec<usize>> = HashMap::new();
    let mut norm_hash_index: HashMap<u64, Vec<usize>> = HashMap::new();

    for (idx, block) in blocks.iter().enumerate() {
        hash_index.entry(block.hash).or_default().push(idx);
        norm_hash_index.entry(block.normalized_hash).or_default().push(idx);
    }

    // Find candidates using hash index
    let mut duplicates = Vec::new();

    if config.use_parallel {
        duplicates = blocks
            .par_iter()
            .enumerate()
            .flat_map(|(i, block1)| {
                let mut local_dups = Vec::new();

                // Check exact hash matches (Type-1 clones)
                if config.detect_type1 {
                    if let Some(candidates) = hash_index.get(&block1.hash) {
                        for &j in candidates {
                            if j > i && blocks[j].file != block1.file {
                                let metrics = calculate_similarity(block1, &blocks[j]);
                                if metrics.overall_similarity >= config.similarity_threshold {
                                    local_dups.push(create_duplicate(block1, &blocks[j], metrics));
                                }
                            }
                        }
                    }
                }

                // Check normalized hash matches (Type-2 clones)
                if config.detect_type2 {
                    if let Some(candidates) = norm_hash_index.get(&block1.normalized_hash) {
                        for &j in candidates {
                            if j > i && blocks[j].file != block1.file && blocks[j].hash != block1.hash {
                                let metrics = calculate_similarity(block1, &blocks[j]);
                                if metrics.overall_similarity >= config.similarity_threshold {
                                    local_dups.push(create_duplicate(block1, &blocks[j], metrics));
                                }
                            }
                        }
                    }
                }

                // Check structural similarity (Type-3 clones)
                if config.detect_type3 {
                    for j in (i + 1)..blocks.len() {
                        let block2 = &blocks[j];
                        if block2.file == block1.file {
                            continue;
                        }
                        
                        // Skip if already found via hash
                        if block2.hash == block1.hash || block2.normalized_hash == block1.normalized_hash {
                            continue;
                        }

                        let metrics = calculate_similarity(block1, block2);
                        if metrics.overall_similarity >= config.similarity_threshold {
                            local_dups.push(create_duplicate(block1, block2, metrics));
                        }
                    }
                }

                local_dups
            })
            .collect();
    } else {
        // Sequential processing
        for i in 0..blocks.len() {
            let block1 = &blocks[i];
            
            for j in (i + 1)..blocks.len() {
                let block2 = &blocks[j];
                
                if block2.file == block1.file {
                    continue;
                }

                let metrics = calculate_similarity(block1, block2);
                if metrics.overall_similarity >= config.similarity_threshold {
                    duplicates.push(create_duplicate(block1, block2, metrics));
                }
            }
        }
    }

    // Sort by similarity (highest first)
    duplicates.sort_by(|a, b| {
        b.similarity
            .partial_cmp(&a.similarity)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // Deduplicate results
    deduplicate_results(duplicates)
}

fn create_duplicate(
    block1: &CodeBlock,
    block2: &CodeBlock,
    metrics: super::similarity::SimilarityMetrics,
) -> EnhancedDuplicateBlock {
    EnhancedDuplicateBlock {
        file1: block1.file.clone(),
        line1: block1.line_start,
        file2: block2.file.clone(),
        line2: block2.line_start,
        content: block1.content.chars().take(100).collect::<String>() + "...",
        similarity: metrics.overall_similarity,
        clone_type: metrics.clone_type,
        token_similarity: metrics.token_similarity,
        structural_similarity: metrics.structural_similarity,
        line_count: block1.line_end - block1.line_start + 1,
    }
}

fn deduplicate_results(mut duplicates: Vec<EnhancedDuplicateBlock>) -> Vec<EnhancedDuplicateBlock> {
    let mut seen = std::collections::HashSet::new();
    duplicates.retain(|dup| {
        let key = format!("{}:{}:{}:{}", dup.file1, dup.line1, dup.file2, dup.line2);
        seen.insert(key)
    });
    duplicates
}

/// Main entry point for duplicate detection
pub fn find_duplicates(
    path: &Path,
    extensions: Option<&[String]>,
    exclude: Option<&[String]>,
    config: DuplicateConfig,
) -> Result<Vec<EnhancedDuplicateBlock>, Box<dyn std::error::Error>> {
    let files = list_files(path, extensions, exclude)?;
    
    // Extract blocks from all files
    let all_blocks: Vec<CodeBlock> = if config.use_parallel {
        files
            .par_iter()
            .filter(|file| {
                file.size <= config.max_file_size as u64 && should_process_file(&file.path, &config)
            })
            .flat_map(|file| {
                let content = read_file_content(&file.path);
                extract_code_blocks(&file.path, &content, &config)
            })
            .collect()
    } else {
        files
            .iter()
            .filter(|file| {
                file.size <= config.max_file_size as u64 && should_process_file(&file.path, &config)
            })
            .flat_map(|file| {
                let content = read_file_content(&file.path);
                extract_code_blocks(&file.path, &content, &config)
            })
            .collect()
    };

    // Find duplicates using hash-based indexing
    Ok(find_duplicates_with_index(all_blocks, &config))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_mostly_comments() {
        let code = r#"
// Comment 1
// Comment 2
// Comment 3
let x = 5;
"#;
        assert!(is_mostly_comments(code));

        let code2 = r#"
let x = 5;
let y = 10;
// Comment
"#;
        assert!(!is_mostly_comments(code2));
    }

    #[test]
    fn test_should_process_file() {
        let config = DuplicateConfig {
            exclude_tests: true,
            exclude_generated: true,
            ..Default::default()
        };

        assert!(!should_process_file("src/test_utils.rs", &config));
        assert!(!should_process_file("src/generated.rs", &config));
        assert!(should_process_file("src/main.rs", &config));
    }

    #[test]
    fn test_extract_code_blocks() {
        let content = r#"
fn test1() {
    let x = 5;
    let y = 10;
}

fn test2() {
    let a = 5;
    let b = 10;
}
"#;
        let config = DuplicateConfig::default();
        let blocks = extract_code_blocks("test.rs", content, &config);
        assert!(!blocks.is_empty());
    }
}
