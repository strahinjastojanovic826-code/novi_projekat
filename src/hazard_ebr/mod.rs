use std::collections::HashSet;
use std::sync::atomic::{AtomicPtr, AtomicU64, Ordering};
use std::sync::Mutex;

pub const MAX_THREADS: usize = 4;

// =============================================================================
// 1. HAZARD POINTERS (HP) ENGINE
// =============================================================================

pub struct HazardPointerDomain {
    // Svaka nit ima svoje "Hazard" mesto gde objavljuje šta trenutno čita
    pub hazards: [AtomicPtr<u8>; MAX_THREADS],
    pub retired_nodes: Mutex<Vec<*mut u8>>,
}

impl HazardPointerDomain {
    pub fn new() -> Self {
        Self {
            hazards: [
                AtomicPtr::new(std::ptr::null_mut()),
                AtomicPtr::new(std::ptr::null_mut()),
                AtomicPtr::new(std::ptr::null_mut()),
                AtomicPtr::new(std::ptr::null_mut()),
            ],
            retired_nodes: Mutex::new(Vec::new()),
        }
    }

    /// Nit objavljuje da čita pokazivač (Hazard registration)
    pub fn publish_hazard(&self, thread_id: usize, ptr: *mut u8) {
        if thread_id < MAX_THREADS {
            self.hazards[thread_id].store(ptr, Ordering::SeqCst);
        }
    }

    /// Nit uklanja objavljeni hazard pokazivač
    pub fn clear_hazard(&self, thread_id: usize) {
        if thread_id < MAX_THREADS {
            self.hazards[thread_id].store(std::ptr::null_mut(), Ordering::SeqCst);
        }
    }

    /// Stavlja čvor u listu penzionisanih
    pub fn retire_node(&self, ptr: *mut u8) {
        let mut retired = self.retired_nodes.lock().unwrap();
        retired.push(ptr);
    }

    /// Čisti penzionisane čvorove koji NIKOME nisu u hazard listi
    pub fn reclaim_hazard_garbage(&self) -> (usize, Vec<*mut u8>) {
        // 1. Sakupljamo sve trenutno aktivne Hazard pokazivače iz svih niti
        let mut active_hazards = HashSet::new();
        for h in &self.hazards {
            let ptr = h.load(Ordering::SeqCst);
            if !ptr.is_null() {
                active_hazards.insert(ptr as usize);
            }
        }

        // 2. Čistimo samo one koji se NE nalaze u active_hazards listi
        let mut retired = self.retired_nodes.lock().unwrap();
        let mut reclaimed = Vec::new();

        retired.retain(|&node_ptr| {
            if active_hazards.contains(&(node_ptr as usize)) {
                true // I dalje je rizično! Zadrži u listi
            } else {
                reclaimed.push(node_ptr);
                false // Bezbedno za brisanje!
            }
        });

        (reclaimed.len(), reclaimed)
    }
}

// =============================================================================
// 2. EPOCH-BASED RECLAMATION (EBR) ENGINE
// =============================================================================

pub struct EbrDomain {
    pub global_epoch: AtomicU64,
    pub thread_epochs: [AtomicU64; MAX_THREADS], // 0 = neaktivna nit
    pub retired_by_epoch: Mutex<[Vec<*mut u8>; 3]>, // Ring buffer za epohe (0, 1, 2)
}

impl EbrDomain {
    pub fn new() -> Self {
        Self {
            global_epoch: AtomicU64::new(1),
            thread_epochs: [
                AtomicU64::new(0),
                AtomicU64::new(0),
                AtomicU64::new(0),
                AtomicU64::new(0),
            ],
            retired_by_epoch: Mutex::new([Vec::new(), Vec::new(), Vec::new()]),
        }
    }

    pub fn enter_critical(&self, thread_id: usize) -> u64 {
        let current = self.global_epoch.load(Ordering::SeqCst);
        self.thread_epochs[thread_id].store(current, Ordering::SeqCst);
        current
    }

    pub fn exit_critical(&self, thread_id: usize) {
        self.thread_epochs[thread_id].store(0, Ordering::SeqCst);
    }

    pub fn retire_node(&self, ptr: *mut u8) {
        let current_epoch = self.global_epoch.load(Ordering::SeqCst);
        let epoch_slot = (current_epoch % 3) as usize;
        let mut retired = self.retired_by_epoch.lock().unwrap();
        retired[epoch_slot].push(ptr);
    }

    pub fn try_advance_and_reclaim(&self) -> (bool, usize) {
        let current_epoch = self.global_epoch.load(Ordering::SeqCst);

        // Proveravamo da li su sve aktivne niti stigle do trenutne epohe
        for thread_epoch in &self.thread_epochs {
            let e = thread_epoch.load(Ordering::SeqCst);
            if e != 0 && e < current_epoch {
                return (false, 0); // Barem jedna nit drži staru epohu!
            }
        }

        // Napredujemo epohu
        self.global_epoch.fetch_add(1, Ordering::SeqCst);

        // Čistimo epohu koja je bezbedna (2 epohe unazad)
        let safe_epoch_slot = ((current_epoch + 1) % 3) as usize;
        let mut retired = self.retired_by_epoch.lock().unwrap();
        let reclaimed_count = retired[safe_epoch_slot].len();
        retired[safe_epoch_slot].clear();

        (true, reclaimed_count)
    }
}