use super::node::BlockPos;

/// Эвристика для pathfinding
pub trait Heuristic {
    fn calculate(&self, from: &BlockPos, to: &BlockPos) -> f64;
}

/// Евклидово расстояние
pub struct EuclideanHeuristic;

impl Heuristic for EuclideanHeuristic {
    fn calculate(&self, from: &BlockPos, to: &BlockPos) -> f64 {
        from.distance_to(to)
    }
}

/// Манхэттенское расстояние
pub struct ManhattanHeuristic;

impl Heuristic for ManhattanHeuristic {
    fn calculate(&self, from: &BlockPos, to: &BlockPos) -> f64 {
        from.manhattan_distance_to(to) as f64
    }
}

/// Диагональное расстояние (Chebyshev)
pub struct DiagonalHeuristic;

impl Heuristic for DiagonalHeuristic {
    fn calculate(&self, from: &BlockPos, to: &BlockPos) -> f64 {
        let dx = (from.x - to.x).abs() as f64;
        let dy = (from.y - to.y).abs() as f64;
        let dz = (from.z - to.z).abs() as f64;
        dx.max(dy).max(dz)
    }
}
