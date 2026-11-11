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
    match read_to_string(&file) {
        Ok(content) => {
            let mut parts = content.split("\n");
            let dimensions: usize = parts
                .next()
                .unwrap_or_else(|| panic!("Primeira linha é obrigatória"))
                .parse()
                .unwrap_or_else(|_| {
                    panic!("Primeira linha deve ser um número inteiro não negativo")
                });

            println!("[info] Montando grafo {}x{}", dimensions, dimensions);
            let mut graph: Graph = Graph::new(dimensions, dimensions);
            let mut start: Option<(usize, usize)> = None;
            let mut end: Option<(usize, usize)> = None;

            for (y, part) in parts.enumerate() {
                for (x, character) in part.chars().enumerate() {
                    match character {
                        'A' => {
                            graph.put(x, y, NodeType::START);
                            println!("[info] Origem encontrada {:?}", (x, y));
                            start = Some((x, y));
                        }
                        'B' => {
                            graph.put(x, y, NodeType::END);
                            println!("[info] Destino encontrado {:?}", (x, y));
                            end = Some((x, y));
                        }
                        ' ' => graph.put(x, y, NodeType::PATH),
                        '_' => graph.put(x, y, NodeType::WATER),
                        '#' => graph.put(x, y, NodeType::WALL),
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
        Err(_) => panic!("Arquivo não encontrado"),
    }
}
