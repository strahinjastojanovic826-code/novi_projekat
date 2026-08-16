use super::bytecode::{Instruction, Opcode};
use super::jit::{JitCompiler, JitCompiledBlock};
use crate::driver::WinQuantumDriver;
use crate::domain::QuquatVal;

pub struct VirtualCpu {
    pub pc: usize,                   // Program Counter
    pub registers: [u64; 4],         // Opšti VM registri (R0 - R3)
    pub is_halted: bool,
    pub jit_cache: Vec<JitCompiledBlock>,
    pub total_cycles: u64,
}

impl VirtualCpu {
    pub fn new() -> Self {
        Self {
            pc: 0,
            registers: [0; 4],
            is_halted: false,
            jit_cache: Vec::new(),
            total_cycles: 0,
        }
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.is_halted = false;
        self.registers = [0; 4];
    }

    /// Izvršava jedan CPU takt (Proverava JIT keš -> Ako nema, koristi Interpreter)
    pub fn step(&mut self, bytecode: &[QuquatVal], driver: &WinQuantumDriver) -> Result<(), &'static str> {
        if self.is_halted || bytecode.is_empty() {
            return Ok(());
        }

        // 1. JIT PROVERA: Da li za trenutni PC imamo kompajliran brzi prolaz?
        if let Some(block) = self.jit_cache.iter().find(|b| b.start_pc == self.pc) {
            block.execute(driver);
            self.pc += block.length * 2;
            self.total_cycles += 1; // JIT izvršava ceo blok u samo 1 taktu!
            return Ok(());
        }

        // 2. INTERPRETER PROLIZ: Dekodiranje instrukcije po instrukciju
        if self.pc >= bytecode.len() {
            self.is_halted = true;
            return Ok(());
        }

        if let Some(inst) = Instruction::decode(&bytecode[self.pc..]) {
            match inst.opcode {
                Opcode::Nop => { self.pc += 2; }
                Opcode::SetQuquat => {
                    let q_idx = (inst.arg1 % 32) as usize;
                    driver.set_ququat(q_idx, QuquatVal::Q01);
                    self.pc += 2;
                }
                Opcode::FlipQuquat => {
                    let q_idx = (inst.arg1 % 32) as usize;
                    let current = driver.get_ququat(q_idx);
                    driver.set_ququat(q_idx, current.next());
                    self.pc += 2;
                }
                Opcode::Syscall => {
                    // Kernel Syscall izvršavanje
                    self.pc += 2;
                }
                Opcode::Halt => {
                    self.is_halted = true;
                }
                _ => { self.pc += 2; }
            }
            self.total_cycles += 1;
        } else {
            self.pc += 1;
        }

        Ok(())
    }

    /// Pokreće JIT analizu nad celokupnim unetim bajtkodom
    pub fn jit_compile(&mut self, instructions: &[Instruction]) {
        if let Some(compiled_block) = JitCompiler::compile_block(instructions, 0) {
            self.jit_cache.push(compiled_block);
        }
    }
}