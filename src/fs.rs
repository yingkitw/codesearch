//! File System Abstraction Module
//!
//! Provides trait-based file system operations for dependency injection and testing.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Trait for file system operations
///
/// This trait abstracts file system operations to enable dependency injection
/// and make code more testable by allowing mock implementations.
///
/// # Examples
///
/// ```
/// use codesearch::fs::{FileSystem, RealFileSystem};
/// use std::path::Path;
///
/// let fs = RealFileSystem;
/// let content = fs.read_to_string(Path::new("Cargo.toml"));
/// assert!(content.is_ok());
/// ```
pub trait FileSystem: Send + Sync {
    /// Read the entire contents of a file into a string
    fn read_to_string(&self, path: &Path) -> io::Result<String>;

    /// Read the entire contents of a file into a byte vector
    fn read(&self, path: &Path) -> io::Result<Vec<u8>>;

    /// Write a string to a file, creating it if it doesn't exist
    fn write(&self, path: &Path, contents: &str) -> io::Result<()>;

    /// Check if a path exists
    fn exists(&self, path: &Path) -> bool;

    /// Check if a path is a file
    fn is_file(&self, path: &Path) -> bool;

    /// Check if a path is a directory
    fn is_dir(&self, path: &Path) -> bool;

    /// Read directory entries
    fn read_dir(&self, path: &Path) -> io::Result<Vec<PathBuf>>;

    /// Get file metadata
    fn metadata(&self, path: &Path) -> io::Result<fs::Metadata>;

    /// Create a directory and all parent directories
    fn create_dir_all(&self, path: &Path) -> io::Result<()>;

    /// Remove a file
    fn remove_file(&self, path: &Path) -> io::Result<()>;
}

/// Real file system implementation
///
/// This implementation uses the standard library's file system operations.
#[derive(Debug, Clone, Copy, Default)]
pub struct RealFileSystem;

impl FileSystem for RealFileSystem {
    fn read_to_string(&self, path: &Path) -> io::Result<String> {
        fs::read_to_string(path)
    }

    fn read(&self, path: &Path) -> io::Result<Vec<u8>> {
        fs::read(path)
    }

    fn write(&self, path: &Path, contents: &str) -> io::Result<()> {
        fs::write(path, contents)
    }

    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    fn is_file(&self, path: &Path) -> bool {
        path.is_file()
    }

    fn is_dir(&self, path: &Path) -> bool {
        path.is_dir()
    }

    fn read_dir(&self, path: &Path) -> io::Result<Vec<PathBuf>> {
        fs::read_dir(path)?
            .map(|entry| entry.map(|e| e.path()))
            .collect()
    }

    fn metadata(&self, path: &Path) -> io::Result<fs::Metadata> {
        fs::metadata(path)
    }

    fn create_dir_all(&self, path: &Path) -> io::Result<()> {
        fs::create_dir_all(path)
    }

    fn remove_file(&self, path: &Path) -> io::Result<()> {
        fs::remove_file(path)
    }
}

/// Mock file system for testing
///
/// This implementation stores files in memory and allows testing without
/// touching the real file system.
///
/// # Examples
///
/// ```
/// use codesearch::fs::{FileSystem, MockFileSystem};
/// use std::path::Path;
///
/// let mut fs = MockFileSystem::new();
/// fs.add_file("test.txt", "Hello, World!");
///
/// let content = fs.read_to_string(Path::new("test.txt")).unwrap();
/// assert_eq!(content, "Hello, World!");
/// ```
#[derive(Debug, Clone, Default)]
pub struct MockFileSystem {
    files: std::collections::HashMap<PathBuf, Vec<u8>>,
}

impl MockFileSystem {
    /// Create a new empty mock file system
    pub fn new() -> Self {
        Self {
            files: std::collections::HashMap::new(),
        }
    }

    /// Add a file to the mock file system
    pub fn add_file(&mut self, path: impl Into<PathBuf>, contents: impl Into<Vec<u8>>) {
        self.files.insert(path.into(), contents.into());
    }

    /// Add a text file to the mock file system
    pub fn add_text_file(&mut self, path: impl Into<PathBuf>, contents: &str) {
        self.add_file(path, contents.as_bytes().to_vec());
    }

    /// Check if a file exists in the mock file system
    pub fn has_file(&self, path: &Path) -> bool {
        self.files.contains_key(path)
    }

    /// Get the number of files in the mock file system
    pub fn file_count(&self) -> usize {
        self.files.len()
    }
}

impl FileSystem for MockFileSystem {
    fn read_to_string(&self, path: &Path) -> io::Result<String> {
        self.files
            .get(path)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "File not found"))
            .and_then(|bytes| {
                String::from_utf8(bytes.clone())
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            })
    }

    fn read(&self, path: &Path) -> io::Result<Vec<u8>> {
        self.files
            .get(path)
            .cloned()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "File not found"))
    }

    fn write(&self, _path: &Path, _contents: &str) -> io::Result<()> {
        // Mock implementation - would need interior mutability for real use
        Ok(())
    }

    fn exists(&self, path: &Path) -> bool {
        self.files.contains_key(path)
    }

    fn is_file(&self, path: &Path) -> bool {
        self.files.contains_key(path)
    }

    fn is_dir(&self, _path: &Path) -> bool {
        false // Simplified for mock
    }

    fn read_dir(&self, _path: &Path) -> io::Result<Vec<PathBuf>> {
        Ok(self.files.keys().cloned().collect())
    }

    fn metadata(&self, path: &Path) -> io::Result<fs::Metadata> {
        if self.files.contains_key(path) {
            // Return real metadata from a temp file for simplicity
            fs::metadata("Cargo.toml")
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "File not found"))
        }
    }

    fn create_dir_all(&self, _path: &Path) -> io::Result<()> {
        Ok(())
    }

    fn remove_file(&self, _path: &Path) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_real_filesystem_read() {
        let fs = RealFileSystem;
        let result = fs.read_to_string(Path::new("Cargo.toml"));
        assert!(result.is_ok());
        assert!(result.unwrap().contains("codesearch"));
    }

    #[test]
    fn test_real_filesystem_exists() {
        let fs = RealFileSystem;
        assert!(fs.exists(Path::new("Cargo.toml")));
        assert!(!fs.exists(Path::new("nonexistent.txt")));
    }

    #[test]
    fn test_mock_filesystem() {
        let mut fs = MockFileSystem::new();
        fs.add_text_file("test.txt", "Hello, World!");

        assert!(fs.exists(Path::new("test.txt")));
        assert!(!fs.exists(Path::new("other.txt")));

        let content = fs.read_to_string(Path::new("test.txt")).unwrap();
        assert_eq!(content, "Hello, World!");
    }

    #[test]
    fn test_mock_filesystem_not_found() {
        let fs = MockFileSystem::new();
        let result = fs.read_to_string(Path::new("nonexistent.txt"));
        assert!(result.is_err());
    }

    #[test]
    fn test_mock_filesystem_multiple_files() {
        let mut fs = MockFileSystem::new();
        fs.add_text_file("file1.txt", "Content 1");
        fs.add_text_file("file2.txt", "Content 2");
        fs.add_text_file("file3.txt", "Content 3");

        assert_eq!(fs.file_count(), 3);
        assert!(fs.has_file(Path::new("file1.txt")));
        assert!(fs.has_file(Path::new("file2.txt")));
        assert!(fs.has_file(Path::new("file3.txt")));
    }
}
