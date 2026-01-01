//! Design Metrics Extractors
//!
//! Functions to extract dependencies, classes, and metrics from source code.

use super::types::ClassMetrics;
use std::collections::{HashMap, HashSet};
use std::path::Path;

pub fn extract_dependencies(content: &str, path: &Path) -> Vec<String> {
    let mut dependencies = Vec::new();
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");

    let patterns = match ext {
        "rs" => vec![r"use\s+([\w:]+)"],
        "py" => vec![r"import\s+([\w.]+)", r"from\s+([\w.]+)\s+import"],
        "js" | "ts" => vec![r#"import\s+.*\s+from\s+['\"]([^'\"]+)['\"]"#, r#"require\(['\"]([^'\"]+)['\"]\)"#],
        "go" => vec![r#"import\s+"([^"]+)""#],
        "java" | "kt" => vec![r"import\s+([\w.]+)"],
        _ => vec![],
    };

    for pattern in patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            for line in content.lines() {
                if let Some(caps) = re.captures(line) {
                    if let Some(dep) = caps.get(1) {
                        let dep_str = dep.as_str();
                        let module = dep_str.split("::").next()
                            .or_else(|| dep_str.split(".").next())
                            .or_else(|| dep_str.split("/").last())
                            .unwrap_or(dep_str);
                        dependencies.push(module.to_string());
                    }
                }
            }
        }
    }

    dependencies.sort();
    dependencies.dedup();
    dependencies
}

pub fn extract_classes_with_metrics(content: &str, path: &Path) -> Vec<ClassMetrics> {
    let mut classes = Vec::new();
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");

    let class_pattern = match ext {
        "rs" => r"(?:struct|enum|trait)\s+(\w+)",
        "py" => r"class\s+(\w+)",
        "js" | "ts" => r"class\s+(\w+)",
        "go" => r"type\s+(\w+)\s+struct",
        "java" | "kt" => r"(?:class|interface)\s+(\w+)",
        _ => return classes,
    };

    if let Ok(class_re) = regex::Regex::new(class_pattern) {
        for cap in class_re.captures_iter(content) {
            if let Some(class_name) = cap.get(1) {
                let mut class_metrics = ClassMetrics::new(class_name.as_str().to_string());

                let (methods, fields) = extract_class_members(content, class_name.as_str(), ext);
                class_metrics.methods = methods;
                class_metrics.fields = fields;

                class_metrics.method_field_usage = analyze_method_field_usage(content, &class_metrics.methods, &class_metrics.fields);
                
                class_metrics.calculate_lcom();
                classes.push(class_metrics);
            }
        }
    }

    classes
}

pub fn extract_class_members(content: &str, _class_name: &str, ext: &str) -> (Vec<String>, Vec<String>) {
    let mut methods = Vec::new();
    let mut fields = Vec::new();

    let method_pattern = match ext {
        "rs" => r"fn\s+(\w+)",
        "py" => r"def\s+(\w+)",
        "js" | "ts" => r"(?:async\s+)?(\w+)\s*\([^)]*\)\s*\{",
        "java" | "kt" => r"(?:public|private|protected)?\s*(?:static)?\s*\w+\s+(\w+)\s*\(",
        _ => return (methods, fields),
    };

    let field_pattern = match ext {
        "rs" => r"(\w+):\s*\w+",
        "py" => r"self\.(\w+)\s*=",
        "js" | "ts" => r"this\.(\w+)\s*=",
        "java" | "kt" => r"(?:private|public|protected)?\s*\w+\s+(\w+);",
        _ => return (methods, fields),
    };

    if let Ok(method_re) = regex::Regex::new(method_pattern) {
        for cap in method_re.captures_iter(content) {
            if let Some(method_name) = cap.get(1) {
                methods.push(method_name.as_str().to_string());
            }
        }
    }

    if let Ok(field_re) = regex::Regex::new(field_pattern) {
        for cap in field_re.captures_iter(content) {
            if let Some(field_name) = cap.get(1) {
                fields.push(field_name.as_str().to_string());
            }
        }
    }

    (methods, fields)
}

pub fn analyze_method_field_usage(content: &str, methods: &[String], fields: &[String]) -> HashMap<String, HashSet<String>> {
    let mut usage = HashMap::new();

    for method in methods {
        let mut used_fields = HashSet::new();
        
        for field in fields {
            if content.contains(&format!("{}", field)) {
                used_fields.insert(field.clone());
            }
        }
        
        usage.insert(method.clone(), used_fields);
    }

    usage
}

pub fn count_abstract_elements(content: &str, path: &Path) -> usize {
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");

    let patterns = match ext {
        "rs" => vec![r"trait\s+\w+", r"pub\s+trait\s+\w+"],
        "py" => vec![r"class\s+\w+\(ABC\)", r"@abstractmethod"],
        "java" | "kt" => vec![r"abstract\s+class", r"interface\s+\w+"],
        "ts" => vec![r"interface\s+\w+", r"abstract\s+class"],
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
