//! Comprehensive Code Metrics Module
//!
//! Implements complexity, size, and maintainability metrics for code quality analysis.

pub mod complexity;
pub mod size;
pub mod maintainability;
pub mod helpers;

use serde::{Deserialize, Serialize};
use std::path::Path;

pub use complexity::{ComplexityMetrics, HalsteadMetrics};
pub use size::SizeMetrics;
pub use maintainability::{MaintainabilityMetrics, CodeChurn};

// ============================================================================
// UNIFIED FILE METRICS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetrics {
    pub file_path: String,
    pub complexity: ComplexityMetrics,
    pub size: SizeMetrics,
    pub maintainability: MaintainabilityMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetrics {
    pub files: Vec<FileMetrics>,
    pub totals: TotalMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotalMetrics {
    pub total_files: usize,
    pub total_lines: usize,
    pub total_sloc: usize,
    pub total_functions: usize,
    pub total_classes: usize,
    pub avg_cyclomatic: f64,
    pub avg_maintainability: f64,
    pub avg_halstead_volume: f64,
    pub total_estimated_bugs: f64,
}

// ============================================================================
// ANALYSIS FUNCTIONS
// ============================================================================

pub fn analyze_file_metrics(path: &Path) -> Result<FileMetrics, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
    
    let size = size::calculate_size_metrics(&content, ext);
    let complexity = complexity::calculate_complexity_metrics(&content, ext);
    let maintainability = maintainability::calculate_maintainability_metrics(&complexity, &size, &content, ext);
    
    Ok(FileMetrics {
        file_path: path.to_string_lossy().to_string(),
        complexity,
        size,
        maintainability,
    })
}

pub fn analyze_project_metrics(
    path: &Path,
    extensions: Option<&[String]>,
    exclude: Option<&[String]>,
) -> Result<ProjectMetrics, Box<dyn std::error::Error>> {
    use walkdir::WalkDir;
    
    let walker = WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| {
            if let Some(name) = e.file_name().to_str() {
                if let Some(exclude_dirs) = exclude {
                    for exclude_dir in exclude_dirs {
                        if name == exclude_dir {
                            return false;
                        }
                    }
                }
            }
            true
        })
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file());

    let files: Vec<_> = walker
        .filter(|entry| {
            let file_path = entry.path();
            if let Some(exts) = extensions {
                if let Some(ext) = file_path.extension().and_then(|s| s.to_str()) {
                    exts.iter().any(|e| e == ext)
                } else {
                    false
                }
            } else {
                true
            }
        })
        .collect();

    let mut file_metrics = Vec::new();
    
    for entry in files {
        if let Ok(metrics) = analyze_file_metrics(entry.path()) {
            file_metrics.push(metrics);
        }
    }

    let totals = calculate_totals(&file_metrics);

    Ok(ProjectMetrics {
        files: file_metrics,
        totals,
    })
}

fn calculate_totals(files: &[FileMetrics]) -> TotalMetrics {
    let total_files = files.len();
    let total_lines: usize = files.iter().map(|f| f.size.total_lines).sum();
    let total_sloc: usize = files.iter().map(|f| f.size.source_lines).sum();
    let total_functions: usize = files.iter().map(|f| f.size.num_functions).sum();
    let total_classes: usize = files.iter().map(|f| f.size.num_classes).sum();
    
    let avg_cyclomatic = if total_files > 0 {
        files.iter().map(|f| f.complexity.cyclomatic_complexity).sum::<usize>() as f64 / total_files as f64
    } else {
        0.0
    };
    
    let avg_maintainability = if total_files > 0 {
        files.iter().map(|f| f.maintainability.maintainability_index).sum::<f64>() / total_files as f64
    } else {
        0.0
    };
    
    let avg_halstead_volume = if total_files > 0 {
        files.iter().map(|f| f.complexity.halstead.volume).sum::<f64>() / total_files as f64
    } else {
        0.0
    };
    
    let total_estimated_bugs: f64 = files.iter().map(|f| f.complexity.halstead.bugs).sum();
    
    TotalMetrics {
        total_files,
        total_lines,
        total_sloc,
        total_functions,
        total_classes,
        avg_cyclomatic,
        avg_maintainability,
        avg_halstead_volume,
        total_estimated_bugs,
    }
}

pub fn print_metrics_report(metrics: &ProjectMetrics, detailed: bool) {
    use colored::*;
    
    println!("\n{}", "Comprehensive Code Metrics Report".cyan().bold());
    println!("{}", "=".repeat(70).cyan());
    
    println!("\n{}", "Project Summary:".green().bold());
    println!("  Total files: {}", metrics.totals.total_files);
    println!("  Total lines: {}", metrics.totals.total_lines);
    println!("  Source lines (SLOC): {}", metrics.totals.total_sloc);
    println!("  Total functions: {}", metrics.totals.total_functions);
    println!("  Total classes: {}", metrics.totals.total_classes);
    
    println!("\n{}", "Complexity Metrics:".green().bold());
    println!("  Avg Cyclomatic Complexity: {:.2}", metrics.totals.avg_cyclomatic);
    println!("  Avg Halstead Volume: {:.2}", metrics.totals.avg_halstead_volume);
    println!("  Estimated Bugs (Halstead): {:.2}", metrics.totals.total_estimated_bugs);
    
    println!("\n{}", "Maintainability:".green().bold());
    println!("  Avg Maintainability Index: {:.2}/100", metrics.totals.avg_maintainability);
    
    let mi_rating = if metrics.totals.avg_maintainability >= 80.0 {
        "Excellent".green()
    } else if metrics.totals.avg_maintainability >= 60.0 {
        "Good".yellow()
    } else {
        "Needs Improvement".red()
    };
    println!("  Rating: {}", mi_rating);
    
    if detailed {
        println!("\n{}", "Detailed File Metrics:".cyan().bold());
        println!("{}", "=".repeat(70).cyan());
        
        let mut sorted_files = metrics.files.clone();
        sorted_files.sort_by(|a, b| {
            b.complexity.cyclomatic_complexity.cmp(&a.complexity.cyclomatic_complexity)
        });
        
        for file in sorted_files.iter().take(20) {
            println!("\n{}", format!("File: {}", file.file_path).green().bold());
            
            println!("  Size Metrics:");
            println!("    Total lines: {}", file.size.total_lines);
            println!("    SLOC: {}", file.size.source_lines);
            println!("    LLOC: {}", file.size.logical_lines);
            println!("    Comment lines: {}", file.size.comment_lines);
            println!("    Code density: {:.2}%", file.size.code_density * 100.0);
            println!("    Comment ratio: {:.2}%", file.size.comment_ratio * 100.0);
            
            println!("  Complexity Metrics:");
            println!("    Cyclomatic: {}", file.complexity.cyclomatic_complexity);
            println!("    Essential: {}", file.complexity.essential_complexity);
            println!("    NPath: {}", file.complexity.npath_complexity);
            
            println!("  Halstead Metrics:");
            println!("    Volume: {:.2}", file.complexity.halstead.volume);
            println!("    Difficulty: {:.2}", file.complexity.halstead.difficulty);
            println!("    Effort: {:.2}", file.complexity.halstead.effort);
            println!("    Time (seconds): {:.2}", file.complexity.halstead.time);
            println!("    Estimated bugs: {:.4}", file.complexity.halstead.bugs);
            
            println!("  Maintainability:");
            println!("    MI: {:.2}/100", file.maintainability.maintainability_index);
            println!("    DIT: {}", file.maintainability.depth_of_inheritance);
            println!("    CBO: {}", file.maintainability.coupling_between_objects);
            println!("    LCOM: {:.2}", file.maintainability.lack_of_cohesion);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_halstead_calculation() {
        let mut operators = HashMap::new();
        operators.insert("+".to_string(), 2);
        operators.insert("=".to_string(), 1);
        
        let mut operands = HashMap::new();
        operands.insert("x".to_string(), 2);
        operands.insert("5".to_string(), 1);
        
        let halstead = HalsteadMetrics::calculate(&operators, &operands);
        
        assert_eq!(halstead.n1, 2);
        assert_eq!(halstead.n2, 2);
        assert!(halstead.volume > 0.0);
    }

    #[test]
    fn test_size_metrics() {
        let content = "fn main() {\n    let x = 5;\n    // comment\n\n    println!(\"test\");\n}";
        let metrics = size::calculate_size_metrics(content, "rs");
        
        assert!(metrics.total_lines > 0);
        assert!(metrics.source_lines > 0);
        assert!(metrics.comment_lines > 0);
    }

    #[test]
    fn test_cyclomatic_complexity() {
        let content = "fn test() { if x { } if y { } for i in 0..10 { } }";
        let complexity = complexity::calculate_cyclomatic_complexity(content, "rs");
        
        assert!(complexity >= 3);
    }

    #[test]
    fn test_maintainability_index() {
        let mi = MaintainabilityMetrics::calculate_maintainability_index(100.0, 5, 50);
        assert!(mi >= 0.0 && mi <= 100.0);
    }
}
