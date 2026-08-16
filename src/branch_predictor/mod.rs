use std::collections::HashMap;

// =============================================================================
// 1. 2-BIT SATURATING COUNTER (AUTOMAT STANJA PREDIKCIJE)
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BranchState {
    StronglyNotTaken = 0, // 00
    WeaklyNotTaken = 1,   // 01
    WeaklyTaken = 2,      // 10
    StronglyTaken = 3,    // 11
}

impl BranchState {
    pub fn is_taken(&self) -> bool {
        matches!(self, BranchState::WeaklyTaken | BranchState::StronglyTaken)
    }

    /// Ažurira stanje na osnovu stvarnog ishoda skoka
    pub fn update(&mut self, actual_taken: bool) {
        *self = match (*self, actual_taken) {
            (BranchState::StronglyNotTaken, false) => BranchState::StronglyNotTaken,
            (BranchState::StronglyNotTaken, true)  => BranchState::WeaklyNotTaken,
            (BranchState::WeaklyNotTaken, false)   => BranchState::StronglyNotTaken,
            (BranchState::WeaklyNotTaken, true)    => BranchState::WeaklyTaken,
            (BranchState::WeaklyTaken, false)      => BranchState::WeaklyNotTaken,
            (BranchState::WeaklyTaken, true)       => BranchState::StronglyTaken,
            (BranchState::StronglyTaken, false)    => BranchState::WeaklyTaken,
            (BranchState::StronglyTaken, true)     => BranchState::StronglyTaken,
        };
    }
}

// =============================================================================
// 2. PREDIKTOR GRANANJA (PHT - Pattern History Table)
// =============================================================================

pub struct BranchPredictor {
    pub pht: HashMap<u64, BranchState>,
}

impl BranchPredictor {
    pub fn new() -> Self {
        Self {
            pht: HashMap::new(),
        }
    }

    /// Pogađa da li grananje na adresi `pc` skače ili ne
    pub fn predict(&self, pc: u64) -> bool {
        let state = self.pht.get(&pc).copied().unwrap_or(BranchState::WeaklyTaken);
        state.is_taken()
    }

    /// Ažurira tabelu istorije nakon što saznamo stvarni rezultat
    pub fn update(&mut self, pc: u64, actual_taken: bool) {
        let state = self.pht.entry(pc).or_insert(BranchState::WeaklyTaken);
        state.update(actual_taken);
    }
}

// =============================================================================
// 3. REORDER BUFFER (ROB) & SPEKULATIVNI ENGINE
// =============================================================================

#[derive(Debug, Clone)]
pub struct SpeculativeInstruction {
    pub pc: u64,
    pub target_pc: u64,
    pub predicted_taken: bool,
    pub spec_value: u64,
}

pub struct QuantumSpeculativeEngine {
    pub predictor: BranchPredictor,
    pub pipeline_stalls: u64,
    pub pipeline_flushes: u64,
    pub committed_instructions: u64,
    pub rob: Vec<SpeculativeInstruction>, // Reorder Buffer
}

impl QuantumSpeculativeEngine {
    pub fn new() -> Self {
        Self {
            predictor: BranchPredictor::new(),
            pipeline_stalls: 0,
            pipeline_flushes: 0,
            committed_instructions: 0,
            rob: Vec::new(),
        }
    }

    /// Izvršava instrukciju grananja spekulativno
    pub fn execute_branch(
        &mut self,
        pc: u64,
        actual_taken: bool,
        target_pc: u64,
        fallthrough_pc: u64,
    ) -> (bool, u64, &'static str) {
        let predicted_taken = self.predictor.predict(pc);

        let speculative_next_pc = if predicted_taken {
            target_pc
        } else {
            fallthrough_pc
        };

        // Spekulativno ubacujemo instrukciju u Reorder Buffer (ROB)
        self.rob.push(SpeculativeInstruction {
            pc,
            target_pc: speculative_next_pc,
            predicted_taken,
            spec_value: 0x42, // Spekulativno izračunata vrednost
        });

        if predicted_taken == actual_taken {
            // POGODAK! Spekulacija je tačna -> Commit
            self.committed_instructions += 1;
            self.predictor.update(pc, actual_taken);
            (
                true,
                speculative_next_pc,
                "PREDICTION HIT: Spekulativni rezultati uspešno ubilježeni (Commit)! 🚀",
            )
        } else {
            // PROMAŠAJ! Spekulacija je pogrešna -> Pipeline Flush
            self.pipeline_flushes += 1;
            self.pipeline_stalls += 15; // Penal od 15 ciklusa zbog pražnjenja cevovoda
            self.rob.clear();           // Odbacujemo sve spekulativne podatke!
            self.predictor.update(pc, actual_taken);

            let correct_pc = if actual_taken {
                target_pc
            } else {
                fallthrough_pc
            };

            (
                false,
                correct_pc,
                "PREDICTION MISSMATCH: Pogrešna spekulacija! Cevovod ispražnjen (Pipeline Flush) 💥",
            )
        }
    }
}