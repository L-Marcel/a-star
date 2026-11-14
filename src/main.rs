use std::rc::Rc;

use crate::{
  algorithm::simple::AStar,
  loader::{Context, load_context},
  render::app::App,
};

pub mod algorithm;
pub mod graph;
pub mod loader;
pub mod render;

fn main() {
  let Context {
    graph,
    start,
    end,
    concurrent,
    no_render,
  } = load_context();

  let algorithm = AStar::new(&graph, start, end);

  let (result, history) = match (concurrent, no_render) {
    (false, true) => (algorithm.start(), vec![]),
    (false, false) => algorithm.start_keeping_history(),
    (true, true) => (algorithm.start_parallel(), vec![]),
    (true, false) => {
      algorithm.start_parallel_keeping_history()
    }
  };

  if !no_render {
    App::new(Rc::new(graph), result, history)
      .initialize()
      .unwrap_or_else(|error| {
        panic!("Aplicação encerrada com erro: {}", error)
      });
  }
}
