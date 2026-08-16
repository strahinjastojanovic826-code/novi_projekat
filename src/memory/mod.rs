pub mod init;

use std::alloc::{alloc, dealloc, realloc, Layout};
use std::ptr;
use std::sync::atomic::{AtomicUsize, Ordering};

pub const PAGE_SIZE: usize = 4096; // Standardna x86_64 4KB stranica

// =============================================================================
// 1. PAGE FRAME ALLOCATOR (Upravljanje 4KB Fizičkim/Virtuelnim Stranicama)
// =============================================================================

pub struct PageFrameAllocator {
    pub start_address: usize,
    pub total_pages: usize,
    pub allocated_pages: AtomicUsize,
}

impl PageFrameAllocator {
    pub fn new(start_address: usize, total_pages: usize) -> Self {
        Self {
            start_address,
            total_pages,
            allocated_pages: AtomicUsize::new(0),
        }
    }

    /// Dodeljuje sledeću slobodnu 4KB stranicu
    pub fn alloc_page(&self) -> Option<usize> {
        let current = self.allocated_pages.fetch_add(1, Ordering::Relaxed);
        if current < self.total_pages {
            Some(self.start_address + (current * PAGE_SIZE))
        } else {
            self.allocated_pages.fetch_sub(1, Ordering::Relaxed);
            None // Nema više slobodnog RAM-a (OOM - Out of Memory)
        }
    }

    /// Vraća broj zauzetih stranica
    pub fn used_pages(&self) -> usize {
        self.allocated_pages.load(Ordering::Relaxed)
    }
}

// =============================================================================
// 2. RUČNI ALIGNMENT & PADDING ENGINE (Bitwise kalkulacije poravnanja)
// =============================================================================

pub struct MemoryAligner;

impl MemoryAligner {
    /// Izračunava sledeću memorijsku adresu poravnatu na `align` (mora biti stepen dvojke)
    #[inline(always)]
    pub fn align_up(addr: usize, align: usize) -> usize {
        debug_assert!(align.is_power_of_two(), "Alignment mora biti stepen broja 2!");
        (addr + align - 1) & !(align - 1)
    }

    /// Proverava da li je adresa već poravnata
    #[inline(always)]
    pub fn is_aligned(addr: usize, align: usize) -> bool {
        (addr & (align - 1)) == 0
    }
}

// =============================================================================
// 3. MANUAL POINTER HEAP ENGINE (Sirovo alociranje, realokacija i oslobađanje)
// =============================================================================

pub struct QuantumMemoryEngine {
    pub page_allocator: PageFrameAllocator,
    pub total_heap_bytes: usize,
    used_bytes: AtomicUsize,
    allocation_count: AtomicUsize,
}

impl QuantumMemoryEngine {
    pub fn new(heap_size: usize) -> Self {
        let total_pages = heap_size / PAGE_SIZE;
        Self {
            page_allocator: PageFrameAllocator::new(0x1000000, total_pages), // Mock start na 16MB
            total_heap_bytes: heap_size,
            used_bytes: AtomicUsize::new(0),
            allocation_count: AtomicUsize::new(0),
        }
    }

    /// Ručna alokacija sirovog bloka memorije (Raw Pointer Allocation)
    pub unsafe fn manual_alloc(&self, size: usize, align: usize) -> Result<*mut u8, &'static str> { unsafe {
        if size == 0 {
            return Err("Veličina alokacije ne sme biti 0!");
        }

        let layout = Layout::from_size_align(size, align).map_err(|_| "Nevažeći Layout za poravnanje!")?;
        let ptr = alloc(layout);

        if ptr.is_null() {
            return Err("Out of Memory: Nedovoljno RAM memorije na Heap-u!");
        }

        self.used_bytes.fetch_add(size, Ordering::Relaxed);
        self.allocation_count.fetch_add(1, Ordering::Relaxed);

        Ok(ptr)
    }}

    /// Ručno oslobađanje memorijskog bloka preko sirovog pokazivača
    pub unsafe fn manual_free(&self, ptr: *mut u8, size: usize, align: usize) { unsafe {
        if ptr.is_null() || size == 0 {
            return;
        }

        let layout = Layout::from_size_align(size, align).expect("Nevažeći Layout pri deinstalaciji");
        dealloc(ptr, layout);

        self.used_bytes.fetch_sub(size, Ordering::Relaxed);
        self.allocation_count.fetch_sub(1, Ordering::Relaxed);
    }}

    /// Ručna realokacija (Povećanje ili smanjenje postjećeg bloka u mestu)
    pub unsafe fn manual_realloc(
        &self,
        ptr: *mut u8,
        old_size: usize,
        new_size: usize,
        align: usize,
    ) -> Result<*mut u8, &'static str> { unsafe {
        if ptr.is_null() {
            return self.manual_alloc(new_size, align);
        }

        if new_size == 0 {
            self.manual_free(ptr, old_size, align);
            return Ok(ptr::null_mut());
        }

        let old_layout = Layout::from_size_align(old_size, align).map_err(|_| "Nevažeći star layout!")?;
        let new_ptr = realloc(ptr, old_layout, new_size);

        if new_ptr.is_null() {
            return Err("Neuspešna realokacija memorije!");
        }

        if new_size > old_size {
            self.used_bytes.fetch_add(new_size - old_size, Ordering::Relaxed);
        } else {
            self.used_bytes.fetch_sub(old_size - new_size, Ordering::Relaxed);
        }

        Ok(new_ptr)
    }}

    /// Ručno čišćenje/pisanje nula preko sirovog pokazivača (Zero Fill)
    pub unsafe fn zero_fill(&self, ptr: *mut u8, size: usize) { unsafe {
        if !ptr.is_null() {
            ptr::write_bytes(ptr, 0, size);
        }
    }}

    /// Ručno kopiranje memorije bajt-po-bajt sa jedne adrese na drugu
    pub unsafe fn copy_mem(&self, src: *const u8, dst: *mut u8, count: usize) { unsafe {
        if !src.is_null() && !dst.is_null() {
            ptr::copy_nonoverlapping(src, dst, count);
        }
    }}

    pub fn used_bytes(&self) -> usize {
        self.used_bytes.load(Ordering::Relaxed)
    }

    pub fn active_allocations(&self) -> usize {
        self.allocation_count.load(Ordering::Relaxed)
    }
}