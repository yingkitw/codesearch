//! Call Graph Module
//!
//! Analyzes function call relationships in code.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallGraph {
    pub nodes: HashMap<String, CallNode>,
    pub edges: Vec<CallEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallNode {
    pub function_name: String,
    pub file_path: String,
    pub line: usize,
    pub is_recursive: bool,
    pub call_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallEdge {
    pub caller: String,
    pub callee: String,
    pub call_site_line: usize,
    pub is_direct: bool,
}

impl CallGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: CallNode) {
        self.nodes.insert(node.function_name.clone(), node);
    }

    pub fn add_edge(&mut self, caller: String, callee: String, call_site_line: usize, is_direct: bool) {
        self.edges.push(CallEdge {
            caller,
            callee,
            call_site_line,
            is_direct,
        });
    }

    pub fn get_callers(&self, function: &str) -> Vec<String> {
        self.edges
            .iter()
            .filter(|e| e.callee == function)
            .map(|e| e.caller.clone())
            .collect()
    }

    pub fn get_callees(&self, function: &str) -> Vec<String> {
        self.edges
            .iter()
            .filter(|e| e.caller == function)
            .map(|e| e.callee.clone())
            .collect()
    }

    pub fn find_recursive_functions(&self) -> Vec<String> {
        let mut recursive = Vec::new();

        for (func_name, _) in &self.nodes {
            if self.is_recursive(func_name) {
                recursive.push(func_name.clone());
            }
        }

        recursive
    }

    fn is_recursive(&self, function: &str) -> bool {
        let mut visited = HashSet::new();
        let mut stack = vec![function.to_string()];

        while let Some(current) = stack.pop() {
            if current == function && !visited.is_empty() {
                return true;
            }

            if visited.insert(current.clone()) {
                for callee in self.get_callees(&current) {
                    stack.push(callee);
                }
            }
        }

        false
    }

    pub fn find_dead_functions(&self) -> Vec<String> {
        let mut called_functions = HashSet::new();

        for edge in &self.edges {
            called_functions.insert(edge.callee.clone());
        }

        self.nodes
            .keys()
            .filter(|func| !called_functions.contains(*func) && *func != "main")
            .cloned()
            .collect()
    }

    pub fn calculate_call_depth(&self, function: &str) -> usize {
        let mut max_depth = 0;
        let mut visited = HashSet::new();
        self.calculate_depth_recursive(function, 0, &mut visited, &mut max_depth);
        max_depth
    }

    fn calculate_depth_recursive(&self, function: &str, depth: usize, visited: &mut HashSet<String>, max_depth: &mut usize) {
        if visited.contains(function) {
            return;
        }

        visited.insert(function.to_string());
        *max_depth = (*max_depth).max(depth);

        for callee in self.get_callees(function) {
            self.calculate_depth_recursive(&callee, depth + 1, visited, max_depth);
        }

        visited.remove(function);
    }

    pub fn find_call_chains(&self, from: &str, to: &str) -> Vec<Vec<String>> {
        let mut chains = Vec::new();
        let mut current_path = vec![from.to_string()];
        let mut visited = HashSet::new();
        
        self.find_chains_recursive(from, to, &mut current_path, &mut visited, &mut chains);
        
        chains
    }

    fn find_chains_recursive(
        &self,
        current: &str,
        target: &str,
        path: &mut Vec<String>,
        visited: &mut HashSet<String>,
        chains: &mut Vec<Vec<String>>,
    ) {
        if current == target {
            chains.push(path.clone());
            return;
        }

        if visited.contains(current) {
            return;
        }

        visited.insert(current.to_string());

        for callee in self.get_callees(current) {
            path.push(callee.clone());
            self.find_chains_recursive(&callee, target, path, visited, chains);
            path.pop();
        }

        visited.remove(current);
    }

    pub fn to_dot(&self) -> String {
        let mut dot = String::from("digraph CallGraph {\n");
        dot.push_str("  rankdir=LR;\n");
        dot.push_str("  node [shape=box];\n\n");

        for (func_name, node) in &self.nodes {
            let color = if node.is_recursive {
                "lightcoral"
            } else if self.get_callers(func_name).is_empty() {
                "lightgreen"
            } else {
                "lightblue"
            };

            dot.push_str(&format!(
                "  \"{}\" [label=\"{}\\n({}:{})\", fillcolor={}, style=filled];\n",
                func_name, func_name, node.file_path, node.line, color
            ));
        }

        dot.push_str("\n");

        for edge in &self.edges {
            let style = if edge.is_direct { "" } else { " [style=dashed]" };
            dot.push_str(&format!("  \"{}\" -> \"{}\"{};\n", edge.caller, edge.callee, style));
        }

        dot.push_str("}\n");
        dot
    }
}

pub fn build_call_graph(
    path: &Path,
    extensions: Option<&[String]>,
    exclude: Option<&[String]>,
) -> Result<CallGraph, Box<dyn std::error::Error>> {
    let mut graph = CallGraph::new();
    let mut function_definitions: HashMap<String, (String, usize)> = HashMap::new();

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

    let func_def_pattern = regex::Regex::new(r"(?:fn|def|function)\s+(\w+)")?;
    let func_call_pattern = regex::Regex::new(r"(\w+)\s*\(")?;

    for entry in &files {
        let file_path = entry.path();
        let content = std::fs::read_to_string(file_path)?;

        for (line_num, line) in content.lines().enumerate() {
            if let Some(caps) = func_def_pattern.captures(line) {
                if let Some(func_name) = caps.get(1) {
                    let func_name_str = func_name.as_str().to_string();
                    function_definitions.insert(
                        func_name_str.clone(),
                        (file_path.to_string_lossy().to_string(), line_num + 1),
                    );

                    let node = CallNode {
                        function_name: func_name_str,
                        file_path: file_path.to_string_lossy().to_string(),
                        line: line_num + 1,
                        is_recursive: false,
                        call_count: 0,
                    };
                    graph.add_node(node);
                }
            }
        }
    }

    for entry in &files {
        let file_path = entry.path();
        let content = std::fs::read_to_string(file_path)?;
        let mut current_function = None;

        for (line_num, line) in content.lines().enumerate() {
            if let Some(caps) = func_def_pattern.captures(line) {
                if let Some(func_name) = caps.get(1) {
                    current_function = Some(func_name.as_str().to_string());
                }
            }

            if let Some(caller) = &current_function {
                for cap in func_call_pattern.captures_iter(line) {
                    if let Some(callee_match) = cap.get(1) {
                        let callee = callee_match.as_str().to_string();
                        
                        if function_definitions.contains_key(&callee) && callee != *caller {
                            graph.add_edge(
                                caller.clone(),
                                callee,
                                line_num + 1,
                                true,
                            );
                        }
                    }
                }
            }
        }
    }

    for func_name in graph.nodes.keys().cloned().collect::<Vec<_>>() {
        if graph.is_recursive(&func_name) {
            if let Some(node) = graph.nodes.get_mut(&func_name) {
                node.is_recursive = true;
            }
        }
    }

    Ok(graph)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_graph_creation() {
        let graph = CallGraph::new();
        assert_eq!(graph.nodes.len(), 0);
        assert_eq!(graph.edges.len(), 0);
    }

    #[test]
    fn test_add_node() {
        let mut graph = CallGraph::new();
        let node = CallNode {
            function_name: "test".to_string(),
            file_path: "test.rs".to_string(),
            line: 1,
            is_recursive: false,
            call_count: 0,
        };
        graph.add_node(node);
        assert_eq!(graph.nodes.len(), 1);
    }

    #[test]
    fn test_get_callees() {
        let mut graph = CallGraph::new();
        
        graph.add_node(CallNode {
            function_name: "main".to_string(),
            file_path: "test.rs".to_string(),
            line: 1,
            is_recursive: false,
            call_count: 0,
        });
        
        graph.add_node(CallNode {
            function_name: "helper".to_string(),
            file_path: "test.rs".to_string(),
            line: 5,
            is_recursive: false,
            call_count: 0,
        });
        
        graph.add_edge("main".to_string(), "helper".to_string(), 2, true);
        
        let callees = graph.get_callees("main");
        assert_eq!(callees.len(), 1);
        assert_eq!(callees[0], "helper");
    }

    #[test]
    fn test_find_dead_functions() {
        let mut graph = CallGraph::new();
        
        graph.add_node(CallNode {
            function_name: "main".to_string(),
            file_path: "test.rs".to_string(),
            line: 1,
            is_recursive: false,
            call_count: 0,
        });
        
        graph.add_node(CallNode {
            function_name: "unused".to_string(),
            file_path: "test.rs".to_string(),
            line: 10,
            is_recursive: false,
            call_count: 0,
        });
        
        let dead = graph.find_dead_functions();
        assert!(dead.contains(&"unused".to_string()));
    }
}
