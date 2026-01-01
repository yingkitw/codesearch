//! Pure Functions for Search Logic
//!
//! This module contains pure functions separated from I/O operations
//! for better testability and maintainability.

use crate::types::Match;

/// Calculate relevance score for a search result (pure function)
///
/// This is a pure function that doesn't perform any I/O operations,
/// making it easy to test and reason about.
///
/// # Arguments
///
/// * `line_content` - The content of the line
/// * `query` - The search query
/// * `line_number` - The line number in the file
/// * `file_extension` - Optional file extension
/// * `is_fuzzy` - Whether fuzzy matching was used
/// * `fuzzy_score` - Optional fuzzy match score
///
/// # Returns
///
/// A relevance score between 0.0 and 100.0
///
/// # Examples
///
/// ```
/// use codesearch::search::pure::calculate_relevance_score_pure;
///
/// let score = calculate_relevance_score_pure(
///     "fn test_function() {",
///     "test",
///     10,
///     Some("rs"),
///     false,
///     None
/// );
/// assert!(score > 0.0 && score <= 100.0);
/// ```
pub fn calculate_relevance_score_pure(
    line_content: &str,
    query: &str,
    line_number: usize,
    file_extension: Option<&str>,
    is_fuzzy: bool,
    fuzzy_score: Option<i64>,
) -> f64 {
    let mut score = 50.0;

    // Boost for exact matches
    if line_content.contains(query) {
        score += 20.0;
    }

    // Boost for case-sensitive matches
    if line_content == query {
        score += 30.0;
    }

    // Boost for matches at start of line
    if line_content.trim_start().starts_with(query) {
        score += 15.0;
    }

    // Boost for shorter lines (more relevant)
    if line_content.len() < 80 {
        score += 10.0;
    }

    // Boost for early lines in file
    if line_number < 50 {
        score += 5.0;
    }

    // Boost for specific file types
    if let Some(ext) = file_extension {
        match ext {
            "rs" | "py" | "js" | "ts" => score += 5.0,
            _ => {}
        }
    }

    // Adjust for fuzzy matching
    if is_fuzzy {
        if let Some(fs) = fuzzy_score {
            score = (score + fs as f64) / 2.0;
        }
    }

    score.clamp(0.0, 100.0)
}

/// Determine relevance category from score (pure function)
///
/// # Examples
///
/// ```
/// use codesearch::search::pure::relevance_category;
///
/// assert_eq!(relevance_category(85.0), "Very High");
/// assert_eq!(relevance_category(65.0), "High");
/// assert_eq!(relevance_category(45.0), "Medium");
/// assert_eq!(relevance_category(25.0), "Low");
/// ```
pub fn relevance_category(score: f64) -> &'static str {
    if score >= 80.0 {
        "Very High"
    } else if score >= 60.0 {
        "High"
    } else if score >= 40.0 {
        "Medium"
    } else {
        "Low"
    }
}

/// Extract matches from a line using regex pattern (pure function)
///
/// # Arguments
///
/// * `line` - The line content
/// * `pattern` - The regex pattern to match
///
/// # Returns
///
/// Vector of Match objects
pub fn extract_matches_pure(_line: &str, start: usize, end: usize, text: &str) -> Match {
    Match {
        start,
        end,
        text: text.to_string(),
    }
}

/// Calculate fuzzy match quality (pure function)
///
/// Returns a normalized score between 0.0 and 1.0
pub fn fuzzy_match_quality(score: i64, query_length: usize, line_length: usize) -> f64 {
    let normalized = score as f64 / (query_length as f64 * 10.0);
    let length_penalty = 1.0 - (line_length as f64 / 1000.0).min(0.5);
    (normalized * length_penalty).clamp(0.0, 1.0)
}

/// Check if a line should be included based on filters (pure function)
pub fn should_include_line(
    line: &str,
    min_length: usize,
    max_length: usize,
    exclude_patterns: &[&str],
) -> bool {
    let len = line.len();
    if len < min_length || len > max_length {
        return false;
    }

    for pattern in exclude_patterns {
        if line.contains(pattern) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_relevance_score_pure() {
        let score = calculate_relevance_score_pure("fn test() {", "test", 10, Some("rs"), false, None);
        assert!(score > 50.0);
        assert!(score <= 100.0);
    }

    #[test]
    fn test_exact_match_boost() {
        let score = calculate_relevance_score_pure("test", "test", 10, None, false, None);
        assert!(score > 80.0); // Should have exact match boost
    }

    #[test]
    fn test_relevance_category() {
        assert_eq!(relevance_category(90.0), "Very High");
        assert_eq!(relevance_category(70.0), "High");
        assert_eq!(relevance_category(50.0), "Medium");
        assert_eq!(relevance_category(30.0), "Low");
    }

    #[test]
    fn test_fuzzy_match_quality() {
        let quality = fuzzy_match_quality(100, 4, 50);
        assert!(quality > 0.0 && quality <= 1.0);
    }

    #[test]
    fn test_should_include_line() {
        assert!(should_include_line("test line", 5, 100, &[]));
        assert!(!should_include_line("test", 5, 100, &[])); // Too short
        assert!(should_include_line("test line", 5, 100, &["exclude"])); // Doesn't contain "exclude"
        assert!(!should_include_line("test exclude line", 5, 100, &["exclude"])); // Contains "exclude"
    }

    #[test]
    fn test_file_extension_boost() {
        let score_rs = calculate_relevance_score_pure("test", "test", 10, Some("rs"), false, None);
        let score_txt = calculate_relevance_score_pure("test", "test", 10, Some("txt"), false, None);
        // Both should be high scores due to exact match, but rs should have slight boost
        assert!(score_rs >= score_txt);
        assert!(score_rs >= 95.0); // Should be near max due to exact match
    }

    #[test]
    fn test_early_line_boost() {
        let score_early = calculate_relevance_score_pure("test", "test", 10, None, false, None);
        let score_late = calculate_relevance_score_pure("test", "test", 100, None, false, None);
        // Both should be high due to exact match, early line gets small boost
        assert!(score_early >= score_late);
        assert!(score_early >= 95.0); // Should be near max
    }
}
