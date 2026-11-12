use crate::{
  algorithm::simple::a_star,
  loader::{Context, load_context},
};

pub mod algorithm;
pub mod graph;
pub mod loader;
pub mod matrix;

fn main() {
  let Context {
    graph,
    start,
    end,
    concurrent,
    print_result,
  } = load_context();

  if !concurrent {
    let result = a_star(&graph, start, end);
    if print_result && let Some(path) = result {
      for position in path {
        println!("{:?}", position);
      }
    }
  }
}
