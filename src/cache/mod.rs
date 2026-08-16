use std::collections::HashMap;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

// =============================================================================
// 1. HARDVERSKA KONTROLA KEŠA (L1/L2/L3 & TLB Primitives)
// =============================================================================

pub struct HardwareCacheControl;

impl HardwareCacheControl {
    /// Izbacuje određenu keš liniju (64 bajta) iz svih nivoa CPU keša (L1, L2, L3)
    #[inline(always)]
    pub unsafe fn flush_cache_line(addr: *const u8) { unsafe {
        #[cfg(target_arch = "x86_64")]
        {
            _mm_clflush(addr);
        }
    }}

    /// Hardverski Prefetch: Učitava podatke sa zadate adrese u L1 CPU keš pre nego što zatrebaju
    #[inline(always)]
    pub unsafe fn prefetch_l1(addr: *const u8) { unsafe {
        #[cfg(target_arch = "x86_64")]
        {
            _mm_prefetch(addr as *const i8, _MM_HINT_T0);
        }
    }}

    /// Hardverski Prefetch: Učitava podatke direktno u L2 keš (preskače L1)
    #[inline(always)]
    pub unsafe fn prefetch_l2(addr: *const u8) { unsafe {
        #[cfg(target_arch = "x86_64")]
        {
            _mm_prefetch(addr as *const i8, _MM_HINT_T1);
        }
    }}

    /// Memory Barrier (MFENCE): Osigurava da se svi prethodni keš upisi i čitanja završe
    #[inline(always)]
    pub fn memory_barrier() {
        #[cfg(target_arch = "x86_64")]
        unsafe {
            _mm_mfence();
        }
    }
}

// =============================================================================
// 2. SOFTVERSKI PAGE / BLOCK CACHE (LRU + Dirty Page Tracking)
// =============================================================================

#[derive(Debug, Clone)]
pub struct CacheBlock {
    pub block_id: u64,
    pub data: Vec<u8>,
    pub is_dirty: bool,       // Traži li upis na disk/RAM (Write-back)
    pub access_counter: u64,  // Koristi se za LRU kalkulaciju
}

pub struct QuantumPageCache {
    pub capacity: usize,
    pub cache_map: HashMap<u64, CacheBlock>,
    pub global_counter: u64,
    
    // Metrike performansi keša
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
    pub dirty_flushes: u64,
}

impl QuantumPageCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            cache_map: HashMap::with_capacity(capacity),
            global_counter: 0,
            hits: 0,
            misses: 0,
            evictions: 0,
            dirty_flushes: 0,
        }
    }

    /// Čita blok iz keša. Ako ne postoji, zabeleži Miss.
    pub fn get(&mut self, block_id: u64) -> Option<&Vec<u8>> {
        self.global_counter += 1;
        let counter = self.global_counter;

        if let Some(block) = self.cache_map.get_mut(&block_id) {
            self.hits += 1;
            block.access_counter = counter; // Osveži LRU tajmer
            Some(&block.data)
        } else {
            self.misses += 1;
            None
        }
    }

    /// Upisuje ili ažurira blok u kešu sa podrškom za LRU izbacivanje
    pub fn put(&mut self, block_id: u64, data: Vec<u8>, is_dirty: bool) {
        self.global_counter += 1;
        let counter = self.global_counter;

        // Ako blok već postoji u kešu, samo ga ažuriraj
        if let Some(block) = self.cache_map.get_mut(&block_id) {
            block.data = data;
            block.is_dirty = block.is_dirty || is_dirty;
            block.access_counter = counter;
            return;
        }

        // Ako je keš pun, izbaci "najstariji" (Least Recently Used) blok
        if self.cache_map.len() >= self.capacity {
            self.evict_lru();
        }

        self.cache_map.insert(
            block_id,
            CacheBlock {
                block_id,
                data,
                is_dirty,
                access_counter: counter,
            },
        );
    }

    /// Pronalazi i izbacuje najmanje korišćen blok (LRU Eviction)
    fn evict_lru(&mut self) {
        let mut lru_block_id = None;
        let mut oldest_access = u64::MAX;

        for (&id, block) in self.cache_map.iter() {
            if block.access_counter < oldest_access {
                oldest_access = block.access_counter;
                lru_block_id = Some(id);
            }
        }

        if let Some(id) = lru_block_id {
            if let Some(removed_block) = self.cache_map.remove(&id) {
                self.evictions += 1;
                if removed_block.is_dirty {
                    // Simulacija Write-back-a na disk ako je blok bio izmenjen
                    self.dirty_flushes += 1;
                }
            }
        }
    }

    /// Trajno upisuje (flush) sve prljave (dirty) stranice na disk
    pub fn flush_all_dirty(&mut self) {
        for block in self.cache_map.values_mut() {
            if block.is_dirty {
                block.is_dirty = false;
                self.dirty_flushes += 1;
            }
        }
    }

    /// Racuna procenat uspešnosti keširanja (Hit Ratio)
    pub fn hit_ratio(&self) -> f32 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            (self.hits as f32 / total as f32) * 100.0
        }
    }
}

// =============================================================================
// 3. GLAVNI QUANTUM CACHE ENGINE
// =============================================================================

pub struct QuantumCacheEngine {
    pub page_cache: QuantumPageCache,
}

impl QuantumCacheEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            page_cache: QuantumPageCache::new(128), // Kapacitet od 128 RAM blokova
        };

        engine.seed_demo_cache();
        engine
    }

    fn seed_demo_cache(&mut self) {
        // Popunjavamo keš demo blokovima
        for i in 1..=5 {
            let mock_data = vec![(i as u8) * 10; 512]; // 512 bajtova po bloku
            self.page_cache.put(i, mock_data, false);
        }
    }
}