//! Code Complexity Metrics
//!
//! Implements various complexity metrics including Cyclomatic, Halstead, Essential, and NPath.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityMetrics {
    pub cyclomatic_complexity: usize,
    pub halstead: HalsteadMetrics,
    pub essential_complexity: usize,
    pub npath_complexity: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HalsteadMetrics {
    pub n1: usize,
    pub n2: usize,
    pub N1: usize,
    pub N2: usize,
    pub vocabulary: usize,
    pub length: usize,
    pub volume: f64,
    pub difficulty: f64,
    pub effort: f64,
    pub time: f64,
    pub bugs: f64,
}

impl HalsteadMetrics {
    pub fn calculate(operators: &HashMap<String, usize>, operands: &HashMap<String, usize>) -> Self {
        let n1 = operators.len();
        let n2 = operands.len();
        let N1: usize = operators.values().sum();
        let N2: usize = operands.values().sum();
        
        let vocabulary = n1 + n2;
        let length = N1 + N2;
        
        let volume = if vocabulary > 0 {
            length as f64 * (vocabulary as f64).log2()
        } else {
            0.0
        };
        
        let difficulty = if n2 > 0 {
            (n1 as f64 / 2.0) * (N2 as f64 / n2 as f64)
        } else {
            0.0
        };
        
        let effort = difficulty * volume;
        let time = effort / 18.0;
        let bugs = volume / 3000.0;
        
        Self {
            n1, n2, N1, N2,
            vocabulary,
            length,
            volume,
            difficulty,
            effort,
            time,
            bugs,
        }
    }
}

pub fn calculate_complexity_metrics(content: &str, ext: &str) -> ComplexityMetrics {
    let cyclomatic = calculate_cyclomatic_complexity(content, ext);
    let halstead = calculate_halstead_metrics(content, ext);
    let essential = calculate_essential_complexity(content, ext);
    let npath = calculate_npath_complexity(content, ext);
    
    ComplexityMetrics {
        cyclomatic_complexity: cyclomatic,
        halstead,
        essential_complexity: essential,
        npath_complexity: npath,
    }
}

pub fn calculate_cyclomatic_complexity(content: &str, ext: &str) -> usize {
    let decision_points = match ext {
        "rs" | "c" | "cpp" | "java" | "js" | "ts" | "go" | "kt" => {
            vec![r"\bif\b", r"\bfor\b", r"\bwhile\b", r"\bcase\b", r"\bmatch\b", r"\bcatch\b", r"\b\?\b", r"\b&&\b", r"\b\|\|\b"]
        }
        "py" => {
            vec![r"\bif\b", r"\bfor\b", r"\bwhile\b", r"\belif\b", r"\bexcept\b", r"\band\b", r"\bor\b"]
        }
        _ => vec![r"\bif\b", r"\bfor\b", r"\bwhile\b"],
    };
    
    let mut complexity = 1;
    
    for pattern in decision_points {
        if let Ok(re) = regex::Regex::new(pattern) {
            complexity += re.find_iter(content).count();
        }
    }
    
    complexity
}

fn calculate_halstead_metrics(content: &str, ext: &str) -> HalsteadMetrics {
    let mut operators: HashMap<String, usize> = HashMap::new();
    let mut operands: HashMap<String, usize> = HashMap::new();
    
    let operator_patterns = match ext {
        "rs" | "c" | "cpp" | "java" | "js" | "ts" | "go" | "kt" => {
            vec![
                r"\+", r"-", r"\*", r"/", r"%", r"=", r"==", r"!=", r"<", r">", 
                r"<=", r">=", r"&&", r"\|\|", r"!", r"&", r"\|", r"\^", r"<<", r">>",
                r"\+=", r"-=", r"\*=", r"/=", r"\?", r":", r"\.", r"->", r"::",
            ]
        }
        "py" => {
            vec![
                r"\+", r"-", r"\*", r"/", r"%", r"=", r"==", r"!=", r"<", r">",
                r"<=", r">=", r"\band\b", r"\bor\b", r"\bnot\b", r"\bin\b", r"\bis\b",
            ]
        }
        _ => vec![r"\+", r"-", r"\*", r"/", r"="],
    };
    
    for pattern in operator_patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            let count = re.find_iter(content).count();
            if count > 0 {
                *operators.entry(pattern.to_string()).or_insert(0) += count;
            }
        }
    }
    
    if let Ok(identifier_re) = regex::Regex::new(r"\b[a-zA-Z_][a-zA-Z0-9_]*\b") {
        for cap in identifier_re.find_iter(content) {
            let operand = cap.as_str().to_string();
            if !is_keyword(&operand, ext) {
                *operands.entry(operand).or_insert(0) += 1;
            }
        }
    }
    
    if let Ok(number_re) = regex::Regex::new(r"\b\d+\.?\d*\b") {
        for cap in number_re.find_iter(content) {
            *operands.entry(cap.as_str().to_string()).or_insert(0) += 1;
        }
    }
    
    HalsteadMetrics::calculate(&operators, &operands)
}

fn calculate_essential_complexity(content: &str, _ext: &str) -> usize {
    let mut essential = 1;
    
    if content.contains("goto") {
        essential += content.matches("goto").count();
    }
    
    let max_nesting = crate::codemetrics::helpers::calculate_max_nesting(content);
    if max_nesting > 3 {
        essential += max_nesting - 3;
    }
    
    essential
}

fn calculate_npath_complexity(content: &str, ext: &str) -> u64 {
    let decision_patterns = match ext {
        "rs" | "c" | "cpp" | "java" | "js" | "ts" | "go" | "kt" => {
            vec![r"\bif\b", r"\bfor\b", r"\bwhile\b", r"\bcase\b", r"\bmatch\b"]
        }
        "py" => {
            vec![r"\bif\b", r"\bfor\b", r"\bwhile\b", r"\belif\b"]
        }
        _ => vec![r"\bif\b", r"\bfor\b", r"\bwhile\b"],
    };
    
    let mut decision_count = 0;
    for pattern in decision_patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            decision_count += re.find_iter(content).count();
        }
    }
    
    2u64.saturating_pow(decision_count.min(30) as u32)
}

fn is_keyword(word: &str, ext: &str) -> bool {
    let keywords = match ext {
        "rs" => vec!["fn", "let", "mut", "if", "else", "for", "while", "loop", "match", "return", "struct", "enum", "impl", "trait", "pub", "use", "mod"],
        "py" => vec!["def", "class", "if", "else", "elif", "for", "while", "return", "import", "from", "as", "with", "try", "except"],
        "java" | "kt" => vec!["class", "interface", "public", "private", "protected", "static", "void", "if", "else", "for", "while", "return", "import"],
        "js" | "ts" => vec!["function", "const", "let", "var", "if", "else", "for", "while", "return", "class", "import", "export"],
        _ => vec![],
    };
    
    keywords.contains(&word)
}
