use std::time::Instant;

use crate::graph::matrix::{Position, Step};

pub trait Observer {
  fn on_expand(
    &mut self,
    position: Position,
    real_cost: usize,
    expected_cost: usize,
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
    _real_cost: usize,
    _expected_cost: usize,
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
    _real_cost: usize,
    _expected_cost: usize,
  ) {
    self.history.push((position, self.start.elapsed()));
  }
}
