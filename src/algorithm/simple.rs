use std::time::Instant;

use crate::{
  algorithm::{
    context::Context, distance::manhattan_distance_to,
  },
  graph::NodeType,
  matrix::{Matrix, Position},
};

pub fn a_star(
  matrix: &Matrix<NodeType>,
  start: Position,
  end: Position,
) -> Option<Vec<Position>> {
  println!("[info] Iniciando busca do melhor caminho");

  let mut context = Context::new(start, end);
  let start_time = Instant::now();

  while let Some((current, real_cost)) = context.pop() {
    if current == end {
      break;
    }

    let current_type: NodeType = matrix[current];
    for neighbor in matrix.get_neightbor_of(current) {
      let neighbor_type: NodeType = matrix[neighbor];

      if neighbor_type.is_wall() {
        continue;
      }

      let neighbor_expected_cost =
        manhattan_distance_to(&neighbor, &end);

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
