//! Code normalization for better duplicate detection

use regex::Regex;
use std::collections::HashMap;

/// Normalize code for comparison
pub fn normalize_code(code: &str) -> String {
    let mut normalized = code.to_string();
    
    // 1. Remove single-line comments
    normalized = remove_single_line_comments(&normalized);
    
    // 2. Remove multi-line comments
    normalized = remove_multi_line_comments(&normalized);
    
    // 3. Normalize whitespace
    normalized = normalize_whitespace(&normalized);
    
    // 4. Remove empty lines
    normalized = normalized
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    
    normalized
}

/// Advanced normalization with variable renaming
pub fn normalize_with_variables(code: &str) -> String {
    let mut normalized = normalize_code(code);
    
    // Replace variable names with placeholders
    normalized = rename_variables(&normalized);
    
    // Normalize string literals
    normalized = normalize_strings(&normalized);
    
    // Normalize numeric literals
    normalized = normalize_numbers(&normalized);
    
    normalized
}

fn remove_single_line_comments(code: &str) -> String {
    let patterns = [
        r"//.*$",           // C-style
        r"#(?![!\[])[^\n]*$", // Python/Shell (but not #! or #[)
    ];
    
    let mut result = code.to_string();
    for pattern in &patterns {
        if let Ok(re) = Regex::new(&format!("(?m){}", pattern)) {
            result = re.replace_all(&result, "").to_string();
        }
    }
    result
}

fn remove_multi_line_comments(code: &str) -> String {
    let patterns = [
        r"/\*[\s\S]*?\*/",  // C-style
        r#"'''[\s\S]*?'''"#, // Python
        r#""""[\s\S]*?""""#, // Python
    ];
    
    let mut result = code.to_string();
    for pattern in &patterns {
        if let Ok(re) = Regex::new(pattern) {
            result = re.replace_all(&result, "").to_string();
        }
    }
    result
}

fn normalize_whitespace(code: &str) -> String {
    // Replace multiple spaces with single space
    let re = Regex::new(r"[ \t]+").unwrap();
    let result = re.replace_all(code, " ");
    
    // Trim each line
    result
        .lines()
        .map(|line| line.trim())
        .collect::<Vec<_>>()
        .join("\n")
}

fn rename_variables(code: &str) -> String {
    // Simple variable renaming - replace identifiers with v1, v2, v3...
    let identifier_re = Regex::new(r"\b([a-zA-Z_][a-zA-Z0-9_]*)\b").unwrap();
    let mut var_map: HashMap<String, String> = HashMap::new();
    let mut counter = 1;
    
    // Reserved keywords that shouldn't be renamed
    let keywords = [
        "fn", "function", "def", "class", "struct", "enum", "trait",
        "if", "else", "for", "while", "loop", "match", "switch",
        "return", "break", "continue", "let", "const", "var",
        "pub", "private", "public", "protected", "static",
        "async", "await", "yield", "import", "from", "use",
        "true", "false", "null", "nil", "None", "Some",
    ];
    
    identifier_re.replace_all(code, |caps: &regex::Captures| {
        let ident = caps.get(1).unwrap().as_str();
        
        // Don't rename keywords
        if keywords.contains(&ident) {
            return ident.to_string();
        }
        
        // Get or create placeholder
        var_map.entry(ident.to_string())
            .or_insert_with(|| {
                let placeholder = format!("v{}", counter);
                counter += 1;
                placeholder
            })
            .clone()
    }).to_string()
}

fn normalize_strings(code: &str) -> String {
    // Replace string literals with placeholder
    let string_re = Regex::new(r#""[^"]*"|'[^']*'"#).unwrap();
    string_re.replace_all(code, "\"STR\"").to_string()
}

fn normalize_numbers(code: &str) -> String {
    // Replace numeric literals with placeholder
    let number_re = Regex::new(r"\b\d+\.?\d*\b").unwrap();
    number_re.replace_all(code, "NUM").to_string()
}

/// Calculate hash for code block
pub fn calculate_hash(code: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    code.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_code() {
        let code = r#"
// This is a comment
fn test() {
    let x = 5;  // inline comment
    /* multi
       line */
    let y = 10;
}
"#;
        let normalized = normalize_code(code);
        assert!(!normalized.contains("//"));
        assert!(!normalized.contains("/*"));
        assert!(!normalized.contains("*/"));
    }

    #[test]
    fn test_normalize_whitespace() {
        let code = "fn   test()   {  }";
        let normalized = normalize_whitespace(code);
        assert_eq!(normalized, "fn test() { }");
    }

    #[test]
    fn test_rename_variables() {
        let code = "let user = 5; let data = user + 10;";
        let normalized = rename_variables(code);
        assert!(normalized.contains("v1"));
        assert!(normalized.contains("v2"));
        assert!(!normalized.contains("user"));
        assert!(!normalized.contains("data"));
    }

    #[test]
    fn test_normalize_strings() {
        let code = r#"let msg = "hello world"; let x = 'test';"#;
        let normalized = normalize_strings(code);
        assert!(normalized.contains("\"STR\""));
        assert!(!normalized.contains("hello"));
    }

    #[test]
    fn test_normalize_numbers() {
        let code = "let x = 42; let y = 3.14;";
        let normalized = normalize_numbers(code);
        assert!(normalized.contains("NUM"));
        assert!(!normalized.contains("42"));
        assert!(!normalized.contains("3.14"));
    }
}
