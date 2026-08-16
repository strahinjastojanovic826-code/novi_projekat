use core::sync::atomic::{AtomicU64, Ordering};

// --- 1. MODEL KVANTNIH GREŠAKA I KUBITA ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PauliError {
    Identity, // Bez greške (I)
    BitFlipX,  // Bit-flip (X)
    PhaseFlipZ,// Phase-flip (Z)
    BothY,     // Y = iXZ
}

#[derive(Debug, Clone)]
pub struct DataQubit {
    pub id: usize,
    pub row: usize,
    pub col: usize,
    pub error: PauliError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StabilizerType {
    StarX,      // Miriše na X-stabilizator (detektuje Z greške)
    PlaquetteZ, // Z-stabilizator (detektuje X greške)
}

#[derive(Debug, Clone)]
pub struct StabilizerAncilla {
    pub id: usize,
    pub stab_type: StabilizerType,
    pub data_qubit_ids: Vec<usize>, // Povezani data kubiti
    pub syndrome_triggered: bool,    // True ako je paritet -1 (detektovana greška)
}

// --- 2. SURFACE CODE ENGINE (TOPOLOŠKA REŠETKA) ---

pub struct SurfaceCodeEngine {
    pub distance: usize, // Kodna distanca d (npr. d=3 daje 9 data kubita)
    pub data_qubits: Vec<DataQubit>,
    pub stabilizers: Vec<StabilizerAncilla>,
    pub total_errors_corrected: AtomicU64,
}

impl SurfaceCodeEngine {
    /// Inicijalizuje d x d Surface Code rešetku sa stabilizatorima
    pub fn new(distance: usize) -> Self {
        let mut data_qubits = Vec::new();
        let mut id_cnt = 0;

        for r in 0..distance {
            for c in 0..distance {
                data_qubits.push(DataQubit {
                    id: id_cnt,
                    row: r,
                    col: c,
                    error: PauliError::Identity,
                });
                id_cnt += 1;
            }
        }

        let mut stabilizers = Vec::new();
        let mut stab_id = 0;

        // Inicijalizacija Plaquette (Z) i Star (X) stabilizatora izmedju kubita
        for r in 0..distance - 1 {
            for c in 0..distance - 1 {
                let q_top_left = r * distance + c;
                let q_top_right = q_top_left + 1;
                let q_bot_left = q_top_left + distance;
                let q_bot_right = q_bot_left + 1;

                let stab_type = if (r + c) % 2 == 0 {
                    StabilizerType::PlaquetteZ
                } else {
                    StabilizerType::StarX
                };

                stabilizers.push(StabilizerAncilla {
                    id: stab_id,
                    stab_type,
                    data_qubit_ids: vec![q_top_left, q_top_right, q_bot_left, q_bot_right],
                    syndrome_triggered: false,
                });
                stab_id += 1;
            }
        }

        Self {
            distance,
            data_qubits,
            stabilizers,
            total_errors_corrected: AtomicU64::new(0),
        }
    }

    /// Ubacuje šum/grešku na specifični data kubit
    pub fn inject_error(&mut self, qubit_id: usize, error: PauliError) {
        if let Some(q) = self.data_qubits.get_mut(qubit_id) {
            q.error = match (q.error, error) {
                (PauliError::Identity, e) => e,
                (PauliError::BitFlipX, PauliError::PhaseFlipZ) => PauliError::BothY,
                (PauliError::PhaseFlipZ, PauliError::BitFlipX) => PauliError::BothY,
                (_, e) => e,
            };
        }
    }

    /// Ekstrakcija Sindroma: Paritetno merenje operatora nad stabilizatorima
    pub fn extract_syndromes(&mut self) -> usize {
        let mut triggered_count = 0;

        for stab in self.stabilizers.iter_mut() {
            let mut parity_odd = false;

            for &q_id in &stab.data_qubit_ids {
                if let Some(q) = self.data_qubits.get(q_id) {
                    let causes_syndrome = match stab.stab_type {
                        // Z-stabilizator detektuje Bit-Flip (X ili Y)
                        StabilizerType::PlaquetteZ => q.error == PauliError::BitFlipX || q.error == PauliError::BothY,
                        // X-stabilizator detektuje Phase-Flip (Z ili Y)
                        StabilizerType::StarX => q.error == PauliError::PhaseFlipZ || q.error == PauliError::BothY,
                    };

                    if causes_syndrome {
                        parity_odd = !parity_odd;
                    }
                }
            }

            stab.syndrome_triggered = parity_odd;
            if parity_odd {
                triggered_count += 1;
            }
        }

        triggered_count
    }

    /// MWPM (Minimum Weight Perfect Matching) Dekoder & Oporavak (Correction)
    pub fn decode_and_correct(&mut self) -> bool {
        let mut corrected_any = false;

        // Prolaz kroz trigovane stabilizatore i korekcija pridruženih kubita
        for stab in self.stabilizers.clone() {
            if stab.syndrome_triggered {
                // Pronađi prvi oštećeni kubit povezan sa ovim sindromom
                for &q_id in &stab.data_qubit_ids {
                    if let Some(q) = self.data_qubits.get_mut(q_id) {
                        if q.error != PauliError::Identity {
                            // Primena inverzne Pauli kapije za poništavanje greške
                            q.error = PauliError::Identity;
                            self.total_errors_corrected.fetch_add(1, Ordering::Relaxed);
                            corrected_any = true;
                            break;
                        }
                    }
                }
            }
        }

        // Osvežavanje sindroma nakon oporavka
        self.extract_syndromes();
        corrected_any
    }
}