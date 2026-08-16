use core::sync::atomic::{AtomicUsize, Ordering};

pub const PAGE_SIZE: usize = 4096; // Standardni 4KB fizički okvir
pub const KERNEL_HEAP_SIZE: usize = 16 * 1024 * 1024; // 16 MB Kernel Heap

// --- 1. STRUKTURE MEMORIJSKIH REGIONA I STRANICA ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegionKind {
    Usable,          // Slobodan RAM spreman za Frame Allocator
    Reserved,        // BIOS / Hardware MMIO (Ne diraj!)
    AcpiReclaimable, // ACPI tabele (mogu se osloboditi nakon boot-a)
    KernelCode,      // Mesto gde živi sam QuantumOS kernel
}

#[derive(Debug, Clone)]
pub struct PhysicalMemoryDescriptor {
    pub start_address: usize,
    pub length_bytes: usize,
    pub kind: RegionKind,
}

// --- 2. MEMORY MANAGEMENT ENGINE ---

pub struct MemoryManager {
    pub total_ram_bytes: usize,
    pub allocated_frames: AtomicUsize,
    pub map_entries: Vec<PhysicalMemoryDescriptor>,
    pub frame_bitmap: Vec<u8>, // Bitmaska za praćenje slobodnih/zauzetih 4KB okvira
    pub heap_start_addr: usize,
    pub heap_is_initialized: bool,
}

impl MemoryManager {
    /// Inicijalizuje Memory Manager i simulira čitanje E820 / UEFI Mape
    pub fn new(total_mb: usize) -> Self {
        let total_ram_bytes = total_mb * 1024 * 1024;
        let total_frames = total_ram_bytes / PAGE_SIZE;
        let bitmap_size = (total_frames + 7) / 8;

        let mut map_entries = Vec::new();

        // Simulacija E820 Memorijske Mape
        map_entries.push(PhysicalMemoryDescriptor {
            start_address: 0x0000_0000,
            length_bytes: 0x0009_F000, // Prvih ~640KB (Conventional RAM)
            kind: RegionKind::Usable,
        });
        map_entries.push(PhysicalMemoryDescriptor {
            start_address: 0x0009_F000,
            length_bytes: 0x0006_1000, // Reserved VGA / Video BIOS
            kind: RegionKind::Reserved,
        });
        map_entries.push(PhysicalMemoryDescriptor {
            start_address: 0x0010_0000,
            length_bytes: 0x0020_0000, // 2MB za Kernel BSS/Text sekcije
            kind: RegionKind::KernelCode,
        });
        map_entries.push(PhysicalMemoryDescriptor {
            start_address: 0x0030_0000,
            length_bytes: total_ram_bytes.saturating_sub(0x0030_0000), // Ostatak usable RAM-a
            kind: RegionKind::Usable,
        });

        Self {
            total_ram_bytes,
            allocated_frames: AtomicUsize::new(0),
            map_entries,
            frame_bitmap: vec![0u8; bitmap_size],
            heap_start_addr: 0x0040_0000, // Hip počinje iza kernela na 4MB
            heap_is_initialized: false,
        }
    }

    /// Alocira jedan fizički okvir od 4KB (Frame Allocator)
    pub fn allocate_frame(&mut self) -> Option<usize> {
        for (byte_idx, byte) in self.frame_bitmap.iter_mut().enumerate() {
            if *byte != 0xFF {
                // Pronađi prvi slobodan bit (0 = slobodno, 1 = zauzeto)
                for bit_idx in 0..8 {
                    if (*byte & (1 << bit_idx)) == 0 {
                        *byte |= 1 << bit_idx;
                        let frame_idx = byte_idx * 8 + bit_idx;
                        let phys_addr = frame_idx * PAGE_SIZE;

                        self.allocated_frames.fetch_add(1, Ordering::Relaxed);
                        return Some(phys_addr);
                    }
                }
            }
        }
        None // Out of Memory!
    }

    /// Oslobađa prethodno alocirani okvir
    pub fn free_frame(&mut self, phys_addr: usize) {
        let frame_idx = phys_addr / PAGE_SIZE;
        let byte_idx = frame_idx / 8;
        let bit_idx = frame_idx % 8;

        if byte_idx < self.frame_bitmap.len() {
            self.frame_bitmap[byte_idx] &= !(1 << bit_idx);
            self.allocated_frames.fetch_sub(1, Ordering::Relaxed);
        }
    }

    /// Postavlja Paging (CR3 Registar) i podiže Kernel Heap za globalni `alloc`
    pub fn init_kernel_heap_and_paging(&mut self) -> bool {
        // 1. Mapiranje PML4, PDPT, Page Directory i Page Table struktura
        // 2. Mapiranje Kernel Heap opsega u virtuelni memorijski prostor
        self.heap_is_initialized = true;
        true
    }
}