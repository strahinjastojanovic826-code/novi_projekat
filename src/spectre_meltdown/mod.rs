use std::sync::atomic::AtomicU8;

pub const STRIDE: usize = 64; // Veličina keš linije (64 bajta)
pub const PROBE_ARRAY_SIZE: usize = 256 * STRIDE; // 256 mogućih bajtova

pub struct SpectreSimulator {
    pub secret_data: Vec<u8>,
    pub public_array: Vec<u8>,
    pub probe_array: [AtomicU8; PROBE_ARRAY_SIZE],
    pub simulated_l1_cache: [bool; 256], // Mock L1 keš stanja po slotu
}

impl SpectreSimulator {
    pub fn new(secret: &str) -> Self {
        let mut probe = unsafe { std::mem::MaybeUninit::<[AtomicU8; PROBE_ARRAY_SIZE]>::uninit().assume_init() };
        for i in 0..PROBE_ARRAY_SIZE {
            probe[i] = AtomicU8::new(0);
        }

        Self {
            secret_data: secret.as_bytes().to_vec(),
            public_array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            probe_array: probe,
            simulated_l1_cache: [false; 256],
        }
    }

    /// FLUSH FAZA: Izbacujemo ceo Probe Array iz keša
    pub fn flush_probe_array(&mut self) {
        for slot in &mut self.simulated_l1_cache {
            *slot = false; // Keš linija poništena (Cache Miss)
        }
    }

    /// VIKTIMA: Funkcija koja ima proveru granica (Bounds Check)
    /// Ali CPU pod spekulacijom izvršava telo funkcije PRE nego što proveri `idx < len`!
    pub fn victim_function(&mut self, idx: usize, force_speculation: bool) {
        let array_len = self.public_array.len();

        // 1. Trening prediktora skokova ili Spekulativni prolaz
        if idx < array_len || force_speculation {
            // TRANSIENT EXECUTION (U stvarnom CPU-u ovo ide van granica nizova!)
            let fetched_byte = if idx < array_len {
                self.public_array[idx]
            } else {
                // Napadač šalje indeks koji gadja TAJNU MEMORIJU van granica!
                let secret_idx = idx - array_len;
                if secret_idx < self.secret_data.len() {
                    self.secret_data[secret_idx]
                } else {
                    0
                }
            };

            // OVO OSTAVLJA OTISAK U KEŠU!
            // CPU učitava probe_array na poziciji (fetched_byte * 64) u L1 Keš!
            let cache_slot = fetched_byte as usize;
            self.simulated_l1_cache[cache_slot] = true; // L1 Hit ostaje!
        }
    }

    /// RELOAD FAZA: Meri vreme pristupa svake opcije od 0 do 255.
    /// Onaj slot koji je u kešu odgovara tajnom bajtu!
    pub fn reload_and_recover_secret(&self) -> (u8, u64) {
        let mut best_byte = 0u8;
        let mut min_latency = u64::MAX;

        for byte_val in 0..=255 {
            // Simulacija merenja vremena preko RDTSC instrukcije
            // Cache Hit = ~5 ciklusa, Cache Miss (RAM/L3) = ~200 ciklusa
            let latency = if self.simulated_l1_cache[byte_val as usize] {
                5 // L1 Hit (Brzo!)
            } else {
                200 // RAM Miss (Sporo!)
            };

            if latency < min_latency {
                min_latency = latency;
                best_byte = byte_val;
            }
        }

        (best_byte, min_latency)
    }
}