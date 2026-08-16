#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    Add,
    Sub,
    Load,
    Nop,
}

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    pub id: u32,
    pub op: Opcode,
    pub dest_reg: Option<usize>,
    pub src_reg1: Option<usize>,
    pub src_reg2: Option<usize>,
}

impl Instruction {
    pub fn nop() -> Self {
        Self {
            id: 0,
            op: Opcode::Nop,
            dest_reg: None,
            src_reg1: None,
            src_reg2: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum StageState {
    Empty,
    Bubble(String), // Prazan ciklus (Stall)
    Executing { inst: Instruction },
}

pub struct CpuPipelineSim {
    pub enable_forwarding: bool,
    pub cycle_count: u64,
    pub instructions_completed: u64,
    pub total_stalls: u64,

    // 5 Faza Cjevovoda
    pub stage_if: StageState,
    pub stage_id: StageState,
    pub stage_ex: StageState,
    pub stage_mem: StageState,
    pub stage_wb: StageState,

    pub instruction_stream: Vec<Instruction>,
    pub pc: usize,
}

impl CpuPipelineSim {
    pub fn new(instructions: Vec<Instruction>, enable_forwarding: bool) -> Self {
        Self {
            enable_forwarding,
            cycle_count: 0,
            instructions_completed: 0,
            total_stalls: 0,
            stage_if: StageState::Empty,
            stage_id: StageState::Empty,
            stage_ex: StageState::Empty,
            stage_mem: StageState::Empty,
            stage_wb: StageState::Empty,
            instruction_stream: instructions,
            pc: 0,
        }
    }

    /// Detekcija RAW Data Hazard-a (Zavisnost podataka)
    fn detect_hazard(&self) -> Option<String> {
        let id_inst = match &self.stage_id {
            StageState::Executing { inst } => inst,
            _ => return None,
        };

        if id_inst.op == Opcode::Nop {
            return None;
        }

        // Proveravamo da li neka od instrukcija ispred u cjevovodu piše u registar koji nama treba
        let pending_stages = [&self.stage_ex, &self.stage_mem];

        for (idx, stage) in pending_stages.iter().enumerate() {
            if let StageState::Executing { inst: prev_inst } = stage {
                if let Some(dest) = prev_inst.dest_reg {
                    // Da li se citaju isti registri?
                    let src1_match = id_inst.src_reg1 == Some(dest);
                    let src2_match = id_inst.src_reg2 == Some(dest);

                    if src1_match || src2_match {
                        // Ako imamo Forwarding, zavisnost EX-to-EX se rešava bez zastoja!
                        if self.enable_forwarding && idx == 0 && prev_inst.op != Opcode::Load {
                            continue; // Forwarding spasava dan!
                        }
                        return Some(format!(
                            "RAW Hazard! Instrukcija #{} (u ID) čeka R{} iz Instrukcije #{}",
                            id_inst.id, dest, prev_inst.id
                        ));
                    }
                }
            }
        }

        None
    }

    /// Simulacija jednog taktskog ciklusa procesora
    pub fn step_clock_cycle(&mut self) -> bool {
        self.cycle_count += 1;

        // 1. WB Faza (Završavanje instrukcije)
        if let StageState::Executing { inst } = &self.stage_wb {
            if inst.op != Opcode::Nop {
                self.instructions_completed += 1;
            }
        }

        // Pomeranje cjevovoda zdesna nalevo (WB <- MEM <- EX)
        self.stage_wb = self.stage_mem.clone();
        self.stage_mem = self.stage_ex.clone();

        // 2. Provera Hazarda na novou ID Faze
        if let Some(hazard_reason) = self.detect_hazard() {
            // Ubacujemo BUBBLE (Stall) u EX fazu!
            self.stage_ex = StageState::Bubble(hazard_reason);
            self.total_stalls += 1;
            // IF i ID faza ostaju zamrznute (ne napreduju ovaj ciklus)
            return true;
        }

        // Ako nema hazarda, napredujemo ID -> EX
        self.stage_ex = self.stage_id.clone();

        // 3. IF -> ID
        self.stage_id = self.stage_if.clone();

        // 4. Fetch Nova Instrukcija (IF)
        if self.pc < self.instruction_stream.len() {
            self.stage_if = StageState::Executing {
                inst: self.instruction_stream[self.pc],
            };
            self.pc += 1;
        } else {
            self.stage_if = StageState::Empty;
        }

        // Provera da li je cjevovod potpuno prazan
        matches!(self.stage_if, StageState::Empty)
            && matches!(self.stage_id, StageState::Empty)
            && matches!(self.stage_ex, StageState::Empty)
            && matches!(self.stage_mem, StageState::Empty)
            && matches!(self.stage_wb, StageState::Empty)
    }

    pub fn calculate_cpi(&self) -> f64 {
        if self.instructions_completed == 0 {
            return 0.0;
        }
        self.cycle_count as f64 / self.instructions_completed as f64
    }
}