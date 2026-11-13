use std::rc::Rc;

use eframe::{
  Frame, Result,
  egui::{TextureHandle, Ui, ViewportBuilder},
};

use crate::graph::{
  matrix::{Matrix, Position},
  nodes::NodeType,
};

pub struct App {
  result: Option<Vec<Position>>,
  history: Option<Vec<Position>>,
  matrix: Rc<Matrix<NodeType>>,
  step_atual: usize,
  texture: Option<TextureHandle>,
}

impl eframe::App for App {
  fn ui(&mut self, ui: &mut Ui, frame: &mut Frame) {}
}

impl App {
  pub fn new(
    matrix: Rc<Matrix<NodeType>>,
    result: Option<Vec<Position>>,
    history: Option<Vec<Position>>,
  ) -> App {
    Self {
      history,
      result,
      step_atual: 0,
      texture: None,
      matrix,
    }
  }

  pub fn initialize(self) -> Result {
    println!("[info] Inicializando aplicação");
    let viewport = ViewportBuilder::default()
      .with_inner_size([1280.0, 720.0])
      .with_min_inner_size([1280.0, 720.0]);

    let options = eframe::NativeOptions {
      viewport,
      ..Default::default()
    };

    let result = eframe::run_native(
      "A-Star",
      options,
      Box::new(|_| Ok(Box::new(self))),
    );

    println!("[info] Encerrando aplicação");

    result
  }
}
