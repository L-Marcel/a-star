use std::{thread, time::Instant};

use crate::{
  algorithm::{
    observers::{
      concurrent::{ConcurrentObserver, ConcurrentState},
      simple::{HistoryObserver, Observer},
    },
    simple::AStar,
  },
  graph::matrix::{Position, Step},
};

impl<'a> AStar<'a> {
  pub fn start_parallel(&self) -> Option<Vec<Position>> {
    let state = ConcurrentState::new(
      self.matrix.get_width(),
      self.matrix.get_height(),
    );

    let mut forward_observer =
      ConcurrentObserver::new(&state, true);

    let mut backward_observer =
      ConcurrentObserver::new(&state, false);

    self.execute_parallel(
      &state,
      &mut forward_observer,
      &mut backward_observer,
    )
  }

  pub fn start_parallel_keeping_history(
    &self,
  ) -> (Option<Vec<Position>>, Vec<Step>) {
    let start_time = Instant::now();
    let state = ConcurrentState::new(
      self.matrix.get_width(),
      self.matrix.get_height(),
    );

    let mut forward_observer =
      ConcurrentObserver::with_observer(
        &state,
        true,
        HistoryObserver::with_start(start_time),
      );

    let mut backward_observer =
      ConcurrentObserver::with_observer(
        &state,
        false,
        HistoryObserver::with_start(start_time),
      );

    let result = self.execute_parallel(
      &state,
      &mut forward_observer,
      &mut backward_observer,
    );

    let mut history =
      forward_observer.into_inner().history();

    history
      .extend(backward_observer.into_inner().history());

    history.sort_by(|(_, instant_a), (_, instant_b)| {
      instant_a.cmp(instant_b)
    });

    (result, history)
  }

  fn execute_parallel<O: Observer + Send>(
    &self,
    state: &ConcurrentState,
    forward_observer: &mut ConcurrentObserver<'_, O>,
    backward_observer: &mut ConcurrentObserver<'_, O>,
  ) -> Option<Vec<Position>> {
    let algorithm =
      AStar::new(self.matrix, self.end, self.start);

    let (
      (direct_forward, context_forward),
      (_, context_backward),
    ) = thread::scope(|scope| {
      let forward =
        scope.spawn(|| self.execute(forward_observer));

      let backward = scope
        .spawn(|| algorithm.execute(backward_observer));

      (forward.join().unwrap(), backward.join().unwrap())
    });

    let Some(meet_node) = state.meet_position() else {
      return direct_forward;
    };

    let mut foward_path =
      context_forward.get_path_to(meet_node)?;

    if meet_node != self.end {
      let mut backward_path =
        context_backward.get_path_to(meet_node)?;

      backward_path.pop();
      backward_path.reverse();
      foward_path.extend(backward_path);
    }

    Some(foward_path)
  }
}
