pub mod graph;

use graph::{AlgorithmType, CellType, GridGraph, PathResult};

pub struct QuantumPathfinderEngine {
    pub graph: GridGraph,
    pub start_pos: (usize, usize),
    pub target_pos: (usize, usize),
    pub active_algorithm: AlgorithmType,
    pub last_result: Option<PathResult>,
    pub logs: Vec<String>,
}

impl QuantumPathfinderEngine {
    pub fn new() -> Self {
        let mut graph = GridGraph::new(10, 10);
        let start = (0, 0);
        let target = (9, 9);

        graph.grid[start.1][start.0] = CellType::Start;
        graph.grid[target.1][target.0] = CellType::Target;

        // Podrazumevani zidovi za demonstraciju
        graph.grid[2][2] = CellType::Wall;
        graph.grid[2][3] = CellType::Wall;
        graph.grid[2][4] = CellType::Wall;
        graph.grid[3][4] = CellType::Wall;
        graph.grid[4][4] = CellType::Wall;

        let mut engine = Self {
            graph,
            start_pos: start,
            target_pos: target,
            active_algorithm: AlgorithmType::AStar,
            last_result: None,
            logs: Vec::new(),
        };

        engine.logs.push("Inicijalizovan Pathfinder Engine [Mreža 10x10 učitana].".into());
        engine
    }

    pub fn toggle_cell(&mut self, x: usize, y: usize) {
        if (x, y) == self.start_pos || (x, y) == self.target_pos {
            return;
        }

        if self.graph.grid[y][x] == CellType::Wall {
            self.graph.grid[y][x] = CellType::Empty;
        } else {
            self.graph.grid[y][x] = CellType::Wall;
        }
    }

    pub fn solve(&mut self) {
        let res = match self.active_algorithm {
            AlgorithmType::AStar | AlgorithmType::Dijkstra | AlgorithmType::Bfs => {
                self.graph.solve_a_star(self.start_pos, self.target_pos)
            }
        };

        let status = if res.success {
            format!("Putanja pronađena! Dužina: {} koraka | Istraženo čvorova: {}", res.path.len(), res.visited_count)
        } else {
            "NEMA PUTANJE: Cilj je blokiran preprekama!".to_string()
        };

        self.logs.push(format!("[{:?}] {}", self.active_algorithm, status));
        self.last_result = Some(res);
    }

    pub fn clear_walls(&mut self) {
        for y in 0..self.graph.height {
            for x in 0..self.graph.width {
                if self.graph.grid[y][x] == CellType::Wall {
                    self.graph.grid[y][x] = CellType::Empty;
                }
            }
        }
        self.graph.reset_results();
        self.logs.push("Uklonjene sve prepreke sa mreže.".into());
    }
}