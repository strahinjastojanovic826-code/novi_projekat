// =============================================================================
// 1. MEMORY PERMISSIONS (W^X State Machine)
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryPermission {
    ReadWrite,       // Dozvoljeno samo pisanje/generisanje opkodova
    ReadOnlyExecute, // Dozvoljeno samo izvršavanje (W^X Pravilo)
    NoAccess,
}

// =============================================================================
// 2. x86_64 JIT CODE EMITTER
// =============================================================================

/// Jednostavni emiter x86_64 mašinskih instrukcija
pub struct X86_64Emitter {
    pub buffer: Vec<u8>,
}

impl X86_64Emitter {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    /// Generiše: `mov eax, imm32` (Opcode: B8 [imm32])
    pub fn emit_mov_eax(&mut self, val: u32) {
        self.buffer.push(0xB8);
        self.buffer.extend_from_slice(&val.to_le_bytes());
    }

    /// Generiše: `add eax, imm32` (Opcode: 05 [imm32])
    pub fn emit_add_eax(&mut self, val: u32) {
        self.buffer.push(0x05);
        self.buffer.extend_from_slice(&val.to_le_bytes());
    }

    /// Generiše: `sub eax, imm32` (Opcode: 2D [imm32])
    pub fn emit_sub_eax(&mut self, val: u32) {
        self.buffer.push(0x2D);
        self.buffer.extend_from_slice(&val.to_le_bytes());
    }

    /// Generiše: `ret` (Opcode: C3)
    pub fn emit_ret(&mut self) {
        self.buffer.push(0xC3);
    }
}

// =============================================================================
// 3. JIT EXECUTABLE PAGE BUFFER (Simulacija mprotect + I-Cache Flush)
// =============================================================================

pub struct JitPageBuffer {
    pub page_address: usize,
    pub permissions: MemoryPermission,
    pub data: Vec<u8>,
    pub icache_flushed: bool,
}

impl JitPageBuffer {
    pub fn new(address: usize, initial_code: &[u8]) -> Self {
        Self {
            page_address: address,
            permissions: MemoryPermission::ReadWrite,
            data: initial_code.to_vec(),
            icache_flushed: false,
        }
    }

    /// Promena W^X prava (Write XOR Execute)
    pub fn protect(&mut self, new_permission: MemoryPermission) {
        self.permissions = new_permission;
        // Kada prelazimo u Execute, sinhronizujemo I-Cache
        if new_permission == MemoryPermission::ReadOnlyExecute {
            self.flush_instruction_cache();
        }
    }

    /// Simulacija osvežavanja CPU Instrukcijskog Keša (I-Cache Invalidation Barrier)
    pub fn flush_instruction_cache(&mut self) {
        self.icache_flushed = true;
    }

    /// Self-Modifying Code Engine: Modifikacija bajta na odabranoj offset lokaciji u RAM-u
    pub fn patch_byte(&mut self, offset: usize, new_byte: u8) -> Result<(), &'static str> {
        if self.permissions == MemoryPermission::ReadOnlyExecute {
            return Err("W^X VIOLATION! Ne možete pisati po izvršnoj stranici bez ponovnog mprotect poziva! 🛡️");
        }
        if offset >= self.data.len() {
            return Err("Offset izvan granica JIT stranice!");
        }
        self.data[offset] = new_byte;
        self.icache_flushed = false; // Keš je sada prljav!
        Ok(())
    }

    /// Virtuelni JIT Interpreter za simulaciju procesorskog izvršavanja emitovanih opkodova
    pub fn execute(&self) -> Result<u32, &'static str> {
        if self.permissions != MemoryPermission::ReadOnlyExecute {
            return Err("EXECUTION DENIED! Stranica nema EXECUTE dozvolu (W^X pravilo)!");
        }
        if !self.icache_flushed {
            return Err("HARDWARE FAULT: Instrukcijski keš (I-Cache) nije osvežen nakon izmene koda!");
        }

        // Simuliramo CPU Instruction Pipeline
        let mut eax: u32 = 0;
        let mut ip = 0; // Instruction Pointer

        while ip < self.data.len() {
            let opcode = self.data[ip];
            match opcode {
                0xB8 => { // mov eax, imm32
                    if ip + 4 >= self.data.len() { break; }
                    let bytes = [self.data[ip+1], self.data[ip+2], self.data[ip+3], self.data[ip+4]];
                    eax = u32::from_le_bytes(bytes);
                    ip += 5;
                }
                0x05 => { // add eax, imm32
                    if ip + 4 >= self.data.len() { break; }
                    let bytes = [self.data[ip+1], self.data[ip+2], self.data[ip+3], self.data[ip+4]];
                    eax = eax.wrapping_add(u32::from_le_bytes(bytes));
                    ip += 5;
                }
                0x2D => { // sub eax, imm32
                    if ip + 4 >= self.data.len() { break; }
                    let bytes = [self.data[ip+1], self.data[ip+2], self.data[ip+3], self.data[ip+4]];
                    eax = eax.wrapping_sub(u32::from_le_bytes(bytes));
                    ip += 5;
                }
                0xC3 => { // ret
                    return Ok(eax);
                }
                _ => return Err("Illegal Instruction / Invalid Opcode!"),
            }
        }

        Ok(eax)
    }
}