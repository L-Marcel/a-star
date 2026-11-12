use crate::loader::{Context, load_context};

pub mod graph;
pub mod loader;
pub mod matrix;

fn main() {
  let _: Context = load_context();

  // Observação, vocês consegueriam usar o grafo (que está dentro do contexto) assim:
  // context.graph[(column, row)] -> retorna um elemento do grafo nessa posição
  // context.graph[position] -> também retorna um elemento do grafo nessa posição
  // na prática Position = (Column, Row)

  // lembrando que para alterar tem que ser declarado mutável
}
