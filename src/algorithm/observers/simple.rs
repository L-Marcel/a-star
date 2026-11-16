use std::time::Instant;

use crate::graph::matrix::{Cost, Position, Step};

pub trait Observer {
  fn on_expand(
    &mut self,
    position: Position,
    real_cost: Cost,
    expected_cost: Cost,
  );

  fn should_stop(&self) -> bool {
    false
  }
}

pub struct NoObserver;

impl NoObserver {
  pub fn new() -> Self {
    NoObserver {}
  }
}

impl Observer for NoObserver {
  fn on_expand(
    &mut self,
    _position: Position,
    _real_cost: Cost,
    _expected_cost: Cost,
  ) {
  }
}

pub struct HistoryObserver {
  start: Instant,
  history: Vec<Step>,
}

impl HistoryObserver {
  pub fn new() -> Self {
    HistoryObserver {
      start: Instant::now(),
      history: vec![],
    }
  }

  pub fn history(self) -> Vec<Step> {
    self.history
  }
}

impl Observer for HistoryObserver {
  fn on_expand(
    &mut self,
    position: Position,
    _real_cost: Cost,
    _expected_cost: Cost,
  ) {
    self.history.push((position, self.start.elapsed()));
  }
}
