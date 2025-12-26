//! Configuration Module
//!
//! Handles configuration loading and defaults.

use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

/// Main configuration structure
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub search: SearchConfig,
    #[serde(default)]
    pub defaults: DefaultsConfig,
}

/// Search-specific configuration
#[derive(Debug, Clone, Deserialize)]
pub struct SearchConfig {
    #[serde(default = "default_fuzzy_threshold")]
    pub fuzzy_threshold: f64,
    #[serde(default = "default_max_results")]
    pub max_results: usize,
    #[serde(default = "default_ignore_case")]
    pub ignore_case: bool,
    #[serde(default = "default_show_line_numbers")]
    pub show_line_numbers: bool,
    #[serde(default = "default_format")]
    pub format: String,
    #[serde(default)]
    pub extensions: Option<Vec<String>>,
    #[serde(default)]
    pub exclude: Option<Vec<String>>,
    #[serde(default = "default_auto_exclude")]
    pub auto_exclude: bool,
    #[serde(default)]
    pub cache: bool,
    #[serde(default)]
    pub semantic: bool,
    #[serde(default)]
    pub rank: bool,
}

/// Default configuration values
#[derive(Debug, Clone, Deserialize)]
pub struct DefaultsConfig {
    #[serde(default)]
    pub exclude_dirs: Option<Vec<String>>,
}

fn default_fuzzy_threshold() -> f64 { 0.6 }
fn default_max_results() -> usize { 10 }
fn default_ignore_case() -> bool { true }
fn default_show_line_numbers() -> bool { true }
fn default_format() -> String { "text".to_string() }
fn default_auto_exclude() -> bool { true }

impl Default for SearchConfig {
    fn default() -> Self {
        SearchConfig {
            fuzzy_threshold: default_fuzzy_threshold(),
            max_results: default_max_results(),
            ignore_case: default_ignore_case(),
            show_line_numbers: default_show_line_numbers(),
            format: default_format(),
            extensions: None,
            exclude: None,
            auto_exclude: default_auto_exclude(),
            cache: false,
            semantic: false,
            rank: false,
        }
    }
}

impl Default for DefaultsConfig {
    fn default() -> Self {
        DefaultsConfig {
            exclude_dirs: None,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            search: SearchConfig::default(),
            defaults: DefaultsConfig::default(),
        }
    }
}

/// Load configuration from file
pub fn load_config() -> Config {
    // Try to load from current directory first, then home directory
    let mut config_paths = vec![
        PathBuf::from(".codesearchrc"),
        PathBuf::from(".codesearch.toml"),
    ];
    
    // Add home directory paths if HOME is set
    if let Ok(home) = std::env::var("HOME") {
        config_paths.push(PathBuf::from(&home).join(".codesearchrc"));
        config_paths.push(PathBuf::from(&home).join(".codesearch.toml"));
    }

    for path in config_paths {
        if path.exists() {
            match fs::read_to_string(&path) {
                Ok(content) => {
                    match toml::from_str::<Config>(&content) {
                        Ok(config) => {
                            if std::env::var("CODESEARCH_DEBUG").is_ok() {
                                eprintln!("Loaded config from: {}", path.display());
                            }
                            return config;
                        }
                        Err(e) => {
                            if std::env::var("CODESEARCH_DEBUG").is_ok() {
                                eprintln!("Failed to parse config from {}: {}", path.display(), e);
                            }
                        }
                    }
                }
                Err(_) => continue,
            }
        }
    }

    Config::default()
}

/// Get default directories to exclude (common build artifacts)
pub fn get_default_exclude_dirs() -> Vec<String> {
    vec![
        "target".to_string(),
        "node_modules".to_string(),
        "dist".to_string(),
        "build".to_string(),
        ".git".to_string(),
        ".cargo".to_string(),
        "__pycache__".to_string(),
        ".venv".to_string(),
        "venv".to_string(),
        ".next".to_string(),
        ".nuxt".to_string(),
        "vendor".to_string(),
        ".gradle".to_string(),
        ".idea".to_string(),
        ".vscode".to_string(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.search.fuzzy_threshold, 0.6);
        assert_eq!(config.search.max_results, 10);
        assert!(config.search.ignore_case);
        assert!(config.search.show_line_numbers);
    }

    #[test]
    fn test_get_default_exclude_dirs() {
        let dirs = get_default_exclude_dirs();
        assert!(dirs.contains(&"target".to_string()));
        assert!(dirs.contains(&"node_modules".to_string()));
        assert!(dirs.contains(&".git".to_string()));
    }

    #[test]
    fn test_load_config_missing_file() {
        // Should return default config when file doesn't exist
        let config = load_config();
        assert_eq!(config.search.max_results, 10);
    }
}

