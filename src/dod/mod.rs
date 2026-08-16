// =============================================================================
// 1. DATA-ORIENTED DESIGN (SoA - Structure of Arrays vs AoS)
// =============================================================================

/// Klasični AoS (Array of Structures) - Loše za CPU keš kada se filtrira samo 1 polje
#[derive(Debug, Clone)]
pub struct ProcessAoS {
    pub pid: u32,
    pub priority: u8,
    pub cpu_usage: f32,
    pub name: [u8; 32], // Ogroman footprint koji prlja L1 keš liniju!
}

/// Data-Oriented SoA (Structure of Arrays) - Sva polja su u sukcesivnim nizovima u RAM-u!
#[derive(Debug, Clone, Default)]
pub struct ProcessTableSoA {
    pub pids: Vec<u32>,
    pub priorities: Vec<u8>,
    pub cpu_usages: Vec<f32>,
    pub is_active: Vec<bool>,
}

impl ProcessTableSoA {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            pids: Vec::with_capacity(capacity),
            priorities: Vec::with_capacity(capacity),
            cpu_usages: Vec::with_capacity(capacity),
            is_active: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, pid: u32, priority: u8, cpu_usage: f32, active: bool) {
        self.pids.push(pid);
        self.priorities.push(priority);
        self.cpu_usages.push(cpu_usage);
        self.is_active.push(active);
    }

    /// DOD SoA Obrada: Povećava CPU cikluse SAMO aktivnim procesima.
    /// Učitava isključivo `is_active` i `cpu_usages` nizove u CPU L1 Keš!
    pub fn update_active_cpu_usages(&mut self, delta: f32) {
        let len = self.pids.len();
        for i in 0..len {
            if self.is_active[i] {
                self.cpu_usages[i] += delta;
            }
        }
    }
}

// =============================================================================
// 2. CACHE-OBLIVIOUS MATRIX ALGORITHMS (Divide-and-Conquer)
// =============================================================================

pub struct QuantumDodEngine;

impl QuantumDodEngine {
    /// Rekurzivna Cache-Oblivious Transpozicija Matrice.
    /// Deli matricu na 4 manje podmatrice sve dok ne stane u najsitingiji CPU keš.
    /// Rad sa matricom dimenzija N x M bez ikakvih parametara o veličini L1/L2 keša!
    pub fn cache_oblivious_transpose(
        src: &[f32],
        dst: &mut [f32],
        row_stride_src: usize,
        row_stride_dst: usize,
        src_row: usize,
        src_col: usize,
        dst_row: usize,
        dst_col: usize,
        rows: usize,
        cols: usize,
    ) {
        // Bazni slučaj: Ako je blok dovoljno mali (npr. <= 16x16), vrši se direktno kopiranje
        const BASE_THRESHOLD: usize = 16;

        if rows <= BASE_THRESHOLD && cols <= BASE_THRESHOLD {
            for r in 0..rows {
                for c in 0..cols {
                    let src_idx = (src_row + r) * row_stride_src + (src_col + c);
                    let dst_idx = (dst_row + c) * row_stride_dst + (dst_col + r);
                    dst[dst_idx] = src[src_idx];
                }
            }
            return;
        }

        // Divide and Conquer: Delimo dimenziju koja je veća
        if rows >= cols {
            let half_rows = rows / 2;
            // Gornja polovina
            Self::cache_oblivious_transpose(
                src, dst, row_stride_src, row_stride_dst,
                src_row, src_col, dst_row, dst_col,
                half_rows, cols,
            );
            // Donja polovina
            Self::cache_oblivious_transpose(
                src, dst, row_stride_src, row_stride_dst,
                src_row + half_rows, src_col, dst_row, dst_col + half_rows,
                rows - half_rows, cols,
            );
        } else {
            let half_cols = cols / 2;
            // Leva polovina
            Self::cache_oblivious_transpose(
                src, dst, row_stride_src, row_stride_dst,
                src_row, src_col, dst_row, dst_col,
                rows, half_cols,
            );
            // Desna polovina
            Self::cache_oblivious_transpose(
                src, dst, row_stride_src, row_stride_dst,
                src_row, src_col + half_cols, dst_row + half_cols, dst_col,
                rows, cols - half_cols,
            );
        }
    }
}