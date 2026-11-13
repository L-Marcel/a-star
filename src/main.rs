use crate::{
  algorithm::simple::AStar,
  loader::{Context, load_context},
  matrix::Position,
};

pub mod algorithm;
pub mod graph;
pub mod loader;
pub mod matrix;

pub fn print_result(result: Option<Vec<Position>>) {
  if let Some(path) = result {
    for position in path {
      println!("{:?}", position);
    }
  }
}

fn main() {
  let Context {
    graph,
    start,
    end,
    concurrent,
    print_result: should_print_result,
    keep_history,
  } = load_context();

  let mut algorithm = AStar::new(&graph, start, end);

  match (concurrent, keep_history) {
    (false, false) => {
      let result = algorithm.start();
      if should_print_result {
        print_result(result);
      }
    }
    (false, true) => {
      let (result, _) = algorithm.start_keeping_history();
      if should_print_result {
        print_result(result);
      }
    }
    (true, false) => {}
    (true, true) => {}
  };
}
