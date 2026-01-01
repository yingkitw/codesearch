//! Semantic Search Enhancement
//!
//! Enhances search queries with semantic patterns for better code understanding.

/// Enhance a query with semantic patterns
pub fn enhance_query_semantically(query: &str) -> String {
    let semantic_patterns = [
        ("function", r"(function|def|fn|func|method|procedure)"),
        ("class", r"(class|struct|interface|trait|type)"),
        ("variable", r"(let|var|const|val|mut)"),
        ("loop", r"(for|while|do|foreach|map|filter)"),
        ("condition", r"(if|else|switch|case|when|match)"),
        ("error", r"(error|exception|panic|fail|throw)"),
        ("test", r"(test|spec|it|describe|assert)"),
        ("import", r"(import|use|require|include|from)"),
        ("return", r"(return|yield|emit)"),
        ("async", r"(async|await|promise|future)"),
        ("comment", r"(//|#|/\*)"),
        ("string", r#"(".*"|'.*'|`.*`)"#),
        ("number", r"\d+"),
        ("boolean", r"(true|false|True|False|TRUE|FALSE)"),
        ("null", r"(null|nil|None|undefined)"),
    ];

    let query_lower = query.to_lowercase();
    for (keyword, pattern) in &semantic_patterns {
        if query_lower.contains(keyword) {
            return pattern.to_string();
        }
    }

    query.to_string()
}
