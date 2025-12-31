//! Multi-metric similarity calculation

use super::types::{CloneType, CodeBlock};
use std::collections::HashSet;

/// Multi-metric similarity result
#[derive(Debug, Clone)]
pub struct SimilarityMetrics {
    pub token_similarity: f64,
    pub structural_similarity: f64,
    pub overall_similarity: f64,
    pub clone_type: CloneType,
}

/// Calculate comprehensive similarity between two code blocks
pub fn calculate_similarity(block1: &CodeBlock, block2: &CodeBlock) -> SimilarityMetrics {
    // 1. Token-based similarity (Jaccard)
    let token_sim = token_similarity(&block1.tokens, &block2.tokens);
    
    // 2. Structural similarity (normalized code comparison)
    let structural_sim = structural_similarity(&block1.normalized, &block2.normalized);
    
    // 3. Determine clone type
    let clone_type = determine_clone_type(
        &block1.content,
        &block2.content,
        &block1.normalized,
        &block2.normalized,
        block1.hash,
        block2.hash,
        block1.normalized_hash,
        block2.normalized_hash,
    );
    
    // 4. Calculate weighted overall similarity
    let overall = match clone_type {
        CloneType::Type1 => 1.0,
        CloneType::Type2 => token_sim * 0.3 + structural_sim * 0.7,
        CloneType::Type3 => token_sim * 0.5 + structural_sim * 0.5,
        CloneType::Type4 => token_sim * 0.8,
    };
    
    SimilarityMetrics {
        token_similarity: token_sim,
        structural_similarity: structural_sim,
        overall_similarity: overall,
        clone_type,
    }
}

/// Token-based similarity using Jaccard index
pub fn token_similarity(tokens1: &[String], tokens2: &[String]) -> f64 {
    if tokens1.is_empty() && tokens2.is_empty() {
        return 1.0;
    }
    if tokens1.is_empty() || tokens2.is_empty() {
        return 0.0;
    }
    
    let set1: HashSet<&String> = tokens1.iter().collect();
    let set2: HashSet<&String> = tokens2.iter().collect();
    
    let intersection = set1.intersection(&set2).count();
    let union = set1.union(&set2).count();
    
    if union == 0 {
        return 0.0;
    }
    
    intersection as f64 / union as f64
}

/// Structural similarity based on normalized code
pub fn structural_similarity(normalized1: &str, normalized2: &str) -> f64 {
    if normalized1 == normalized2 {
        return 1.0;
    }
    
    // Use Levenshtein distance ratio
    let distance = levenshtein_distance(normalized1, normalized2);
    let max_len = normalized1.len().max(normalized2.len());
    
    if max_len == 0 {
        return 1.0;
    }
    
    1.0 - (distance as f64 / max_len as f64)
}

/// Calculate Levenshtein distance between two strings
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();
    
    if len1 == 0 {
        return len2;
    }
    if len2 == 0 {
        return len1;
    }
    
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];
    
    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }
    
    let chars1: Vec<char> = s1.chars().collect();
    let chars2: Vec<char> = s2.chars().collect();
    
    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if chars1[i - 1] == chars2[j - 1] { 0 } else { 1 };
            matrix[i][j] = (matrix[i - 1][j] + 1)
                .min(matrix[i][j - 1] + 1)
                .min(matrix[i - 1][j - 1] + cost);
        }
    }
    
    matrix[len1][len2]
}

/// Determine clone type based on various similarity metrics
fn determine_clone_type(
    _content1: &str,
    _content2: &str,
    normalized1: &str,
    normalized2: &str,
    _hash1: u64,
    _hash2: u64,
    norm_hash1: u64,
    norm_hash2: u64,
) -> CloneType {
    // Type-1: Exact match after normalization (whitespace/comments removed)
    if normalized1 == normalized2 {
        return CloneType::Type1;
    }
    
    // Type-2: Same structure with renamed variables
    if norm_hash1 == norm_hash2 {
        return CloneType::Type2;
    }
    
    // Type-3: Similar with modifications
    let structural_sim = structural_similarity(normalized1, normalized2);
    if structural_sim > 0.7 {
        return CloneType::Type3;
    }
    
    // Type-4: Semantic similarity (for now, just low structural similarity)
    CloneType::Type4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_similarity() {
        let tokens1 = vec!["hello".to_string(), "world".to_string()];
        let tokens2 = vec!["hello".to_string(), "world".to_string()];
        let sim = token_similarity(&tokens1, &tokens2);
        assert!((sim - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_token_similarity_partial() {
        let tokens1 = vec!["hello".to_string(), "world".to_string()];
        let tokens2 = vec!["hello".to_string(), "there".to_string()];
        let sim = token_similarity(&tokens1, &tokens2);
        assert!(sim > 0.0 && sim < 1.0);
    }

    #[test]
    fn test_structural_similarity_identical() {
        let sim = structural_similarity("hello world", "hello world");
        assert!((sim - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_structural_similarity_different() {
        let sim = structural_similarity("hello world", "goodbye world");
        assert!(sim < 1.0);
    }

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("", ""), 0);
        assert_eq!(levenshtein_distance("hello", "hello"), 0);
        assert_eq!(levenshtein_distance("hello", "hallo"), 1);
        assert_eq!(levenshtein_distance("hello", ""), 5);
    }
}
