use std::collections::HashMap;
use crate::domain::QuquatVal;

#[derive(Debug, Clone)]
pub struct DbRecord {
    pub key: String,
    pub data: Vec<QuquatVal>,
    pub created_at_tick: u64,
    pub version: u32,
}

pub struct StorageEngine {
    pub index: HashMap<String, DbRecord>,
    pub total_keys: usize,
}

impl StorageEngine {
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
            total_keys: 0,
        }
    }

    pub fn insert(&mut self, key: &str, data: Vec<QuquatVal>, tick: u64) {
        let version = self.index.get(key).map_or(1, |r| r.version + 1);
        let record = DbRecord {
            key: key.to_string(),
            data,
            created_at_tick: tick,
            version,
        };
        self.index.insert(key.to_string(), record);
        self.total_keys = self.index.len();
    }

    pub fn get(&self, key: &str) -> Option<&DbRecord> {
        self.index.get(key)
    }

    pub fn delete(&mut self, key: &str) -> bool {
        let removed = self.index.remove(key).is_some();
        self.total_keys = self.index.len();
        removed
    }
}