use std::collections::HashMap;

// =============================================================================
// 1. MOESI STANJA KEŠ LINIJE
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoesiState {
    Modified,  // M: Prljavo, Ekskluzivno
    Owner,     // O: Prljavo, Deljeno (Samo u MOESI)
    Exclusive, // E: Čisto, Ekskluzivno
    Shared,    // S: Čisto/Deljeno
    Invalid,   // I: Nevažeće
}

#[derive(Debug, Clone)]
pub struct CacheLine {
    pub tag: u64,
    pub data: u64,
    pub state: MoesiState,
}

// =============================================================================
// 2. DOGAĐAJI NA MAGISTRALI (Bus Events / Snooping)
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BusOp {
    BusRd,     // Zahtev za čitanjem (Neko jezgro želi da čita)
    BusRdX,    // Zahtev za čitanjem sa namerom pisanja (Read Invalidate)
    BusUpgr,   // Zahtev za nadogradnju sa Shared -> Modified
}

// =============================================================================
// 3. CORE CACHE CONTROLLER
// =============================================================================

pub struct CoreCache {
    pub core_id: usize,
    pub lines: HashMap<u64, CacheLine>,
}

impl CoreCache {
    pub fn new(core_id: usize) -> Self {
        Self {
            core_id,
            lines: HashMap::new(),
        }
    }

    pub fn get_line(&self, tag: u64) -> Option<&CacheLine> {
        self.lines.get(&tag)
    }

    pub fn set_line(&mut self, tag: u64, data: u64, state: MoesiState) {
        self.lines.insert(tag, CacheLine { tag, data, state });
    }
}

// =============================================================================
// 4. MULTI-CORE BUS & COHERENCE ENGINE (MOESI Protocol)
// =============================================================================

pub struct QuantumCacheCoherenceEngine {
    pub caches: Vec<CoreCache>,
    pub main_memory: HashMap<u64, u64>,
}

impl QuantumCacheCoherenceEngine {
    pub fn new(num_cores: usize) -> Self {
        let mut caches = Vec::new();
        for i in 0..num_cores {
            caches.push(CoreCache::new(i));
        }

        Self {
            caches,
            main_memory: HashMap::new(),
        }
    }

    /// Čitanje iz jezgra (Local Read Request)
    pub fn read(&mut self, core_id: usize, address: u64) -> (u64, &'static str) {
        // Check local hit
        if let Some(line) = self.caches[core_id].get_line(address) {
            if line.state != MoesiState::Invalid {
                return (line.data, "L1 Cache HIT (Lokalno)");
            }
        }

        // Local MISS -> Šaljemo BusRd na magistralu svim ostalim jezgrima (Snooping)
        let mut supplier_data = None;
        let mut supplied_from_owner = false;
        let mut other_has_it = false;

        for id in 0..self.caches.len() {
            if id == core_id {
                continue;
            }

            if let Some(line) = self.caches[id].lines.get_mut(&address) {
                match line.state {
                    MoesiState::Modified => {
                        // MOESI magija: Prelazi u OWNER umesto da piše u RAM!
                        line.state = MoesiState::Owner;
                        supplier_data = Some(line.data);
                        supplied_from_owner = true;
                        other_has_it = true;
                    }
                    MoesiState::Owner => {
                        supplier_data = Some(line.data);
                        supplied_from_owner = true;
                        other_has_it = true;
                    }
                    MoesiState::Exclusive => {
                        line.state = MoesiState::Shared;
                        supplier_data = Some(line.data);
                        other_has_it = true;
                    }
                    MoesiState::Shared => {
                        supplier_data = Some(line.data);
                        other_has_it = true;
                    }
                    MoesiState::Invalid => {}
                }
            }
        }

        let (final_data, source_msg) = if let Some(data) = supplier_data {
            let msg = if supplied_from_owner {
                "Cache MISS -> Dobavljeno iz drugog L1 Keša (MOESI Cache-to-Cache Transfer) ⚡"
            } else {
                "Cache MISS -> Dobavljeno iz drugog L1 Keša (Shared Transfer)"
            };
            (data, msg)
        } else {
            // Ako niko nema, učitavamo iz glavne RAM memorije
            let ram_data = *self.main_memory.entry(address).or_insert(0xDEADBEEF);
            (ram_data, "Cache MISS -> Dobavljeno iz Glavne RAM Memorije 🐢")
        };

        // Novo stanje za jezgro koje je zatražilo čitanje
        let new_state = if other_has_it {
            MoesiState::Shared
        } else {
            MoesiState::Exclusive
        };

        self.caches[core_id].set_line(address, final_data, new_state);
        (final_data, source_msg)
    }

    /// Pisanje iz jezgra (Local Write Request)
    pub fn write(&mut self, core_id: usize, address: u64, val: u64) -> &'static str {
        // Šaljemo BusRdX ili BusUpgr da invalidiramo kopije u drugim jezgrima
        for id in 0..self.caches.len() {
            if id == core_id {
                continue;
            }

            if let Some(line) = self.caches[id].lines.get_mut(&address) {
                // Sva ostala jezgra gube svoju kopiju!
                line.state = MoesiState::Invalid;
            }
        }

        // Postavljamo sopstveno stanje u MODIFIED
        self.caches[core_id].set_line(address, val, MoesiState::Modified);
        "Upis završen -> Dodeljeno MODIFIED stanje (Sve ostale keš linije INVALIDIRANE!) 💥"
    }
}