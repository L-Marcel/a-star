use std::{thread, time::Instant};

use crate::{
  algorithm::{
    observers::{
      concurrent::{ConcurrentObserver, ConcurrentState},
      simple::{HistoryObserver, Observer},
    },
    simple::AStar,
  },
  graph::{
    matrix::{Cost, Position, Step},
    nodes::NodeType,
  },
};

impl<'a> AStar<'a> {
  pub fn start_parallel(
    &self,
  ) -> Option<(Vec<Position>, Cost)> {
    let stop_on_first_meet =
      !self.matrix.contains(&NodeType::WATER);

    let state = ConcurrentState::new(
      self.matrix.get_width(),
      self.matrix.get_height(),
      stop_on_first_meet,
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
  ) -> (Option<(Vec<Position>, Cost)>, Vec<Step>) {
    let start_time = Instant::now();

    let stop_on_first_meet =
      !self.matrix.contains(&NodeType::WATER);

    let state = ConcurrentState::new(
      self.matrix.get_width(),
      self.matrix.get_height(),
      stop_on_first_meet,
    );

    let mut forward_observer =
      ConcurrentObserver::with_observer(
        &state,
        true,
        HistoryObserver::with_start(start_time, true),
      );

    let mut backward_observer =
      ConcurrentObserver::with_observer(
        &state,
        false,
        HistoryObserver::with_start(start_time, false),
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

    history.sort_by(
      |(_, instant_a, _), (_, instant_b, _)| {
        instant_a.cmp(instant_b)
      },
    );

    (result, history)
  }

  fn execute_parallel<O: Observer + Send>(
    &self,
    state: &ConcurrentState,
    forward_observer: &mut ConcurrentObserver<'_, O>,
    backward_observer: &mut ConcurrentObserver<'_, O>,
  ) -> Option<(Vec<Position>, Cost)> {
    println!("[info] Iniciando busca do melhor caminho");
    let start_time = Instant::now();

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

    println!(
      "[time] Unificando resultados em {:?}",
      start_time.elapsed()
    );

    let result =
      if let Some((total_cost, meet_node)) = state.best_meet()
      {
        let (mut forward_path, _) =
          context_forward.get_path_to(meet_node)?;

        if meet_node != self.end {
          let (mut backward_path, _) =
            context_backward.get_path_to(meet_node)?;

          backward_path.pop();
          backward_path.reverse();
          forward_path.extend(backward_path);
        }

        Some((forward_path, total_cost))
      } else {
        direct_forward
      };

    println!(
      "[time] Resposta encontrada em {:?}",
      start_time.elapsed()
    );

    result
  }
}
