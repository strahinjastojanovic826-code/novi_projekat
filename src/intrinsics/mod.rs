use std::sync::atomic::{compiler_fence, Ordering};

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

pub struct QuantumIntrinsicsEngine;

impl QuantumIntrinsicsEngine {
    // =========================================================================
    // 1. HARDVERSKO MERENJE CIKLUSA I CPU TIMING (RDTSC)
    // =========================================================================

    /// Čita 64-bitni Time-Stamp Counter (TSC) procesora.
    /// Vraća tačan broj ciklusa procesora od trenutka uključenja racunara!
    #[inline(always)]
    pub unsafe fn rdtsc() -> u64 { unsafe {
        #[cfg(target_arch = "x86_64")]
        {
            _rdtsc()
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            0
        }
    }}

    /// Štedi energiju CPU jezgra tokom aktivnog spin-lock čekanja
    #[inline(always)]
    pub fn cpu_pause() {
        #[cfg(target_arch = "x86_64")]
        unsafe {
            _mm_pause();
        }
    }

    /// Zaustavlja kompajler da ne pretumbava instrukcije iznad i ispod ove linije
    #[inline(always)]
    pub fn compiler_memory_barrier() {
        compiler_fence(Ordering::SeqCst);
    }

    // =========================================================================
    // 2. BITWISE HARDVERSKE AKCELERACIJE (POPCNT, LZCNT, TZCNT, BSWAP)
    // =========================================================================

    /// Broji koliko jedinica ima u 64-bitnom broju (POPCNT - Population Count)
    #[inline(always)]
    pub fn popcount_64(value: u64) -> u32 {
        value.count_ones()
    }

    /// Broji vodeće nule sa leve strane (LZCNT - Leading Zeros Count)
    #[inline(always)]
    pub fn leading_zeros_64(value: u64) -> u32 {
        value.leading_zeros()
    }

    /// Broji prateće nule sa desne strane (TZCNT - Trailing Zeros Count)
    #[inline(always)]
    pub fn trailing_zeros_64(value: u64) -> u32 {
        value.trailing_zeros()
    }

    /// Zamena bajtova (BSWAP - Little Endian <-> Big Endian pretvaranje u 1 ciklusu)
    #[inline(always)]
    pub fn byteswap_64(value: u64) -> u64 {
        value.swap_bytes()
    }

    // =========================================================================
    // 3. LOW-LEVEL COMPILER MEMORY PRIMITIVES (Memcpy & Memset u 64-bit rečima)
    // =========================================================================

    /// Intrinsik verzija kopiranja memorije bez eksternog libc-a (64-bit word transfer)
    pub unsafe fn intrinsic_memcpy(dest: *mut u8, src: *const u8, count: usize) { unsafe {
        let mut d = dest;
        let mut s = src;
        let mut n = count;

        // Kopiramo po 8 bajtova (64-bit) u jednom koraku
        while n >= 8 {
            *(d as *mut u64) = *(s as *const u64);
            d = d.add(8);
            s = s.add(8);
            n -= 8;
        }

        // Preostale pojedinačne bajtove
        while n > 0 {
            *d = *s;
            d = d.add(1);
            s = s.add(1);
            n -= 1;
        }
    }}

    /// Intrinsik verzija popunjavanja memorije u 64-bitnim blokovima
    pub unsafe fn intrinsic_memset(dest: *mut u8, value: u8, count: usize) { unsafe {
        let mut d = dest;
        let mut n = count;

        // Repliciramo bajt 8 puta unutar 64-bitne reči
        let val64 = (value as u64) * 0x0101_0101_0101_0101;

        while n >= 8 {
            *(d as *mut u64) = val64;
            d = d.add(8);
            n -= 8;
        }

        while n > 0 {
            *d = value;
            d = d.add(1);
            n -= 1;
        }
    }}
}