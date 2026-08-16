use core::sync::atomic::{AtomicU64, Ordering};

// --- 1. MODEL SILICIJUMSKE STRUKTURE I DOPIRANJA ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DopantType {
    NType,
    PType,
    Intrinsic,
}

#[derive(Debug, Clone)]
pub struct SiliconNode {
    pub id: usize,
    pub metal_layer: u8,           // Metalni sloj (M1 - M8)
    pub is_cut_by_fib: bool,        // Presečen provodnik jonizujućim snopom
    pub is_bridged_by_fib: bool,    // Načinjen novi premosnik (FIB Via)
    pub expected_dopant: DopantType,
    pub actual_dopant: DopantType,   // Razlika označava Dopant Trojan!
}

impl SiliconNode {
    pub fn new(id: usize, metal_layer: u8, dopant: DopantType) -> Self {
        Self {
            id,
            metal_layer,
            is_cut_by_fib: false,
            is_bridged_by_fib: false,
            expected_dopant: dopant,
            actual_dopant: dopant,
        }
    }
}

// --- 2. FORENSIC INSPECTION ENGINE ---

pub struct SiliconForensicsEngine {
    pub nodes: Vec<SiliconNode>,
    pub fib_edits_count: AtomicU64,
    pub trojans_detected_count: AtomicU64,
}

impl SiliconForensicsEngine {
    pub fn new(num_nodes: usize) -> Self {
        let mut nodes = Vec::new();
        for i in 0..num_nodes {
            let dopant = if i % 2 == 0 { DopantType::NType } else { DopantType::PType };
            nodes.push(SiliconNode::new(i, (i % 4) as u8 + 1, dopant));
        }

        Self {
            nodes,
            fib_edits_count: AtomicU64::new(0),
            trojans_detected_count: AtomicU64::new(0),
        }
    }

    /// Fizicki FIB rez: Preseca metalni vod na odredjenom sloju
    pub fn execute_fib_cut(&mut self, node_id: usize) {
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.is_cut_by_fib = true;
            self.fib_edits_count.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Fizisko FIB premošćavanje: Pravi novi kontakt izmedju vodova
    pub fn execute_fib_bridge(&mut self, node_id: usize) {
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.is_bridged_by_fib = true;
            self.fib_edits_count.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Injekcija Dopant Trojana (Promena $N \rightarrow P$ ili $P \rightarrow N$ bez izmene topografije)
    pub fn inject_dopant_trojan(&mut self, node_id: usize) {
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.actual_dopant = match node.expected_dopant {
                DopantType::NType => DopantType::PType,
                DopantType::PType => DopantType::NType,
                DopantType::Intrinsic => DopantType::NType,
            };
        }
    }

    /// SEM (Scanning Electron Microscopy) Scan: Detektuje vidljiva mehanička oštećenja (FIB cuts/bridges)
    pub fn scan_sem_topography(&self) -> Vec<(usize, &'static str)> {
        let mut anomalies = Vec::new();
        for node in &self.nodes {
            if node.is_cut_by_fib {
                anomalies.push((node.id, "FIB Structural Cut Detected (Laser/Ion Laser Beam)"));
            }
            if node.is_bridged_by_fib {
                anomalies.push((node.id, "FIB Metal Deposition Bridge Detected"));
            }
        }
        anomalies
    }

    /// SCM (Scanning Capacitance Microscopy) Scan: Detektuje pod-površinski Dopant Trojan
    pub fn scan_scm_dopant_profile(&mut self) -> Vec<(usize, DopantType, DopantType)> {
        let mut trojans = Vec::new();

        for node in &self.nodes {
            if node.expected_dopant != node.actual_dopant {
                trojans.push((node.id, node.expected_dopant, node.actual_dopant));
                self.trojans_detected_count.fetch_add(1, Ordering::Relaxed);
            }
        }

        trojans
    }
}