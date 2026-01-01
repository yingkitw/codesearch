//! Code Size Metrics
//!
//! Implements size-related metrics including LOC, SLOC, LLOC, and code density.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SizeMetrics {
    pub total_lines: usize,
    pub source_lines: usize,
    pub logical_lines: usize,
    pub comment_lines: usize,
    pub blank_lines: usize,
    pub num_classes: usize,
    pub num_methods: usize,
    pub num_functions: usize,
    pub code_density: f64,
    pub comment_ratio: f64,
}

impl SizeMetrics {
    pub fn new() -> Self {
        Self {
            total_lines: 0,
            source_lines: 0,
            logical_lines: 0,
            comment_lines: 0,
            blank_lines: 0,
            num_classes: 0,
            num_methods: 0,
            num_functions: 0,
            code_density: 0.0,
            comment_ratio: 0.0,
        }
    }

    pub fn calculate_ratios(&mut self) {
        if self.total_lines > 0 {
            self.code_density = self.source_lines as f64 / self.total_lines as f64;
        }
        if self.source_lines > 0 {
            self.comment_ratio = self.comment_lines as f64 / self.source_lines as f64;
        }
    }
}

pub fn calculate_size_metrics(content: &str, ext: &str) -> SizeMetrics {
    let mut metrics = SizeMetrics::new();
    
    let lines: Vec<&str> = content.lines().collect();
    metrics.total_lines = lines.len();
    
    let is_comment_line = |line: &str, ext: &str| -> bool {
        let trimmed = line.trim();
        match ext {
            "rs" | "c" | "cpp" | "java" | "js" | "ts" | "go" | "kt" => {
                trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with("*")
            }
            "py" | "rb" | "sh" => trimmed.starts_with("#"),
            _ => false,
        }
    };
    
    let is_logical_line = |line: &str, ext: &str| -> bool {
        let trimmed = line.trim();
        if trimmed.is_empty() || is_comment_line(line, ext) {
            return false;
        }
        
        match ext {
            "rs" | "c" | "cpp" | "java" | "js" | "ts" | "go" | "kt" => {
                trimmed.ends_with(';') || 
                trimmed.contains("if ") || 
                trimmed.contains("for ") ||
                trimmed.contains("while ") ||
                trimmed.contains("return")
            }
            "py" => {
                !trimmed.ends_with(':') && 
                (trimmed.contains('=') || 
                 trimmed.starts_with("return") ||
                 trimmed.starts_with("if ") ||
                 trimmed.starts_with("for "))
            }
            _ => !trimmed.is_empty(),
        }
    };
    
    for line in &lines {
        let trimmed = line.trim();
        
        if trimmed.is_empty() {
            metrics.blank_lines += 1;
        } else if is_comment_line(line, ext) {
            metrics.comment_lines += 1;
        } else {
            metrics.source_lines += 1;
            
            if is_logical_line(line, ext) {
                metrics.logical_lines += 1;
            }
        }
    }
    
    metrics.num_classes = crate::codemetrics::helpers::count_classes(content, ext);
    metrics.num_functions = crate::codemetrics::helpers::count_functions(content, ext);
    metrics.num_methods = metrics.num_functions;
    
    metrics.calculate_ratios();
    metrics
}
