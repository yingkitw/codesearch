//! Graph Command Handlers
//!
//! Handles all graph-related CLI commands (CFG, DFG, PDG, etc.).

use crate::{cfg, dfg, pdg};
use std::path::Path;

/// Handle generic graph command
///
/// # Arguments
///
/// * `graph_type` - Type of graph to generate (ast, cfg, dfg, etc.)
/// * `path` - The file or directory to analyze
/// * `output` - Optional output file path
pub fn handle_graph_command(
    graph_type: &str,
    path: &Path,
    output: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    match graph_type {
        "cfg" => handle_cfg_command(path, output),
        "dfg" => handle_dfg_command(path, output),
        "pdg" => handle_pdg_command(path, output),
        _ => Err(format!("Unknown graph type: {}", graph_type).into()),
    }
}

/// Handle CFG (Control Flow Graph) command
///
/// Generates a control flow graph for the specified file.
///
/// # Arguments
///
/// * `path` - The file to analyze
/// * `output` - Optional output file path for DOT format
///
/// # Examples
///
/// ```no_run
/// use codesearch::commands::graph::handle_cfg_command;
/// use std::path::Path;
///
/// handle_cfg_command(Path::new("src/main.rs"), None).unwrap();
/// ```
pub fn handle_cfg_command(
    path: &Path,
    output: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let cfg_results = cfg::analyze_file_cfg(path, "main")?;
    
    if let Some(output_path) = output {
        let dot = cfg_results.to_dot();
        std::fs::write(output_path, dot)?;
        println!("CFG exported to: {}", output_path);
    } else {
        println!("CFG Analysis:");
        println!("  Nodes: {}", cfg_results.nodes.len());
        println!("  Edges: {}", cfg_results.edges.len());
    }
    
    Ok(())
}

/// Handle DFG (Data Flow Graph) command
///
/// Generates a data flow graph for the specified file.
///
/// # Arguments
///
/// * `path` - The file to analyze
/// * `output` - Optional output file path for DOT format
///
/// # Examples
///
/// ```no_run
/// use codesearch::commands::graph::handle_dfg_command;
/// use std::path::Path;
///
/// handle_dfg_command(Path::new("src/main.rs"), None).unwrap();
/// ```
pub fn handle_dfg_command(
    path: &Path,
    output: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let dfg_results = dfg::analyze_file_dfg(path, "main")?;
    
    if let Some(output_path) = output {
        let dot = dfg_results.to_dot();
        std::fs::write(output_path, dot)?;
        println!("DFG exported to: {}", output_path);
    } else {
        println!("DFG Analysis:");
        println!("  Variables: {}", dfg_results.variables.len());
        println!("  Flows: {}", dfg_results.flows.len());
    }
    
    Ok(())
}

/// Handle PDG (Program Dependency Graph) command
///
/// Generates a program dependency graph for the specified file.
///
/// # Arguments
///
/// * `path` - The file to analyze
/// * `output` - Optional output file path for DOT format
pub fn handle_pdg_command(
    path: &Path,
    output: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let pdg_results = pdg::analyze_file_pdg(path, "main")?;
    
    if let Some(output_path) = output {
        let dot = pdg_results.to_dot();
        std::fs::write(output_path, dot)?;
        println!("PDG exported to: {}", output_path);
    } else {
        println!("PDG Analysis:");
        println!("  Nodes: {}", pdg_results.nodes.len());
        println!("  Dependencies: {}", pdg_results.edges.len());
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    #[test]
    fn test_handle_cfg_command() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("test.rs");
        fs::write(&file, "fn main() { if true { } }").unwrap();

        let result = handle_cfg_command(&file, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_dfg_command() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("test.rs");
        fs::write(&file, "fn main() { let x = 1; let y = x; }").unwrap();

        let result = handle_dfg_command(&file, None);
        assert!(result.is_ok());
    }
}
