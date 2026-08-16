use std::collections::HashMap;
use crate::domain::QuquatVal;
use super::storage::StorageEngine;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TxState {
    Active,
    Committed,
    RolledBack,
}

pub struct Transaction {
    pub tx_id: u64,
    pub state: TxState,
    pub write_buffer: HashMap<String, Vec<QuquatVal>>,
    pub delete_buffer: Vec<String>,
}

impl Transaction {
    pub fn new(tx_id: u64) -> Self {
        Self {
            tx_id,
            state: TxState::Active,
            write_buffer: HashMap::new(),
            delete_buffer: Vec::new(),
        }
    }

    pub fn set(&mut self, key: &str, data: Vec<QuquatVal>) {
        if self.state == TxState::Active {
            self.write_buffer.insert(key.to_string(), data);
        }
    }

    pub fn delete(&mut self, key: &str) {
        if self.state == TxState::Active {
            self.delete_buffer.push(key.to_string());
        }
    }

    /// Atomska primena svih pripremljenih izmena u In-Memory skladište
    pub fn commit(&mut self, storage: &mut StorageEngine, tick: u64) -> bool {
    if self.state != TxState::Active {
        return false;
    }

    // HashMap::drain() ne traži argumente
    for (k, v) in self.write_buffer.drain() {
        storage.insert(&k, v, tick);
    }

    // Vec::drain(..) ZAHTEVA opseg (..)
    for k in self.delete_buffer.drain(..) {
        storage.delete(&k);
    }

    self.state = TxState::Committed;
    true
}

    pub fn rollback(&mut self) {
        if self.state == TxState::Active {
            self.write_buffer.clear();
            self.delete_buffer.clear();
            self.state = TxState::RolledBack;
        }
    }
}