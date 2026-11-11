#[derive(Debug, Clone, Copy)]
pub enum NodeType {
    START,
    END,
    PATH,
    WATER,
    WALL,
}

impl NodeType {
    pub fn get_cost_to(&self, node: NodeType) -> Option<u8> {
        match (self, node) {
            (_, NodeType::WALL) => None,
            (NodeType::WALL, _) => None,
            (_, NodeType::WATER) => Some(2),
            (NodeType::WATER, _) => Some(2),
            (_, _) => Some(1),
        }
    }
}

#[derive(Debug)]
pub struct Graph {
    nodes: Vec<NodeType>,
    width: usize,
    height: usize,
}

impl Graph {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            nodes: vec![NodeType::WALL; width * height],
            width,
            height,
        }
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        (y * self.width) + x
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> NodeType {
        self.nodes[self.get_index(x, y)]
    }

    pub fn put(&mut self, x: usize, y: usize, node: NodeType) {
        let index: usize = self.get_index(x, y);
        self.nodes[index] = node;
    }
}
