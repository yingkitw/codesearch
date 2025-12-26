//! Cache Module
//!
//! Provides intelligent caching for faster repeated searches.

use dashmap::DashMap;
use std::collections::HashMap;
use std::sync::OnceLock;
use std::time::SystemTime;

use crate::types::SearchResult;

/// Global search cache instance
static SEARCH_CACHE: OnceLock<SearchCache> = OnceLock::new();

/// Get the global search cache
pub fn get_search_cache() -> &'static SearchCache {
    SEARCH_CACHE.get_or_init(SearchCache::new)
}

/// Cache for search results and file modification times
pub struct SearchCache {
    results: DashMap<String, Vec<SearchResult>>,
    file_mtimes: DashMap<String, SystemTime>,
}

impl SearchCache {
    /// Create a new search cache
    pub fn new() -> Self {
        SearchCache {
            results: DashMap::new(),
            file_mtimes: DashMap::new(),
        }
    }

    /// Generate a cache key from search parameters
    pub fn get_cache_key(
        &self,
        query: &str,
        path: &str,
        extensions: Option<&[String]>,
        fuzzy: bool,
    ) -> String {
        let ext_str = extensions
            .map(|e| e.join(","))
            .unwrap_or_else(|| "all".to_string());
        format!("{}:{}:{}:{}", query, path, ext_str, fuzzy)
    }

    /// Get cached results if available
    pub fn get(&self, key: &str) -> Option<Vec<SearchResult>> {
        self.results.get(key).map(|v| v.clone())
    }

    /// Store results in cache
    pub fn set(&self, key: String, results: Vec<SearchResult>) {
        self.results.insert(key, results);
    }

    /// Check if a file has been modified since last cache
    pub fn is_file_modified(&self, file_path: &str) -> bool {
        if let Ok(metadata) = std::fs::metadata(file_path) {
            if let Ok(mtime) = metadata.modified() {
                if let Some(cached_mtime) = self.file_mtimes.get(file_path) {
                    if *cached_mtime == mtime {
                        return false; // Not modified
                    }
                }
                // Update cache with new mtime
                self.file_mtimes.insert(file_path.to_string(), mtime);
            }
        }
        true // Modified or couldn't check
    }

    /// Clear the entire cache
    #[allow(dead_code)]
    pub fn clear(&self) {
        self.results.clear();
        self.file_mtimes.clear();
    }

    /// Get cache statistics
    #[allow(dead_code)]
    pub fn stats(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();
        stats.insert("result_entries".to_string(), self.results.len());
        stats.insert("file_entries".to_string(), self.file_mtimes.len());
        stats
    }
}

impl Default for SearchCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_key_generation() {
        let cache = SearchCache::new();
        let key1 = cache.get_cache_key("test", "/path", Some(&["rs".to_string()]), false);
        let key2 = cache.get_cache_key("test", "/path", Some(&["rs".to_string()]), true);
        assert_ne!(key1, key2);
    }

    #[test]
    fn test_cache_set_get() {
        let cache = SearchCache::new();
        let key = "test:key".to_string();
        let results = vec![];
        cache.set(key.clone(), results);
        assert!(cache.get(&key).is_some());
    }

    #[test]
    fn test_cache_clear() {
        let cache = SearchCache::new();
        cache.set("key1".to_string(), vec![]);
        cache.clear();
        assert!(cache.get("key1").is_none());
    }
}

