use std::{fs::read_to_string, path::PathBuf, time::Instant};

use clap::Parser;

use crate::graph::{Graph, NodeType};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    pub file: PathBuf,

    #[arg(short, long)]
    pub concurrent: bool,
}

pub struct Context {
    pub graph: Graph,
    pub start: (usize, usize),
    pub end: (usize, usize),
    pub concurrent: bool,
}

pub fn load_context() -> Context {
    let start_time = Instant::now();
    let Cli { file, concurrent } = Cli::parse();

    if concurrent {
        println!("[info] Versão concorrente do algoritmo selecionada");
    } else {
        println!("[info] Versão simples do algoritmo selecionada")
    }

    println!("[info] Carregando arquivo: {:?}", file);

    let content = read_to_string(&file).unwrap_or_else(|_| panic!("Arquivo não encontrado"));
    let mut parts = content.split("\n");

    let dimensions: usize = parts
        .next()
        .unwrap_or_else(|| panic!("Primeira linha é obrigatória"))
        .parse()
        .unwrap_or_else(|_| panic!("Primeira linha deve ser um número inteiro não negativo que represente as dimensões do grafo"));

    println!("[info] Montando grafo {}x{}", dimensions, dimensions);
    let mut graph: Graph = Graph::new(dimensions, dimensions);
    let mut start: Option<(usize, usize)> = None;
    let mut end: Option<(usize, usize)> = None;

    for (row, part) in parts.enumerate() {
        for (column, character) in part.chars().enumerate() {
            match character {
                'A' => {
                    graph.put(column, row, NodeType::START);
                    println!("[info] Origem encontrada {:?}", (column, row));
                    start = Some((column, row));
                }
                'B' => {
                    graph.put(column, row, NodeType::END);
                    println!("[info] Destino encontrado {:?}", (column, row));
                    end = Some((column, row));
                }
                ' ' => graph.put(column, row, NodeType::PATH),
                '_' => graph.put(column, row, NodeType::WATER),
                '#' => graph.put(column, row, NodeType::WALL),
                _ => panic!("Caractere inválido"),
            }
        }
    }

    let start = start.unwrap_or_else(|| panic!("Origem não encontrado"));
    let end = end.unwrap_or_else(|| panic!("Destino não encontrado"));

    println!("[time] Contexto carregado em {:?}", start_time.elapsed());

    Context {
        graph,
        concurrent,
        start,
        end,
    }
}
