//! Helper functions for dead code detection

/// Check if a function name is special and should be excluded from detection
pub fn is_special_function(name: &str) -> bool {
    matches!(
        name,
        "main" | "new" | "default" | "init" | "__init__" | "setup" | "teardown"
            | "test" | "run" | "start" | "stop" | "get" | "set" | "from" | "into"
            | "drop" | "clone" | "fmt" | "eq" | "hash" | "cmp" | "partial_cmp"
            | "serialize" | "deserialize" | "encode" | "decode"
            | "constructor" | "destructor" | "finalize"
    ) || name.starts_with("test_")
      || name.starts_with("Test")
      || name.starts_with("_")
      || name.starts_with("on")
      || name.starts_with("handle")
}

/// Truncate a string to a maximum length
pub fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}

/// Check if a line contains commented-out code
pub fn is_commented_out_code(line: &str) -> bool {
    let (_prefix, rest) = if line.starts_with("// ") {
        ("// ", line.trim_start_matches("// "))
    } else if line.starts_with("//") && !line.starts_with("///") && !line.starts_with("//!") {
        ("//", line.trim_start_matches("//").trim())
    } else if line.starts_with("# ") && !line.starts_with("#!") && !line.starts_with("#[") {
        ("# ", line.trim_start_matches("# "))
    } else {
        return false;
    };
    
    if rest.len() < 5 {
        return false;
    }
    
    if rest.starts_with("TODO") || rest.starts_with("FIXME") || rest.starts_with("NOTE")
        || rest.starts_with("HACK") || rest.starts_with("XXX") || rest.starts_with("WARN")
        || rest.starts_with("@") || rest.starts_with("*") || rest.starts_with("-")
        || rest.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) && !rest.contains('(')
    {
        return false;
    }
    
    let code_indicators = [
        (";", true),
        ("fn ", false), ("def ", false), ("function ", false),
        ("class ", false), ("struct ", false),
        ("let ", true), ("const ", true), ("var ", true),
        ("if ", false), ("for ", false), ("while ", false),
        ("return ", false),
    ];
    
    for (indicator, requires_semicolon) in &code_indicators {
        if rest.contains(indicator) {
            if *requires_semicolon && rest.ends_with(';') {
                return true;
            } else if !requires_semicolon {
                if rest.contains('{') || rest.contains('}') || rest.ends_with(';') || rest.ends_with(':') || rest.contains('(') {
                    return true;
                }
            }
        }
    }
    
    if rest.contains(" = ") && rest.ends_with(';') && !rest.contains("//") {
        return true;
    }
    
    false
}

/// Extract import name from an import statement
pub fn extract_import_name(line: &str) -> Option<String> {
    let line = line.trim_end_matches(';').trim();
    
    if line.starts_with("use ") {
        let path = line.trim_start_matches("use ").trim();
        if let Some(last) = path.split("::").last() {
            let name = last.trim_matches(|c| c == '{' || c == '}');
            if !name.is_empty() && name != "*" && !name.contains(',') {
                return Some(name.to_string());
            }
        }
    } else if line.starts_with("import ") {
        let name = line.trim_start_matches("import ").trim();
        if let Some(first) = name.split_whitespace().next() {
            return Some(first.to_string());
        }
    } else if line.starts_with("from ") && line.contains(" import ") {
        if let Some(after_import) = line.split(" import ").nth(1) {
            let name = after_import.split(',').next().unwrap_or("").trim();
            if !name.is_empty() && name != "*" {
                return Some(name.to_string());
            }
        }
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_special_function() {
        assert!(is_special_function("main"));
        assert!(is_special_function("test_something"));
        assert!(is_special_function("_private"));
        assert!(is_special_function("handle_event"));
        assert!(is_special_function("on_click"));
        assert!(!is_special_function("helper"));
        assert!(!is_special_function("calculate"));
    }

    #[test]
    fn test_truncate_string() {
        assert_eq!(truncate_string("short", 10), "short");
        assert_eq!(truncate_string("this is a long string", 10), "this is...");
    }

    #[test]
    fn test_is_commented_out_code() {
        assert!(is_commented_out_code("// let x = 10;"));
        assert!(is_commented_out_code("// function test() {"));
        assert!(!is_commented_out_code("// This is a comment"));
        assert!(!is_commented_out_code("// TODO: fix this"));
    }

    #[test]
    fn test_extract_import_name() {
        assert_eq!(extract_import_name("use std::io::Write;"), Some("Write".to_string()));
        assert_eq!(extract_import_name("import os"), Some("os".to_string()));
        assert_eq!(extract_import_name("from os import path"), Some("path".to_string()));
    }
}
