//! Dependency Graph Analysis Module
//!
//! Provides dependency graph construction and analysis for codebases.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyGraph {
    pub nodes: HashMap<String, DependencyNode>,
    pub edges: Vec<DependencyEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyNode {
    pub path: String,
    pub module_name: String,
    pub imports: Vec<String>,
    pub exports: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyEdge {
    pub from: String,
    pub to: String,
    pub edge_type: EdgeType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EdgeType {
    Import,
    Export,
    Call,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: DependencyNode) {
        self.nodes.insert(node.path.clone(), node);
    }

    pub fn add_edge(&mut self, from: String, to: String, edge_type: EdgeType) {
        self.edges.push(DependencyEdge { from, to, edge_type });
    }

    pub fn get_dependencies(&self, path: &str) -> Vec<String> {
        self.edges
            .iter()
            .filter(|e| e.from == path)
            .map(|e| e.to.clone())
            .collect()
    }

    pub fn get_dependents(&self, path: &str) -> Vec<String> {
        self.edges
            .iter()
            .filter(|e| e.to == path)
            .map(|e| e.from.clone())
            .collect()
    }

    pub fn find_circular_dependencies(&self) -> Vec<Vec<String>> {
        let mut cycles = Vec::new();
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        for node_path in self.nodes.keys() {
            if !visited.contains(node_path) {
                self.detect_cycle(node_path, &mut visited, &mut rec_stack, &mut Vec::new(), &mut cycles);
            }
        }

        cycles
    }

    fn detect_cycle(
        &self,
        node: &str,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
        path: &mut Vec<String>,
        cycles: &mut Vec<Vec<String>>,
    ) {
        visited.insert(node.to_string());
        rec_stack.insert(node.to_string());
        path.push(node.to_string());

        for dep in self.get_dependencies(node) {
            if !visited.contains(&dep) {
                self.detect_cycle(&dep, visited, rec_stack, path, cycles);
            } else if rec_stack.contains(&dep) {
                if let Some(cycle_start) = path.iter().position(|p| p == &dep) {
                    let cycle = path[cycle_start..].to_vec();
                    if !cycles.contains(&cycle) {
                        cycles.push(cycle);
                    }
                }
            }
        }

        path.pop();
        rec_stack.remove(node);
    }

    pub fn get_dependency_depth(&self, path: &str) -> usize {
        let mut max_depth = 0;
        let mut visited = HashSet::new();
        self.calculate_depth(path, 0, &mut visited, &mut max_depth);
        max_depth
    }

    fn calculate_depth(&self, node: &str, depth: usize, visited: &mut HashSet<String>, max_depth: &mut usize) {
        if visited.contains(node) {
            return;
        }

        visited.insert(node.to_string());
        *max_depth = (*max_depth).max(depth);

        for dep in self.get_dependencies(node) {
            self.calculate_depth(&dep, depth + 1, visited, max_depth);
        }
    }

    pub fn get_leaf_nodes(&self) -> Vec<String> {
        self.nodes
            .keys()
            .filter(|path| self.get_dependencies(path).is_empty())
            .cloned()
            .collect()
    }

    pub fn get_root_nodes(&self) -> Vec<String> {
        self.nodes
            .keys()
            .filter(|path| self.get_dependents(path).is_empty())
            .cloned()
            .collect()
    }

    pub fn to_dot(&self) -> String {
        let mut dot = String::from("digraph Dependencies {\n");
        dot.push_str("  rankdir=LR;\n");
        dot.push_str("  node [shape=box];\n\n");

        for (path, node) in &self.nodes {
            let label = node.module_name.clone();
            dot.push_str(&format!("  \"{}\" [label=\"{}\"];\n", path, label));
        }

        dot.push_str("\n");

        for edge in &self.edges {
            let color = match edge.edge_type {
                EdgeType::Import => "blue",
                EdgeType::Export => "green",
                EdgeType::Call => "red",
            };
            dot.push_str(&format!("  \"{}\" -> \"{}\" [color={}];\n", edge.from, edge.to, color));
        }

        dot.push_str("}\n");
        dot
    }
}

pub fn build_dependency_graph(
    path: &Path,
    extensions: Option<&[String]>,
    exclude: Option<&[String]>,
) -> Result<DependencyGraph, Box<dyn std::error::Error>> {
    let mut graph = DependencyGraph::new();

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
        if let Ok(node) = extract_dependencies(file) {
            graph.add_node(node);
        }
    }

    for file in &files {
        let file_str = file.to_string_lossy().to_string();
        let imports: Vec<String> = if let Some(node) = graph.nodes.get(&file_str) {
            node.imports.clone()
        } else {
            Vec::new()
        };
        
        for import in imports {
            if let Some(target_path) = resolve_import(&import, file, &files) {
                graph.add_edge(
                    file_str.clone(),
                    target_path,
                    EdgeType::Import,
                );
            }
        }
    }

    Ok(graph)
}

fn extract_dependencies(path: &Path) -> Result<DependencyNode, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
    let imports = extract_imports_from_content(&content, ext);
    let exports = extract_exports_from_content(&content, ext);

    let module_name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string();

    Ok(DependencyNode {
        path: path.to_string_lossy().to_string(),
        module_name,
        imports,
        exports,
    })
}

fn extract_imports_from_content(content: &str, ext: &str) -> Vec<String> {
    let mut imports = Vec::new();

    let patterns = match ext {
        "rs" => vec![r"use\s+([\w:]+)"],
        "py" => vec![r"import\s+([\w.]+)", r"from\s+([\w.]+)\s+import"],
        "js" | "ts" => vec![r#"import\s+.*\s+from\s+['"]([^'"]+)['"]"#, r#"require\(['"]([^'"]+)['"]\)"#],
        "go" => vec![r#"import\s+"([^"]+)""#],
        _ => vec![],
    };

    for pattern in patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            for line in content.lines() {
                if let Some(caps) = re.captures(line) {
                    if let Some(import) = caps.get(1) {
                        imports.push(import.as_str().to_string());
                    }
                }
            }
        }
    }

    imports
}

fn extract_exports_from_content(content: &str, ext: &str) -> Vec<String> {
    let mut exports = Vec::new();

    let patterns = match ext {
        "rs" => vec![r"pub\s+fn\s+(\w+)", r"pub\s+struct\s+(\w+)", r"pub\s+enum\s+(\w+)"],
        "py" => vec![r"def\s+(\w+)", r"class\s+(\w+)"],
        "js" | "ts" => vec![r"export\s+(?:function|class|const|let|var)\s+(\w+)"],
        _ => vec![],
    };

    for pattern in patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            for line in content.lines() {
                if let Some(caps) = re.captures(line) {
                    if let Some(export) = caps.get(1) {
                        exports.push(export.as_str().to_string());
                    }
                }
            }
        }
    }

    exports
}

fn resolve_import(import: &str, _current_file: &Path, all_files: &[PathBuf]) -> Option<String> {
    for file in all_files {
        if let Some(stem) = file.file_stem().and_then(|s| s.to_str()) {
            if import.contains(stem) {
                return Some(file.to_string_lossy().to_string());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dependency_graph_creation() {
        let mut graph = DependencyGraph::new();
        
        let node = DependencyNode {
            path: "test.rs".to_string(),
            module_name: "test".to_string(),
            imports: vec!["std::io".to_string()],
            exports: vec!["main".to_string()],
        };
        
        graph.add_node(node);
        assert_eq!(graph.nodes.len(), 1);
    }

    #[test]
    fn test_add_edge() {
        let mut graph = DependencyGraph::new();
        graph.add_edge("a.rs".to_string(), "b.rs".to_string(), EdgeType::Import);
        
        assert_eq!(graph.edges.len(), 1);
        assert_eq!(graph.edges[0].from, "a.rs");
        assert_eq!(graph.edges[0].to, "b.rs");
    }

    #[test]
    fn test_get_dependencies() {
        let mut graph = DependencyGraph::new();
        graph.add_edge("a.rs".to_string(), "b.rs".to_string(), EdgeType::Import);
        graph.add_edge("a.rs".to_string(), "c.rs".to_string(), EdgeType::Import);
        
        let deps = graph.get_dependencies("a.rs");
        assert_eq!(deps.len(), 2);
    }

    #[test]
    fn test_circular_dependency_detection() {
        let mut graph = DependencyGraph::new();
        
        graph.add_node(DependencyNode {
            path: "a.rs".to_string(),
            module_name: "a".to_string(),
            imports: vec![],
            exports: vec![],
        });
        
        graph.add_node(DependencyNode {
            path: "b.rs".to_string(),
            module_name: "b".to_string(),
            imports: vec![],
            exports: vec![],
        });
        
        graph.add_edge("a.rs".to_string(), "b.rs".to_string(), EdgeType::Import);
        graph.add_edge("b.rs".to_string(), "a.rs".to_string(), EdgeType::Import);
        
        let cycles = graph.find_circular_dependencies();
        assert!(!cycles.is_empty());
    }
}
