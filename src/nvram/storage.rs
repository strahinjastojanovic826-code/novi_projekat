use super::variable::{NvramAttribute, NvramVariable};
use std::collections::HashMap;

pub struct NvramStorage {
    pub memory: HashMap<String, NvramVariable>,
    pub max_capacity_bytes: usize,
    pub current_used_bytes: usize,
}

impl NvramStorage {
    pub fn new(capacity_kb: usize) -> Self {
        Self {
            memory: HashMap::new(),
            max_capacity_bytes: capacity_kb * 1024,
            current_used_bytes: 0,
        }
    }

    pub fn set(&mut self, name: &str, value: &[u8], attributes: Vec<NvramAttribute>) -> Result<(), String> {
        if let Some(existing) = self.memory.get(name) {
            if existing.is_read_only() {
                return Err(format!("NVRAM Greška: Promenljiva '{}' je Read-Only i ne može se menjati!", name));
            }
        }

        let entry_size = name.len() + value.len();

        if self.current_used_bytes + entry_size > self.max_capacity_bytes {
            return Err("NVRAM Greška: Nedovoljno memorijskog prostora u čipu!".into());
        }

        let var = NvramVariable {
            name: name.to_string(),
            value: value.to_vec(),
            attributes,
        };

        self.current_used_bytes += entry_size;
        self.memory.insert(name.to_string(), var);
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&NvramVariable> {
        self.memory.get(name)
    }

    pub fn delete(&mut self, name: &str) -> Result<(), String> {
        if let Some(var) = self.memory.get(name) {
            if var.is_read_only() {
                return Err(format!("Promenljiva '{}' je zaštićena od brisanja!", name));
            }
            self.memory.remove(name);
            Ok(())
        } else {
            Err("Promenljiva ne postoji u NVRAM-u.".into())
        }
    }
}

//Nekad ni sam ne znam sta hocu
//Cas ovo pa malo ovo pa malo ono i tako u nedogled