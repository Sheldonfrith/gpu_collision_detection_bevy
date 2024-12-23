use std::collections::HashMap;
use std::hash::Hash;
use std::time::Instant;

pub struct LruCache<K, V> {
    map: HashMap<K, (V, Instant)>,
    capacity: usize,
}

impl<K: Hash + Eq + Clone, V> LruCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        LruCache {
            map: HashMap::with_capacity(capacity),
            capacity,
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if let Some((value, timestamp)) = self.map.get_mut(key) {
            *timestamp = Instant::now();
            Some(value)
        } else {
            None
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        // If we're at capacity, remove the least recently used item
        if self.map.len() >= self.capacity {
            let oldest_key = self
                .map
                .iter()
                .min_by_key(|(_, (_, timestamp))| timestamp)
                .map(|(k, _)| k.clone());

            if let Some(oldest_key) = oldest_key {
                self.map.remove(&oldest_key);
            }
        }

        self.map.insert(key, (value, Instant::now()));
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }
}
