use std::cmp::Ordering;

use crate::graph::matrix::{Cost, Position};

pub struct Enqueued {
  pub position: Position,
  pub total_cost: Cost,
  pub real_cost: Cost,
}

impl PartialEq for Enqueued {
  fn eq(&self, other: &Self) -> bool {
    self.total_cost == other.total_cost
      && self.real_cost == other.real_cost
      && self.position == other.position
  }
}

impl Eq for Enqueued {}

impl Ord for Enqueued {
  fn cmp(&self, other: &Self) -> Ordering {
    self
      .total_cost
      .cmp(&other.total_cost)
      .then_with(|| other.real_cost.cmp(&self.real_cost))
      .then_with(|| self.position.cmp(&other.position))
  }
}

impl PartialOrd for Enqueued {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}
