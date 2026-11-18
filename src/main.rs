use std::rc::Rc;

use crate::{
  algorithm::simple::AStar,
  graph::matrix::{Cost, Position, Step},
  loader::{Context, load_context},
  render::app::App,
};

pub mod algorithm;
pub mod graph;
pub mod loader;
pub mod render;

pub fn execute(
  algorithm: &AStar,
  concurrent: bool,
  no_render: bool,
) -> (Option<(Vec<Position>, Cost)>, Vec<Step>) {
  let (result, history) = match (concurrent, no_render) {
    (false, true) => (algorithm.start(), vec![]),
    (false, false) => algorithm.start_keeping_history(),
    (true, true) => (algorithm.start_parallel(), vec![]),
    (true, false) => {
      algorithm.start_parallel_keeping_history()
    }
  };

  if let Some((path, cost)) = &result {
    println!("[info] Custo do menor caminho foi {}", cost);
    println!("[info] Menor caminho tem {} nós", path.len());
  }

  (result, history)
}

fn main() {
  let Context {
    graph,
    start,
    end,
    concurrent,
    no_render,
    benchmark,
  } = load_context();

  let algorithm = AStar::new(&graph, start, end);

  let (result, history) = if benchmark {
    println!("---------------[sequencial]--------------");
    let result = execute(&algorithm, false, true);
    drop(result);
    println!("---------------[concorrente]--------------");
    execute(&algorithm, true, true)
  } else {
    execute(&algorithm, concurrent, no_render)
  };

  if !no_render && !benchmark {
    let path = result.map(|(path, _)| path);
    App::new(Rc::new(graph), path, history)
      .initialize()
      .unwrap_or_else(|error| {
        panic!("Aplicação encerrada com erro: {}", error)
      });
  }
}
