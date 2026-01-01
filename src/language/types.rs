//! Language Type Definitions
//!
//! Data structures for language metadata.

/// Supported programming language with its metadata
#[derive(Debug, Clone)]
pub struct LanguageInfo {
    pub name: &'static str,
    pub extensions: &'static [&'static str],
    pub function_patterns: &'static [&'static str],
    pub class_patterns: &'static [&'static str],
    pub comment_patterns: &'static [&'static str],
    #[allow(dead_code)]
    pub import_patterns: &'static [&'static str],
}
