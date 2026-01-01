//! Test Fixtures Module
//!
//! Provides reusable test fixtures for consistent testing across the codebase.

use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// Test fixture for a temporary directory with sample files
pub struct TestWorkspace {
    pub dir: TempDir,
    pub files: Vec<PathBuf>,
}

impl TestWorkspace {
    /// Create a new test workspace with default files
    pub fn new() -> Self {
        let dir = TempDir::new().expect("Failed to create temp dir");
        let mut files = Vec::new();

        // Create sample Rust file
        let rust_file = dir.path().join("main.rs");
        fs::write(
            &rust_file,
            r#"fn main() {
    println!("Hello, world!");
}

fn test_function() {
    let x = 42;
    println!("Test: {}", x);
}

#[test]
fn test_example() {
    assert_eq!(2 + 2, 4);
}
"#,
        )
        .expect("Failed to write Rust file");
        files.push(rust_file);

        // Create sample Python file
        let python_file = dir.path().join("script.py");
        fs::write(
            &python_file,
            r#"def main():
    print("Hello, world!")

def test_function():
    x = 42
    print(f"Test: {x}")

if __name__ == "__main__":
    main()
"#,
        )
        .expect("Failed to write Python file");
        files.push(python_file);

        // Create sample JavaScript file
        let js_file = dir.path().join("app.js");
        fs::write(
            &js_file,
            r#"function main() {
    console.log("Hello, world!");
}

function testFunction() {
    const x = 42;
    console.log(`Test: ${x}`);
}

main();
"#,
        )
        .expect("Failed to write JavaScript file");
        files.push(js_file);

        Self { dir, files }
    }

    /// Create a workspace with custom files
    pub fn with_files(files: &[(&str, &str)]) -> Self {
        let dir = TempDir::new().expect("Failed to create temp dir");
        let mut file_paths = Vec::new();

        for (name, content) in files {
            let file_path = dir.path().join(name);
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent).expect("Failed to create parent dir");
            }
            fs::write(&file_path, content).expect("Failed to write file");
            file_paths.push(file_path);
        }

        Self {
            dir,
            files: file_paths,
        }
    }

    /// Get the workspace path
    pub fn path(&self) -> &Path {
        self.dir.path()
    }

    /// Add a file to the workspace
    pub fn add_file(&mut self, name: &str, content: &str) -> PathBuf {
        let file_path = self.dir.path().join(name);
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).expect("Failed to create parent dir");
        }
        fs::write(&file_path, content).expect("Failed to write file");
        self.files.push(file_path.clone());
        file_path
    }

    /// Create a subdirectory
    pub fn create_subdir(&self, name: &str) -> PathBuf {
        let subdir = self.dir.path().join(name);
        fs::create_dir_all(&subdir).expect("Failed to create subdirectory");
        subdir
    }
}

impl Default for TestWorkspace {
    fn default() -> Self {
        Self::new()
    }
}

/// Sample code snippets for testing
pub mod samples {
    pub const RUST_FUNCTION: &str = r#"
fn calculate_sum(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn test_calculate_sum() {
    assert_eq!(calculate_sum(2, 2), 4);
}
"#;

    pub const PYTHON_CLASS: &str = r#"
class Calculator:
    def __init__(self):
        self.result = 0
    
    def add(self, x, y):
        self.result = x + y
        return self.result
    
    def test_add(self):
        assert self.add(2, 2) == 4
"#;

    pub const JAVASCRIPT_MODULE: &str = r#"
export class Calculator {
    constructor() {
        this.result = 0;
    }
    
    add(x, y) {
        this.result = x + y;
        return this.result;
    }
}

export function test() {
    const calc = new Calculator();
    console.assert(calc.add(2, 2) === 4);
}
"#;

    pub const COMPLEX_RUST: &str = r#"
use std::collections::HashMap;

pub struct Cache<K, V> {
    data: HashMap<K, V>,
    capacity: usize,
}

impl<K: Eq + std::hash::Hash, V> Cache<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: HashMap::new(),
            capacity,
        }
    }
    
    pub fn insert(&mut self, key: K, value: V) {
        if self.data.len() >= self.capacity {
            // Simple eviction: clear all
            self.data.clear();
        }
        self.data.insert(key, value);
    }
    
    pub fn get(&self, key: &K) -> Option<&V> {
        self.data.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cache() {
        let mut cache = Cache::new(2);
        cache.insert("key1", "value1");
        cache.insert("key2", "value2");
        assert_eq!(cache.get(&"key1"), Some(&"value1"));
    }
}
"#;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workspace_creation() {
        let workspace = TestWorkspace::new();
        assert!(workspace.path().exists());
        assert_eq!(workspace.files.len(), 3);
    }

    #[test]
    fn test_workspace_with_custom_files() {
        let workspace = TestWorkspace::with_files(&[
            ("test1.txt", "content1"),
            ("test2.txt", "content2"),
        ]);
        assert_eq!(workspace.files.len(), 2);
    }

    #[test]
    fn test_add_file() {
        let mut workspace = TestWorkspace::new();
        let file = workspace.add_file("new.txt", "new content");
        assert!(file.exists());
        assert_eq!(workspace.files.len(), 4);
    }

    #[test]
    fn test_create_subdir() {
        let workspace = TestWorkspace::new();
        let subdir = workspace.create_subdir("subdir");
        assert!(subdir.exists());
        assert!(subdir.is_dir());
    }
}
