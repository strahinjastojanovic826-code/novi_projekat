use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

// =============================================================================
// 1. TAGGED POINTER (Verzionisani pokazivač - Zaštita od ABA)
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TaggedPointer {
    pub address: u64,
    pub tag: u64, // Monotoni brojač verzije
}

impl TaggedPointer {
    pub fn new(address: u64, tag: u64) -> Self {
        Self { address, tag }
    }
}

// =============================================================================
// 2. EPOCH-BASED RECLAMATION (EBR) ENGINE
// =============================================================================

pub struct EpochManager {
    pub global_epoch: AtomicU64,
    pub active_threads: Mutex<HashMap<usize, u64>>, // thread_id -> local_epoch
    pub retired_nodes: Mutex<Vec<(u64, u64)>>,       // (retired_at_epoch, node_address)
}

impl EpochManager {
    pub fn new() -> Self {
        Self {
            global_epoch: AtomicU64::new(1),
            active_threads: Mutex::new(HashMap::new()),
            retired_nodes: Mutex::new(Vec::new()),
        }
    }

    /// Registruje ulazak niti u kritičnu sekciju sa trenutnom globalnom epohom
    pub fn enter_critical(&self, thread_id: usize) -> u64 {
        let current_epoch = self.global_epoch.load(Ordering::SeqCst);
        let mut threads = self.active_threads.lock().unwrap();
        threads.insert(thread_id, current_epoch);
        current_epoch
    }

    /// Izlazak niti iz kritične sekcije
    pub fn exit_critical(&self, thread_id: usize) {
        let mut threads = self.active_threads.lock().unwrap();
        threads.remove(&thread_id);
    }

    /// Odlaže oslobađanje čvora (Penzionisanje čvora u trenutnoj epohi)
    pub fn retire_node(&self, node_address: u64) {
        let current_epoch = self.global_epoch.load(Ordering::SeqCst);
        let mut retired = self.retired_nodes.lock().unwrap();
        retired.push((current_epoch, node_address));
    }

    /// Napreduje globalnu epohu ako je bezbedno
    pub fn try_advance_epoch(&self) -> bool {
        let current_epoch = self.global_epoch.load(Ordering::SeqCst);
        let threads = self.active_threads.lock().unwrap();

        // Ako bilo koja nit i dalje visi u najstarijoj epohi, ne smemo ići dalje!
        for &thread_epoch in threads.values() {
            if thread_epoch < current_epoch {
                return false;
            }
        }

        self.global_epoch.fetch_add(1, Ordering::SeqCst);
        true
    }

    /// Čisti penzionisane čvorove čija je epoha prešla u bezbednu zonu
    pub fn reclaim_garbage(&self) -> (usize, Vec<u64>) {
        let current_epoch = self.global_epoch.load(Ordering::SeqCst);
        let mut retired = self.retired_nodes.lock().unwrap();

        let mut reclaimed_addresses = Vec::new();
        // Čvorovi penzionisani bar 2 epohe unazad su bezbedni za brisanje!
        retired.retain(|&(retired_epoch, addr)| {
            if current_epoch >= retired_epoch + 2 {
                reclaimed_addresses.push(addr);
                false // Uklanja iz retired liste (Oslobađa)
            } else {
                true // Zadržava u listi čekanja
            }
        });

        (reclaimed_addresses.len(), reclaimed_addresses)
    }
}

// =============================================================================
// 3. LOCK-FREE TREIBER STACK SIMULATOR (ABA Protected)
// =============================================================================

pub struct LockFreeAbaEngine {
    pub head: Mutex<TaggedPointer>,
    pub ebr: EpochManager,
    pub cas_failures_prevented: u64,
}

impl LockFreeAbaEngine {
    pub fn new(initial_addr: u64) -> Self {
        Self {
            head: Mutex::new(TaggedPointer::new(initial_addr, 1)),
            ebr: EpochManager::new(),
            cas_failures_prevented: 0,
        }
    }

    /// Simulacija CAS operacije sa Tagged Pointer zaštitom
    pub fn compare_and_swap(
        &mut self,
        expected: TaggedPointer,
        new_addr: u64,
    ) -> Result<TaggedPointer, &'static str> {
        let mut current_head = self.head.lock().unwrap();

        // CAS proverava i ADRESU i TAG!
        if current_head.address == expected.address && current_head.tag == expected.tag {
            let new_head = TaggedPointer::new(new_addr, current_head.tag + 1);
            *current_head = new_head;
            Ok(new_head)
        } else if current_head.address == expected.address && current_head.tag != expected.tag {
            self.cas_failures_prevented += 1;
            Err("ABA DETEKOVANA! Adresa se poklapa, ali je Tag (verzija) promenjen! CAS Blokiran 🛡️")
        } else {
            Err("CAS FAILED: Pokazivač se u potpunosti promenio.")
        }
    }
}