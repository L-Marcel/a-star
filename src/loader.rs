use std::{
  fs::read_to_string, path::PathBuf, time::Instant,
};

use clap::Parser;

use crate::{
  graph::NodeType,
  matrix::{Matrix, Position},
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
  #[arg(short, long, value_name = "FILE")]
  pub file: PathBuf,

  #[arg(short, long, default_value_t = false)]
  pub concurrent: bool,

  #[arg(short, long, default_value_t = false)]
  pub with_water: bool,

  #[arg(short, long, default_value_t = false)]
  pub print_result: bool,
}

pub struct Context {
  pub graph: Matrix<NodeType>,
  pub start: Position,
  pub end: Position,
  pub concurrent: bool,
  pub print_result: bool,
}

pub fn load_context() -> Context {
  let start_time = Instant::now();

  let Cli {
    file,
    concurrent,
    with_water,
    print_result,
  } = Cli::parse();

  if concurrent {
    println!(
      "[info] Versão concorrente do algoritmo selecionada"
    );
  } else {
    println!(
      "[info] Versão simples do algoritmo selecionada"
    )
  }

  println!("[info] Carregando arquivo: {:?}", file);

  let content = read_to_string(&file)
    .unwrap_or_else(|_| panic!("Arquivo não encontrado"));
  let mut parts = content.split("\n");

  let dimensions: usize = parts
    .next()
    .unwrap_or_else(|| panic!("[error] Primeira linha é obrigatória"))
    .parse()
    .unwrap_or_else(|_| panic!("[error] Primeira linha deve ser um número inteiro não negativo que represente as dimensões do grafo"));

  println!(
    "[info] Montando grafo {}x{}",
    dimensions, dimensions
  );

  let mut start: Option<Position> = None;
  let mut end: Option<Position> = None;
  let mut graph: Matrix<NodeType> =
    Matrix::new(dimensions, dimensions, NodeType::WALL);

  for (row, part) in parts.enumerate() {
    for (column, character) in part.chars().enumerate() {
      match (character, with_water) {
        ('A', _) => {
          graph[(column, row)] = NodeType::START;
          start = Some((column, row));
          println!(
            "[info] Origem encontrada {:?}",
            (column, row)
          );
        }
        ('B', _) => {
          graph[(column, row)] = NodeType::END;
          end = Some((column, row));
          println!(
            "[info] Destino encontrado {:?}",
            (column, row)
          );
        }
        (' ', _) => graph[(column, row)] = NodeType::PATH,
        ('_', true) => {
          graph[(column, row)] = NodeType::WATER
        }
        ('_', false) => {
          graph[(column, row)] = NodeType::WALL
        }
        ('#', _) => graph[(column, row)] = NodeType::WALL,
        _ => panic!("[error] Caractere inválido"),
      }
    }
  }

  let start = start.unwrap_or_else(|| {
    panic!("[error] Origem não encontrado")
  });
  let end = end.unwrap_or_else(|| {
    panic!("[error] Destino não encontrado")
  });

  println!(
    "[time] Contexto carregado em {:?}",
    start_time.elapsed()
  );

  Context {
    graph,
    concurrent,
    print_result,
    start,
    end,
  }
}
