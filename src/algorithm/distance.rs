use crate::graph::matrix::Position;

pub fn manhattan_distance_to(
  from: &Position,
  to: &Position,
) -> usize {
  let dx = from.0.abs_diff(to.0);
  let dy = from.1.abs_diff(to.1);
  dx + dy
}
