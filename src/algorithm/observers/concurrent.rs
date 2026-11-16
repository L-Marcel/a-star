use std::{
  sync::{
    Mutex,
    atomic::{AtomicBool, AtomicUsize, Ordering},
  },
  time::Duration,
};

use crate::{
  algorithm::observers::simple::{NoObserver, Observer},
  graph::matrix::{Cost, Position, position_to_index},
};

pub struct ConcurrentState {
  real_costs_forward: Vec<AtomicUsize>,
  real_costs_backward: Vec<AtomicUsize>,
  min_total_forward: AtomicUsize,
  min_total_backward: AtomicUsize,
  best_cost: AtomicUsize,
  best_meet: Mutex<Option<(Cost, Position)>>,
  stop: AtomicBool,
  stop_on_first_meet: bool,
  width: usize,
}

impl ConcurrentState {
  pub fn new(
    width: usize,
    height: usize,
    stop_on_first_meet: bool,
  ) -> Self {
    Self {
      real_costs_forward: (0..width * height)
        .map(|_| AtomicUsize::new(usize::MAX))
        .collect(),
      real_costs_backward: (0..width * height)
        .map(|_| AtomicUsize::new(usize::MAX))
        .collect(),
      min_total_forward: AtomicUsize::new(0),
      min_total_backward: AtomicUsize::new(0),
      best_cost: AtomicUsize::new(usize::MAX),
      best_meet: Mutex::new(None),
      stop: AtomicBool::new(false),
      stop_on_first_meet,
      width,
    }
  }

  pub fn meet_position(&self) -> Option<Position> {
    self
      .best_meet
      .lock()
      .unwrap()
      .map(|(_, position)| position)
  }
}

pub struct ConcurrentObserver<'a, O: Observer = NoObserver>
{
  state: &'a ConcurrentState,
  is_forward: bool,
  inner: O,
}

impl<'a> ConcurrentObserver<'a, NoObserver> {
  pub fn new(
    state: &'a ConcurrentState,
    is_forward: bool,
  ) -> Self {
    Self::with_observer(
      state,
      is_forward,
      NoObserver::new(),
    )
  }
}

impl<'a, O: Observer> ConcurrentObserver<'a, O> {
  pub fn with_observer(
    state: &'a ConcurrentState,
    is_forward: bool,
    inner: O,
  ) -> Self {
    Self {
      state,
      is_forward,
      inner,
    }
  }

  pub fn into_inner(self) -> O {
    self.inner
  }

  pub fn get_costs(
    &self,
  ) -> (&Vec<AtomicUsize>, &Vec<AtomicUsize>) {
    if self.is_forward {
      (
        &self.state.real_costs_forward,
        &self.state.real_costs_backward,
      )
    } else {
      (
        &self.state.real_costs_backward,
        &self.state.real_costs_forward,
      )
    }
  }

  pub fn get_mins(&self) -> (&AtomicUsize, &AtomicUsize) {
    if self.is_forward {
      (
        &self.state.min_total_forward,
        &self.state.min_total_backward,
      )
    } else {
      (
        &self.state.min_total_backward,
        &self.state.min_total_forward,
      )
    }
  }
}

impl<'a, O: Observer> Observer
  for ConcurrentObserver<'a, O>
{
  fn on_expand(
    &mut self,
    position: Position,
    real_cost: Cost,
    expected_cost: Cost,
  ) {
    self.inner.on_expand(
      position,
      real_cost,
      expected_cost,
    );
    let index =
      position_to_index(position, self.state.width);

    let (real_costs, other_real_costs) = self.get_costs();
    let (min_total, other_min_total) = self.get_mins();
    let current_min = real_cost + expected_cost;

    real_costs[index].store(real_cost, Ordering::Relaxed);
    min_total.store(current_min, Ordering::Relaxed);

    let other_real_cost =
      other_real_costs[index].load(Ordering::Relaxed);

    if other_real_cost < usize::MAX {
      let candidate_cost = real_cost + other_real_cost;

      let mut best_meet =
        self.state.best_meet.lock().unwrap();

      let is_better =
        best_meet.is_none_or(|(best_cost, _)| {
          candidate_cost < best_cost
        });

      if is_better {
        *best_meet = Some((candidate_cost, position));
        self
          .state
          .best_cost
          .store(candidate_cost, Ordering::Relaxed);
      }

      if self.state.stop_on_first_meet {
        self.state.stop.store(true, Ordering::Relaxed);
        return;
      }
    }

    let best_cost =
      self.state.best_cost.load(Ordering::Relaxed);

    let other_min = other_min_total.load(Ordering::Relaxed);

    if best_cost < usize::MAX
      && current_min.max(other_min) >= best_cost
    {
      self.state.stop.store(true, Ordering::Relaxed);
    }
  }

  fn should_stop(&self) -> bool {
    self.state.stop.load(Ordering::Relaxed)
      || self.inner.should_stop()
  }

  fn on_finish(&mut self, elapsed: Duration) {
    let side = if self.is_forward {
      "forward"
    } else {
      "backward"
    };

    println!(
      "[time][{}] Thread sendo encerrada em {:?}",
      side, elapsed
    );
  }
}
