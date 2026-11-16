use std::sync::{
  Mutex,
  atomic::{AtomicBool, AtomicUsize},
};

use crate::{
  algorithm::observers::simple::Observer,
  graph::matrix::{Cost, Position},
};

pub struct ConcurrentObserver {}

pub struct ConcurrentState {
  real_costs_forward: Vec<AtomicUsize>,
  real_costs_backward: Vec<AtomicUsize>,
  min_total_forward: AtomicUsize,
  min_total_backward: AtomicUsize,
  best_meet: Mutex<Option<(Cost, Position)>>,
  stop: AtomicBool,
  width: usize,
}

impl Observer for ConcurrentObserver {
  fn on_expand(
    &mut self,
    position: Position,
    real_cost: Cost,
    expected_cost: Cost,
  ) {
    todo!()
  }
}
