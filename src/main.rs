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
    keep_history,
    render_window,
  } = load_context();

  let mut algorithm = AStar::new(&graph, start, end);

  let (result, history) = match (concurrent, keep_history) {
    (false, false) => (algorithm.start(), None),
    (false, true) => {
      let (result, history) =
        algorithm.start_keeping_history();
      (result, Some(history))
    }
    (true, false) => (None, None),
    (true, true) => (None, None),
  };

  if render_window {
    App::new(Rc::new(graph), result, history)
      .initialize()
      .unwrap_or_else(|error| {
        panic!("Aplicação encerrada com erro: {}", error)
      });
  }
}
