use core::sync::atomic::{AtomicU64, Ordering};

pub const IDT_ENTRIES: usize = 256;

// --- 1. LONG MODE IDT ENTRY (16 BAJTOVA ZA x86_64) ---

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct IdtEntry {
    pub offset_low: u16,       // Offset bitovi 0..15
    pub gdt_selector: u16,     // Kernel Code Segment Selector (npr. 0x08)
    pub ist: u8,               // Interrupt Stack Table indeks (0..7)
    pub type_attributes: u8,   // Present bit, DPL (Ring 0-3), Gate Type (0x8E = Interrupt Gate)
    pub offset_mid: u16,       // Offset bitovi 16..31
    pub offset_high: u32,      // Offset bitovi 32..63
    pub zero: u32,             // Rezervisano (mora biti 0)
}

impl IdtEntry {
    pub const fn missing() -> Self {
        Self {
            offset_low: 0,
            gdt_selector: 0,
            ist: 0,
            type_attributes: 0,
            offset_mid: 0,
            offset_high: 0,
            zero: 0,
        }
    }

    pub fn set_handler_fn(&mut self, handler_addr: u64) {
        self.offset_low = (handler_addr & 0xFFFF) as u16;
        self.gdt_selector = 0x08; // Kernel Code Segment
        self.ist = 0;             // Koristi primarni Kernel Stack
        self.type_attributes = 0x8E; // Present, Ring 0, 64-bit Interrupt Gate
        self.offset_mid = ((handler_addr >> 16) & 0xFFFF) as u16;
        self.offset_high = ((handler_addr >> 32) & 0xFFFFFFFF) as u32;
        self.zero = 0;
    }
}

#[repr(C, packed)]
pub struct IdtPointer {
    pub limit: u16,
    pub base: u64,
}

// --- 2. INTERRUPT CONTROLLER ENGINE ---

pub struct InterruptEngine {
    pub idt: [IdtEntry; IDT_ENTRIES],
    pub pic_remapped: bool,
    pub total_interrupts_handled: AtomicU64,
}

impl InterruptEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            idt: [IdtEntry::missing(); IDT_ENTRIES],
            pic_remapped: false,
            total_interrupts_handled: AtomicU64::new(0),
        };
        engine.init_default_handlers();
        engine
    }

    /// Postavlja standardne hendlere za CPU izuzetke (0..31) i IRQ linije (32..47)
    fn init_default_handlers(&mut self) {
        // Mock adrese rutinirujućih hendlera u kernelu
        self.idt[0].set_handler_fn(0xFFFFFFFF_80001000);  // #DE Divide-by-Zero
        self.idt[8].set_handler_fn(0xFFFFFFFF_80001080);  // #DF Double Fault
        self.idt[13].set_handler_fn(0xFFFFFFFF_800010D0); // #GP General Protection Fault
        self.idt[14].set_handler_fn(0xFFFFFFFF_800010E0); // #PF Page Fault

        // IRQ Linije (nakon PIC remapa)
        self.idt[32].set_handler_fn(0xFFFFFFFF_80002000); // IRQ0 - PIT Timer
        self.idt[33].set_handler_fn(0xFFFFFFFF_80002010); // IRQ1 - PS/2 Keyboard
    }

    /// Remapiranje 8259 PIC kontrolera (IRQ 0..7 sa 0x08 prebacuje na 0x20/32)
    pub fn remap_pic(&mut self) {
        // Slanje ICQ1, ICQ2, ICQ3, ICQ4 komandi na Port 0x20 (Master) i Port 0xA0 (Slave)
        self.pic_remapped = true;
    }

    /// Simulira okidanje prekida i skok kroz IDT tabelu
    pub fn dispatch_interrupt(&self, vector: u8) -> Result<&'static str, &'static str> {
        let entry = self.idt[vector as usize];
        if entry.type_attributes == 0 {
            return Err("UNHANDLED_INTERRUPT: Vektor nema registrovan hendler (Unhandled Fault)!");
        }

        self.total_interrupts_handled.fetch_add(1, Ordering::Relaxed);

        match vector {
            0 => Ok("HANDLED: #DE (Divide-by-Zero) korigovan."),
            8 => Ok("HANDLED: #DF (Double Fault) - Prebačeno na rezervni IST stack."),
            13 => Ok("HANDLED: #GP (General Protection) - Ring-3 narušavanje priviligija sprečeno."),
            14 => Ok("HANDLED: #PF (Page Fault) - Alocirana nova virtuelna stranica u CR2."),
            32 => Ok("HANDLED: IRQ0 Timer tick obrađen u sheduleru."),
            33 => Ok("HANDLED: IRQ1 Taster registrovan u PS/2 baferu."),
            _ => Ok("HANDLED: Generic Interrupt Vector obrađen."),
        }
    }

    /// Generiše IDT pointer strukturu spremnu za `lidt` instrukciju
    pub fn get_lidt_pointer(&self) -> IdtPointer {
        IdtPointer {
            limit: (core::mem::size_of::<[IdtEntry; IDT_ENTRIES]>() - 1) as u16,
            base: self.idt.as_ptr() as u64,
        }
    }
}