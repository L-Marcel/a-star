use std::{
  cmp::Reverse,
  collections::{BinaryHeap, HashMap},
};

use crate::{
  algorithm::{
    distance::manhattan_distance_to, enqueued::Enqueued,
  },
  graph::matrix::{Cost, Position},
};

pub struct Context {
  min_heap: BinaryHeap<Reverse<Enqueued>>,
  min_cost: HashMap<Position, usize>,
  predecessor: HashMap<Position, Position>,
  start: Position,
}

impl Context {
  pub fn new(start: Position, end: Position) -> Context {
    let mut context = Self {
      min_heap: BinaryHeap::new(),
      min_cost: HashMap::new(),
      predecessor: HashMap::new(),
      start,
    };

    let expected_cost = manhattan_distance_to(&start, &end);
    context.predecessor.insert(start, start);
    context.push(start, 0, expected_cost);
    context
  }

  pub fn get_path_to(
    &self,
    target: Position,
  ) -> Option<(Vec<Position>, Cost)> {
    let total_cost = *self.min_cost.get(&target)?;
    let mut current = target;
    let mut path = vec![];

    while current != self.start {
      path.push(current);
      current = *self.predecessor.get(&current)?;
    }

    path.push(self.start);
    path.reverse();
    Some((path, total_cost))
  }

  pub fn set_predecessor(
    &mut self,
    position: Position,
    predecessor: Position,
  ) {
    self.predecessor.insert(position, predecessor);
  }

  pub fn is_cost_better(
    &self,
    position: Position,
    real_cost: Cost,
  ) -> bool {
    self.min_cost.get(&position).is_none_or(
      |&min_real_cost| real_cost < min_real_cost,
    )
  }

  pub fn push(
    &mut self,
    position: Position,
    real_cost: Cost,
    expected_cost: Cost,
  ) {
    let total_cost = real_cost + expected_cost;
    self.min_heap.push(Reverse(Enqueued {
      position,
      total_cost,
      real_cost,
    }));
    self.min_cost.insert(position, real_cost);
  }

  pub fn pop(&mut self) -> Option<(Position, Cost, Cost)> {
    while let Some(Reverse(Enqueued {
      position,
      total_cost,
      real_cost,
    })) = self.min_heap.pop()
    {
      let best_real_cost =
        *self.min_cost.get(&position).unwrap();

      if real_cost > best_real_cost {
        continue;
      }

      let expected_cost = total_cost - real_cost;
      return Some((position, real_cost, expected_cost));
    }

    None
  }
}
