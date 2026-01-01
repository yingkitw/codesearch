//! Language Utilities
//!
//! Helper functions for language detection and information retrieval.

use super::types::LanguageInfo;
use super::definitions::get_supported_languages;
use std::path::Path;

/// Get language info by file extension
pub fn get_language_by_extension(ext: &str) -> Option<LanguageInfo> {
    let ext_lower = ext.to_lowercase();
    get_supported_languages()
        .into_iter()
        .find(|lang| lang.extensions.iter().any(|e| e.to_lowercase() == ext_lower))
}

/// Get all supported file extensions
#[allow(dead_code)]
pub fn get_all_supported_extensions() -> Vec<&'static str> {
    get_supported_languages()
        .iter()
        .flat_map(|lang| lang.extensions.iter().copied())
        .collect()
}

/// Get language name by file path
#[allow(dead_code)]
pub fn get_language_name(file_path: &str) -> String {
    if let Some(ext) = Path::new(file_path).extension().and_then(|s| s.to_str()) {
        if let Some(lang) = get_language_by_extension(ext) {
            return lang.name.to_string();
        }
    }
    "Unknown".to_string()
}
