//! Incremental Indexing Module
//!
//! Provides incremental indexing for large codebases with persistent storage.

use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::SystemTime;
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexEntry {
    pub path: String,
    pub size: u64,
    pub modified: SystemTime,
    pub lines: usize,
    pub functions: Vec<String>,
    pub classes: Vec<String>,
    pub imports: Vec<String>,
}

#[derive(Debug)]
pub struct CodeIndex {
    entries: Arc<DashMap<String, IndexEntry>>,
    index_path: PathBuf,
}

impl CodeIndex {
    pub fn new(index_path: PathBuf) -> Self {
        let entries = Arc::new(DashMap::new());
        
        if index_path.exists() {
            if let Ok(data) = fs::read_to_string(&index_path) {
                if let Ok(loaded_entries) = serde_json::from_str::<HashMap<String, IndexEntry>>(&data) {
                    for (key, value) in loaded_entries {
                        entries.insert(key, value);
                    }
                }
            }
        }
        
        Self { entries, index_path }
    }

    pub fn index_file(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let metadata = fs::metadata(path)?;
        let modified = metadata.modified()?;
        let size = metadata.len();
        
        let path_str = path.to_string_lossy().to_string();
        
        if let Some(entry) = self.entries.get(&path_str) {
            if entry.modified == modified && entry.size == size {
                return Ok(());
            }
        }
        
        let content = fs::read_to_string(path)?;
        let lines = content.lines().count();
        
        let functions = extract_functions(&content, path);
        let classes = extract_classes(&content, path);
        let imports = extract_imports(&content, path);
        
        let entry = IndexEntry {
            path: path_str.clone(),
            size,
            modified,
            lines,
            functions,
            classes,
            imports,
        };
        
        self.entries.insert(path_str, entry);
        Ok(())
    }

    pub fn index_directory(&self, path: &Path, extensions: Option<&[String]>, exclude: Option<&[String]>) -> Result<(), Box<dyn std::error::Error>> {
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

        use rayon::prelude::*;
        files.par_iter().for_each(|file| {
            let _ = self.index_file(file);
        });

        Ok(())
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let entries_map: HashMap<String, IndexEntry> = self.entries
            .iter()
            .map(|entry| (entry.key().clone(), entry.value().clone()))
            .collect();
        
        let json = serde_json::to_string_pretty(&entries_map)?;
        
        if let Some(parent) = self.index_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        fs::write(&self.index_path, json)?;
        Ok(())
    }

    pub fn get(&self, path: &str) -> Option<IndexEntry> {
        self.entries.get(path).map(|e| e.value().clone())
    }

    pub fn search_functions(&self, pattern: &str) -> Vec<(String, String)> {
        let regex = match regex::Regex::new(pattern) {
            Ok(r) => r,
            Err(_) => return Vec::new(),
        };
        let mut results = Vec::new();

        for entry in self.entries.iter() {
            for func in &entry.value().functions {
                if regex.is_match(func) {
                    results.push((entry.value().path.clone(), func.clone()));
                }
            }
        }

        results
    }

    pub fn search_classes(&self, pattern: &str) -> Vec<(String, String)> {
        let regex = match regex::Regex::new(pattern) {
            Ok(r) => r,
            Err(_) => return Vec::new(),
        };
        let mut results = Vec::new();

        for entry in self.entries.iter() {
            for class in &entry.value().classes {
                if regex.is_match(class) {
                    results.push((entry.value().path.clone(), class.clone()));
                }
            }
        }

        results
    }

    pub fn get_stats(&self) -> IndexStats {
        let total_files = self.entries.len();
        let total_lines: usize = self.entries.iter().map(|e| e.value().lines).sum();
        let total_functions: usize = self.entries.iter().map(|e| e.value().functions.len()).sum();
        let total_classes: usize = self.entries.iter().map(|e| e.value().classes.len()).sum();
        
        IndexStats {
            total_files,
            total_lines,
            total_functions,
            total_classes,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexStats {
    pub total_files: usize,
    pub total_lines: usize,
    pub total_functions: usize,
    pub total_classes: usize,
}

fn extract_functions(content: &str, path: &Path) -> Vec<String> {
    let mut functions = Vec::new();
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
    
    let patterns = match ext {
        "rs" => vec![r"fn\s+(\w+)"],
        "py" => vec![r"def\s+(\w+)"],
        "js" | "ts" => vec![r"function\s+(\w+)", r"const\s+(\w+)\s*=\s*\(", r"(\w+)\s*:\s*\([^)]*\)\s*=>"],
        "go" => vec![r"func\s+(\w+)"],
        "java" | "kt" => vec![r"(?:public|private|protected)?\s*(?:static)?\s*\w+\s+(\w+)\s*\("],
        _ => vec![],
    };
    
    for pattern in patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            for line in content.lines() {
                if let Some(caps) = re.captures(line) {
                    if let Some(name) = caps.get(1) {
                        functions.push(name.as_str().to_string());
                    }
                }
            }
        }
    }
    
    functions
}

fn extract_classes(content: &str, path: &Path) -> Vec<String> {
    let mut classes = Vec::new();
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
    
    let patterns = match ext {
        "rs" => vec![r"struct\s+(\w+)", r"enum\s+(\w+)", r"trait\s+(\w+)"],
        "py" => vec![r"class\s+(\w+)"],
        "js" | "ts" => vec![r"class\s+(\w+)"],
        "go" => vec![r"type\s+(\w+)\s+struct"],
        "java" | "kt" => vec![r"(?:public|private)?\s*class\s+(\w+)", r"interface\s+(\w+)"],
        _ => vec![],
    };
    
    for pattern in patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            for line in content.lines() {
                if let Some(caps) = re.captures(line) {
                    if let Some(name) = caps.get(1) {
                        classes.push(name.as_str().to_string());
                    }
                }
            }
        }
    }
    
    classes
}

fn extract_imports(content: &str, path: &Path) -> Vec<String> {
    let mut imports = Vec::new();
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
    
    let patterns = match ext {
        "rs" => vec![r"use\s+([\w:]+)"],
        "py" => vec![r"import\s+([\w.]+)", r"from\s+([\w.]+)\s+import"],
        "js" | "ts" => vec![r#"import\s+.*\s+from\s+['"]([^'"]+)['"]"#, r#"require\(['"]([^'"]+)['"]\)"#],
        "go" => vec![r#"import\s+"([^"]+)""#],
        "java" | "kt" => vec![r"import\s+([\w.]+)"],
        _ => vec![],
    };
    
    for pattern in patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            for line in content.lines() {
                if let Some(caps) = re.captures(line) {
                    if let Some(name) = caps.get(1) {
                        imports.push(name.as_str().to_string());
                    }
                }
            }
        }
    }
    
    imports
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_index_creation() {
        let dir = tempdir().unwrap();
        let index_path = dir.path().join("index.json");
        let index = CodeIndex::new(index_path);
        
        assert_eq!(index.entries.len(), 0);
    }

    #[test]
    fn test_index_file() {
        let dir = tempdir().unwrap();
        let index_path = dir.path().join("index.json");
        let index = CodeIndex::new(index_path);
        
        let test_file = dir.path().join("test.rs");
        let mut file = fs::File::create(&test_file).unwrap();
        writeln!(file, "fn main() {{\n    println!(\"Hello\");\n}}").unwrap();
        
        index.index_file(&test_file).unwrap();
        
        let entry = index.get(&test_file.to_string_lossy()).unwrap();
        assert_eq!(entry.lines, 3);
        assert!(entry.functions.contains(&"main".to_string()));
    }

    #[test]
    fn test_extract_functions_rust() {
        let content = "fn main() {}\nfn helper() {}";
        let path = Path::new("test.rs");
        let functions = extract_functions(content, path);
        
        assert_eq!(functions.len(), 2);
        assert!(functions.contains(&"main".to_string()));
        assert!(functions.contains(&"helper".to_string()));
    }

    #[test]
    fn test_extract_classes_rust() {
        let content = "struct Config {}\nenum Status {}";
        let path = Path::new("test.rs");
        let classes = extract_classes(content, path);
        
        assert_eq!(classes.len(), 2);
        assert!(classes.contains(&"Config".to_string()));
        assert!(classes.contains(&"Status".to_string()));
    }
}
