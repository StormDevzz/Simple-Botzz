use super::node::{BlockPos, PathNode};
use super::heuristic::{Heuristic, EuclideanHeuristic};
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

/// Pathfinder для поиска пути (A* алгоритм)
pub struct Pathfinder {
    heuristic: Box<dyn Heuristic>,
    max_iterations: usize,
}

impl Pathfinder {
    pub fn new() -> Self {
        Self {
            heuristic: Box::new(EuclideanHeuristic),
            max_iterations: 10000,
        }
    }

    pub fn with_heuristic(mut self, heuristic: Box<dyn Heuristic>) -> Self {
        self.heuristic = heuristic;
        self
    }

    pub fn with_max_iterations(mut self, max: usize) -> Self {
        self.max_iterations = max;
        self
    }

    /// Находит путь от start до goal
    pub fn find_path(&self, start: BlockPos, goal: BlockPos, is_walkable: impl Fn(&BlockPos) -> bool) -> Option<Vec<BlockPos>> {
        let mut open_set = BinaryHeap::new();
        let mut came_from: HashMap<BlockPos, BlockPos> = HashMap::new();
        let mut g_score: HashMap<BlockPos, f64> = HashMap::new();
        let mut f_score: HashMap<BlockPos, f64> = HashMap::new();

        g_score.insert(start, 0.0);
        f_score.insert(start, self.heuristic.calculate(&start, &goal));

        open_set.push(PathNode::new(start).with_costs(0.0, *f_score.get(&start).unwrap()));

        let mut iterations = 0;

        while let Some(current) = open_set.pop() {
            iterations += 1;
            if iterations > self.max_iterations {
                return None; // Превышен лимит итераций
            }

            if current.position == goal {
                return self.reconstruct_path(&came_from, current.position);
            }

            for neighbor in self.get_neighbors(&current.position) {
                if !is_walkable(&neighbor) {
                    continue;
                }

                let tentative_g = *g_score.get(&current.position).unwrap_or(&0.0) + 1.0;

                if tentative_g < *g_score.get(&neighbor).unwrap_or(&f64::MAX) {
                    came_from.insert(neighbor, current.position);
                    g_score.insert(neighbor, tentative_g);
                    let f = tentative_g + self.heuristic.calculate(&neighbor, &goal);
                    f_score.insert(neighbor, f);

                    open_set.push(PathNode::new(neighbor).with_costs(tentative_g, f));
                }
            }
        }

        None // Путь не найден
    }

    fn reconstruct_path(&self, came_from: &HashMap<BlockPos, BlockPos>, current: BlockPos) -> Option<Vec<BlockPos>> {
        let mut path = vec![current];
        let mut mut_current = current;

        while let Some(&prev) = came_from.get(&mut_current) {
            path.push(prev);
            mut_current = prev;
        }

        path.reverse();
        Some(path)
    }

    fn get_neighbors(&self, pos: &BlockPos) -> Vec<BlockPos> {
        vec![
            BlockPos::new(pos.x + 1, pos.y, pos.z),
            BlockPos::new(pos.x - 1, pos.y, pos.z),
            BlockPos::new(pos.x, pos.y + 1, pos.z),
            BlockPos::new(pos.x, pos.y - 1, pos.z),
            BlockPos::new(pos.x, pos.y, pos.z + 1),
            BlockPos::new(pos.x, pos.y, pos.z - 1),
        ]
    }
}

impl Default for Pathfinder {
    fn default() -> Self {
        Self::new()
    }
}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_cost().partial_cmp(&self.f_cost()).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for PathNode {}

impl PartialEq for PathNode {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}
