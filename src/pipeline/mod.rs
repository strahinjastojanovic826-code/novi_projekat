use std::arch::asm;

// =============================================================================
// 1. HARDVERSKI PORT I/O (Direktna komunikacija sa periferijama)
// =============================================================================

pub struct HardwarePort;

impl HardwarePort {
    /// Slanje bajta na hardverski port (OUTB instrukcija)
    #[inline(always)]
    pub unsafe fn outb(port: u16, value: u8) { unsafe {
        #[cfg(target_arch = "x86_64")]
        asm!(
            "out dx, al",
            in("dx") port,
            in("al") value,
            options(nomem, nostack, preserves_flags)
        );
    }}

    /// Čitanje bajta sa hardverskog porta (INB instrukcija)
    #[inline(always)]
    pub unsafe fn inb(port: u16) -> u8 { unsafe {
        let value: u8;
        #[cfg(target_arch = "x86_64")]
        asm!(
            "in al, dx",
            out("al") value,
            in("dx") port,
            options(nomem, nostack, preserves_flags)
        );
        #[cfg(not(target_arch = "x86_64"))]
        { value = 0; }
        value
    }}

    /// I/O Čekanje (Kratka pauza slanjem bajta na prazan port 0x80)
    #[inline(always)]
    pub unsafe fn io_wait() { unsafe {
        Self::outb(0x80, 0);
    }}
}

// =============================================================================
// 2. KONTROLNI REGISTRI PROCESORA (CR0 & CR3 - Paging & Protected Mode)
// =============================================================================

pub struct CpuControlRegisters;

impl CpuControlRegisters {
    /// Čita CR0 registar (Sadrži flegove za Paging, Protected Mode i Write Protect)
    #[inline(always)]
    pub unsafe fn read_cr0() -> u64 { unsafe {
        let cr0: u64;
        #[cfg(target_arch = "x86_64")]
        asm!(
            "mov {}, cr0",
            out(reg) cr0,
            options(nomem, nostack, preserves_flags)
        );
        #[cfg(not(target_arch = "x86_64"))]
        { cr0 = 0x80010031; }
        cr0
    }}

    /// Čita CR3 registar (Pokazivač na glavnu tabelu stranica - Page Directory Base)
    #[inline(always)]
    pub unsafe fn read_cr3() -> u64 { unsafe {
        let cr3: u64;
        #[cfg(target_arch = "x86_64")]
        asm!(
            "mov {}, cr3",
            out(reg) cr3,
            options(nomem, nostack, preserves_flags)
        );
        #[cfg(not(target_arch = "x86_64"))]
        { cr3 = 0x00000000_00100000; }
        cr3
    }}
}

// =============================================================================
// 3. CPUID INSPEKCIJA HARDVERA PREKO INLINE ASEMBLERA
// =============================================================================

pub struct CpuIdEngine;

impl CpuIdEngine {
    /// Dobija naziv proizvođača procesora (GenuineIntel / AuthenticAMD)
    pub unsafe fn get_vendor_string() -> String { unsafe {
        let ebx: u32;
        let edx: u32;
        let ecx: u32;

        #[cfg(target_arch = "x86_64")]
        asm!(
            "push rbx",
            "cpuid",
            "mov {0:e}, ebx",
            "pop rbx",
            out(reg) ebx,
            out("edx") edx,
            out("ecx") ecx,
            in("eax") 0u32,
            options(nomem, nostack)
        );
        #[cfg(not(target_arch = "x86_64"))]
        {
            ebx = 0x756e6547; edx = 0x49656e69; ecx = 0x6c65746e; // "GenuineIntel"
        }

        let mut bytes = Vec::new();
        bytes.extend_from_slice(&ebx.to_le_bytes());
        bytes.extend_from_slice(&edx.to_le_bytes());
        bytes.extend_from_slice(&ecx.to_le_bytes());

        String::from_utf8_lossy(&bytes).into_owned()
    }}
}

// =============================================================================
// 4. INSTRUCTION PIPELINE OPTIMIZATION ENGINE
// =============================================================================

pub struct IntrinsicPipelineEngine;

impl IntrinsicPipelineEngine {
    /// Asemblerski paralelizovan pajplajn za brz vektorski proračun sa odmotavanjem petlje (Loop Unrolling)
    pub unsafe fn fast_assembly_pipeline_add(src_a: &[u64], src_b: &[u64], dst: &mut [u64]) { unsafe {
        let len = src_a.len().min(src_b.len()).min(dst.len());
        let mut i = 0;

        // Pajplajn obrada po 4 elementa odjednom preko 64-bitnih registara (RAX, RBX, RCX, RDX)
        while i + 4 <= len {
            #[cfg(target_arch = "x86_64")]
            {
                let a_ptr = src_a.as_ptr().add(i);
                let b_ptr = src_b.as_ptr().add(i);
                let d_ptr = dst.as_mut_ptr().add(i);

                asm!(
                    // Učitavanje i sabiranje 4 nezavisna registra u opsegu pajplajna
                    "mov r8, [{a}]",
                    "add r8, [{b}]",
                    "mov [{d}], r8",

                    "mov r9, [{a} + 8]",
                    "add r9, [{b} + 8]",
                    "mov [{d} + 8], r9",

                    "mov r10, [{a} + 16]",
                    "add r10, [{b} + 16]",
                    "mov [{d} + 16], r10",

                    "mov r11, [{a} + 24]",
                    "add r11, [{b} + 24]",
                    "mov [{d} + 24], r11",

                    a = in(reg) a_ptr,
                    b = in(reg) b_ptr,
                    d = in(reg) d_ptr,
                    out("r8") _, out("r9") _, out("r10") _, out("r11") _,
                    options(nostack)
                );
            }
            #[cfg(not(target_arch = "x86_64"))]
            {
                for j in 0..4 {
                    dst[i + j] = src_a[i + j] + src_b[i + j];
                }
            }
            i += 4;
        }

        // Obrada preostalih pojedinačnih elemenata
        while i < len {
            dst[i] = src_a[i] + src_b[i];
            i += 1;
        }
    }}
}