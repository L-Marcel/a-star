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

  #[arg(short, long, default_value_t = false)]
  pub keep_history: bool,
}

pub struct Context {
  pub graph: Matrix<NodeType>,
  pub start: Position,
  pub end: Position,
  pub concurrent: bool,
  pub print_result: bool,
  pub keep_history: bool,
}

fn print_config(
  enabled: bool,
  enabled_message: &str,
  disabled_message: &str,
) {
  if enabled {
    println!("{}", enabled_message);
  } else {
    println!("{}", disabled_message);
  }
}

pub fn load_context() -> Context {
  let start_time = Instant::now();

  let Cli {
    file,
    concurrent,
    with_water,
    print_result,
    keep_history,
  } = Cli::parse();

  print_config(
    concurrent,
    "[info] Versão concorrente do algoritmo selecionada",
    "[info] Versão simples do algoritmo selecionada",
  );

  print_config(
    print_result,
    "[info] Impressão do resultado habilitada",
    "[info] Impressão do resultado desabilitada",
  );

  print_config(
    with_water,
    "[info] Conversão de blocos de água para paredes desabilitada",
    "[info] Conversão de blocos de água para paredes habilitada",
  );

  print_config(
    keep_history,
    "[info] Mantimento do histórico de execução habilitado",
    "[info] Mantimento do histórico de execução desabilitado",
  );

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
    keep_history,
    start,
    end,
  }
}
