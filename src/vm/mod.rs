pub mod bytecode;
pub mod jit;
pub mod cpu;

pub use bytecode::{Instruction, Opcode};
pub use cpu::VirtualCpu;

use crate::driver::WinQuantumDriver;
use crate::domain::QuquatVal;

pub struct QuantumVM {
    pub cpu: VirtualCpu,
    pub loaded_program: Vec<QuquatVal>,
}

impl QuantumVM {
    pub fn new() -> Self {
        Self {
            cpu: VirtualCpu::new(),
            loaded_program: Vec::new(),
        }
    }

    pub fn load_program(&mut self, program: Vec<QuquatVal>) {
        self.loaded_program = program;
        self.cpu.reset();
    }

    pub fn tick(&mut self, driver: &WinQuantumDriver) {
        let _ = self.cpu.step(&self.loaded_program, driver);
    }
}