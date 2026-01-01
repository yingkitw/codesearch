//! Helper Functions for Code Metrics
//!
//! Provides utility functions for counting code elements and analyzing structure.

pub fn count_classes(content: &str, ext: &str) -> usize {
    let patterns = match ext {
        "rs" => vec![r"struct\s+\w+", r"enum\s+\w+", r"trait\s+\w+"],
        "py" => vec![r"class\s+\w+"],
        "java" | "kt" => vec![r"class\s+\w+", r"interface\s+\w+"],
        "js" | "ts" => vec![r"class\s+\w+"],
        "go" => vec![r"type\s+\w+\s+struct"],
        _ => vec![],
    };
    
    let mut count = 0;
    for pattern in patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            count += re.find_iter(content).count();
        }
    }
    count
}

pub fn count_functions(content: &str, ext: &str) -> usize {
    let patterns = match ext {
        "rs" => vec![r"fn\s+\w+"],
        "py" => vec![r"def\s+\w+"],
        "java" | "kt" => vec![r"(?:public|private|protected)?\s*\w+\s+\w+\s*\("],
        "js" | "ts" => vec![r"function\s+\w+", r"\w+\s*:\s*\([^)]*\)\s*=>"],
        "go" => vec![r"func\s+\w+"],
        _ => vec![],
    };
    
    let mut count = 0;
    for pattern in patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            count += re.find_iter(content).count();
        }
    }
    count
}

pub fn count_fields(content: &str, ext: &str) -> usize {
    let patterns = match ext {
        "rs" => vec![r"\w+:\s*\w+,"],
        "py" => vec![r"self\.\w+\s*="],
        "java" | "kt" => vec![r"(?:private|public|protected)\s+\w+\s+\w+;"],
        "js" | "ts" => vec![r"this\.\w+\s*="],
        _ => vec![],
    };
    
    let mut count = 0;
    for pattern in patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            count += re.find_iter(content).count();
        }
    }
    count.max(1)
}

pub fn calculate_max_nesting(content: &str) -> usize {
    let mut max_depth = 0;
    let mut current_depth = 0;
    
    for ch in content.chars() {
        match ch {
            '{' | '(' | '[' => {
                current_depth += 1;
                max_depth = max_depth.max(current_depth);
            }
            '}' | ')' | ']' => {
                if current_depth > 0 {
                    current_depth -= 1;
                }
            }
            _ => {}
        }
    }
    
    max_depth
}
