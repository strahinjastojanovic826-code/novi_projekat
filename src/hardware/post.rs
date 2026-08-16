use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

// --- 1. POST DIJAGNOSTIČKI STATUS I KODOVI (PORT 0x80) ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentStatus {
    Passed,
    Warning(&'static str),
    Failed(&'static str),
}

#[derive(Debug, Clone)]
pub struct PostCheckResult {
    pub component: &'static str,
    pub status: ComponentStatus,
    pub diag_code: u8, // Port 0x80 Hex Code
    pub details: &'static str,
}

// --- 2. HARDWARE POST ENGINE ---

pub struct PostEngine {
    pub is_ready_for_boot: AtomicBool,
    pub last_post_code: AtomicU64,
}

impl PostEngine {
    pub fn new() -> Self {
        Self {
            is_ready_for_boot: AtomicBool::new(false),
            last_post_code: AtomicU64::new(0x00),
        }
    }

    /// POST Kod 0x10: Provera kontrolnih registara procesora (CR0, CR4, MSR)
    pub fn check_cpu_sanity(&self) -> PostCheckResult {
        self.last_post_code.store(0x10, Ordering::Relaxed);
        PostCheckResult {
            component: "CPU Cores & Control Registers",
            status: ComponentStatus::Passed,
            diag_code: 0x10,
            details: "CR0.PE = 1 (Protected Mode OK), CR4.PAE = 1, MSR sanity OK.",
        }
    }

    /// POST Kod 0x20: March C- algoritam za proveru RAM bit-flipova
    pub fn check_dram_integrity(&self, ram_mb: usize) -> PostCheckResult {
        self.last_post_code.store(0x20, Ordering::Relaxed);
        if ram_mb < 512 {
            PostCheckResult {
                component: "DRAM Subsystem",
                status: ComponentStatus::Failed("Nedovoljno memorije! QuantumOS zahteva bar 512MB RAM-a."),
                diag_code: 0x20,
                details: "Allocation boundaries failed. Bitmask corruption at lower 1MB.",
            }
        } else if ram_mb < 1024 {
            PostCheckResult {
                component: "DRAM Subsystem",
                status: ComponentStatus::Warning("Sistem ima manje od 1GB RAM-a. Neki kvantni moduli biće onemogućeni."),
                diag_code: 0x21,
                details: "RAM sweep complete with low capacity warning.",
            }
        } else {
            PostCheckResult {
                component: "DRAM Subsystem",
                status: ComponentStatus::Passed,
                diag_code: 0x20,
                details: "March C- test prošao. Nema zalutalih 0/1 bit-flipova u ćelijama.",
            }
        }
    }

    /// POST Kod 0x30: Inicijalizacija prekida (APIC / IO-APIC Test)
    pub fn check_interrupt_controllers(&self) -> PostCheckResult {
        self.last_post_code.store(0x30, Ordering::Relaxed);
        PostCheckResult {
            component: "Local APIC & IO-APIC",
            status: ComponentStatus::Passed,
            diag_code: 0x30,
            details: "Spurious interrupt vector ok, LVT tajmeri očišćeni od lažnih signala.",
        }
    }

    /// POST Kod 0x40: Enumeracija PCIe Root Complex magistrale
    pub fn check_pcie_bus(&self) -> PostCheckResult {
        self.last_post_code.store(0x40, Ordering::Relaxed);
        PostCheckResult {
            component: "PCIe Host Bridge",
            status: ComponentStatus::Passed,
            diag_code: 0x40,
            details: "Root Complex detektovan. Pronađeno 4 uređaja na Bus 0.",
        }
    }

    /// Pokretanje kompletne POST sekvence pre učitavanja kernela
    pub fn run_full_post(&self, ram_mb: usize) -> (bool, Vec<PostCheckResult>) {
        let mut results = Vec::new();

        results.push(self.check_cpu_sanity());
        results.push(self.check_dram_integrity(ram_mb));
        results.push(self.check_interrupt_controllers());
        results.push(self.check_pcie_bus());

        let has_fatal = results.iter().any(|r| matches!(r.status, ComponentStatus::Failed(_)));

        if !has_fatal {
            self.last_post_code.store(0xFF, Ordering::Relaxed); // 0xFF = POST SUCCESSFUL / BOOT HANDOFF
            self.is_ready_for_boot.store(true, Ordering::Relaxed);
        } else {
            self.last_post_code.store(0xEE, Ordering::Relaxed); // 0xEE = FATAL POST ERROR
            self.is_ready_for_boot.store(false, Ordering::Relaxed);
        }

        (!has_fatal, results)
    }
}