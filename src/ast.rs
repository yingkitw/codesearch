//! AST-Based Code Analysis Module
//!
//! Provides syntax tree analysis for precise code structure understanding.

use serde::{Deserialize, Serialize};
use std::path::Path;
use tree_sitter::{Language, Parser};

fn get_rust_language() -> Language {
    tree_sitter_rust::language()
}

fn get_python_language() -> Language {
    tree_sitter_python::language()
}

fn get_javascript_language() -> Language {
    tree_sitter_javascript::language()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstNode {
    pub kind: String,
    pub name: String,
    pub start_line: usize,
    pub end_line: usize,
    pub children: Vec<AstNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstAnalysis {
    pub functions: Vec<FunctionInfo>,
    pub classes: Vec<ClassInfo>,
    pub imports: Vec<ImportInfo>,
    pub variables: Vec<VariableInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionInfo {
    pub name: String,
    pub line: usize,
    pub parameters: Vec<String>,
    pub return_type: Option<String>,
    pub is_async: bool,
    pub is_public: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassInfo {
    pub name: String,
    pub line: usize,
    pub methods: Vec<String>,
    pub fields: Vec<String>,
    pub is_public: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportInfo {
    pub module: String,
    pub line: usize,
    pub items: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableInfo {
    pub name: String,
    pub line: usize,
    pub is_const: bool,
    pub is_mutable: bool,
}

pub struct AstParser {
    parser: Parser,
    #[allow(dead_code)]
    language: Language,
}

impl AstParser {
    pub fn new_rust() -> Result<Self, Box<dyn std::error::Error>> {
        let mut parser = Parser::new();
        let language = get_rust_language();
        parser.set_language(language)?;
        Ok(Self { parser, language })
    }

    pub fn new_python() -> Result<Self, Box<dyn std::error::Error>> {
        let mut parser = Parser::new();
        let language = get_python_language();
        parser.set_language(language)?;
        Ok(Self { parser, language })
    }

    pub fn new_javascript() -> Result<Self, Box<dyn std::error::Error>> {
        let mut parser = Parser::new();
        let language = get_javascript_language();
        parser.set_language(language)?;
        Ok(Self { parser, language })
    }

    pub fn for_extension(ext: &str) -> Result<Self, Box<dyn std::error::Error>> {
        match ext {
            "rs" => Self::new_rust(),
            "py" => Self::new_python(),
            "js" | "ts" => Self::new_javascript(),
            _ => Err("Unsupported language".into()),
        }
    }

    pub fn parse_file(&mut self, path: &Path) -> Result<AstAnalysis, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        self.parse_content(&content)
    }

    pub fn parse_content(&mut self, content: &str) -> Result<AstAnalysis, Box<dyn std::error::Error>> {
        let tree = self.parser.parse(content, None)
            .ok_or("Failed to parse content")?;

        let root_node = tree.root_node();
        
        let mut analysis = AstAnalysis {
            functions: Vec::new(),
            classes: Vec::new(),
            imports: Vec::new(),
            variables: Vec::new(),
        };

        self.extract_functions(&root_node, content, &mut analysis.functions);
        self.extract_classes(&root_node, content, &mut analysis.classes);
        self.extract_imports(&root_node, content, &mut analysis.imports);
        self.extract_variables(&root_node, content, &mut analysis.variables);

        Ok(analysis)
    }

    fn extract_functions(&self, node: &tree_sitter::Node, source: &str, functions: &mut Vec<FunctionInfo>) {
        let mut cursor = node.walk();
        
        for child in node.children(&mut cursor) {
            let kind = child.kind();
            
            if kind == "function_item" || kind == "function_declaration" || kind == "function_definition" {
                if let Some(name_node) = child.child_by_field_name("name") {
                    let name = name_node.utf8_text(source.as_bytes()).unwrap_or("").to_string();
                    let line = child.start_position().row + 1;
                    
                    let parameters = self.extract_parameters(&child, source);
                    let is_async = source[child.byte_range()].contains("async");
                    let is_public = source[child.byte_range()].contains("pub");
                    
                    functions.push(FunctionInfo {
                        name,
                        line,
                        parameters,
                        return_type: None,
                        is_async,
                        is_public,
                    });
                }
            }
            
            self.extract_functions(&child, source, functions);
        }
    }

    fn extract_classes(&self, node: &tree_sitter::Node, source: &str, classes: &mut Vec<ClassInfo>) {
        let mut cursor = node.walk();
        
        for child in node.children(&mut cursor) {
            let kind = child.kind();
            
            if kind == "struct_item" || kind == "class_declaration" || kind == "class_definition" {
                if let Some(name_node) = child.child_by_field_name("name") {
                    let name = name_node.utf8_text(source.as_bytes()).unwrap_or("").to_string();
                    let line = child.start_position().row + 1;
                    let is_public = source[child.byte_range()].contains("pub");
                    
                    classes.push(ClassInfo {
                        name,
                        line,
                        methods: Vec::new(),
                        fields: Vec::new(),
                        is_public,
                    });
                }
            }
            
            self.extract_classes(&child, source, classes);
        }
    }

    fn extract_imports(&self, node: &tree_sitter::Node, source: &str, imports: &mut Vec<ImportInfo>) {
        let mut cursor = node.walk();
        
        for child in node.children(&mut cursor) {
            let kind = child.kind();
            
            if kind == "use_declaration" || kind == "import_statement" || kind == "import_from_statement" {
                let line = child.start_position().row + 1;
                let text = child.utf8_text(source.as_bytes()).unwrap_or("");
                
                imports.push(ImportInfo {
                    module: text.to_string(),
                    line,
                    items: Vec::new(),
                });
            }
            
            self.extract_imports(&child, source, imports);
        }
    }

    fn extract_variables(&self, node: &tree_sitter::Node, source: &str, variables: &mut Vec<VariableInfo>) {
        let mut cursor = node.walk();
        
        for child in node.children(&mut cursor) {
            let kind = child.kind();
            
            if kind == "let_declaration" || kind == "const_item" || kind == "variable_declaration" {
                if let Some(name_node) = child.child_by_field_name("pattern").or_else(|| child.child_by_field_name("name")) {
                    let name = name_node.utf8_text(source.as_bytes()).unwrap_or("").to_string();
                    let line = child.start_position().row + 1;
                    let text = child.utf8_text(source.as_bytes()).unwrap_or("");
                    
                    let is_const = text.contains("const");
                    let is_mutable = text.contains("mut");
                    
                    variables.push(VariableInfo {
                        name,
                        line,
                        is_const,
                        is_mutable,
                    });
                }
            }
            
            self.extract_variables(&child, source, variables);
        }
    }

    fn extract_parameters(&self, node: &tree_sitter::Node, source: &str) -> Vec<String> {
        let mut parameters = Vec::new();
        
        if let Some(params_node) = node.child_by_field_name("parameters") {
            let mut cursor = params_node.walk();
            for param in params_node.children(&mut cursor) {
                if param.kind() == "parameter" {
                    if let Some(name_node) = param.child_by_field_name("pattern").or_else(|| param.child_by_field_name("name")) {
                        let name = name_node.utf8_text(source.as_bytes()).unwrap_or("").to_string();
                        parameters.push(name);
                    }
                }
            }
        }
        
        parameters
    }

    pub fn find_function_calls(&mut self, content: &str, function_name: &str) -> Vec<usize> {
        let tree = match self.parser.parse(content, None) {
            Some(t) => t,
            None => return Vec::new(),
        };

        let root_node = tree.root_node();
        let mut lines = Vec::new();
        
        self.find_calls_recursive(&root_node, content, function_name, &mut lines);
        
        lines
    }

    fn find_calls_recursive(&self, node: &tree_sitter::Node, source: &str, function_name: &str, lines: &mut Vec<usize>) {
        let mut cursor = node.walk();
        
        for child in node.children(&mut cursor) {
            if child.kind() == "call_expression" {
                if let Some(func_node) = child.child_by_field_name("function") {
                    let name = func_node.utf8_text(source.as_bytes()).unwrap_or("");
                    if name == function_name {
                        lines.push(child.start_position().row + 1);
                    }
                }
            }
            
            self.find_calls_recursive(&child, source, function_name, lines);
        }
    }
}

pub fn analyze_file(path: &Path) -> Result<AstAnalysis, Box<dyn std::error::Error>> {
    let ext = path.extension()
        .and_then(|s| s.to_str())
        .ok_or("No file extension")?;
    
    let mut parser = AstParser::for_extension(ext)?;
    parser.parse_file(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_parse_rust_functions() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "fn main() {{}}\nfn helper() {{}}").unwrap();
        
        let mut parser = AstParser::new_rust().unwrap();
        let analysis = parser.parse_file(file.path()).unwrap();
        
        assert_eq!(analysis.functions.len(), 2);
        assert!(analysis.functions.iter().any(|f| f.name == "main"));
        assert!(analysis.functions.iter().any(|f| f.name == "helper"));
    }

    #[test]
    fn test_parse_rust_structs() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "struct Config {{}}\nstruct Data {{}}").unwrap();
        
        let mut parser = AstParser::new_rust().unwrap();
        let analysis = parser.parse_file(file.path()).unwrap();
        
        assert_eq!(analysis.classes.len(), 2);
    }
}
