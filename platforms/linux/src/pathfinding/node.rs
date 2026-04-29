use std::fmt;

/// Координаты блока в мире
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl BlockPos {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn distance_to(&self, other: &BlockPos) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        let dz = (self.z - other.z) as f64;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    pub fn manhattan_distance_to(&self, other: &BlockPos) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

impl fmt::Display for BlockPos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

/// Узел для pathfinding
#[derive(Debug, Clone)]
pub struct PathNode {
    pub position: BlockPos,
    pub g_cost: f64, // Стоимость от старта
    pub h_cost: f64, // Эвристическая стоимость до цели
    pub parent: Option<Box<PathNode>>,
}

impl PathNode {
    pub fn new(position: BlockPos) -> Self {
        Self {
            position,
            g_cost: 0.0,
            h_cost: 0.0,
            parent: None,
        }
    }

    pub fn f_cost(&self) -> f64 {
        self.g_cost + self.h_cost
    }

    pub fn with_costs(mut self, g: f64, h: f64) -> Self {
        self.g_cost = g;
        self.h_cost = h;
        self
    }

    pub fn with_parent(mut self, parent: PathNode) -> Self {
        self.parent = Some(Box::new(parent));
        self
    }
}
