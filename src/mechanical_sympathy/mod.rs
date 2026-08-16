use std::sync::atomic::AtomicU64;

// =============================================================================
// 1. MECHANICAL SYMPATHY: CACHE LINE ALIGNMENT (Spriječavanje False Sharing-a)
// =============================================================================

/// Nezaštićeni brojači: Leže jedan pored drugog u istoj 64-bajtnoj L1 keš liniji.
/// Izazivaju "Cache Invalidation Storm" kada se ažuriraju iz više niti!
#[derive(Debug)]
pub struct UnpaddedCounters {
    pub counter_a: AtomicU64,
    pub counter_b: AtomicU64,
}

impl UnpaddedCounters {
    pub fn new() -> Self {
        Self {
            counter_a: AtomicU64::new(0),
            counter_b: AtomicU64::new(0),
        }
    }
}

/// Aligned brojač: #[repr(align(64))] primorava kompajler da svakom brojaču
/// dodeli zasebnu L1 Keš liniju (64 bajta).
#[repr(align(64))]
#[derive(Debug)]
pub struct CacheAlignedCounter(pub AtomicU64);

#[derive(Debug)]
pub struct PaddedCounters {
    pub counter_a: CacheAlignedCounter,
    pub counter_b: CacheAlignedCounter,
}

impl PaddedCounters {
    pub fn new() -> Self {
        Self {
            counter_a: CacheAlignedCounter(AtomicU64::new(0)),
            counter_b: CacheAlignedCounter(AtomicU64::new(0)),
        }
    }
}

// =============================================================================
// 2. MECHANICAL SYMPATHY: CACHE LOCALITY (Row-Major vs Column-Major)
// =============================================================================

pub struct MemoryStrideTester;

impl MemoryStrideTester {
    /// Sequential / Row-Major: Savršeno prate L1 Prefetcher (Sledstveni bajtovi)
    pub fn row_major_sum(matrix: &[Vec<u64>], size: usize) -> u64 {
        let mut sum = 0;
        for r in 0..size {
            for c in 0..size {
                sum += matrix[r][c];
            }
        }
        sum
    }

    /// Strided / Column-Major: Skakanje po memoriji -> Izaziva L1 Cache Misses!
    pub fn col_major_sum(matrix: &[Vec<u64>], size: usize) -> u64 {
        let mut sum = 0;
        for c in 0..size {
            for r in 0..size {
                sum += matrix[r][c];
            }
        }
        sum
    }
}

// =============================================================================
// 3. ZERO-COST ABSTRACTIONS: HIGH-LEVEL ITERATORS VS MANUAL LOOPS
// =============================================================================

pub struct ZeroCostTester;

impl ZeroCostTester {
    /// Elegantni functional pipeline (Zero-Cost Abstraction)
    /// Kompajler ga monomorfizuje i odmotava u identičan SIMD/Asembler kod!
    pub fn functional_iterator(data: &[u64]) -> u64 {
        data.iter()
            .filter(|&&x| x % 2 == 0)
            .map(|&x| x * 3)
            .fold(0, |acc, x| acc + x)
    }

    /// Ručna, rovana imperative loop varijanta
    pub fn manual_loop(data: &[u64]) -> u64 {
        let mut acc = 0;
        let len = data.len();
        let mut i = 0;
        while i < len {
            let x = data[i];
            if x % 2 == 0 {
                acc += x * 3;
            }
            i += 1;
        }
        acc
    }
}