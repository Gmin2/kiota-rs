use indexmap::IndexMap;
use std::collections::HashSet;

/// Case-insensitive multi-value header map for requests.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RequestHeaders {
    headers: IndexMap<String, HashSet<String>>,
}

impl RequestHeaders {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, key: &str, value: &str) {
        self.headers
            .entry(key.to_lowercase())
            .or_default()
            .insert(value.to_string());
    }

    /// Adds the value only if the key doesn't already exist.
    pub fn try_add(&mut self, key: &str, value: &str) -> bool {
        let lower = key.to_lowercase();
        if self.headers.contains_key(&lower) {
            return false;
        }
        self.headers
            .entry(lower)
            .or_default()
            .insert(value.to_string());
        true
    }

    pub fn get(&self, key: &str) -> Option<&HashSet<String>> {
        self.headers.get(&key.to_lowercase())
    }

    pub fn remove(&mut self, key: &str) {
        self.headers.shift_remove(&key.to_lowercase());
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.headers.contains_key(&key.to_lowercase())
    }

    pub fn clear(&mut self) {
        self.headers.clear();
    }

    pub fn add_all(&mut self, other: &RequestHeaders) {
        for (key, values) in &other.headers {
            let entry = self.headers.entry(key.clone()).or_default();
            entry.extend(values.iter().cloned());
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &HashSet<String>)> {
        self.headers.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.headers.is_empty()
    }
}

/// Case-insensitive multi-value header map for responses.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ResponseHeaders {
    headers: IndexMap<String, HashSet<String>>,
}

impl ResponseHeaders {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, key: &str, value: &str) {
        self.headers
            .entry(key.to_lowercase())
            .or_default()
            .insert(value.to_string());
    }

    pub fn get(&self, key: &str) -> Option<&HashSet<String>> {
        self.headers.get(&key.to_lowercase())
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &HashSet<String>)> {
        self.headers.iter()
    }
}
