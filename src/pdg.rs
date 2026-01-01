//! Program Dependency Graph (PDG) Module
//!
//! Combines Control Flow Graph (CFG) and Data Flow Graph (DFG) for comprehensive analysis.

use crate::cfg::{ControlFlowGraph, EdgeType as CfgEdgeType};
use crate::dfg::{DataFlowGraph, DataFlowType};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramDependencyGraph {
    pub function_name: String,
    pub file_path: String,
    pub nodes: HashMap<usize, PdgNode>,
    pub edges: Vec<PdgEdge>,
    pub cfg: Option<ControlFlowGraph>,
    pub dfg: Option<DataFlowGraph>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdgNode {
    pub id: usize,
    pub node_type: PdgNodeType,
    pub label: String,
    pub line: usize,
    pub statement: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PdgNodeType {
    Statement,
    Predicate,
    Entry,
    Exit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdgEdge {
    pub from: usize,
    pub to: usize,
    pub dependency_type: DependencyType,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DependencyType {
    ControlDependence,
    DataDependence,
    Both,
}

impl ProgramDependencyGraph {
    pub fn new(function_name: String, file_path: String) -> Self {
        Self {
            function_name,
            file_path,
            nodes: HashMap::new(),
            edges: Vec::new(),
            cfg: None,
            dfg: None,
        }
    }

    pub fn from_cfg_and_dfg(cfg: ControlFlowGraph, dfg: DataFlowGraph) -> Self {
        let mut pdg = Self::new(cfg.function_name.clone(), cfg.file_path.clone());
        
        let mut node_id = 0;
        let mut line_to_node: HashMap<usize, usize> = HashMap::new();

        for (block_id, block) in &cfg.basic_blocks {
            for line in block.start_line..=block.end_line {
                let node = PdgNode {
                    id: node_id,
                    node_type: PdgNodeType::Statement,
                    label: format!("Block {}", block_id),
                    line,
                    statement: block.instructions.join("; "),
                };
                line_to_node.insert(line, node_id);
                pdg.nodes.insert(node_id, node);
                node_id += 1;
            }
        }

        for edge in &cfg.edges {
            if let (Some(&from_node), Some(&to_node)) = (
                cfg.basic_blocks.get(&edge.from).map(|b| b.start_line).and_then(|l| line_to_node.get(&l)),
                cfg.basic_blocks.get(&edge.to).map(|b| b.start_line).and_then(|l| line_to_node.get(&l)),
            ) {
                pdg.add_edge(
                    from_node,
                    to_node,
                    DependencyType::ControlDependence,
                    edge.condition.clone(),
                );
            }
        }

        for edge in &dfg.edges {
            if let (Some(from_node), Some(to_node)) = (dfg.nodes.get(&edge.from), dfg.nodes.get(&edge.to)) {
                if let (Some(&from_id), Some(&to_id)) = (
                    line_to_node.get(&from_node.line),
                    line_to_node.get(&to_node.line),
                ) {
                    let existing_edge = pdg.edges.iter_mut().find(|e| e.from == from_id && e.to == to_id);
                    
                    if let Some(existing) = existing_edge {
                        if existing.dependency_type == DependencyType::ControlDependence {
                            existing.dependency_type = DependencyType::Both;
                        }
                    } else {
                        pdg.add_edge(from_id, to_id, DependencyType::DataDependence, edge.label.clone());
                    }
                }
            }
        }

        pdg.cfg = Some(cfg);
        pdg.dfg = Some(dfg);
        pdg
    }

    pub fn add_node(&mut self, node: PdgNode) {
        self.nodes.insert(node.id, node);
    }

    pub fn add_edge(&mut self, from: usize, to: usize, dependency_type: DependencyType, label: Option<String>) {
        self.edges.push(PdgEdge {
            from,
            to,
            dependency_type,
            label,
        });
    }

    pub fn program_slice(&self, criterion_node: usize) -> HashSet<usize> {
        let mut slice = HashSet::new();
        let mut worklist = vec![criterion_node];

        while let Some(node) = worklist.pop() {
            if slice.insert(node) {
                for edge in &self.edges {
                    if edge.to == node {
                        worklist.push(edge.from);
                    }
                }
            }
        }

        slice
    }

    pub fn forward_slice(&self, criterion_node: usize) -> HashSet<usize> {
        let mut slice = HashSet::new();
        let mut worklist = vec![criterion_node];

        while let Some(node) = worklist.pop() {
            if slice.insert(node) {
                for edge in &self.edges {
                    if edge.from == node {
                        worklist.push(edge.to);
                    }
                }
            }
        }

        slice
    }

    pub fn find_parallel_opportunities(&self) -> Vec<Vec<usize>> {
        let mut parallel_groups = Vec::new();
        let mut independent_nodes: HashMap<usize, HashSet<usize>> = HashMap::new();

        let node_ids: Vec<usize> = self.nodes.keys().copied().collect();
        
        for &node_id in &node_ids {
            let mut independent = HashSet::new();
            
            for &other_id in &node_ids {
                if node_id != other_id && !self.has_dependency(node_id, other_id) && !self.has_dependency(other_id, node_id) {
                    independent.insert(other_id);
                }
            }
            
            independent_nodes.insert(node_id, independent);
        }

        let mut visited = HashSet::new();
        for (node_id, independents) in &independent_nodes {
            if !visited.contains(node_id) && !independents.is_empty() {
                let mut group = vec![*node_id];
                for &ind_id in independents {
                    if !visited.contains(&ind_id) {
                        group.push(ind_id);
                        visited.insert(ind_id);
                    }
                }
                if group.len() > 1 {
                    parallel_groups.push(group);
                }
                visited.insert(*node_id);
            }
        }

        parallel_groups
    }

    fn has_dependency(&self, from: usize, to: usize) -> bool {
        self.edges.iter().any(|e| e.from == from && e.to == to)
    }

    pub fn find_tainted_nodes(&self, taint_source: usize) -> HashSet<usize> {
        self.forward_slice(taint_source)
    }

    pub fn to_dot(&self) -> String {
        let mut dot = format!("digraph PDG_{} {{\n", self.function_name);
        dot.push_str("  rankdir=TB;\n");
        dot.push_str("  node [shape=box];\n\n");

        for (id, node) in &self.nodes {
            let color = match node.node_type {
                PdgNodeType::Entry => "lightgreen",
                PdgNodeType::Exit => "lightcoral",
                PdgNodeType::Predicate => "lightyellow",
                PdgNodeType::Statement => "lightblue",
            };

            dot.push_str(&format!(
                "  {} [label=\"{}\\nLine {}\", fillcolor={}, style=filled];\n",
                id, node.label, node.line, color
            ));
        }

        dot.push_str("\n");

        for edge in &self.edges {
            let (color, style) = match edge.dependency_type {
                DependencyType::ControlDependence => ("red", "solid"),
                DependencyType::DataDependence => ("blue", "dashed"),
                DependencyType::Both => ("purple", "bold"),
            };

            let label = edge.label.as_ref().map(|l| format!(" [label=\"{}\"]", l)).unwrap_or_default();
            dot.push_str(&format!(
                "  {} -> {} [color={}, style={}{}];\n",
                edge.from, edge.to, color, style, label
            ));
        }

        dot.push_str("}\n");
        dot
    }
}

pub fn build_pdg_from_source(content: &str, function_name: &str, file_path: &str) -> Result<ProgramDependencyGraph, Box<dyn std::error::Error>> {
    let cfg = crate::cfg::build_cfg_from_source(content, function_name, file_path)?;
    let dfg = crate::dfg::build_dfg_from_source(content, function_name, file_path)?;
    
    Ok(ProgramDependencyGraph::from_cfg_and_dfg(cfg, dfg))
}

pub fn analyze_file_pdg(path: &Path) -> Result<Vec<ProgramDependencyGraph>, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let mut pdgs = Vec::new();

    let function_pattern = regex::Regex::new(r"(?:fn|def|function)\s+(\w+)")?;

    for cap in function_pattern.captures_iter(&content) {
        if let Some(func_name) = cap.get(1) {
            if let Ok(pdg) = build_pdg_from_source(&content, func_name.as_str(), &path.to_string_lossy()) {
                pdgs.push(pdg);
            }
        }
    }

    Ok(pdgs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pdg_creation() {
        let pdg = ProgramDependencyGraph::new("test_func".to_string(), "test.rs".to_string());
        assert_eq!(pdg.function_name, "test_func");
        assert_eq!(pdg.nodes.len(), 0);
    }

    #[test]
    fn test_add_node() {
        let mut pdg = ProgramDependencyGraph::new("test".to_string(), "test.rs".to_string());
        let node = PdgNode {
            id: 0,
            node_type: PdgNodeType::Statement,
            label: "test".to_string(),
            line: 1,
            statement: "let x = 5;".to_string(),
        };
        pdg.add_node(node);
        assert_eq!(pdg.nodes.len(), 1);
    }

    #[test]
    fn test_program_slice() {
        let mut pdg = ProgramDependencyGraph::new("test".to_string(), "test.rs".to_string());
        
        pdg.add_node(PdgNode {
            id: 0,
            node_type: PdgNodeType::Statement,
            label: "s0".to_string(),
            line: 1,
            statement: "x = 1".to_string(),
        });
        
        pdg.add_node(PdgNode {
            id: 1,
            node_type: PdgNodeType::Statement,
            label: "s1".to_string(),
            line: 2,
            statement: "y = x + 1".to_string(),
        });
        
        pdg.add_edge(0, 1, DependencyType::DataDependence, None);
        
        let slice = pdg.program_slice(1);
        assert!(slice.contains(&0));
        assert!(slice.contains(&1));
    }

    #[test]
    fn test_forward_slice() {
        let mut pdg = ProgramDependencyGraph::new("test".to_string(), "test.rs".to_string());
        
        pdg.add_node(PdgNode {
            id: 0,
            node_type: PdgNodeType::Statement,
            label: "s0".to_string(),
            line: 1,
            statement: "x = 1".to_string(),
        });
        
        pdg.add_node(PdgNode {
            id: 1,
            node_type: PdgNodeType::Statement,
            label: "s1".to_string(),
            line: 2,
            statement: "y = x + 1".to_string(),
        });
        
        pdg.add_edge(0, 1, DependencyType::DataDependence, None);
        
        let slice = pdg.forward_slice(0);
        assert!(slice.contains(&0));
        assert!(slice.contains(&1));
    }
}
