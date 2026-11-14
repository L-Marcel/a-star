use std::thread;

use crate::{
  algorithm::simple::AStar,
  graph::matrix::{Position, Step},
};

impl<'a> AStar<'a> {
  pub fn start_parallel(&self) -> Option<Vec<Position>> {
    let algorithm =
      AStar::new(self.matrix, self.end, self.start);

    thread::scope(|scope| {
      let first = scope.spawn(|| self.start());
      let second = scope.spawn(|| algorithm.start());

      let result = first.join().unwrap();
      let _ = second.join().unwrap();

      result
    })
  }

  pub fn start_parallel_keeping_history(
    &self,
  ) -> (Option<Vec<Position>>, Vec<Step>) {
    let algorithm =
      AStar::new(self.matrix, self.end, self.start);

    thread::scope(|scope| {
      let first =
        scope.spawn(|| self.start_keeping_history());

      let second =
        scope.spawn(|| algorithm.start_keeping_history());

      let (result, mut first_history) =
        first.join().unwrap();

      let (_, second_history) = second.join().unwrap();

      first_history.extend(second_history);
      first_history.sort_by(
        |(_, instant_a), (_, instant_b)| {
          instant_a.cmp(instant_b)
        },
      );

      (result, first_history)
    })
  }
}
