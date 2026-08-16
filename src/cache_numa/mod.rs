use std::sync::atomic::AtomicU64;

// =============================================================================
// 1. DATA STRUCTURES: FALSE SHARING VS CACHE-PADDED
// =============================================================================

/// Dva brojača u ISTOJ keš liniji od 64 bajta -> Uzrokuje FALSE SHARING!
#[repr(C)]
pub struct FalseSharingCounters {
    pub thread_0_val: AtomicU64, // 8 bajtova
    pub thread_1_val: AtomicU64, // 8 bajtova
    // Preostalih 48 bajtova u istoj keš liniji popunjavaju drugi podaci
}

impl FalseSharingCounters {
    pub const fn new() -> Self {
        Self {
            thread_0_val: AtomicU64::new(0),
            thread_1_val: AtomicU64::new(0),
        }
    }
}

/// Brojači sa poravnanjem na granicu od 64 bajta -> SPREČAVA FALSE SHARING!
#[repr(C, align(64))]
pub struct CachePaddedCounter {
    pub val: AtomicU64,
    // Rust automatski dodaje 56 bajtova paddinga do punih 64 bajta!
}

impl CachePaddedCounter {
    pub const fn new() -> Self {
        Self {
            val: AtomicU64::new(0),
        }
    }
}

// =============================================================================
// 2. NUMA TOPOLOGY & LATENCY SIMULATOR
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumaNode {
    Node0_Local,
    Node1_RemoteSocket,
}

pub struct NumaLatencyModel;

impl NumaLatencyModel {
    /// Vraća procenjeno vreme pristupa u nanosekundama zavisno od keš stanja i NUMA čvora
    pub fn estimate_access_latency(
        is_false_sharing: bool,
        node: NumaNode,
        bouncing_contentions: u32,
    ) -> u64 {
        let base_latency = match node {
            NumaNode::Node0_Local => 1, // L1 Cache hit (1ns)
            NumaNode::Node1_RemoteSocket => 90, // Remote RAM/L3 Interconnect hit (90ns)
        };

        if !is_false_sharing {
            return base_latency;
        }

        // Akumulirani penal za MESI Invalidate poruke i Interconnect Bouncing
        let bounce_penalty = match node {
            NumaNode::Node0_Local => bouncing_contentions as u64 * 15, // L1/L3 Invalidation stall
            NumaNode::Node1_RemoteSocket => bouncing_contentions as u64 * 220, // QPI/UPI Interconnect Traffic Stall!
        };

        base_latency + bounce_penalty
    }
}