use std::time::Instant;

use crate::{
  algorithm::{
    context::Context,
    distance::manhattan_distance_to,
    observer::{HistoryObserver, NoObserver, Observer},
  },
  graph::{
    matrix::{Matrix, Position, Step},
    nodes::NodeType,
  },
};

pub type SearchResult = (Option<Vec<Position>>, Context);

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

  pub fn start(&self) -> Option<Vec<Position>> {
    let mut observer = NoObserver::new();
    let (result, _) = self.execute(&mut observer);
    result
  }

  pub fn start_keeping_history(
    &self,
  ) -> (Option<Vec<Position>>, Vec<Step>) {
    let mut observer = HistoryObserver::new();
    let (result, _) = self.execute(&mut observer);
    (result, observer.history())
  }

  pub fn execute(
    &self,
    observer: &mut impl Observer,
  ) -> (Option<Vec<Position>>, Context) {
    println!("[info] Iniciando busca do melhor caminho");

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
          + current_type.get_cost_to(neighbor_type).unwrap()
            as usize;

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

    println!(
      "[time] Resposta encontrada em {:?}",
      start_time.elapsed()
    );

    (result, context)
  }
}
