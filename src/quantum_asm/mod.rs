pub mod surface_code;

use std::f64::consts::FRAC_1_SQRT_2;

// =============================================================================
// 1. COMPLEX NUMBER MATH (BEZ SPOLJNIH CRATE ZAVISNOSTI)
// =============================================================================

#[derive(Debug, Clone, Copy)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    pub fn zero() -> Self { Self::new(0.0, 0.0) }
    pub fn one() -> Self { Self::new(1.0, 0.0) }

    pub fn norm_sq(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    pub fn add(&self, other: Self) -> Self {
        Self::new(self.re + other.re, self.im + other.im)
    }

    pub fn mul(&self, other: Self) -> Self {
        Self::new(
            self.re * other.re - self.im * other.im,
            self.re * other.im + self.im * other.re,
        )
    }

    pub fn scale(&self, factor: f64) -> Self {
        Self::new(self.re * factor, self.im * factor)
    }
}

// =============================================================================
// 2. QUBIT & DECOHERENCE NOISE ENGINE
// =============================================================================

/// Qubit stanje: |ψ⟩ = α|0⟩ + β|1⟩
#[derive(Debug, Clone, Copy)]
pub struct Qubit {
    pub alpha: Complex, // Amplituda za stanje |0⟩
    pub beta: Complex,  // Amplituda za stanje |1⟩
}

impl Qubit {
    pub fn zero() -> Self {
        Self {
            alpha: Complex::one(),
            beta: Complex::zero(),
        }
    }

    /// Primena Hadamard Kapije (H) -> Stvara Superpoziciju!
    pub fn apply_hadamard(&mut self) {
        let new_alpha = self.alpha.add(self.beta).scale(FRAC_1_SQRT_2);
        let new_beta = self.alpha.add(self.beta.scale(-1.0)).scale(FRAC_1_SQRT_2);
        self.alpha = new_alpha;
        self.beta = new_beta;
    }

    /// Primena Pauli-X Kapije (Kvantni NOT)
    pub fn apply_pauli_x(&mut self) {
        std::mem::swap(&mut self.alpha, &mut self.beta);
    }

    /// Primena Pauli-Z Kapije (Phase Flip)
    pub fn apply_pauli_z(&mut self) {
        self.beta = self.beta.scale(-1.0);
    }

    /// SIMULACIJA DEKOHERENCIJE: T1 (Energy Relaxation) & T2 (Dephasing)
    pub fn apply_decoherence(&mut self, execution_time_us: f64, t1_us: f64, t2_us: f64) {
        // 1. T1 Relaxation (|1⟩ decay ka |0⟩)
        let decay_prob = 1.0 - (-execution_time_us / t1_us).exp();
        if rand_prob() < decay_prob {
            // Kubit gubi energiju i pada u stanje |0⟩
            self.alpha = Complex::one();
            self.beta = Complex::zero();
            return;
        }

        // 2. T2 Dephasing (Gubitak kvantne faze)
        let dephase_prob = 1.0 - (-execution_time_us / t2_us).exp();
        if rand_prob() < dephase_prob {
            // Faza izmedju α i β dobija slučajni šum
            let phase_noise = (rand_prob() - 0.5) * std::f64::consts::PI;
            let cos_p = phase_noise.cos();
            let sin_p = phase_noise.sin();
            let noise_rot = Complex::new(cos_p, sin_p);
            self.beta = self.beta.mul(noise_rot);
        }
    }

    /// Kvantno Merenje (Measurement): Kolaps talasne funkcije u 0 ili 1!
    pub fn measure(&mut self) -> u8 {
        let prob_zero = self.alpha.norm_sq();
        if rand_prob() < prob_zero {
            self.alpha = Complex::one();
            self.beta = Complex::zero();
            0
        } else {
            self.alpha = Complex::zero();
            self.beta = Complex::one();
            1
        }
    }
}

// =============================================================================
// 3. QUANTUM CIRCUIT & EXECUTION ENGINE
// =============================================================================

#[derive(Debug, Clone)]
pub enum QiskitInstruction {
    H { qubit: usize },
    X { qubit: usize },
    Z { qubit: usize },
    Cnot { control: usize, target: usize },
    Wait { duration_us: f64 },
    Measure { qubit: usize },
}

pub struct QuantumRegister {
    pub qubits: Vec<Qubit>,
    pub t1_us: f64, // T1 vreme opuštanja (npr. 50 μs)
    pub t2_us: f64, // T2 vreme dekoherencije (npr. 30 μs)
}

impl QuantumRegister {
    pub fn new(num_qubits: usize, t1_us: f64, t2_us: f64) -> Self {
        Self {
            qubits: vec![Qubit::zero(); num_qubits],
            t1_us,
            t2_us,
        }
    }

    pub fn execute_instruction(&mut self, inst: &QiskitInstruction) -> Option<(usize, u8)> {
        match inst {
            QiskitInstruction::H { qubit } => {
                if *qubit < self.qubits.len() {
                    self.qubits[*qubit].apply_hadamard();
                }
                None
            }
            QiskitInstruction::X { qubit } => {
                if *qubit < self.qubits.len() {
                    self.qubits[*qubit].apply_pauli_x();
                }
                None
            }
            QiskitInstruction::Z { qubit } => {
                if *qubit < self.qubits.len() {
                    self.qubits[*qubit].apply_pauli_z();
                }
                None
            }
            QiskitInstruction::Cnot { control, target } => {
                if *control < self.qubits.len() && *target < self.qubits.len() {
                    // Ako je kontrolni kubit u stanju 1 (ili ima visoku verovatnoću), obrni target
                    let prob_control_one = self.qubits[*control].beta.norm_sq();
                    if prob_control_one > 0.5 {
                        self.qubits[*target].apply_pauli_x();
                    }
                }
                None
            }
            QiskitInstruction::Wait { duration_us } => {
                // Prolazak vremena izlaže sve kubite dekoherenciji!
                for q in &mut self.qubits {
                    q.apply_decoherence(*duration_us, self.t1_us, self.t2_us);
                }
                None
            }
            QiskitInstruction::Measure { qubit } => {
                if *qubit < self.qubits.len() {
                    let result = self.qubits[*qubit].measure();
                    Some((*qubit, result))
                } else {
                    None
                }
            }
        }
    }
}

// Pomoćna funkcija za slučajni broj [0.0, 1.0)
fn rand_prob() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos();
    (nanos % 10000) as f64 / 10000.0
}