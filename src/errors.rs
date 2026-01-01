//! Error Types Module
//!
//! Defines custom error types for better error handling and debugging.

use std::path::PathBuf;
use thiserror::Error;

/// Errors that can occur during search operations
#[derive(Error, Debug)]
pub enum SearchError {
    /// Invalid regex pattern provided
    #[error("Invalid regex pattern: {pattern}")]
    InvalidPattern {
        pattern: String,
        #[source]
        source: regex::Error,
    },

    /// File not found at the specified path
    #[error("File not found: {path}")]
    FileNotFound { path: PathBuf },

    /// Directory not found at the specified path
    #[error("Directory not found: {path}")]
    DirectoryNotFound { path: PathBuf },

    /// I/O error occurred during file operations
    #[error("I/O error: {message}")]
    IoError {
        message: String,
        #[source]
        source: std::io::Error,
    },

    /// Cache operation failed
    #[error("Cache error: {message}")]
    CacheError { message: String },

    /// Search operation was cancelled
    #[error("Search cancelled")]
    Cancelled,

    /// Maximum results limit exceeded
    #[error("Maximum results limit exceeded: {limit}")]
    MaxResultsExceeded { limit: usize },

    /// Invalid search options provided
    #[error("Invalid search options: {message}")]
    InvalidOptions { message: String },
}

/// Errors that can occur during code analysis operations
#[derive(Error, Debug)]
pub enum AnalysisError {
    /// File parsing failed
    #[error("Failed to parse file: {path}")]
    ParseError {
        path: PathBuf,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// Unsupported file type
    #[error("Unsupported file type: {extension}")]
    UnsupportedFileType { extension: String },

    /// Analysis failed due to invalid input
    #[error("Invalid input: {message}")]
    InvalidInput { message: String },

    /// Complexity calculation failed
    #[error("Complexity calculation failed: {message}")]
    ComplexityError { message: String },

    /// I/O error during analysis
    #[error("I/O error during analysis: {message}")]
    IoError {
        message: String,
        #[source]
        source: std::io::Error,
    },
}

/// Errors that can occur during graph operations
#[derive(Error, Debug)]
pub enum GraphError {
    /// Failed to build graph
    #[error("Failed to build graph: {message}")]
    BuildError { message: String },

    /// Invalid graph structure
    #[error("Invalid graph structure: {message}")]
    InvalidStructure { message: String },

    /// Node not found in graph
    #[error("Node not found: {node_id}")]
    NodeNotFound { node_id: String },

    /// Cycle detected in graph
    #[error("Cycle detected in graph")]
    CycleDetected,

    /// Graph export failed
    #[error("Failed to export graph: {format}")]
    ExportError { format: String },
}

/// Errors that can occur during remote operations
#[derive(Error, Debug)]
pub enum RemoteError {
    /// Network request failed
    #[error("Network request failed: {url}")]
    NetworkError {
        url: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// Authentication failed
    #[error("Authentication failed")]
    AuthenticationFailed,

    /// API rate limit exceeded
    #[error("API rate limit exceeded")]
    RateLimitExceeded,

    /// Repository not found
    #[error("Repository not found: {repo}")]
    RepositoryNotFound { repo: String },

    /// Clone operation failed
    #[error("Failed to clone repository: {repo}")]
    CloneFailed { repo: String },
}

// Conversion implementations for common error types

impl From<std::io::Error> for SearchError {
    fn from(err: std::io::Error) -> Self {
        SearchError::IoError {
            message: err.to_string(),
            source: err,
        }
    }
}

impl From<regex::Error> for SearchError {
    fn from(err: regex::Error) -> Self {
        SearchError::InvalidPattern {
            pattern: String::new(),
            source: err,
        }
    }
}

impl From<std::io::Error> for AnalysisError {
    fn from(err: std::io::Error) -> Self {
        AnalysisError::IoError {
            message: err.to_string(),
            source: err,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_error_display() {
        let err = SearchError::FileNotFound {
            path: PathBuf::from("/test/file.rs"),
        };
        assert_eq!(err.to_string(), "File not found: /test/file.rs");
    }

    #[test]
    fn test_analysis_error_display() {
        let err = AnalysisError::UnsupportedFileType {
            extension: "xyz".to_string(),
        };
        assert_eq!(err.to_string(), "Unsupported file type: xyz");
    }

    #[test]
    fn test_graph_error_display() {
        let err = GraphError::NodeNotFound {
            node_id: "node123".to_string(),
        };
        assert_eq!(err.to_string(), "Node not found: node123");
    }

    #[test]
    fn test_remote_error_display() {
        let err = RemoteError::RepositoryNotFound {
            repo: "user/repo".to_string(),
        };
        assert_eq!(err.to_string(), "Repository not found: user/repo");
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let search_err: SearchError = io_err.into();
        assert!(matches!(search_err, SearchError::IoError { .. }));
    }

    #[test]
    fn test_error_source_chain() {
        let regex_err = regex::Regex::new("[").unwrap_err();
        let search_err = SearchError::InvalidPattern {
            pattern: "[".to_string(),
            source: regex_err,
        };
        
        assert!(search_err.source().is_some());
    }
}
