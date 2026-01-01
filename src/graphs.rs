//! Unified Graph Analysis Module
//!
//! Provides a common interface for all graph types and unified visualization.

use crate::ast::{analyze_file, AstAnalysis};
use crate::callgraph::{build_call_graph, CallGraph};
use crate::cfg::{analyze_file_cfg, ControlFlowGraph};
use crate::depgraph::{build_dependency_graph, DependencyGraph};
use crate::dfg::{analyze_file_dfg, DataFlowGraph};
use crate::pdg::{analyze_file_pdg, ProgramDependencyGraph};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GraphType {
    AST,
    CFG,
    DFG,
    CallGraph,
    DependencyGraph,
    PDG,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphAnalysisResult {
    pub graph_type: GraphType,
    pub file_path: String,
    pub summary: GraphSummary,
    pub dot_output: Option<String>,
    pub json_output: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphSummary {
    pub node_count: usize,
    pub edge_count: usize,
    pub key_findings: Vec<String>,
}

pub struct GraphAnalyzer {
    pub path: String,
    pub extensions: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
}

impl GraphAnalyzer {
    pub fn new(path: String) -> Self {
        Self {
            path,
            extensions: None,
            exclude: None,
        }
    }

    pub fn with_extensions(mut self, extensions: Vec<String>) -> Self {
        self.extensions = Some(extensions);
        self
    }

    pub fn with_exclude(mut self, exclude: Vec<String>) -> Self {
        self.exclude = Some(exclude);
        self
    }

    pub fn analyze_ast(&self, file_path: &Path) -> Result<GraphAnalysisResult, Box<dyn std::error::Error>> {
        let ast = analyze_file(file_path)?;
        
        let node_count = ast.functions.len() + ast.classes.len() + ast.imports.len() + ast.variables.len();
        let mut key_findings = Vec::new();
        
        key_findings.push(format!("Functions: {}", ast.functions.len()));
        key_findings.push(format!("Classes: {}", ast.classes.len()));
        key_findings.push(format!("Imports: {}", ast.imports.len()));
        key_findings.push(format!("Variables: {}", ast.variables.len()));

        if !ast.functions.is_empty() {
            let public_funcs = ast.functions.iter().filter(|f| f.is_public).count();
            key_findings.push(format!("Public functions: {}", public_funcs));
        }

        Ok(GraphAnalysisResult {
            graph_type: GraphType::AST,
            file_path: file_path.to_string_lossy().to_string(),
            summary: GraphSummary {
                node_count,
                edge_count: 0,
                key_findings,
            },
            dot_output: None,
            json_output: Some(serde_json::to_string_pretty(&ast)?),
        })
    }

    pub fn analyze_cfg(&self, file_path: &Path) -> Result<Vec<GraphAnalysisResult>, Box<dyn std::error::Error>> {
        let cfgs = analyze_file_cfg(file_path)?;
        let mut results = Vec::new();

        for cfg in cfgs {
            let mut key_findings = Vec::new();
            
            key_findings.push(format!("Basic blocks: {}", cfg.basic_blocks.len()));
            key_findings.push(format!("Edges: {}", cfg.edges.len()));
            key_findings.push(format!("Cyclomatic complexity: {}", cfg.calculate_cyclomatic_complexity()));
            
            let unreachable = cfg.find_unreachable_blocks();
            if !unreachable.is_empty() {
                key_findings.push(format!("Unreachable blocks: {}", unreachable.len()));
            }
            
            let loops = cfg.find_loops();
            if !loops.is_empty() {
                key_findings.push(format!("Loops detected: {}", loops.len()));
            }

            results.push(GraphAnalysisResult {
                graph_type: GraphType::CFG,
                file_path: file_path.to_string_lossy().to_string(),
                summary: GraphSummary {
                    node_count: cfg.basic_blocks.len(),
                    edge_count: cfg.edges.len(),
                    key_findings,
                },
                dot_output: Some(cfg.to_dot()),
                json_output: Some(serde_json::to_string_pretty(&cfg)?),
            });
        }

        Ok(results)
    }

    pub fn analyze_dfg(&self, file_path: &Path) -> Result<Vec<GraphAnalysisResult>, Box<dyn std::error::Error>> {
        let dfgs = analyze_file_dfg(file_path)?;
        let mut results = Vec::new();

        for dfg in dfgs {
            let mut key_findings = Vec::new();
            
            key_findings.push(format!("Data nodes: {}", dfg.nodes.len()));
            key_findings.push(format!("Data flows: {}", dfg.edges.len()));
            
            let unused = dfg.find_unused_variables();
            if !unused.is_empty() {
                key_findings.push(format!("Unused variables: {}", unused.len()));
            }
            
            let redundant = dfg.find_redundant_computations();
            if !redundant.is_empty() {
                key_findings.push(format!("Redundant computations: {}", redundant.len()));
            }

            results.push(GraphAnalysisResult {
                graph_type: GraphType::DFG,
                file_path: file_path.to_string_lossy().to_string(),
                summary: GraphSummary {
                    node_count: dfg.nodes.len(),
                    edge_count: dfg.edges.len(),
                    key_findings,
                },
                dot_output: Some(dfg.to_dot()),
                json_output: Some(serde_json::to_string_pretty(&dfg)?),
            });
        }

        Ok(results)
    }

    pub fn analyze_call_graph(&self) -> Result<GraphAnalysisResult, Box<dyn std::error::Error>> {
        let path = Path::new(&self.path);
        let graph = build_call_graph(path, self.extensions.as_deref(), self.exclude.as_deref())?;
        
        let mut key_findings = Vec::new();
        
        key_findings.push(format!("Functions: {}", graph.nodes.len()));
        key_findings.push(format!("Function calls: {}", graph.edges.len()));
        
        let recursive = graph.find_recursive_functions();
        if !recursive.is_empty() {
            key_findings.push(format!("Recursive functions: {}", recursive.len()));
        }
        
        let dead = graph.find_dead_functions();
        if !dead.is_empty() {
            key_findings.push(format!("Dead functions: {}", dead.len()));
        }

        Ok(GraphAnalysisResult {
            graph_type: GraphType::CallGraph,
            file_path: self.path.clone(),
            summary: GraphSummary {
                node_count: graph.nodes.len(),
                edge_count: graph.edges.len(),
                key_findings,
            },
            dot_output: Some(graph.to_dot()),
            json_output: Some(serde_json::to_string_pretty(&graph)?),
        })
    }

    pub fn analyze_dependency_graph(&self) -> Result<GraphAnalysisResult, Box<dyn std::error::Error>> {
        let path = Path::new(&self.path);
        let graph = build_dependency_graph(path, self.extensions.as_deref(), self.exclude.as_deref())?;
        
        let mut key_findings = Vec::new();
        
        key_findings.push(format!("Modules: {}", graph.nodes.len()));
        key_findings.push(format!("Dependencies: {}", graph.edges.len()));
        
        let cycles = graph.find_circular_dependencies();
        if !cycles.is_empty() {
            key_findings.push(format!("Circular dependencies: {}", cycles.len()));
        }
        
        let roots = graph.get_root_nodes();
        key_findings.push(format!("Root modules: {}", roots.len()));
        
        let leaves = graph.get_leaf_nodes();
        key_findings.push(format!("Leaf modules: {}", leaves.len()));

        Ok(GraphAnalysisResult {
            graph_type: GraphType::DependencyGraph,
            file_path: self.path.clone(),
            summary: GraphSummary {
                node_count: graph.nodes.len(),
                edge_count: graph.edges.len(),
                key_findings,
            },
            dot_output: Some(graph.to_dot()),
            json_output: Some(serde_json::to_string_pretty(&graph)?),
        })
    }

    pub fn analyze_pdg(&self, file_path: &Path) -> Result<Vec<GraphAnalysisResult>, Box<dyn std::error::Error>> {
        let pdgs = analyze_file_pdg(file_path)?;
        let mut results = Vec::new();

        for pdg in pdgs {
            let mut key_findings = Vec::new();
            
            key_findings.push(format!("Nodes: {}", pdg.nodes.len()));
            key_findings.push(format!("Dependencies: {}", pdg.edges.len()));
            
            let control_deps = pdg.edges.iter().filter(|e| e.dependency_type == crate::pdg::DependencyType::ControlDependence).count();
            let data_deps = pdg.edges.iter().filter(|e| e.dependency_type == crate::pdg::DependencyType::DataDependence).count();
            let both_deps = pdg.edges.iter().filter(|e| e.dependency_type == crate::pdg::DependencyType::Both).count();
            
            key_findings.push(format!("Control dependencies: {}", control_deps));
            key_findings.push(format!("Data dependencies: {}", data_deps));
            key_findings.push(format!("Both: {}", both_deps));
            
            let parallel_ops = pdg.find_parallel_opportunities();
            if !parallel_ops.is_empty() {
                key_findings.push(format!("Parallelization opportunities: {}", parallel_ops.len()));
            }

            results.push(GraphAnalysisResult {
                graph_type: GraphType::PDG,
                file_path: file_path.to_string_lossy().to_string(),
                summary: GraphSummary {
                    node_count: pdg.nodes.len(),
                    edge_count: pdg.edges.len(),
                    key_findings,
                },
                dot_output: Some(pdg.to_dot()),
                json_output: Some(serde_json::to_string_pretty(&pdg)?),
            });
        }

        Ok(results)
    }

    pub fn analyze_all(&self, file_path: &Path) -> Result<Vec<GraphAnalysisResult>, Box<dyn std::error::Error>> {
        let mut results = Vec::new();

        if let Ok(ast_result) = self.analyze_ast(file_path) {
            results.push(ast_result);
        }

        if let Ok(cfg_results) = self.analyze_cfg(file_path) {
            results.extend(cfg_results);
        }

        if let Ok(dfg_results) = self.analyze_dfg(file_path) {
            results.extend(dfg_results);
        }

        if let Ok(pdg_results) = self.analyze_pdg(file_path) {
            results.extend(pdg_results);
        }

        Ok(results)
    }
}

pub fn export_graph_to_file(result: &GraphAnalysisResult, output_path: &Path, format: &str) -> Result<(), Box<dyn std::error::Error>> {
    match format {
        "dot" => {
            if let Some(dot) = &result.dot_output {
                std::fs::write(output_path, dot)?;
            }
        }
        "json" => {
            if let Some(json) = &result.json_output {
                std::fs::write(output_path, json)?;
            }
        }
        _ => return Err("Unsupported format".into()),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_analyzer_creation() {
        let analyzer = GraphAnalyzer::new(".".to_string());
        assert_eq!(analyzer.path, ".");
    }

    #[test]
    fn test_graph_analyzer_with_extensions() {
        let analyzer = GraphAnalyzer::new(".".to_string())
            .with_extensions(vec!["rs".to_string(), "py".to_string()]);
        assert!(analyzer.extensions.is_some());
        assert_eq!(analyzer.extensions.unwrap().len(), 2);
    }
}
