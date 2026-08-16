pub mod storage;
pub mod transaction;
pub mod persistence;

pub use storage::StorageEngine;
pub use transaction::Transaction;
pub use persistence::DbSnapshotter;

use crate::domain::QuquatVal;
use crate::vfs::QuquatVFS;

pub struct QuquatDB {
    pub storage: StorageEngine,
    pub active_tx: Option<Transaction>,
    pub next_tx_id: u64,
    pub logs: Vec<String>,
}

impl QuquatDB {
    pub fn new() -> Self {
        Self {
            storage: StorageEngine::new(),
            active_tx: None,
            next_tx_id: 1,
            logs: vec!["[QSTORE] In-Memory Data Store Inicijalizovan.".to_string()],
        }
    }

    pub fn begin_transaction(&mut self) -> u64 {
        let tx_id = self.next_tx_id;
        self.next_tx_id += 1;
        self.active_tx = Some(Transaction::new(tx_id));
        self.logs.push(format!("[TX] Započeta transakcija #{}", tx_id));
        tx_id
    }

    pub fn set(&mut self, key: &str, data: Vec<QuquatVal>, tick: u64) {
        if let Some(tx) = &mut self.active_tx {
            tx.set(key, data);
            self.logs.push(format!("[TX_STAGE] Ključ '{}' pripremljen u transakciji.", key));
        } else {
            self.storage.insert(key, data, tick);
            self.logs.push(format!("[SET] Ključ '{}' direktno upisan.", key));
        }
    }

    pub fn commit_transaction(&mut self, tick: u64) -> bool {
        if let Some(mut tx) = self.active_tx.take() {
            let tx_id = tx.tx_id;
            if tx.commit(&mut self.storage, tick) {
                self.logs.push(format!("[TX_COMMIT] Transakcija #{} uspešno primenjena!", tx_id));
                return true;
            }
        }
        self.logs.push("[TX_ERR] Nema aktivne transakcije za Commit!".to_string());
        false
    }

    pub fn rollback_transaction(&mut self) {
        if let Some(mut tx) = self.active_tx.take() {
            tx.rollback();
            self.logs.push(format!("[TX_ROLLBACK] Transakcija #{} poništena!", tx.tx_id));
        }
    }

    pub fn snapshot(&mut self, vfs: &mut QuquatVFS) {
        match DbSnapshotter::save_to_vfs(&self.storage, vfs) {
            Ok(bytes) => self.logs.push(format!("[SNAPSHOT] Baza uspešno sačuvana na VFS ({}) kvata", bytes)),
            Err(e) => self.logs.push(format!("[SNAPSHOT_ERR] Greška pri snimanju: {}", e)),
        }
    }
}