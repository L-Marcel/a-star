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

    fn get_index(&self, column: usize, row: usize) -> usize {
        (row * self.width) + column
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get(&self, column: usize, row: usize) -> NodeType {
        self.nodes[self.get_index(column, row)]
    }

    pub fn put(&mut self, column: usize, row: usize, node: NodeType) {
        let index: usize = self.get_index(column, row);
        self.nodes[index] = node;
    }
}
