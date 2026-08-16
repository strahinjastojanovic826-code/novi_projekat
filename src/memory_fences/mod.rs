use std::collections::HashMap;

// =============================================================================
// 1. TIPOVI UPIŠA I BARIJERA
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreType {
    Temporal,    // Standardni upis: Prolazi kroz L1/L2 keš (Zagađuje keš)
    NonTemporal, // Streaming upis (MOVNT): Zaobilazi keš, ide u Write-Combining (WC) bafer
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FenceType {
    Lfence, // Load Fence
    Sfence, // Store Fence (Flush-uje WC bafer u RAM)
    Mfence, // Full Memory Fence (I Load i Store)
}

// =============================================================================
// 2. SIMULACIJA L1 KEŠA I ZAGAĐIVANJA (Cache Pollution)
// =============================================================================

pub struct L1Cache {
    pub capacity: usize,
    pub lines: HashMap<u64, Vec<u8>>,
    pub evictions: u64, // Broj izbačenih (izgubljenih) korisnih podataka iz keša
}

impl L1Cache {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            lines: HashMap::new(),
            evictions: 0,
        }
    }

    pub fn insert(&mut self, addr: u64, data: Vec<u8>) {
        if self.lines.len() >= self.capacity && !self.lines.contains_key(&addr) {
            // Izbacujemo najstariji podatak iz keša (Cache Eviction)
            if let Some(first_key) = self.lines.keys().next().copied() {
                self.lines.remove(&first_key);
                self.evictions += 1;
            }
        }
        self.lines.insert(addr, data);
    }
}

// =============================================================================
// 3. MEMORY FENCE & STREAMING STORE ENGINE
// =============================================================================

pub struct QuantumFenceEngine {
    pub l1_cache: L1Cache,
    pub main_ram: HashMap<u64, Vec<u8>>,
    pub wc_buffer: Vec<(u64, Vec<u8>)>, // Write-Combining Buffer za Non-Temporal upise
    pub temporal_stores: u64,
    pub non_temporal_stores: u64,
    pub sfence_flushes: u64,
}

impl QuantumFenceEngine {
    pub fn new(l1_capacity: usize) -> Self {
        Self {
            l1_cache: L1Cache::new(l1_capacity),
            main_ram: HashMap::new(),
            wc_buffer: Vec::new(),
            temporal_stores: 0,
            non_temporal_stores: 0,
            sfence_flushes: 0,
        }
    }

    /// Izvršava STORE instrukciju (Upis u memoriju)
    pub fn store(&mut self, addr: u64, data: Vec<u8>, store_type: StoreType) -> &'static str {
        match store_type {
            StoreType::Temporal => {
                self.temporal_stores += 1;
                // Temporal store učitava u L1 keš i time potencijalno izbacuje druge podatke!
                self.l1_cache.insert(addr, data.clone());
                self.main_ram.insert(addr, data);
                "TEMPORAL STORE: Podatak učitan u L1 keš i RAM! (Opasnost od Cache Pollution-a ⚠️)"
            }
            StoreType::NonTemporal => {
                self.non_temporal_stores += 1;
                // Non-Temporal store ZAOBILAZI L1 keš i ide u Write-Combining (WC) bafer!
                self.wc_buffer.push((addr, data));
                "NON-TEMPORAL STORE (MOVNT): Zaobiđen L1 keš! Podatak čeka u WC Baferu ⚡"
            }
        }
    }

    /// Izvršava MEMORY FENCE instrukciju
    pub fn fence(&mut self, fence_type: FenceType) -> String {
        match fence_type {
            FenceType::Sfence | FenceType::Mfence => {
                if self.wc_buffer.is_empty() {
                    "SFENCE: Nema neizvršenih Non-Temporal upisa u WC baferu.".to_string()
                } else {
                    self.sfence_flushes += 1;
                    let count = self.wc_buffer.len();
                    for (addr, data) in self.wc_buffer.drain(..) {
                        self.main_ram.insert(addr, data);
                    }
                    format!(
                        "SFENCE EXECUTED: Uspešno sinhronizovano {} Non-Temporal upisa iz WC bafera u RAM! 🛡️",
                        count
                    )
                }
            }
            FenceType::Lfence => {
                "LFENCE EXECUTED: Blokirane naredne komande dok se sve LOAD instrukcije ne završe! 🔒".to_string()
            }
        }
    }
}