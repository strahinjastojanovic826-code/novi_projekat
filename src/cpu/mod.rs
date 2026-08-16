use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

// --- 1. MODEL CONTROL REGISTARA I MSR-A ---

#[derive(Debug, Clone, Copy)]
pub struct ControlRegisters {
    pub cr0: u64,  // PE (Protected Mode), PG (Paging), WP (Write Protect)
    pub cr4: u64,  // PAE, PGE, SMEP, SMAP, OSXSAVE
    pub efer: u64, // Extended Feature Enable Register (LME, LMA, NXE)
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct MicrocodeHeader {
    pub header_version: u32,
    pub patch_revision: u32,
    pub date: u32,             // BCD format: BCD YYYYMMDD
    pub processor_sig: u32,    // CPUID Family/Model/Stepping
    pub checksum: u32,
    pub loader_revision: u32,
    pub processor_flags: u32,
    pub data_size: u32,
    pub total_size: u32,
}

// --- 2. CPU SETUP ENGINE ---

pub struct CpuSetupEngine {
    pub active_microcode_rev: AtomicU32,
    pub current_registers: ControlRegisters,
    pub total_patches_applied: AtomicU32,
    pub msr_ia32_bios_updt_trig: u32, // MSR 0x79
}

impl CpuSetupEngine {
    pub fn new() -> Self {
        Self {
            active_microcode_rev: AtomicU32::new(0x00000010), // Inicijalna fabrička verzija
            current_registers: ControlRegisters {
                cr0: 0x0000_0000,
                cr4: 0x0000_0000,
                efer: 0x0000_0000,
            },
            total_patches_applied: AtomicU32::new(0),
            msr_ia32_bios_updt_trig: 0x79,
        }
    }

    /// Čita CPUID i MSR `0x8B` (`IA32_BIOS_SIGN_ID`) radi verifikacije verzije mikrokoda
    pub fn read_microcode_revision(&self) -> u32 {
        self.active_microcode_rev.load(Ordering::Relaxed)
    }

    /// Aplicira binarni mikrokod patch upisom adrese zaglavlja u MSR `0x79`
    pub fn apply_microcode_patch(&mut self, header: MicrocodeHeader) -> Result<u32, &'static str> {
        if header.header_version != 1 {
            return Err("INVALID_HEADER: Nepodržana verzija mikrokod zaglavlja!");
        }

        let current_rev = self.active_microcode_rev.load(Ordering::Relaxed);
        if header.patch_revision <= current_rev {
            return Err("SKIPPED: Instalirani mikrokod je već noviji ili jednak ponuđenom.");
        }

        // Simulacija upisa u MSR 0x79 (IA32_BIOS_UPDT_TRIG)
        self.active_microcode_rev.store(header.patch_revision, Ordering::Relaxed);
        self.total_patches_applied.fetch_add(1, Ordering::Relaxed);

        Ok(header.patch_revision)
    }

    /// Konfiguriše x86_64 kontrolne registre za maksimalnu sigurnost i Long Mode
    pub fn setup_control_registers(&mut self) -> ControlRegisters {
        // --- CR0 SETUP ---
        // Bit 0: PE (Protected Mode Enable)
        // Bit 16: WP (Write Protect - Sprečava ring-0 da piše po read-only stranicama)
        // Bit 31: PG (Paging Enable)
        let cr0 = (1 << 0) | (1 << 16) | (1 << 31);

        // --- CR4 SETUP ---
        // Bit 5: PAE (Physical Address Extension)
        // Bit 7: PGE (Page Global Enable)
        // Bit 20: SMEP (Supervisor Mode Execution Prevention - Blokira Ring-0 izvršavanje sa User stranica)
        // Bit 21: SMAP (Supervisor Mode Access Prevention)
        let cr4 = (1 << 5) | (1 << 7) | (1 << 20) | (1 << 21);

        // --- EFER (MSR 0xC0000080) SETUP ---
        // Bit 8: LME (Long Mode Enable)
        // Bit 10: LMA (Long Mode Active)
        // Bit 11: NXE (No-Execute Enable - Blokira izvršavanje koda na stack/heap memoriji)
        let efer = (1 << 8) | (1 << 10) | (1 << 11);

        self.current_registers = ControlRegisters { cr0, cr4, efer };
        self.current_registers
    }
}