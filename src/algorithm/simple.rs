use std::time::Instant;

use crate::{
  algorithm::{
    context::Context, distance::manhattan_distance_to,
  },
  graph::matrix::{Matrix, Position},
  graph::nodes::NodeType,
};

pub struct AStar<'a> {
  matrix: &'a Matrix<NodeType>,
  start: Position,
  end: Position,
}

impl<'a> AStar<'a> {
  pub fn new(
    matrix: &'a Matrix<NodeType>,
    start: Position,
    end: Position,
  ) -> AStar<'a> {
    Self { matrix, start, end }
  }

  pub fn start(&mut self) -> Option<Vec<Position>> {
    self.execute(|_| {})
  }

  pub fn start_keeping_history(
    &mut self,
  ) -> (Option<Vec<Position>>, Vec<Position>) {
    let mut history: Vec<Position> = Vec::new();

    let result =
      self.execute(|position| history.push(position));

    (result, history)
  }

  fn execute(
    &mut self,
    mut on_visit: impl FnMut(Position),
  ) -> Option<Vec<Position>> {
    println!("[info] Iniciando busca do melhor caminho");

    let mut context = Context::new(self.start, self.end);
    let start_time = Instant::now();

    while let Some((current, real_cost)) = context.pop() {
      on_visit(current);
      if current == self.end {
        break;
      }

      let current_type: NodeType = self.matrix[current];
      let neighbors = self.matrix.get_neightbor_of(current);
      for neighbor in neighbors {
        let neighbor_type: NodeType = self.matrix[neighbor];

        if neighbor_type.is_wall() {
          continue;
        }

        let neighbor_expected_cost =
          manhattan_distance_to(&neighbor, &self.end);

        let neighbor_real_cost = real_cost
          + current_type.get_cost_to(neighbor_type).unwrap()
            as usize;

        let is_better = context
          .is_cost_better(neighbor, neighbor_real_cost);

        if is_better {
          context.set_predecessor(neighbor, current);
          context.push(
            neighbor,
            neighbor_real_cost,
            neighbor_expected_cost,
          );
        }
      }
    }

    let result = context.get_better_path();

    println!(
      "[time] Resposta encontrada em {:?}",
      start_time.elapsed()
    );

    result
  }
}
