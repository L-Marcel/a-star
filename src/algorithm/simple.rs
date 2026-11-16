use std::time::Instant;

use crate::{
  algorithm::{
    context::Context,
    distance::manhattan_distance_to,
    observers::simple::{
      HistoryObserver, NoObserver, Observer,
    },
  },
  graph::{
    matrix::{Matrix, Position, Step},
    nodes::{Node, NodeType},
  },
};

pub struct AStar<'a> {
  pub matrix: &'a Matrix<NodeType>,
  pub start: Position,
  pub end: Position,
}

impl<'a> AStar<'a> {
  pub fn new(
    matrix: &'a Matrix<NodeType>,
    start: Position,
    end: Position,
  ) -> AStar<'a> {
    Self { matrix, start, end }
  }

  pub fn start(&self) -> Option<Vec<Node>> {
    let mut observer = NoObserver::new();
    self.start_with_log(&mut observer)
  }

  pub fn start_keeping_history(
    &self,
  ) -> (Option<Vec<Node>>, Vec<Step>) {
    let mut observer = HistoryObserver::new();
    let result = self.start_with_log(&mut observer);
    (result, observer.history())
  }

  fn start_with_log(
    &self,
    observer: &mut impl Observer,
  ) -> Option<Vec<Node>> {
    println!("[info] Iniciando busca do melhor caminho");
    let start_time = Instant::now();
    let (result, _) = self.execute(observer);

    println!(
      "[time] Resposta encontrada em {:?}",
      start_time.elapsed()
    );

    result
  }

  pub fn execute(
    &self,
    observer: &mut impl Observer,
  ) -> (Option<Vec<Node>>, Context) {
    let mut context = Context::new(self.start, self.end);
    let start_time = Instant::now();

    while !observer.should_stop()
      && let Some((current, real_cost, expected_cost)) =
        context.pop()
    {
      observer.on_expand(current, real_cost, expected_cost);

      if current == self.end {
        break;
      }

      let current_type: NodeType = self.matrix[current];
      let neighbors = self.matrix.get_neightbor_of(current);
      for neighbor in neighbors {
        let neighbor_type: NodeType = self.matrix[neighbor];

        if neighbor_type.is_wall() {
          continue;
        }

        let neighbor_expected_cost =
          manhattan_distance_to(&neighbor, &self.end);

        let neighbor_real_cost = real_cost
          + current_type
            .get_cost_to(neighbor_type)
            .unwrap();

        let is_better = context
          .is_cost_better(neighbor, neighbor_real_cost);

        if is_better {
          context.set_predecessor(neighbor, current);
          context.push(
            neighbor,
            neighbor_real_cost,
            neighbor_expected_cost,
          );
        }
      }
    }

    let result = context.get_path_to(self.end);

    observer.on_finish(start_time.elapsed());

    (result, context)
  }
}
