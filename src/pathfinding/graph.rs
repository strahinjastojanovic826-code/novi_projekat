
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlgorithmType {
    AStar,
    Dijkstra,
    Bfs,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CellType {
    Empty,
    Start,
    Target,
    Wall,
    Path,
    Visited,
}

#[derive(Debug, Clone)]
pub struct PathResult {
    pub path: Vec<(usize, usize)>,
    pub visited_count: usize,
    pub total_cost: f32,
    pub success: bool,
}

pub struct GridGraph {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<CellType>>,
}

impl GridGraph {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            grid: vec![vec![CellType::Empty; width]; height],
        }
    }

    pub fn reset_results(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid[y][x] == CellType::Path || self.grid[y][x] == CellType::Visited {
                    self.grid[y][x] = CellType::Empty;
                }
            }
        }
    }

    /// Manhattan Heuristika za A* algoritam
    fn heuristic(a: (usize, usize), b: (usize, usize)) -> f32 {
        (a.0 as f32 - b.0 as f32).abs() + (a.1 as f32 - b.1 as f32).abs()
    }

    /// Pronalaženje putanje pomoću A* (A-Star) algoritma
    pub fn solve_a_star(&mut self, start: (usize, usize), target: (usize, usize)) -> PathResult {
        self.reset_results();

        let mut open_set = vec![start];
        let mut came_from = std::collections::HashMap::new();

        let mut g_score = vec![vec![f32::INFINITY; self.width]; self.height];
        let mut f_score = vec![vec![f32::INFINITY; self.width]; self.height];

        g_score[start.1][start.0] = 0.0;
        f_score[start.1][start.0] = Self::heuristic(start, target);

        let mut visited_count = 0;

        while !open_set.is_empty() {
            // Pronađi čvor sa najmanjim f_score-om
            open_set.sort_by(|a, b| f_score[a.1][a.0].partial_cmp(&f_score[b.1][b.0]).unwrap());
            let current = open_set.remove(0);

            if current == target {
                // Reconstruct Path
                let mut path = vec![current];
                let mut curr = current;
                while let Some(&prev) = came_from.get(&curr) {
                    curr = prev;
                    path.push(curr);
                }
                path.reverse();

                for &(px, py) in &path {
                    if (px, py) != start && (px, py) != target {
                        self.grid[py][px] = CellType::Path;
                    }
                }

                return PathResult {
                    path: path.clone(),
                    visited_count,
                    total_cost: g_score[target.1][target.0],
                    success: true,
                };
            }

            visited_count += 1;
            if current != start && current != target {
                self.grid[current.1][current.0] = CellType::Visited;
            }

            // Susedni čvorovi (gore, dole, levo, desno)
            let neighbors = self.get_neighbors(current);
            for neighbor in neighbors {
                let tentative_g = g_score[current.1][current.0] + 1.0;

                if tentative_g < g_score[neighbor.1][neighbor.0] {
                    came_from.insert(neighbor, current);
                    g_score[neighbor.1][neighbor.0] = tentative_g;
                    f_score[neighbor.1][neighbor.0] = tentative_g + Self::heuristic(neighbor, target);

                    if !open_set.contains(&neighbor) {
                        open_set.push(neighbor);
                    }
                }
            }
        }

        PathResult {
            path: Vec::new(),
            visited_count,
            total_cost: 0.0,
            success: false,
        }
    }

    fn get_neighbors(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        let (x, y) = (pos.0 as i32, pos.1 as i32);

        let dirs = [(0, -1), (0, 1), (-1, 0), (1, 0)];
        for (dx, dy) in dirs {
            let nx = x + dx;
            let ny = y + dy;

            if nx >= 0 && nx < self.width as i32 && ny >= 0 && ny < self.height as i32 {
                let ux = nx as usize;
                let uy = ny as usize;
                if self.grid[uy][ux] != CellType::Wall {
                    neighbors.push((ux, uy));
                }
            }
        }

        neighbors
    }
}