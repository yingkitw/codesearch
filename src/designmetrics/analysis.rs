//! Design Metrics Analysis
//!
//! Main analysis functions for calculating design quality metrics.

use super::types::{DesignMetrics, ModuleMetrics};
use super::extractors::{extract_dependencies, extract_classes_with_metrics, count_abstract_elements};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn analyze_design_metrics(
    path: &Path,
    extensions: Option<&[String]>,
    exclude: Option<&[String]>,
) -> Result<DesignMetrics, Box<dyn std::error::Error>> {
    let mut metrics = DesignMetrics::new();
    let mut module_dependencies: HashMap<String, HashSet<String>> = HashMap::new();

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

    let files: Vec<PathBuf> = walker
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
        .map(|e| e.path().to_path_buf())
        .collect();

    for file in &files {
        let module_name = file.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        let content = std::fs::read_to_string(file)?;
        let mut module_metrics = ModuleMetrics::new(module_name.clone(), file.to_string_lossy().to_string());

        let dependencies = extract_dependencies(&content, file);
        module_metrics.efferent_coupling = dependencies.len();
        module_metrics.dependencies = dependencies.clone();
        module_dependencies.insert(module_name.clone(), dependencies.into_iter().collect());

        let classes = extract_classes_with_metrics(&content, file);
        module_metrics.classes = classes;
        module_metrics.calculate_cohesion();

        let abstract_count = count_abstract_elements(&content, file);
        let total_classes = module_metrics.classes.len().max(1);
        module_metrics.abstractness = abstract_count as f64 / total_classes as f64;

        metrics.add_module(module_metrics);
    }

    for (module_name, module_metrics) in metrics.modules.iter_mut() {
        let mut dependents = Vec::new();
        
        for (other_module, other_deps) in &module_dependencies {
            if other_module != module_name && other_deps.contains(module_name) {
                dependents.push(other_module.clone());
            }
        }

        module_metrics.afferent_coupling = dependents.len();
        module_metrics.dependents = dependents;
        module_metrics.calculate_instability();
        module_metrics.calculate_distance_from_main();
    }

    metrics.calculate_overall_stats();

    Ok(metrics)
}
