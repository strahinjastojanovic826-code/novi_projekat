use core::sync::atomic::{AtomicU64, Ordering};

// --- 1. TERMODINAMIČKE KONSTANTE (LANDAUER LIMIT) ---

pub const BOLTZMANN_K: f64 = 1.380649e-23; // J/K
pub const ROOM_TEMP_K: f64 = 300.0;        // 300 Kelvin (Sobna temperatura)
pub const LN_2: f64 = 0.69314718056;

/// Minimalna energija oslobođena u okruženje pri brisanju 1 bita (Landauerov limit)
pub fn landauer_limit_joules() -> f64 {
    BOLTZMANN_K * ROOM_TEMP_K * LN_2 // ~2.87e-21 Džula po bitu
}

// --- 2. REVERZIBILNA LOGIČKA KOLA ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReversibleGate {
    Not(usize),                        // 1-bit NOT
    Cnot { control: usize, target: usize }, // 2-bit Feynman Gate
    Toffoli { c1: usize, c2: usize, target: usize }, // 3-bit CCNOT (Universal)
    Fredkin { control: usize, swap1: usize, swap2: usize }, // 3-bit CSWAP (Conservative)
}

// --- 3. REVERSIBLE ENGINE & UNCOMPUTATION STACK ---

pub struct ReversibleEngine {
    pub registers: Vec<bool>,
    pub history_stack: Vec<ReversibleGate>,
    pub bits_erased_count: AtomicU64,
    pub total_gates_executed: AtomicU64,
}

impl ReversibleEngine {
    pub fn new(num_bits: usize) -> Self {
        Self {
            registers: vec![false; num_bits],
            history_stack: Vec::new(),
            bits_erased_count: AtomicU64::new(0),
            total_gates_executed: AtomicU64::new(0),
        }
    }

    /// Izvršavanje reverzibilnog kola (0 Džula termodinamičke disipacije)
    pub fn apply_gate(&mut self, gate: ReversibleGate) {
        match gate {
            ReversibleGate::Not(t) => {
                if t < self.registers.len() {
                    self.registers[t] = !self.registers[t];
                }
            }
            ReversibleGate::Cnot { control, target } => {
                if control < self.registers.len() && target < self.registers.len() {
                    if self.registers[control] {
                        self.registers[target] = !self.registers[target];
                    }
                }
            }
            ReversibleGate::Toffoli { c1, c2, target } => {
                if c1 < self.registers.len() && c2 < self.registers.len() && target < self.registers.len() {
                    if self.registers[c1] && self.registers[c2] {
                        self.registers[target] = !self.registers[target];
                    }
                }
            }
            ReversibleGate::Fredkin { control, swap1, swap2 } => {
                if control < self.registers.len() && swap1 < self.registers.len() && swap2 < self.registers.len() {
                    if self.registers[control] {
                        self.registers.swap(swap1, swap2);
                    }
                }
            }
        }

        self.history_stack.push(gate);
        self.total_gates_executed.fetch_add(1, Ordering::Relaxed);
    }

    /// Uncomputation step: Vraćanje stanja unazad kroz istoriju kola bez brisanja bitova
    pub fn uncompute_last(&mut self) -> Result<(), &'static str> {
        if let Some(gate) = self.history_stack.pop() {
            // Sva reverzibilna kola su sopstveni inverzi (Gate == Gate^-1)
            match gate {
                ReversibleGate::Not(t) => self.registers[t] = !self.registers[t],
                ReversibleGate::Cnot { control, target } => {
                    if self.registers[control] {
                        self.registers[target] = !self.registers[target];
                    }
                }
                ReversibleGate::Toffoli { c1, c2, target } => {
                    if self.registers[c1] && self.registers[c2] {
                        self.registers[target] = !self.registers[target];
                    }
                }
                ReversibleGate::Fredkin { control, swap1, swap2 } => {
                    if self.registers[control] {
                        self.registers.swap(swap1, swap2);
                    }
                }
            }
            Ok(())
        } else {
            Err("Uncomputation Stack je prazan!")
        }
    }

    /// Destruktivno (Irreversibile) brisanje bita -> Aktivira Landauerov termodinamički penal
    pub fn irreversible_erase_bit(&mut self, index: usize) {
        if index < self.registers.len() && self.registers[index] {
            self.registers[index] = false;
            self.bits_erased_count.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Izračunava ukupnu disipiranu energiju u Džulima
    pub fn calculate_dissipated_energy(&self) -> f64 {
        let erased = self.bits_erased_count.load(Ordering::Relaxed) as f64;
        erased * landauer_limit_joules()
    }
}