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
