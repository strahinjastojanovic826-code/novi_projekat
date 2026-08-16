#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PagePerm {
    ReadWrite,   // Stek i Heap (NX / No-Execute bit aktivan)
    ReadOnly,    // Konstante i rodata
    ReadExecute, // .text sekcija (Samo ovde je dozvoljen RIP)
}

pub struct AslrEngine {
    pub kernel_text_base: u64,
    pub stack_base: u64,
    pub heap_base: u64,
}

pub struct ShadowStackCfi {
    // Hardware-assisted (Intel CET) simulacija senovitog steka
    shadow_stack: Vec<u64>,
}

impl AslrEngine {
    // Inicijalizuje ASLR sa entropijom iz RDTSC ili Kvantnog RNG-a
    pub fn new(seed: u64) -> Self {
        let text_offset = (seed & 0x1FFF) << 21; // 2MB poravnanje za Huge Pages
        let stack_offset = (seed & 0xFFFF) << 12;
        let heap_offset = (seed & 0xFF) << 16;

        Self {
            kernel_text_base: 0xFFFFFFFF80000000 | text_offset,
            stack_base: 0x7FFF00000000 | stack_offset,
            heap_base: 0x55FFFF000000 | heap_offset,
        }
    }

    // DEP / NX Provera: Onemogućava izvršavanje koda sa steka ili hipa
    pub fn validate_execution_fetch(&self, target_addr: u64, perm: PagePerm) -> Result<(), &'static str> {
        if perm != PagePerm::ReadExecute {
            return Err("DEP/NX Violation: Pokušaj izvršavanja koda sa Non-Executable memorije!");
        }
        Ok(())
    }
}

impl ShadowStackCfi {
    pub fn new() -> Self {
        Self { shadow_stack: Vec::new() }
    }

    // Poziva se prilikom 'CALL' instrukcije - čuva legitimnu adresu na Shadow Stack
    pub fn push_call(&mut self, return_address: u64) {
        self.shadow_stack.push(return_address);
    }

    // Poziva se prilikom 'RET' instrukcije - sprečava ROP preusmeravanje steka
    pub fn verify_ret(&mut self, actual_return_address: u64) -> Result<(), &'static str> {
        if let Some(expected_addr) = self.shadow_stack.pop() {
            if expected_addr != actual_return_address {
                return Err("CRITICAL CFI FAULT: Detektovana manipulacija stekom (ROP Chain Attack)!");
            }
            Ok(())
        } else {
            Err("Shadow Stack Underflow!")
        }
    }

    // Skener izvršnog segmenta za detekciju opasnih 'pop reg; ret' gadgeta
    pub fn scan_rop_gadgets(text_bytes: &[u8], base_addr: u64) -> Vec<(u64, &'static str)> {
        let mut gadgets = Vec::new();
        for i in 0..text_bytes.len().saturating_sub(1) {
            match (text_bytes[i], text_bytes[i + 1]) {
                (0x5F, 0xC3) => gadgets.push((base_addr + i as u64, "pop rdi; ret")),
                (0x58, 0xC3) => gadgets.push((base_addr + i as u64, "pop rax; ret")),
                (0x5E, 0xC3) => gadgets.push((base_addr + i as u64, "pop rsi; ret")),
                (0xFF, 0xE4) => gadgets.push((base_addr + i as u64, "jmp rsp")),
                _ => {}
            }
        }
        gadgets
    }
}