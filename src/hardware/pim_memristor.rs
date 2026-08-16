use core::sync::atomic::{AtomicU64, Ordering};

// --- 1. MEMRISTOR CELL MODEL (HP Memristor Resistance Dynamics) ---

#[derive(Debug, Clone, Copy)]
pub struct Memristor {
    pub r_off: f32,  // High Resistance State (HRS) - Logička 0 (~100 kΩ)
    pub r_on: f32,   // Low Resistance State (LRS) - Logička 1 (~1 kΩ)
    pub r_curr: f32, // Trenutni otpor ćelije
}

impl Memristor {
    pub fn new(r_off: f32, r_on: f32) -> Self {
        Self {
            r_off,
            r_on,
            r_curr: r_off, // Podrazumevano u stanju visoke otpornosti
        }
    }

    /// Provodnost (Conductance G = 1 / R) u Siemensima
    pub fn conductance(&self) -> f32 {
        1.0 / self.r_curr
    }

    /// Modifikacija otpora primenom naponskog pulsa (Memristive Switching Dynamics)
    pub fn apply_pulse(&mut self, voltage: f32, dt: f32) {
        let k = 50_000.0; // Konstanta dinamike materijala
        let delta_r = -k * voltage * dt;
        self.r_curr = (self.r_curr + delta_r).clamp(self.r_on, self.r_off);
    }
}

// --- 2. MEMRISTOR CROSSBAR ARRAY (In-Memory Analog Computing) ---

pub struct MemristorCrossbar {
    pub rows: usize,
    pub cols: usize,
    pub grid: Vec<Vec<Memristor>>,
}

impl MemristorCrossbar {
    pub fn new(rows: usize, cols: usize) -> Self {
        let grid = vec![vec![Memristor::new(100_000.0, 1_000.0); cols]; rows];
        Self { rows, cols, grid }
    }

    /// Analogni PIM VMM: Računa I_out = G * V_in direkno u memorijskoj matrici (Kirchhoff & Ohm)
    pub fn execute_vmm(&self, v_in: &[f32]) -> Vec<f32> {
        let mut i_out = vec![0.0; self.cols];
        for col in 0..self.cols {
            let mut current_sum = 0.0;
            for row in 0..self.rows {
                let v = if row < v_in.len() { v_in[row] } else { 0.0 };
                let g = self.grid[row][col].conductance();
                current_sum += v * g; // Ohm-ov zakon: I = V * G
            }
            i_out[col] = current_sum; // Kirchoff-ov zakon sumiranja struje
        }
        i_out
    }
}

// --- 3. PIM EXECUTION ENGINE ---

pub struct PimEngine {
    pub crossbar: MemristorCrossbar,
    pub total_pim_ops: AtomicU64,
    pub bus_bytes_saved: AtomicU64,
}

impl PimEngine {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            crossbar: MemristorCrossbar::new(rows, cols),
            total_pim_ops: AtomicU64::new(0),
            bus_bytes_saved: AtomicU64::new(0),
        }
    }

    /// Programiranje težina matrice u otpor memristora (0.0 -> R_OFF, 1.0 -> R_ON)
    pub fn program_weights(&mut self, weights: &[Vec<f32>]) {
        for r in 0..self.crossbar.rows.min(weights.len()) {
            for c in 0..self.crossbar.cols.min(weights[r].len()) {
                let norm = weights[r][c].clamp(0.0, 1.0);
                let target_r = self.crossbar.grid[r][c].r_off - norm * (self.crossbar.grid[r][c].r_off - self.crossbar.grid[r][c].r_on);
                self.crossbar.grid[r][c].r_curr = target_r;
            }
        }
    }

    /// Izvršava operaciju množenja bez prenosa podataka na CPU (PIM Execution)
    pub fn compute_in_memory(&mut self, input_vector: &[f32]) -> Vec<f32> {
        self.total_pim_ops.fetch_add(1, Ordering::Relaxed);
        
        // Ušteda u bajtovima koji ne moraju putovati kroz memorijsku magistralu
        let saved_bytes = (input_vector.len() * 4 + self.crossbar.cols * 4) as u64;
        self.bus_bytes_saved.fetch_add(saved_bytes, Ordering::Relaxed);

        self.crossbar.execute_vmm(input_vector)
    }
}