use std::{
  cmp::Reverse,
  collections::{BinaryHeap, HashMap},
};

use crate::{
  algorithm::{
    distance::manhattan_distance_to, enqueued::Enqueued,
  },
  matrix::Position,
};

pub struct Context {
  min_heap: BinaryHeap<Reverse<Enqueued>>,
  min_cost: HashMap<Position, usize>,
  predecessor: HashMap<Position, Position>,
  start: Position,
  end: Position,
}

impl Context {
  pub fn new(start: Position, end: Position) -> Context {
    let mut context = Self {
      min_heap: BinaryHeap::new(),
      min_cost: HashMap::new(),
      predecessor: HashMap::new(),
      start,
      end,
    };

    let expected_cost = manhattan_distance_to(&start, &end);
    context.predecessor.insert(start, start);
    context.push(start, 1, expected_cost);
    context
  }

  pub fn get_better_path(&self) -> Option<Vec<Position>> {
    let mut current = self.end;
    let mut path = Vec::new();

    while current != self.start {
      path.push(current);
      current = self.predecessor[&current];
    }

    path.reverse();
    if current == self.start {
      Some(path)
    } else {
      None
    }
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
    real_cost: usize,
  ) -> bool {
    self.min_cost.get(&position).is_none_or(
      |&expected_cost| real_cost < expected_cost,
    )
  }

  pub fn push(
    &mut self,
    position: Position,
    real_cost: usize,
    expected_cost: usize,
  ) {
    self
      .min_heap
      .push(Reverse(Enqueued(position, expected_cost)));
    self.min_cost.insert(position, real_cost);
  }

  pub fn pop(&mut self) -> Option<(Position, usize)> {
    self.min_heap.pop().map(
      |Reverse(Enqueued(position, _))| {
        let real_cost =
          *self.min_cost.get(&position).unwrap();
        (position, real_cost)
      },
    )
  }
}
