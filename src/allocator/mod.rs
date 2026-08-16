use std::alloc::Layout;
use std::ptr;
use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};

// =============================================================================
// 1. ARENA (BUMP) ALLOCATOR (Brzo usmereno alociranje + $O(1)$ Bulk Reset)
// =============================================================================

pub struct ArenaAllocator {
    buffer: Vec<u8>,
    offset: AtomicUsize,
    capacity: usize,
}

impl ArenaAllocator {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: vec![0; capacity],
            offset: AtomicUsize::new(0),
            capacity,
        }
    }

    /// Alocira memoriju pomeranjem Bump pointera unapred sa poravnanjem
    pub unsafe fn alloc(&self, layout: Layout) -> Result<*mut u8, &'static str> {
        let size = layout.size();
        let align = layout.align();

        loop {
            let current_offset = self.offset.load(Ordering::Relaxed);
            let base_ptr = self.buffer.as_ptr() as usize + current_offset;
            
            // Bitwise kalkulacija poravnanja
            let aligned_ptr = (base_ptr + align - 1) & !(align - 1);
            let padding = aligned_ptr - base_ptr;
            let new_offset = current_offset + padding + size;

            if new_offset > self.capacity {
                return Err("Arena Allocator: Nema dovoljno kapaciteta!");
            }

            if self
                .offset
                .compare_exchange_weak(
                    current_offset,
                    new_offset,
                    Ordering::Release,
                    Ordering::Relaxed,
                )
                .is_ok()
            {
                return Ok(aligned_ptr as *mut u8);
            }
        }
    }

    /// O(1) Instant oslobađanje kompletne memorije re-setovanjem bump offseta na 0
    pub fn reset(&self) {
        self.offset.store(0, Ordering::Release);
    }

    pub fn used_bytes(&self) -> usize {
        self.offset.load(Ordering::Relaxed)
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

// =============================================================================
// 2. EMBEDDED FREE-LIST POOL ALLOCATOR (Lock-Free, Nula fragmentacije, O(1))
// =============================================================================

pub struct FixedPoolAllocator<const CHUNK_SIZE: usize> {
    _raw_buffer: Vec<u8>,
    head: AtomicPtr<u8>,
    total_chunks: usize,
    free_chunks: AtomicUsize,
}

impl<const CHUNK_SIZE: usize> FixedPoolAllocator<CHUNK_SIZE> {
    pub fn new(total_chunks: usize) -> Self {
        // Svaki blok mora biti dovoljno velik da primi bar jedan sirovi pokazivač (8 bajtova)
        assert!(
            CHUNK_SIZE >= std::mem::size_of::<*mut u8>(),
            "Veličina bloka mora biti najmanje 8 bajtova!"
        );

        let total_bytes = CHUNK_SIZE * total_chunks;
        let mut raw_buffer = vec![0u8; total_bytes];

        // Izgradnja ugrađene spregnute liste (Embedded Free-List) unutar samog bafera
        unsafe {
            let base_ptr = raw_buffer.as_mut_ptr();
            for i in 0..(total_chunks - 1) {
                let current_chunk = base_ptr.add(i * CHUNK_SIZE) as *mut *mut u8;
                let next_chunk = base_ptr.add((i + 1) * CHUNK_SIZE);
                *current_chunk = next_chunk;
            }

            // Poslednji blok pokazuje na NULL
            let last_chunk = base_ptr.add((total_chunks - 1) * CHUNK_SIZE) as *mut *mut u8;
            *last_chunk = ptr::null_mut();
        }

        let initial_head = raw_buffer.as_mut_ptr();

        Self {
            _raw_buffer: raw_buffer,
            head: AtomicPtr::new(initial_head),
            total_chunks,
            free_chunks: AtomicUsize::new(total_chunks),
        }
    }

    /// Alocira jedan fiksni blok u O(1) vremenu iz ugrađene liste slobodnih blokova
    pub fn alloc_chunk(&self) -> Result<*mut u8, &'static str> {
        loop {
            let current_head = self.head.load(Ordering::Acquire);

            if current_head.is_null() {
                return Err("Pool Allocator: Nema slobodnih blokova na raspolaganju!");
            }

            // Učitavamo pokazivač na sledeći slobodan blok koji je upisan UNUTAR trenutnog bloka
            let next_head = unsafe { *(current_head as *const *mut u8) };

            if self
                .head
                .compare_exchange_weak(
                    current_head,
                    next_head,
                    Ordering::Release,
                    Ordering::Relaxed,
                )
                .is_ok()
            {
                self.free_chunks.fetch_sub(1, Ordering::Relaxed);
                return Ok(current_head);
            }
        }
    }

    /// Vraća alocirani blok u slobodni Pool u O(1) vremenu bez ikakve fragmentacije
    pub unsafe fn free_chunk(&self, ptr: *mut u8) { unsafe {
        if ptr.is_null() {
            return;
        }

        loop {
            let current_head = self.head.load(Ordering::Acquire);

            // Upisujemo trenutni head u prvi deo bloka koji vraćamo
            *(ptr as *mut *mut u8) = current_head;

            // Atomska zamena glave liste
            if self
                .head
                .compare_exchange_weak(
                    current_head,
                    ptr,
                    Ordering::Release,
                    Ordering::Relaxed,
                )
                .is_ok()
            {
                self.free_chunks.fetch_add(1, Ordering::Relaxed);
                break;
            }
        }
    }}

    pub fn free_chunks_count(&self) -> usize {
        self.free_chunks.load(Ordering::Relaxed)
    }

    pub fn total_chunks_count(&self) -> usize {
        self.total_chunks
    }
}

unsafe impl<const CHUNK_SIZE: usize> Send for FixedPoolAllocator<CHUNK_SIZE> {}
unsafe impl<const CHUNK_SIZE: usize> Sync for FixedPoolAllocator<CHUNK_SIZE> {}