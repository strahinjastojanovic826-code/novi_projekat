use std::collections::HashSet;

// =============================================================================
// 1. VAN EMDE BOAS (vEB) RECURSIVE LAYOUT ENGINE
// =============================================================================

#[derive(Debug, Clone)]
pub struct VebTreeEngine {
    pub height: usize,
    pub size: usize,
    pub veb_nodes: Vec<i64>, // Rekurzivni vEB raspored u memoriji
    pub bfs_nodes: Vec<i64>, // Standardni Heap/BFS raspored u memoriji
}

impl VebTreeEngine {
    pub fn new(height: usize) -> Self {
        let size = (1 << height) - 1;
        let mut bfs_nodes = vec![0i64; size];

        // Popunjavamo balansirano stablo sortiranim vrednostima radi binarne pretrage
        let mut sorted_vals: Vec<i64> = (1..=(size as i64)).collect();
        Self::fill_bfs_tree(&mut bfs_nodes, &mut sorted_vals, 0, 0, height);

        let mut veb_nodes = vec![0i64; size];
        let mut veb_map = vec![0usize; size];
        let mut current_veb_idx = 0;

        Self::build_veb_mapping(0, height, &mut current_veb_idx, &mut veb_map);

        for bfs_idx in 0..size {
            let target_veb_idx = veb_map[bfs_idx];
            veb_nodes[target_veb_idx] = bfs_nodes[bfs_idx];
        }

        Self {
            height,
            size,
            veb_nodes,
            bfs_nodes,
        }
    }

    /// In-order popunjavanje BFS stabla sortiranim elementima
    fn fill_bfs_tree(
        bfs: &mut [i64],
        vals: &mut Vec<i64>,
        bfs_idx: usize,
        depth: usize,
        max_depth: usize,
    ) {
        if depth >= max_depth || bfs_idx >= bfs.len() {
            return;
        }

        // Levo dete
        Self::fill_bfs_tree(bfs, vals, 2 * bfs_idx + 1, depth + 1, max_depth);

        // Koren
        if !vals.is_empty() {
            bfs[bfs_idx] = vals.remove(0);
        }

        // Desno dete
        Self::fill_bfs_tree(bfs, vals, 2 * bfs_idx + 2, depth + 1, max_depth);
    }

    /// Rekurzivna izgradnja van Emde Boas mapiranja indeksa
    fn build_veb_mapping(
        bfs_idx: usize,
        h: usize,
        veb_counter: &mut usize,
        veb_map: &mut [usize],
    ) {
        if h == 1 {
            veb_map[bfs_idx] = *veb_counter;
            *veb_counter += 1;
            return;
        }

        let h_top = (h + 1) / 2;
        let h_bot = h - h_top;

        // 1. Rekurzivno mapiramo GORNJE podstablo
        Self::build_veb_mapping(bfs_idx, h_top, veb_counter, veb_map);

        // 2. Rekurzivno mapiramo sva DONJA podstabla
        Self::map_bottom_subtrees(bfs_idx, 0, h_top, h_bot, veb_counter, veb_map);
    }

    fn map_bottom_subtrees(
        current_bfs: usize,
        current_depth: usize,
        target_depth: usize,
        h_bot: usize,
        veb_counter: &mut usize,
        veb_map: &mut [usize],
    ) {
        if current_depth == target_depth - 1 {
            let left_child = 2 * current_bfs + 1;
            let right_child = 2 * current_bfs + 2;

            if left_child < veb_map.len() {
                Self::build_veb_mapping(left_child, h_bot, veb_counter, veb_map);
            }
            if right_child < veb_map.len() {
                Self::build_veb_mapping(right_child, h_bot, veb_counter, veb_map);
            }
            return;
        }

        let left = 2 * current_bfs + 1;
        let right = 2 * current_bfs + 2;
        if left < veb_map.len() {
            Self::map_bottom_subtrees(left, current_depth + 1, target_depth, h_bot, veb_counter, veb_map);
        }
        if right < veb_map.len() {
            Self::map_bottom_subtrees(right, current_depth + 1, target_depth, h_bot, veb_counter, veb_map);
        }
    }
}

// =============================================================================
// 2. CACHE MISS SIMULATOR (64-byte L1 Cache Line Simulation)
// =============================================================================

pub struct CacheSimulator;

impl CacheSimulator {
    /// Izračunava broj jedinstvenih L1 keš linija koje se povlače iz RAM-a pri pretrazi
    /// Pretpostavka: 1 element = 8 bajtova (u64/i64). Keš linija = 64 bajta (8 elemenata po liniji).
    pub fn simulate_search_misses(
        tree: &VebTreeEngine,
        target_val: i64,
        elements_per_cache_line: usize,
    ) -> (usize, usize) {
        let mut bfs_cache_lines = HashSet::new();
        let mut veb_cache_lines = HashSet::new();

        // 1. Pretraga u BFS rasporedu
        let mut curr_bfs = 0;
        while curr_bfs < tree.bfs_nodes.len() {
            let line_id = curr_bfs / elements_per_cache_line;
            bfs_cache_lines.insert(line_id);

            let val = tree.bfs_nodes[curr_bfs];
            if target_val == val {
                break;
            } else if target_val < val {
                curr_bfs = 2 * curr_bfs + 1;
            } else {
                curr_bfs = 2 * curr_bfs + 2;
            }
        }

        // 2. Pretraga u vEB rasporedu (Mapiramo BFS putanju u vEB indeks)
        let mut veb_map = vec![0usize; tree.size];
        let mut veb_counter = 0;
        VebTreeEngine::build_veb_mapping(0, tree.height, &mut veb_counter, &mut veb_map);

        curr_bfs = 0;
        while curr_bfs < tree.bfs_nodes.len() {
            let veb_idx = veb_map[curr_bfs];
            let line_id = veb_idx / elements_per_cache_line;
            veb_cache_lines.insert(line_id);

            let val = tree.bfs_nodes[curr_bfs];
            if target_val == val {
                break;
            } else if target_val < val {
                curr_bfs = 2 * curr_bfs + 1;
            } else {
                curr_bfs = 2 * curr_bfs + 2;
            }
        }

        (bfs_cache_lines.len(), veb_cache_lines.len())
    }
}