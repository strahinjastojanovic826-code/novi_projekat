#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElfClass {
    Bit32,
    Bit64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElfType {
    Relocatable,
    Executable,
    SharedObject,
}

#[derive(Debug, Clone)]
pub struct ElfSection {
    pub name: String,
    pub section_type: u32,
    pub address: u64,
    pub offset: u64,
    pub size: u64,
}

#[derive(Debug, Clone)]
pub struct ElfSymbol {
    pub name: String,
    pub value: u64,
    pub size: u64,
    pub symbol_type: String,
}

#[derive(Debug, Clone)]
pub struct ElfFile {
    pub class: ElfClass,
    pub elf_type: ElfType,
    pub entry_point: u64,
    pub sections: Vec<ElfSection>,
    pub symbols: Vec<ElfSymbol>,
}

pub struct ElfParser;

impl ElfParser {
    pub fn parse(bytes: &[u8]) -> Result<ElfFile, String> {
        if bytes.len() < 16 {
            return Err("Nedovoljna dužina fajla za ELF zaglavlje.".into());
        }

        // Provera Magic Bytes: \x7F 'E' 'L' 'F'
        if bytes[0] != 0x7F || bytes[1] != b'E' || bytes[2] != b'L' || bytes[3] != b'F' {
            return Err("Nevažeći ELF Magic Bytes! Očekivano \\x7FELF.".into());
        }

        let class = match bytes[4] {
            1 => ElfClass::Bit32,
            2 => ElfClass::Bit64,
            _ => return Err("Nepoznata ELF klasa (ni 32-bit ni 64-bit).".into()),
        };

        // Tip fajla (bytes 16..18)
        let e_type = if bytes.len() >= 18 {
            u16::from_le_bytes([bytes[16], bytes[17]])
        } else {
            1
        };

        let elf_type = match e_type {
            1 => ElfType::Relocatable,
            2 => ElfType::Executable,
            3 => ElfType::SharedObject,
            _ => ElfType::Executable,
        };

        let entry_point = 0x00400000; // Simulirani Entry Point

        // Simulacija ekstrakcije sekcija i simbola iz binarnog toka
        let mut sections = Vec::new();
        sections.push(ElfSection {
            name: ".text".into(),
            section_type: 1, // SHT_PROGBITS
            address: 0x00401000,
            offset: 0x1000,
            size: (bytes.len() as u64).saturating_sub(64),
        });
        sections.push(ElfSection {
            name: ".rodata".into(),
            section_type: 1,
            address: 0x00402000,
            offset: 0x2000,
            size: 256,
        });
        sections.push(ElfSection {
            name: ".symtab".into(),
            section_type: 2, // SHT_SYMTAB
            address: 0x00403000,
            offset: 0x3000,
            size: 128,
        });

        let mut symbols = Vec::new();
        symbols.push(ElfSymbol {
            name: "_start".into(),
            value: entry_point,
            size: 32,
            symbol_type: "STT_FUNC".into(),
        });
        symbols.push(ElfSymbol {
            name: "quantum_init_module".into(),
            value: 0x00401020,
            size: 64,
            symbol_type: "STT_FUNC".into(),
        });
        symbols.push(ElfSymbol {
            name: "MODULE_VERSION".into(),
            value: 0x00402004,
            size: 4,
            symbol_type: "STT_OBJECT".into(),
        });

        Ok(ElfFile {
            class,
            elf_type,
            entry_point,
            sections,
            symbols,
        })
    }
}