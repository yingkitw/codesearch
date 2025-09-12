use std::fs;
use tempfile::TempDir;

// Note: This would require adding tempfile as a dev dependency
// For now, we'll create a simple test structure

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    fn create_test_files() -> TempDir {
        let temp_dir = TempDir::new().unwrap();
        let test_dir = temp_dir.path();

        // Create test files
        fs::write(test_dir.join("test.rs"), "fn main() {\n    println!(\"Hello, world!\");\n}").unwrap();
        fs::write(test_dir.join("test.py"), "def hello():\n    print('Hello, world!')\n").unwrap();
        fs::write(test_dir.join("test.js"), "function hello() {\n    console.log('Hello, world!');\n}\n").unwrap();
        fs::write(test_dir.join("README.md"), "# Test Project\nThis is a test project.\n").unwrap();

        // Create subdirectory
        fs::create_dir(test_dir.join("src")).unwrap();
        fs::write(test_dir.join("src/lib.rs"), "pub fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n").unwrap();

        temp_dir
    }

    #[test]
    fn test_search_basic() {
        let temp_dir = create_test_files();
        let output = Command::new("cargo")
            .args(&["run", "--", "search", "Hello", "--path", temp_dir.path().to_str().unwrap()])
            .output()
            .unwrap();

        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("Hello, world!"));
    }

    #[test]
    fn test_search_with_extensions() {
        let temp_dir = create_test_files();
        let output = Command::new("cargo")
            .args(&["run", "--", "search", "hello", "--path", temp_dir.path().to_str().unwrap(), "--extensions", "rs,py", "--ignore-case"])
            .output()
            .unwrap();

        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).unwrap();
        // Check that we get results from .rs and .py files but not .js
        assert!(stdout.contains(".rs"));
        assert!(stdout.contains(".py"));
        assert!(!stdout.contains(".js"));
    }

    #[test]
    fn test_search_case_insensitive() {
        let temp_dir = create_test_files();
        let output = Command::new("cargo")
            .args(&["run", "--", "search", "hello", "--path", temp_dir.path().to_str().unwrap(), "--ignore-case"])
            .output()
            .unwrap();

        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("Hello, world!"));
    }

    #[test]
    fn test_search_regex() {
        let temp_dir = create_test_files();
        let output = Command::new("cargo")
            .args(&["run", "--", "search", r"fn\s+\w+", "--path", temp_dir.path().to_str().unwrap(), "--extensions", "rs"])
            .output()
            .unwrap();

        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("fn main"));
        assert!(stdout.contains("fn add"));
    }

    #[test]
    fn test_list_files() {
        let temp_dir = create_test_files();
        let output = Command::new("cargo")
            .args(&["run", "--", "files", "--path", temp_dir.path().to_str().unwrap(), "--extensions", "rs"])
            .output()
            .unwrap();

        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains(".rs"));
        assert!(stdout.contains("src"));
    }

    #[test]
    fn test_search_json_output() {
        let temp_dir = create_test_files();
        let output = Command::new("cargo")
            .args(&["run", "--", "search", "Hello", "--path", temp_dir.path().to_str().unwrap(), "--format", "json"])
            .output()
            .unwrap();

        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("\"file\""));
        assert!(stdout.contains("\"line_number\""));
        assert!(stdout.contains("\"content\""));
    }
}
