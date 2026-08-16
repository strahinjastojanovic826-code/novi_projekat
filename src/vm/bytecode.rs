use crate::domain::QuquatVal;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    Nop = 0x00,
    SetQuquat = 0x01,  // SET <ququat_idx 0..31> <val 0..3>
    FlipQuquat = 0x02, // FLIP <ququat_idx 0..31>
    ShiftLeft = 0x03,  // SHIFT_L <count>
    ShiftRight = 0x04, // SHIFT_R <count>
    Syscall = 0x05,    // SYSCALL <id>
    Jmp = 0x06,        // JMP <target_pc>
    Halt = 0xFF,       // HALT
}

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    pub opcode: Opcode,
    pub arg1: u8,
    pub arg2: u8,
}

impl Instruction {
    pub fn encode(&self) -> Vec<QuquatVal> {
        // Svaka instrukcija se pakuje u tačno 2 kvata (4 bajta / bita opcija)
        let op_val = match self.opcode {
            Opcode::Nop => QuquatVal::Q00,
            Opcode::SetQuquat => QuquatVal::Q01,
            Opcode::FlipQuquat => QuquatVal::Q10,
            Opcode::Syscall => QuquatVal::Q11,
            _ => QuquatVal::Q00,
        };
        vec![op_val, QuquatVal::Q01]
    }

    pub fn decode(bytes: &[QuquatVal]) -> Option<Self> {
        if bytes.len() < 2 { return None; }
        
        let opcode = match bytes[0] {
            QuquatVal::Q00 => Opcode::Nop,
            QuquatVal::Q01 => Opcode::SetQuquat,
            QuquatVal::Q10 => Opcode::FlipQuquat,
            QuquatVal::Q11 => Opcode::Syscall,
        };

        Some(Self {
            opcode,
            arg1: bytes[1] as u8,
            arg2: 0,
        })
    }
}