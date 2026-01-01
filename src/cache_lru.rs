//! LRU Cache Implementation
//!
//! Provides an LRU (Least Recently Used) cache with automatic eviction.

use lru::LruCache;
use std::hash::Hash;
use std::num::NonZeroUsize;
use std::sync::{Arc, Mutex};

/// Thread-safe LRU cache with automatic eviction
///
/// This cache automatically evicts the least recently used items when
/// the capacity is reached, preventing unbounded memory growth.
///
/// # Examples
///
/// ```
/// use codesearch::cache_lru::LruCacheWrapper;
///
/// let cache = LruCacheWrapper::new(100);
/// cache.insert("key1".to_string(), vec![1, 2, 3]);
/// assert_eq!(cache.get(&"key1".to_string()), Some(vec![1, 2, 3]));
/// ```
pub struct LruCacheWrapper<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    cache: Arc<Mutex<LruCache<K, V>>>,
}

impl<K, V> LruCacheWrapper<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    /// Create a new LRU cache with the specified capacity
    ///
    /// # Arguments
    ///
    /// * `capacity` - Maximum number of items to store
    ///
    /// # Panics
    ///
    /// Panics if capacity is 0
    pub fn new(capacity: usize) -> Self {
        let cap = NonZeroUsize::new(capacity).expect("Capacity must be non-zero");
        Self {
            cache: Arc::new(Mutex::new(LruCache::new(cap))),
        }
    }

    /// Insert a key-value pair into the cache
    ///
    /// If the cache is at capacity, the least recently used item is evicted.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to insert
    /// * `value` - The value to associate with the key
    pub fn insert(&self, key: K, value: V) {
        let mut cache = self.cache.lock().unwrap();
        cache.put(key, value);
    }

    /// Get a value from the cache
    ///
    /// Returns `None` if the key is not present. Updates the item's
    /// position to mark it as recently used.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to look up
    pub fn get(&self, key: &K) -> Option<V> {
        let mut cache = self.cache.lock().unwrap();
        cache.get(key).cloned()
    }

    /// Check if a key exists in the cache
    pub fn contains(&self, key: &K) -> bool {
        let cache = self.cache.lock().unwrap();
        cache.contains(key)
    }

    /// Remove a key from the cache
    pub fn remove(&self, key: &K) -> Option<V> {
        let mut cache = self.cache.lock().unwrap();
        cache.pop(key)
    }

    /// Clear all items from the cache
    pub fn clear(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
    }

    /// Get the current number of items in the cache
    pub fn len(&self) -> usize {
        let cache = self.cache.lock().unwrap();
        cache.len()
    }

    /// Check if the cache is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get the cache capacity
    pub fn capacity(&self) -> usize {
        let cache = self.cache.lock().unwrap();
        cache.cap().get()
    }
}

impl<K, V> Clone for LruCacheWrapper<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    fn clone(&self) -> Self {
        Self {
            cache: Arc::clone(&self.cache),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lru_cache_basic() {
        let cache = LruCacheWrapper::new(2);
        cache.insert("key1", "value1");
        cache.insert("key2", "value2");

        assert_eq!(cache.get(&"key1"), Some("value1"));
        assert_eq!(cache.get(&"key2"), Some("value2"));
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn test_lru_eviction() {
        let cache = LruCacheWrapper::new(2);
        cache.insert("key1", "value1");
        cache.insert("key2", "value2");
        cache.insert("key3", "value3"); // Should evict key1

        assert_eq!(cache.get(&"key1"), None); // Evicted
        assert_eq!(cache.get(&"key2"), Some("value2"));
        assert_eq!(cache.get(&"key3"), Some("value3"));
    }

    #[test]
    fn test_lru_access_updates_order() {
        let cache = LruCacheWrapper::new(2);
        cache.insert("key1", "value1");
        cache.insert("key2", "value2");
        
        // Access key1 to make it recently used
        cache.get(&"key1");
        
        // Insert key3, should evict key2 (least recently used)
        cache.insert("key3", "value3");

        assert_eq!(cache.get(&"key1"), Some("value1")); // Still present
        assert_eq!(cache.get(&"key2"), None); // Evicted
        assert_eq!(cache.get(&"key3"), Some("value3"));
    }

    #[test]
    fn test_lru_contains() {
        let cache = LruCacheWrapper::new(10);
        cache.insert("key1", "value1");

        assert!(cache.contains(&"key1"));
        assert!(!cache.contains(&"key2"));
    }

    #[test]
    fn test_lru_remove() {
        let cache = LruCacheWrapper::new(10);
        cache.insert("key1", "value1");

        assert_eq!(cache.remove(&"key1"), Some("value1"));
        assert_eq!(cache.get(&"key1"), None);
    }

    #[test]
    fn test_lru_clear() {
        let cache = LruCacheWrapper::new(10);
        cache.insert("key1", "value1");
        cache.insert("key2", "value2");

        cache.clear();
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
    }

    #[test]
    fn test_lru_capacity() {
        let cache = LruCacheWrapper::<String, String>::new(100);
        assert_eq!(cache.capacity(), 100);
    }

    #[test]
    fn test_lru_thread_safety() {
        use std::thread;

        let cache = LruCacheWrapper::new(100);
        let cache_clone = cache.clone();

        let handle = thread::spawn(move || {
            cache_clone.insert("key1", "value1");
        });

        cache.insert("key2", "value2");
        handle.join().unwrap();

        assert!(cache.contains(&"key1") || cache.contains(&"key2"));
    }
}
