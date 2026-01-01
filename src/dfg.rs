//! Data Flow Graph (DFG) Module
//!
//! Analyzes data flow between variables and operations in code.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFlowGraph {
    pub function_name: String,
    pub file_path: String,
    pub nodes: HashMap<usize, DfgNode>,
    pub edges: Vec<DfgEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DfgNode {
    pub id: usize,
    pub node_type: DfgNodeType,
    pub name: String,
    pub line: usize,
    pub definition: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DfgNodeType {
    Variable,
    Parameter,
    Constant,
    Operation,
    FunctionCall,
    Return,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DfgEdge {
    pub from: usize,
    pub to: usize,
    pub edge_type: DataFlowType,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DataFlowType {
    Definition,
    Use,
    Assignment,
    Parameter,
    Return,
}

impl DataFlowGraph {
    pub fn new(function_name: String, file_path: String) -> Self {
        Self {
            function_name,
            file_path,
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: DfgNode) {
        self.nodes.insert(node.id, node);
    }

    pub fn add_edge(&mut self, from: usize, to: usize, edge_type: DataFlowType, label: Option<String>) {
        self.edges.push(DfgEdge {
            from,
            to,
            edge_type,
            label,
        });
    }

    pub fn find_variable_uses(&self, variable_name: &str) -> Vec<usize> {
        self.nodes
            .iter()
            .filter(|(_, node)| node.name == variable_name)
            .map(|(id, _)| *id)
            .collect()
    }

    pub fn find_data_dependencies(&self, node_id: usize) -> Vec<usize> {
        self.edges
            .iter()
            .filter(|e| e.to == node_id)
            .map(|e| e.from)
            .collect()
    }

    pub fn find_variable_lifetime(&self, variable_name: &str) -> (usize, usize) {
        let uses = self.find_variable_uses(variable_name);
        if uses.is_empty() {
            return (0, 0);
        }

        let lines: Vec<usize> = uses
            .iter()
            .filter_map(|id| self.nodes.get(id))
            .map(|node| node.line)
            .collect();

        let min_line = *lines.iter().min().unwrap_or(&0);
        let max_line = *lines.iter().max().unwrap_or(&0);

        (min_line, max_line)
    }

    pub fn find_unused_variables(&self) -> Vec<String> {
        let mut defined = HashSet::new();
        let mut used = HashSet::new();

        for (_, node) in &self.nodes {
            if node.node_type == DfgNodeType::Variable || node.node_type == DfgNodeType::Parameter {
                defined.insert(node.name.clone());
            }
        }

        for edge in &self.edges {
            if edge.edge_type == DataFlowType::Use {
                if let Some(from_node) = self.nodes.get(&edge.from) {
                    used.insert(from_node.name.clone());
                }
            }
        }

        defined.difference(&used).cloned().collect()
    }

    pub fn find_redundant_computations(&self) -> Vec<(usize, usize)> {
        let mut redundant = Vec::new();
        let mut operations: HashMap<String, Vec<usize>> = HashMap::new();

        for (id, node) in &self.nodes {
            if node.node_type == DfgNodeType::Operation {
                if let Some(def) = &node.definition {
                    operations.entry(def.clone()).or_insert_with(Vec::new).push(*id);
                }
            }
        }

        for (_, ids) in operations {
            if ids.len() > 1 {
                for i in 0..ids.len() - 1 {
                    for j in i + 1..ids.len() {
                        redundant.push((ids[i], ids[j]));
                    }
                }
            }
        }

        redundant
    }

    pub fn to_dot(&self) -> String {
        let mut dot = format!("digraph DFG_{} {{\n", self.function_name);
        dot.push_str("  rankdir=LR;\n");
        dot.push_str("  node [shape=ellipse];\n\n");

        for (id, node) in &self.nodes {
            let (shape, color) = match node.node_type {
                DfgNodeType::Variable => ("ellipse", "lightblue"),
                DfgNodeType::Parameter => ("ellipse", "lightgreen"),
                DfgNodeType::Constant => ("box", "lightyellow"),
                DfgNodeType::Operation => ("diamond", "lightcoral"),
                DfgNodeType::FunctionCall => ("box", "lightpink"),
                DfgNodeType::Return => ("ellipse", "lightgray"),
            };

            dot.push_str(&format!(
                "  {} [label=\"{}\\n(line {})\", shape={}, fillcolor={}, style=filled];\n",
                id, node.name, node.line, shape, color
            ));
        }

        dot.push_str("\n");

        for edge in &self.edges {
            let label = edge.label.as_ref().map(|l| format!(" [label=\"{}\"]", l)).unwrap_or_default();
            let style = match edge.edge_type {
                DataFlowType::Definition => " [color=blue]",
                DataFlowType::Use => " [color=green]",
                DataFlowType::Assignment => " [color=red]",
                DataFlowType::Parameter => " [style=dashed]",
                DataFlowType::Return => " [style=dotted]",
            };

            dot.push_str(&format!("  {} -> {}{}{};\n", edge.from, edge.to, label, style));
        }

        dot.push_str("}\n");
        dot
    }
}

pub fn build_dfg_from_source(content: &str, function_name: &str, file_path: &str) -> Result<DataFlowGraph, Box<dyn std::error::Error>> {
    let mut dfg = DataFlowGraph::new(function_name.to_string(), file_path.to_string());
    let mut node_id = 0;
    let mut variable_map: HashMap<String, usize> = HashMap::new();

    let lines: Vec<&str> = content.lines().collect();
    let mut in_function = false;
    let mut brace_depth = 0;

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        if !in_function && (trimmed.contains(&format!("fn {}", function_name))
            || trimmed.contains(&format!("def {}", function_name))
            || trimmed.contains(&format!("function {}", function_name))) {
            in_function = true;

            let param_pattern = regex::Regex::new(r"\(([^)]*)\)")?;
            if let Some(params) = param_pattern.captures(trimmed) {
                if let Some(param_str) = params.get(1) {
                    for param in param_str.as_str().split(',') {
                        let param_name = param.trim().split(':').next().unwrap_or("").trim();
                        if !param_name.is_empty() {
                            let node = DfgNode {
                                id: node_id,
                                node_type: DfgNodeType::Parameter,
                                name: param_name.to_string(),
                                line: idx + 1,
                                definition: Some(param.trim().to_string()),
                            };
                            variable_map.insert(param_name.to_string(), node_id);
                            dfg.add_node(node);
                            node_id += 1;
                        }
                    }
                }
            }
            continue;
        }

        if in_function {
            if trimmed.contains('{') {
                brace_depth += trimmed.matches('{').count();
            }
            if trimmed.contains('}') {
                brace_depth = brace_depth.saturating_sub(trimmed.matches('}').count());
                if brace_depth == 0 {
                    break;
                }
            }

            let assignment_pattern = regex::Regex::new(r"(?:let|const|var|mut)?\s*(\w+)\s*=\s*(.+?)(?:;|$)")?;
            if let Some(caps) = assignment_pattern.captures(trimmed) {
                if let (Some(var_name), Some(expr)) = (caps.get(1), caps.get(2)) {
                    let var_name_str = var_name.as_str();
                    let expr_str = expr.as_str();

                    let var_node_id = node_id;
                    let var_node = DfgNode {
                        id: var_node_id,
                        node_type: DfgNodeType::Variable,
                        name: var_name_str.to_string(),
                        line: idx + 1,
                        definition: Some(expr_str.to_string()),
                    };
                    variable_map.insert(var_name_str.to_string(), var_node_id);
                    dfg.add_node(var_node);
                    node_id += 1;

                    let var_pattern = regex::Regex::new(r"\b(\w+)\b")?;
                    for cap in var_pattern.captures_iter(expr_str) {
                        if let Some(used_var) = cap.get(1) {
                            let used_var_str = used_var.as_str();
                            if let Some(&used_var_id) = variable_map.get(used_var_str) {
                                dfg.add_edge(used_var_id, var_node_id, DataFlowType::Use, None);
                            }
                        }
                    }
                }
            }

            if trimmed.starts_with("return") {
                let return_node = DfgNode {
                    id: node_id,
                    node_type: DfgNodeType::Return,
                    name: "return".to_string(),
                    line: idx + 1,
                    definition: Some(trimmed.to_string()),
                };
                dfg.add_node(return_node);

                let return_expr = trimmed.strip_prefix("return").unwrap_or("").trim();
                let var_pattern = regex::Regex::new(r"\b(\w+)\b")?;
                for cap in var_pattern.captures_iter(return_expr) {
                    if let Some(used_var) = cap.get(1) {
                        if let Some(&used_var_id) = variable_map.get(used_var.as_str()) {
                            dfg.add_edge(used_var_id, node_id, DataFlowType::Return, None);
                        }
                    }
                }

                node_id += 1;
            }
        }
    }

    Ok(dfg)
}

pub fn analyze_file_dfg(path: &Path) -> Result<Vec<DataFlowGraph>, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let mut dfgs = Vec::new();

    let function_pattern = regex::Regex::new(r"(?:fn|def|function)\s+(\w+)")?;

    for cap in function_pattern.captures_iter(&content) {
        if let Some(func_name) = cap.get(1) {
            if let Ok(dfg) = build_dfg_from_source(&content, func_name.as_str(), &path.to_string_lossy()) {
                dfgs.push(dfg);
            }
        }
    }

    Ok(dfgs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfg_creation() {
        let dfg = DataFlowGraph::new("test_func".to_string(), "test.rs".to_string());
        assert_eq!(dfg.function_name, "test_func");
        assert_eq!(dfg.nodes.len(), 0);
    }

    #[test]
    fn test_add_node() {
        let mut dfg = DataFlowGraph::new("test".to_string(), "test.rs".to_string());
        let node = DfgNode {
            id: 0,
            node_type: DfgNodeType::Variable,
            name: "x".to_string(),
            line: 1,
            definition: Some("5".to_string()),
        };
        dfg.add_node(node);
        assert_eq!(dfg.nodes.len(), 1);
    }

    #[test]
    fn test_find_variable_uses() {
        let mut dfg = DataFlowGraph::new("test".to_string(), "test.rs".to_string());
        
        dfg.add_node(DfgNode {
            id: 0,
            node_type: DfgNodeType::Variable,
            name: "x".to_string(),
            line: 1,
            definition: Some("5".to_string()),
        });
        
        dfg.add_node(DfgNode {
            id: 1,
            node_type: DfgNodeType::Variable,
            name: "x".to_string(),
            line: 2,
            definition: None,
        });
        
        let uses = dfg.find_variable_uses("x");
        assert_eq!(uses.len(), 2);
    }

    #[test]
    fn test_find_unused_variables() {
        let mut dfg = DataFlowGraph::new("test".to_string(), "test.rs".to_string());
        
        dfg.add_node(DfgNode {
            id: 0,
            node_type: DfgNodeType::Variable,
            name: "unused".to_string(),
            line: 1,
            definition: Some("5".to_string()),
        });
        
        dfg.add_node(DfgNode {
            id: 1,
            node_type: DfgNodeType::Variable,
            name: "used".to_string(),
            line: 2,
            definition: Some("10".to_string()),
        });
        
        dfg.add_edge(1, 2, DataFlowType::Use, None);
        
        let unused = dfg.find_unused_variables();
        assert!(unused.contains(&"unused".to_string()));
    }
}
