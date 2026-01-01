//! Maintainability Metrics
//!
//! Implements maintainability-related metrics including MI, DIT, CBO, and LCOM.

use serde::{Deserialize, Serialize};
use super::complexity::ComplexityMetrics;
use super::size::SizeMetrics;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintainabilityMetrics {
    pub maintainability_index: f64,
    pub code_churn: CodeChurn,
    pub depth_of_inheritance: usize,
    pub coupling_between_objects: usize,
    pub lack_of_cohesion: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeChurn {
    pub lines_added: usize,
    pub lines_deleted: usize,
    pub lines_modified: usize,
    pub churn_rate: f64,
}

impl MaintainabilityMetrics {
    pub fn calculate_maintainability_index(
        halstead_volume: f64,
        cyclomatic_complexity: usize,
        lines_of_code: usize,
    ) -> f64 {
        let v = halstead_volume.max(1.0);
        let g = cyclomatic_complexity as f64;
        let loc = lines_of_code.max(1) as f64;
        
        let mi = 171.0 - 5.2 * v.ln() - 0.23 * g - 16.2 * loc.ln();
        let normalized = ((mi / 171.0) * 100.0).max(0.0).min(100.0);
        normalized
    }
}

pub fn calculate_maintainability_metrics(
    complexity: &ComplexityMetrics,
    size: &SizeMetrics,
    content: &str,
    ext: &str,
) -> MaintainabilityMetrics {
    let mi = MaintainabilityMetrics::calculate_maintainability_index(
        complexity.halstead.volume,
        complexity.cyclomatic_complexity,
        size.source_lines,
    );
    
    let churn = CodeChurn {
        lines_added: 0,
        lines_deleted: 0,
        lines_modified: 0,
        churn_rate: 0.0,
    };
    
    let dit = calculate_depth_of_inheritance(content, ext);
    let cbo = calculate_coupling_between_objects(content, ext);
    let lcom = calculate_lack_of_cohesion(content, ext);
    
    MaintainabilityMetrics {
        maintainability_index: mi,
        code_churn: churn,
        depth_of_inheritance: dit,
        coupling_between_objects: cbo,
        lack_of_cohesion: lcom,
    }
}

fn calculate_depth_of_inheritance(content: &str, ext: &str) -> usize {
    let inheritance_patterns = match ext {
        "rs" => vec![r"impl\s+\w+\s+for\s+\w+"],
        "py" => vec![r"class\s+\w+\([^)]+\)"],
        "java" | "kt" => vec![r"extends\s+\w+"],
        "js" | "ts" => vec![r"extends\s+\w+"],
        _ => vec![],
    };
    
    let mut max_depth = 0;
    for pattern in inheritance_patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            let count = re.find_iter(content).count();
            max_depth = max_depth.max(count);
        }
    }
    
    max_depth
}

fn calculate_coupling_between_objects(content: &str, ext: &str) -> usize {
    let import_patterns = match ext {
        "rs" => vec![r"use\s+[\w:]+"],
        "py" => vec![r"import\s+\w+", r"from\s+\w+\s+import"],
        "java" | "kt" => vec![r"import\s+[\w.]+"],
        "js" | "ts" => vec![r"import\s+.*from", r"require\("],
        "go" => vec![r#"import\s+"[^"]+""#],
        _ => vec![],
    };
    
    let mut coupling = 0;
    for pattern in import_patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            coupling += re.find_iter(content).count();
        }
    }
    
    coupling
}

fn calculate_lack_of_cohesion(content: &str, ext: &str) -> f64 {
    let method_count = crate::codemetrics::helpers::count_functions(content, ext);
    let field_count = crate::codemetrics::helpers::count_fields(content, ext);
    
    if method_count == 0 || field_count == 0 {
        return 0.0;
    }
    
    let ratio = method_count as f64 / field_count as f64;
    (ratio - 1.0).max(0.0).min(1.0)
}
