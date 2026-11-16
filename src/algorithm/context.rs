use std::{
  cmp::Reverse,
  collections::{BinaryHeap, HashMap},
};

use crate::{
  algorithm::{
    distance::manhattan_distance_to, enqueued::Enqueued,
  },
  graph::{
    matrix::{Cost, Position},
    nodes::Node,
  },
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
  ) -> Option<Vec<Node>> {
    let mut current = target;
    let mut path = vec![];

    while current != self.start {
      let cost = *self.min_cost.get(&current)?;
      path.push((current, cost));
      current = *self.predecessor.get(&current)?;
    }

    if current == self.start {
      path.push((self.start, 0));
      path.reverse();
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
    real_cost: Cost,
  ) -> bool {
    self.min_cost.get(&position).is_none_or(
      |&expected_cost| real_cost < expected_cost,
    )
  }

  pub fn push(
    &mut self,
    position: Position,
    real_cost: Cost,
    expected_cost: Cost,
  ) {
    self
      .min_heap
      .push(Reverse(Enqueued(position, expected_cost)));
    self.min_cost.insert(position, real_cost);
  }

  pub fn pop(&mut self) -> Option<(Position, Cost, Cost)> {
    self.min_heap.pop().map(
      |Reverse(Enqueued(position, expected_cost))| {
        let real_cost =
          *self.min_cost.get(&position).unwrap();
        (position, real_cost, expected_cost)
      },
    )
  }
}
