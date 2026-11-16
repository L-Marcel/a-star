use std::rc::Rc;

use eframe::{
  Frame, Result,
  egui::{CentralPanel, Ui, ViewportBuilder},
};

use crate::{
  graph::{
    matrix::{Matrix, Position, Step},
    nodes::NodeType,
  },
  render::{
    footer::render_footer, grid::GridCanvas,
    theme::configure_styles,
  },
};

pub struct App {
  grid: GridCanvas,
  current_step: usize,
}

impl eframe::App for App {
  fn ui(&mut self, ui: &mut Ui, _: &mut Frame) {
    CentralPanel::default().show(ui, |ui| {
      self.grid.render(ui, self.current_step);
      render_footer(
        ui,
        self.grid.history(),
        &mut self.current_step,
      );
    });
  }
}

impl App {
  pub fn new(
    matrix: Rc<Matrix<NodeType>>,
    result: Option<Vec<Position>>,
    history: Vec<Step>,
  ) -> App {
    Self {
      grid: GridCanvas::new(matrix, result, history),
      current_step: 0,
    }
  }

  pub fn initialize(self) -> Result {
    println!("[info] Inicializando aplicação");
    let viewport = ViewportBuilder::default()
      .with_inner_size([1280.0, 720.0])
      .with_min_inner_size([640.0, 480.0]);

    let options = eframe::NativeOptions {
      viewport,
      ..Default::default()
    };

    let result = eframe::run_native(
      "A-Star",
      options,
      Box::new(|cc| {
        configure_styles(&cc.egui_ctx);
        Ok(Box::new(self))
      }),
    );

    println!("[info] Encerrando aplicação");

    result
  }
}
