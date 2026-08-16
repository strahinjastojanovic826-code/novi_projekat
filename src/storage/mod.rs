use std::collections::BTreeMap;
use core::sync::atomic::{AtomicU64, Ordering};

// --- 1. BLOOM FILTER (Fast Non-Membership Check) ---

#[derive(Debug, Clone)]
pub struct BloomFilter {
    bitset: u64, // 64-bitni bitset za brzu proveru prisustva ključa
}

impl BloomFilter {
    pub fn new() -> Self {
        Self { bitset: 0 }
    }

    fn hash_fn1(data: &[u8]) -> usize {
        let mut hash: u64 = 0xcbf29ce484222325;
        for &byte in data {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        (hash % 64) as usize
    }

    fn hash_fn2(data: &[u8]) -> usize {
        let mut hash: u64 = 5381;
        for &byte in data {
            hash = ((hash << 5).wrapping_add(hash)).wrapping_add(byte as u64);
        }
        (hash % 64) as usize
    }

    pub fn insert(&mut self, key: &str) {
        let bytes = key.as_bytes();
        let pos1 = Self::hash_fn1(bytes);
        let pos2 = Self::hash_fn2(bytes);
        self.bitset |= (1 << pos1) | (1 << pos2);
    }

    pub fn contains(&self, key: &str) -> bool {
        let bytes = key.as_bytes();
        let pos1 = Self::hash_fn1(bytes);
        let pos2 = Self::hash_fn2(bytes);
        let mask = (1 << pos1) | (1 << pos2);
        (self.bitset & mask) == mask
    }
}

//Na ovom os simulatoru ce se praviti filmovi
//Zahvalite mi kasnije

// --- 2. SSTABLE (Sorted String Table na sekundarnom skladištu) ---

#[derive(Debug, Clone)]
pub struct SsTable {
    pub id: u64,
    pub level: usize,
    pub data: Vec<(String, Option<String>)>, // None oznaćava Tombstone (obrisano)
    pub sparse_index: Vec<(String, usize)>,  // Sparse Index: Ključ -> Pozicija u Vektoru
    pub bloom_filter: BloomFilter,
}

impl SsTable {
    pub fn new(id: u64, level: usize, entries: Vec<(String, Option<String>)>) -> Self {
        let mut bloom_filter = BloomFilter::new();
        let mut sparse_index = Vec::new();

        // Kreiranje Bloom filtera i Sparse Indeksa na svakih 2 zapisa (blok)
        for (i, (key, _)) in entries.iter().enumerate() {
            bloom_filter.insert(key);
            if i % 2 == 0 {
                sparse_index.push((key.clone(), i));
            }
        }

        Self {
            id,
            level,
            data: entries,
            sparse_index,
            bloom_filter,
        }
    }

    /// Binary Search preko Sparse Indeksa
    pub fn get(&self, key: &str) -> Option<Option<String>> {
        // 1. Provera preko Bloom Filter-a (izbegava traženje ako ključ sigurno ne postoji)
        if !self.bloom_filter.contains(key) {
            return None; 
        }

        // 2. Pretraga preko Sparse Indeksa za lociranje bloka
        let mut block_offset = 0;
        for (indexed_key, offset) in &self.sparse_index {
            if key >= indexed_key.as_str() {
                block_offset = *offset;
            } else {
                break;
            }
        }

        // 3. Linearna pretraga unutar uskog bloka
        for (k, v) in &self.data[block_offset..] {
            if k == key {
                return Some(v.clone()); // Vraća Some(Some(val)) ili Some(None) za Tombstone
            }
        }

        None
    }
}

// --- 3. MEMTABLE & WAL ENGINE ---

pub struct LsmStorageEngine {
    pub memtable: BTreeMap<String, Option<String>>,
    pub memtable_bytes: usize,
    pub max_memtable_bytes: usize,
    pub wal_log: Vec<String>, // Write-Ahead Log za sekvencijalni zapis na disk
    pub l0_sstables: Vec<SsTable>,
    pub l1_sstables: Vec<SsTable>,
    next_sstable_id: AtomicU64,
}

impl LsmStorageEngine {
    pub fn new(max_memtable_bytes: usize) -> Self {
        Self {
            memtable: BTreeMap::new(),
            memtable_bytes: 0,
            max_memtable_bytes,
            wal_log: Vec::new(),
            l0_sstables: Vec::new(),
            l1_sstables: Vec::new(),
            next_sstable_id: AtomicU64::new(1),
        }
    }

    /// Upis podatka (PUT)
    pub fn put(&mut self, key: &str, val: &str) {
        // 1. Upis u Write-Ahead Log (WAL) za oporavak od kraha
        self.wal_log.push(format!("PUT:{}:{}", key, val));

        // 2. Upis u in-memory MemTable
        let entry_size = key.len() + val.len();
        self.memtable.insert(key.to_string(), Some(val.to_string()));
        self.memtable_bytes += entry_size;

        // 3. Auto-Flush ako je MemTable popunjen
        if self.memtable_bytes >= self.max_memtable_bytes {
            self.flush_memtable();
        }
    }

    /// Brisanje podatka (DELETE / Tombstone)
    pub fn delete(&mut self, key: &str) {
        self.wal_log.push(format!("DEL:{}", key));
        self.memtable.insert(key.to_string(), None); // Tombstone Marker
        self.memtable_bytes += key.len();

        if self.memtable_bytes >= self.max_memtable_bytes {
            self.flush_memtable();
        }
    }

    /// Čitanje podatka (GET) sa hijerarhijskim pretraživanjem
    pub fn get(&self, key: &str) -> Option<String> {
        // Nivo 1: Provera u in-memory MemTable-u
        if let Some(val_opt) = self.memtable.get(key) {
            return val_opt.clone(); // Vraća ili vrednost ili None (ako je brisan)
        }

        // Nivo 2: Provera u L0 SSTable-ima (od najnovije ka najstarijoj)
        for sstable in self.l0_sstables.iter().rev() {
            if let Some(res) = sstable.get(key) {
                return res; // Ako vrati Some(None), izbrisan je preko Tombstone-a
            }
        }

        // Nivo 3: Provera u L1 SSTable-ima (Kompaktovani sloj)
        for sstable in &self.l1_sstables {
            if let Some(res) = sstable.get(key) {
                return res;
            }
        }

        None
    }

    /// Prelivanje iz MemTable na disk u novu L0 SSTabelu (Flush)
    pub fn flush_memtable(&mut self) {
        if self.memtable.is_empty() {
            return;
        }

        let id = self.next_sstable_id.fetch_add(1, Ordering::Relaxed);
        let entries: Vec<(String, Option<String>)> = self.memtable.iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        let sstable = SsTable::new(id, 0, entries);
        self.l0_sstables.push(sstable);

        // Resetovanje MemTable-a i WAL-a
        self.memtable.clear();
        self.memtable_bytes = 0;
        self.wal_log.clear(); // WAL više nije potreban za ove podatke
    }

    /// Spajanje i čišćenje slojeva (Major Compaction L0 -> L1)
    pub fn compact(&mut self) {
        if self.l0_sstables.is_empty() {
            return;
        }

        let mut merged_map: BTreeMap<String, Option<String>> = BTreeMap::new();

        // 1. Spajanje postojecih L1 tabela
        for sstable in &self.l1_sstables {
            for (k, v) in &sstable.data {
                merged_map.insert(k.clone(), v.clone());
            }
        }

        // 2. Preklapanje sa L0 tabelama (L0 podaci imaju prioritet jer su noviji)
        for sstable in &self.l0_sstables {
            for (k, v) in &sstable.data {
                merged_map.insert(k.clone(), v.clone());
            }
        }

        // 3. Eliminacija Tombstone-ova (stvarno brisanje iz memorije)
        let compacted_entries: Vec<(String, Option<String>)> = merged_map.into_iter()
            .filter(|(_, v)| v.is_some()) // Uklanja sve None (Tombstone)
            .collect();

        let id = self.next_sstable_id.fetch_add(1, Ordering::Relaxed);
        let new_l1_sstable = SsTable::new(id, 1, compacted_entries);

        // Ažuriranje LSM strukture
        self.l0_sstables.clear();
        self.l1_sstables = vec![new_l1_sstable];
    }
}