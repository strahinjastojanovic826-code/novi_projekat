use super::bytecode::{Instruction, Opcode};
use crate::driver::WinQuantumDriver;

#[derive(Debug, Clone)]
pub struct JitCompiledBlock {
    pub start_pc: usize,
    pub length: usize,
    pub and_mask: u64,
    pub or_mask: u64,
}

impl JitCompiledBlock {
    /// JIT Izvršavanje: Primenjuje pre-kompajliranu bitmasku direktno nad tranzistorima u 1 taktu
    pub fn execute(&self, driver: &WinQuantumDriver) {
        let current = driver.read_register();
        let optimized = (current & self.and_mask) | self.or_mask;
        driver.write_register(optimized);
    }
}

pub struct JitCompiler;

impl JitCompiler {
    /// Analizira blok instrukcija i prevodi sukcesivne SET/FLIP instrukcije u 64-bitnu masku
    pub fn compile_block(instructions: &[Instruction], start_pc: usize) -> Option<JitCompiledBlock> {
        if instructions.is_empty() { return None; }

        let mut and_mask: u64 = !0; // Svi bitovi 1
        let mut or_mask: u64 = 0;
        let mut count = 0;

        for inst in instructions {
            match inst.opcode {
                Opcode::SetQuquat => {
                    let q_idx = (inst.arg1 % 32) as u64;
                    let val = (inst.arg2 % 4) as u64;
                    let bit_shift = q_idx * 2;

                    // Očisti 2 bita na ciljnoj poziciji
                    and_mask &= !(0b11 << bit_shift);
                    // Postavi nove vrednosti
                    or_mask |= val << bit_shift;
                    count += 1;
                }
                Opcode::Halt => break,
                _ => return None, // Ako naiđe na složeniji Syscall, vraća na sporiji Interpreter
            }
        }

        if count > 2 { // JIT se aktivira ako blok ima više od 2 spora operanda
            Some(JitCompiledBlock {
                start_pc,
                length: count,
                and_mask,
                or_mask,
            })
        } else {
            None
        }
    }
}