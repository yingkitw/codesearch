//! Control Flow Graph (CFG) Module
//!
//! Analyzes control flow within functions, tracking branches, loops, and jumps.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlFlowGraph {
    pub function_name: String,
    pub file_path: String,
    pub basic_blocks: HashMap<usize, BasicBlock>,
    pub edges: Vec<CfgEdge>,
    pub entry_block: usize,
    pub exit_blocks: Vec<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicBlock {
    pub id: usize,
    pub start_line: usize,
    pub end_line: usize,
    pub instructions: Vec<String>,
    pub block_type: BlockType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BlockType {
    Entry,
    Normal,
    Branch,
    Loop,
    Exit,
    Return,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CfgEdge {
    pub from: usize,
    pub to: usize,
    pub edge_type: EdgeType,
    pub condition: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EdgeType {
    Sequential,
    Conditional,
    LoopBack,
    Break,
    Continue,
    Return,
}

impl ControlFlowGraph {
    pub fn new(function_name: String, file_path: String) -> Self {
        Self {
            function_name,
            file_path,
            basic_blocks: HashMap::new(),
            edges: Vec::new(),
            entry_block: 0,
            exit_blocks: Vec::new(),
        }
    }

    pub fn add_block(&mut self, block: BasicBlock) {
        self.basic_blocks.insert(block.id, block);
    }

    pub fn add_edge(&mut self, from: usize, to: usize, edge_type: EdgeType, condition: Option<String>) {
        self.edges.push(CfgEdge {
            from,
            to,
            edge_type,
            condition,
        });
    }

    pub fn find_unreachable_blocks(&self) -> Vec<usize> {
        let mut reachable = HashSet::new();
        let mut queue = VecDeque::new();
        
        queue.push_back(self.entry_block);
        reachable.insert(self.entry_block);

        while let Some(block_id) = queue.pop_front() {
            for edge in &self.edges {
                if edge.from == block_id && !reachable.contains(&edge.to) {
                    reachable.insert(edge.to);
                    queue.push_back(edge.to);
                }
            }
        }

        self.basic_blocks
            .keys()
            .filter(|id| !reachable.contains(id))
            .copied()
            .collect()
    }

    pub fn find_loops(&self) -> Vec<Vec<usize>> {
        let mut loops = Vec::new();
        
        for edge in &self.edges {
            if edge.edge_type == EdgeType::LoopBack {
                let loop_nodes = self.find_loop_body(edge.to, edge.from);
                if !loop_nodes.is_empty() {
                    loops.push(loop_nodes);
                }
            }
        }

        loops
    }

    fn find_loop_body(&self, header: usize, back_edge_from: usize) -> Vec<usize> {
        let mut loop_body = HashSet::new();
        let mut stack = vec![back_edge_from];
        loop_body.insert(header);

        while let Some(node) = stack.pop() {
            if loop_body.insert(node) {
                for edge in &self.edges {
                    if edge.to == node && !loop_body.contains(&edge.from) {
                        stack.push(edge.from);
                    }
                }
            }
        }

        let mut result: Vec<usize> = loop_body.into_iter().collect();
        result.sort();
        result
    }

    pub fn calculate_cyclomatic_complexity(&self) -> usize {
        let edges = self.edges.len();
        let nodes = self.basic_blocks.len();
        let exit_points = self.exit_blocks.len().max(1);
        
        edges.saturating_sub(nodes) + 2 * exit_points
    }

    pub fn to_dot(&self) -> String {
        let mut dot = format!("digraph CFG_{} {{\n", self.function_name);
        dot.push_str("  rankdir=TB;\n");
        dot.push_str("  node [shape=box];\n\n");

        for (id, block) in &self.basic_blocks {
            let color = match block.block_type {
                BlockType::Entry => "lightgreen",
                BlockType::Exit | BlockType::Return => "lightcoral",
                BlockType::Branch => "lightyellow",
                BlockType::Loop => "lightblue",
                BlockType::Normal => "white",
            };
            
            let label = format!("Block {}\\nLines {}-{}", id, block.start_line, block.end_line);
            dot.push_str(&format!("  {} [label=\"{}\", fillcolor={}, style=filled];\n", id, label, color));
        }

        dot.push_str("\n");

        for edge in &self.edges {
            let label = match &edge.condition {
                Some(cond) => format!(" [label=\"{}\"]", cond),
                None => String::new(),
            };
            
            let style = match edge.edge_type {
                EdgeType::LoopBack => " [style=dashed, color=blue]",
                EdgeType::Conditional => " [color=red]",
                EdgeType::Break | EdgeType::Continue => " [style=dotted]",
                _ => "",
            };
            
            dot.push_str(&format!("  {} -> {}{}{};\n", edge.from, edge.to, label, style));
        }

        dot.push_str("}\n");
        dot
    }
}

pub fn build_cfg_from_source(content: &str, function_name: &str, file_path: &str) -> Result<ControlFlowGraph, Box<dyn std::error::Error>> {
    let mut cfg = ControlFlowGraph::new(function_name.to_string(), file_path.to_string());
    let mut block_id = 0;

    let lines: Vec<&str> = content.lines().collect();
    let mut in_function = false;
    let mut brace_depth = 0;

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        
        if !in_function && (trimmed.contains(&format!("fn {}", function_name)) 
            || trimmed.contains(&format!("def {}", function_name))
            || trimmed.contains(&format!("function {}", function_name))) {
            in_function = true;
            let line_num = idx + 1;
            
            let entry_block = BasicBlock {
                id: block_id,
                start_line: line_num,
                end_line: line_num,
                instructions: vec![trimmed.to_string()],
                block_type: BlockType::Entry,
            };
            cfg.add_block(entry_block);
            cfg.entry_block = block_id;
            block_id += 1;
            continue;
        }

        if in_function {
            if trimmed.contains('{') {
                brace_depth += trimmed.matches('{').count();
            }
            if trimmed.contains('}') {
                let close_count = trimmed.matches('}').count();
                brace_depth = if brace_depth > close_count {
                    brace_depth - close_count
                } else {
                    0
                };
                if brace_depth == 0 {
                    break;
                }
            }

            if trimmed.starts_with("if ") || trimmed.starts_with("else if ") {
                let branch_block = BasicBlock {
                    id: block_id,
                    start_line: idx + 1,
                    end_line: idx + 1,
                    instructions: vec![trimmed.to_string()],
                    block_type: BlockType::Branch,
                };
                cfg.add_block(branch_block);
                
                if block_id > 0 {
                    cfg.add_edge(block_id - 1, block_id, EdgeType::Sequential, None);
                }
                
                block_id += 1;
            } else if trimmed.starts_with("while ") || trimmed.starts_with("for ") || trimmed.starts_with("loop") {
                let loop_block = BasicBlock {
                    id: block_id,
                    start_line: idx + 1,
                    end_line: idx + 1,
                    instructions: vec![trimmed.to_string()],
                    block_type: BlockType::Loop,
                };
                cfg.add_block(loop_block);
                
                if block_id > 0 {
                    cfg.add_edge(block_id - 1, block_id, EdgeType::Sequential, None);
                }
                
                block_id += 1;
            } else if trimmed.starts_with("return") {
                let return_block = BasicBlock {
                    id: block_id,
                    start_line: idx + 1,
                    end_line: idx + 1,
                    instructions: vec![trimmed.to_string()],
                    block_type: BlockType::Return,
                };
                cfg.add_block(return_block);
                cfg.exit_blocks.push(block_id);
                
                if block_id > 0 {
                    cfg.add_edge(block_id - 1, block_id, EdgeType::Return, None);
                }
                
                block_id += 1;
            }
        }
    }

    if cfg.exit_blocks.is_empty() && block_id > 0 {
        cfg.exit_blocks.push(block_id - 1);
    }

    Ok(cfg)
}

pub fn analyze_file_cfg(path: &Path) -> Result<Vec<ControlFlowGraph>, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let mut cfgs = Vec::new();

    let function_pattern = regex::Regex::new(r"(?:fn|def|function)\s+(\w+)")?;
    
    for cap in function_pattern.captures_iter(&content) {
        if let Some(func_name) = cap.get(1) {
            if let Ok(cfg) = build_cfg_from_source(&content, func_name.as_str(), &path.to_string_lossy()) {
                cfgs.push(cfg);
            }
        }
    }

    Ok(cfgs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cfg_creation() {
        let cfg = ControlFlowGraph::new("test_func".to_string(), "test.rs".to_string());
        assert_eq!(cfg.function_name, "test_func");
        assert_eq!(cfg.basic_blocks.len(), 0);
    }

    #[test]
    fn test_add_block() {
        let mut cfg = ControlFlowGraph::new("test".to_string(), "test.rs".to_string());
        let block = BasicBlock {
            id: 0,
            start_line: 1,
            end_line: 5,
            instructions: vec!["test".to_string()],
            block_type: BlockType::Entry,
        };
        cfg.add_block(block);
        assert_eq!(cfg.basic_blocks.len(), 1);
    }

    #[test]
    fn test_find_unreachable_blocks() {
        let mut cfg = ControlFlowGraph::new("test".to_string(), "test.rs".to_string());
        
        cfg.add_block(BasicBlock {
            id: 0,
            start_line: 1,
            end_line: 1,
            instructions: vec![],
            block_type: BlockType::Entry,
        });
        
        cfg.add_block(BasicBlock {
            id: 1,
            start_line: 2,
            end_line: 2,
            instructions: vec![],
            block_type: BlockType::Normal,
        });
        
        cfg.add_block(BasicBlock {
            id: 2,
            start_line: 3,
            end_line: 3,
            instructions: vec![],
            block_type: BlockType::Normal,
        });
        
        cfg.add_edge(0, 1, EdgeType::Sequential, None);
        
        let unreachable = cfg.find_unreachable_blocks();
        assert_eq!(unreachable.len(), 1);
        assert!(unreachable.contains(&2));
    }

    #[test]
    fn test_cyclomatic_complexity() {
        let mut cfg = ControlFlowGraph::new("test".to_string(), "test.rs".to_string());
        
        cfg.add_block(BasicBlock {
            id: 0,
            start_line: 1,
            end_line: 1,
            instructions: vec![],
            block_type: BlockType::Entry,
        });
        
        cfg.add_block(BasicBlock {
            id: 1,
            start_line: 2,
            end_line: 2,
            instructions: vec![],
            block_type: BlockType::Branch,
        });
        
        cfg.add_edge(0, 1, EdgeType::Sequential, None);
        cfg.exit_blocks.push(1);
        
        let complexity = cfg.calculate_cyclomatic_complexity();
        assert!(complexity > 0);
    }
}
