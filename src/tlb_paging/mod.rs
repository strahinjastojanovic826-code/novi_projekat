use std::collections::HashMap;

pub const PAGE_SIZE_4KB: u64 = 4096;
pub const PAGE_SIZE_2MB: u64 = 2 * 1024 * 1024; // 2048 KB

// =============================================================================
// 1. TIPOVI STRANICA I TLB UNOS (TLB Entry)
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageType {
    Standard4KB,
    Huge2MB,
}

#[derive(Debug, Clone, Copy)]
pub struct TlbEntry {
    pub vpn: u64,            // Virtual Page Number
    pub pfn: u64,            // Physical Frame Number
    pub page_type: PageType,
    pub valid: bool,
}

// =============================================================================
// 2. TLB (Translation Lookaside Buffer Cache)
// =============================================================================

pub struct Tlb {
    pub capacity: usize,
    pub entries: Vec<TlbEntry>,
}

impl Tlb {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            entries: Vec::new(),
        }
    }

    /// Pretražuje TLB za datu virtuelnu adresu
    pub fn lookup(&self, va: u64) -> Option<(u64, PageType)> {
        for entry in &self.entries {
            if !entry.valid {
                continue;
            }

            let page_size = match entry.page_type {
                PageType::Standard4KB => PAGE_SIZE_4KB,
                PageType::Huge2MB => PAGE_SIZE_2MB,
            };

            let vpn = va / page_size;
            if entry.vpn == vpn {
                let offset = va % page_size;
                let pa = (entry.pfn * page_size) + offset;
                return Some((pa, entry.page_type));
            }
        }
        None
    }

    /// Ubacuje novi unos u TLB (FIFO zamena ako je pun)
    pub fn insert(&mut self, vpn: u64, pfn: u64, page_type: PageType) {
        if self.entries.len() >= self.capacity {
            self.entries.remove(0); // Evikcija najstarijeg unosa
        }
        self.entries.push(TlbEntry {
            vpn,
            pfn,
            page_type,
            valid: true,
        });
    }

    /// Pražnjenje TLB-a (npr. prilikom context switch-a ili INVLPG instrukcije)
    pub fn flush(&mut self) {
        self.entries.clear();
    }
}

// =============================================================================
// 3. STRANIČNE TABELE (Page Tables)
// =============================================================================

pub struct PageTable {
    pub mappings_4k: HashMap<u64, u64>, // VPN -> PFN za 4KB
    pub mappings_2m: HashMap<u64, u64>, // VPN -> PFN za 2MB Huge Pages
}

impl PageTable {
    pub fn new() -> Self {
        Self {
            mappings_4k: HashMap::new(),
            mappings_2m: HashMap::new(),
        }
    }

    pub fn map_4k(&mut self, va: u64, pa: u64) {
        let vpn = va / PAGE_SIZE_4KB;
        let pfn = pa / PAGE_SIZE_4KB;
        self.mappings_4k.insert(vpn, pfn);
    }

    pub fn map_2m(&mut self, va: u64, pa: u64) {
        let vpn = va / PAGE_SIZE_2MB;
        let pfn = pa / PAGE_SIZE_2MB;
        self.mappings_2m.insert(vpn, pfn);
    }
}

// =============================================================================
// 4. MMU (Memory Management Unit) ENGINE
// =============================================================================

pub struct QuantumMmu {
    pub tlb: Tlb,
    pub page_table: PageTable,
    pub tlb_hits: u64,
    pub tlb_misses: u64,
    pub page_walks: u64,
}

impl QuantumMmu {
    pub fn new(tlb_capacity: usize) -> Self {
        Self {
            tlb: Tlb::new(tlb_capacity),
            page_table: PageTable::new(),
            tlb_hits: 0,
            tlb_misses: 0,
            page_walks: 0,
        }
    }

    /// Translacija virtuelne u fizičku adresu uz nadgledanje TLB-a
    pub fn translate(&mut self, va: u64) -> Result<(u64, &'static str), &'static str> {
        // 1. TLB Lookup
        if let Some((pa, page_type)) = self.tlb.lookup(va) {
            self.tlb_hits += 1;
            let msg = match page_type {
                PageType::Standard4KB => "TLB HIT (4KB Standard Page) ⚡",
                PageType::Huge2MB => "TLB HIT (2MB Huge Page) 🚀",
            };
            return Ok((pa, msg));
        }

        // 2. TLB Miss -> Spori Page Table Walk kroz RAM
        self.tlb_misses += 1;
        self.page_walks += 1;

        // Prvo proveravamo 2MB Huge Page mapiranje
        let vpn_2m = va / PAGE_SIZE_2MB;
        if let Some(&pfn_2m) = self.page_table.mappings_2m.get(&vpn_2m) {
            let offset = va % PAGE_SIZE_2MB;
            let pa = (pfn_2m * PAGE_SIZE_2MB) + offset;
            self.tlb.insert(vpn_2m, pfn_2m, PageType::Huge2MB);
            return Ok((pa, "TLB MISS -> Page Walk Uspešan (Mapirano preko 2MB Huge Page)"));
        }

        // Zatim proveravamo 4KB Standard mapiranje
        let vpn_4k = va / PAGE_SIZE_4KB;
        if let Some(&pfn_4k) = self.page_table.mappings_4k.get(&vpn_4k) {
            let offset = va % PAGE_SIZE_4KB;
            let pa = (pfn_4k * PAGE_SIZE_4KB) + offset;
            self.tlb.insert(vpn_4k, pfn_4k, PageType::Standard4KB);
            return Ok((pa, "TLB MISS -> Page Walk Uspešan (Mapirano preko 4KB Standard Page)"));
        }

        Err("PAGE FAULT: Virtuelna adresa nije mapirana u straničnim tabelama! 💥")
    }
}