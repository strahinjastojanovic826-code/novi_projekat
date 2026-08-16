use std::collections::HashMap;

// =============================================================================
// 1. eBPF REGISTRI (11 x 64-bit Registara)
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EbpfRegister {
    R0 = 0,  // Povratna vrednost eBPF programa ili Helper poziva
    R1 = 1,  // Argument 1 / Pokazivač na Context (npr. Mrežni Paket)
    R2 = 2,  // Argument 2
    R3 = 3,  // Argument 3
    R4 = 4,  // Argument 4
    R5 = 5,  // Argument 5
    R6 = 6,  // Sačuvani registri
    R7 = 7,
    R8 = 8,
    R9 = 9,
    R10 = 10, // Read-Only Stack Frame Pointer
}

// =============================================================================
// 2. eBPF STRUKTURA INSTRUKCIJE (64-bit / 8 Bajtova po instrukciji)
// =============================================================================

#[derive(Debug, Clone, Copy)]
pub struct EbpfInstruction {
    pub opcode: u8,
    pub dst: u8,
    pub src: u8,
    pub offset: i16,
    pub imm: i32,
}

// eBPF Opcode Konstante
pub const BPF_MOV64_IMM: u8 = 0xB7;
pub const BPF_ADD64_IMM: u8 = 0x07;
pub const BPF_ADD64_REG: u8 = 0x0F;
pub const BPF_SUB64_IMM: u8 = 0x17;
pub const BPF_JEQ_IMM: u8   = 0x15;
pub const BPF_CALL: u8      = 0x85;
pub const BPF_EXIT: u8      = 0x95;

// =============================================================================
// 3. eBPF MAPS (In-Kernel Key-Value skladište za komunikaciju sa User-Space)
// =============================================================================

pub struct EbpfMap {
    pub map: HashMap<u64, u64>,
}

impl EbpfMap {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    pub fn lookup(&self, key: u64) -> Option<u64> {
        self.map.get(&key).copied()
    }

    pub fn update(&mut self, key: u64, value: u64) {
        self.map.insert(key, value);
    }
}

// =============================================================================
// 4. eBPF KERNEL INTERPRETER & VERIFIER
// =============================================================================

pub struct QuantumEbpfEngine {
    registers: [u64; 11],
    _stack: Vec<u8>,
}

impl QuantumEbpfEngine {
    pub fn new() -> Self {
        let mut registers = [0u64; 11];
        let stack = vec![0u8; 512]; // Standardni eBPF 512B stak
        registers[EbpfRegister::R10 as usize] = stack.as_ptr() as u64 + 512;

        Self {
            registers,
            _stack: stack,
        }
    }

    /// In-Kernel Verifikator: Proverava bezbednost eBPF programa pre izvršenja
    pub fn verify(prog: &[EbpfInstruction]) -> Result<(), &'static str> {
        if prog.is_empty() {
            return Err("eBPF Verifier: Program ne sme biti prazan!");
        }

        // Provera da li program garantovano ima izlaz (BPF_EXIT)
        let has_exit = prog.iter().any(|ins| ins.opcode == BPF_EXIT);
        if !has_exit {
            return Err("eBPF Verifier: Program nema BPF_EXIT (Moguća beskonačna petlja)!");
        }

        // Provera opsega registara
        for ins in prog {
            if ins.dst > 10 || ins.src > 10 {
                return Err("eBPF Verifier: Pristup nepostojećem registru!");
            }
        }

        Ok(())
    }

    /// Izvršava eBPF bajtkod nad datim kontekstom (npr. mrežnim paketom)
    pub fn execute(
        &mut self,
        prog: &[EbpfInstruction],
        ctx_ptr: u64,
        map: &mut EbpfMap,
    ) -> Result<u64, &'static str> {
        // Prvo pokrećemo statičku verifikaciju
        Self::verify(prog)?;

        // R1 dobija pokazivač na Context (Mrežni paket / Syscall podatke)
        self.registers[EbpfRegister::R1 as usize] = ctx_ptr;
        let mut pc = 0;

        while pc < prog.len() {
            let ins = prog[pc];

            match ins.opcode {
                BPF_MOV64_IMM => {
                    self.registers[ins.dst as usize] = ins.imm as i64 as u64;
                }
                BPF_ADD64_IMM => {
                    self.registers[ins.dst as usize] =
                        self.registers[ins.dst as usize].wrapping_add(ins.imm as i64 as u64);
                }
                BPF_ADD64_REG => {
                    self.registers[ins.dst as usize] =
                        self.registers[ins.dst as usize].wrapping_add(self.registers[ins.src as usize]);
                }
                BPF_SUB64_IMM => {
                    self.registers[ins.dst as usize] =
                        self.registers[ins.dst as usize].wrapping_sub(ins.imm as i64 as u64);
                }
                BPF_JEQ_IMM => {
                    if self.registers[ins.dst as usize] == ins.imm as u64 {
                        pc = (pc as i32 + ins.offset as i32) as usize;
                    }
                }
                BPF_CALL => {
                    // Poziv unutrašnjih Kernel Helper funkcija
                    match ins.imm {
                        1 => {
                            // Helper 1: Map Lookup (R1: Key -> R0: Value)
                            let key = self.registers[EbpfRegister::R1 as usize];
                            self.registers[EbpfRegister::R0 as usize] =
                                map.lookup(key).unwrap_or(0);
                        }
                        2 => {
                            // Helper 2: Map Update (R1: Key, R2: Value)
                            let key = self.registers[EbpfRegister::R1 as usize];
                            let val = self.registers[EbpfRegister::R2 as usize];
                            map.update(key, val);
                            self.registers[EbpfRegister::R0 as usize] = 0;
                        }
                        _ => return Err("eBPF: Nepoznat Helper Call ID!"),
                    }
                }
                BPF_EXIT => {
                    // Povratna vrednost u R0 (0 = DROP packet, 1 = PASS packet)
                    return Ok(self.registers[EbpfRegister::R0 as usize]);
                }
                _ => return Err("eBPF: Nepoznata instrukcija!"),
            }
            pc += 1;
        }

        Ok(self.registers[EbpfRegister::R0 as usize])
    }
}