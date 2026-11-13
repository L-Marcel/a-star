use std::rc::Rc;

use eframe::{
  Frame, Result,
  egui::{
    CentralPanel, Slider, TextureHandle, Ui,
    ViewportBuilder, vec2,
  },
};

use crate::graph::{
  matrix::{Matrix, Position},
  nodes::NodeType,
};

pub struct App {
  result: Option<Vec<Position>>,
  history: Option<Vec<Position>>,
  matrix: Rc<Matrix<NodeType>>,
  current_step: usize,
  texture: Option<TextureHandle>,
}

pub const FOOTER_HEIGHT: f32 = 60.0;
pub const STEP_TEXT_WIDTH: f32 = 140.0;
pub const CURRENT_STEP_TEXT_WITDH: f32 = 160.0;

impl eframe::App for App {
  fn ui(&mut self, ui: &mut Ui, _: &mut Frame) {
    CentralPanel::default().show(ui, |ui| {
      self.render_grid(ui);
      self.render_footer(ui);
    });
  }
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
      current_step: 0,
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

  fn render_grid(&mut self, ui: &mut Ui) {
    let grid_height = ui.available_height()
      - ui.spacing().item_spacing.y
      - FOOTER_HEIGHT;

    ui.allocate_ui(
      vec2(ui.available_width(), grid_height),
      |ui| {
        ui.centered_and_justified(|ui| {
          ui.label("Meu grid gigante vem aqui em cima!");
        });
      },
    );
  }

  fn render_footer(&mut self, ui: &mut Ui) {
    ui.allocate_ui(
      vec2(ui.available_width(), FOOTER_HEIGHT),
      |ui| {
        ui.horizontal(|ui| {
          if let Some(history) = self.history.as_ref() {
            ui.allocate_ui(
              vec2(CURRENT_STEP_TEXT_WITDH, FOOTER_HEIGHT),
              |ui| {
                ui.centered_and_justified(|ui| {
                  ui.label(format!(
                    "Nó visitado {:?}",
                    history[self.current_step]
                  ));
                });
              },
            );

            Self::render_slider(
              ui,
              &mut self.current_step,
              history.len(),
            );

            ui.allocate_ui(
              vec2(STEP_TEXT_WIDTH, FOOTER_HEIGHT),
              |ui| {
                ui.centered_and_justified(|ui| {
                  ui.label("Etapa");
                });
              },
            );
          }
        });
      },
    );
  }

  fn render_slider(
    ui: &mut Ui,
    current_step: &mut usize,
    max_step: usize,
  ) {
    let gaps = ui.spacing().item_spacing.x * 2.0;
    let value_box_width = 55.0;

    ui.spacing_mut().slider_width = (ui.available_width()
      - STEP_TEXT_WIDTH
      - gaps
      - value_box_width)
      .max(10.0);

    ui.add(Slider::new(
      current_step,
      0..=(max_step.saturating_sub(1)),
    ));
  }
}
