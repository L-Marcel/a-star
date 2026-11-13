use std::rc::Rc;

use eframe::{
  Frame, Result,
  egui::{
    Align, CentralPanel, Context, FontId, Layout, Slider,
    TextStyle, TextureHandle, Ui, ViewportBuilder, vec2,
  },
};

use crate::graph::{
  matrix::{Matrix, Position, Step},
  nodes::NodeType,
};

pub struct App {
  result: Option<Vec<Position>>,
  history: Option<Vec<Step>>,
  matrix: Rc<Matrix<NodeType>>,
  current_step: usize,
  texture: Option<TextureHandle>,
}

pub const FONT_SIZE: f32 = 16.0;
pub const SLIDER_HEIGHT: f32 = 28.0;
pub const SLIDER_RAIL_HEIGHT: f32 = 10.0;
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
    history: Option<Vec<Step>>,
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
      .with_min_inner_size([640.0, 480.0]);

    let options = eframe::NativeOptions {
      viewport,
      ..Default::default()
    };

    let result = eframe::run_native(
      "A-Star",
      options,
      Box::new(|cc| {
        Self::configure_styles(&cc.egui_ctx);
        Ok(Box::new(self))
      }),
    );

    println!("[info] Encerrando aplicação");

    result
  }

  fn configure_styles(context: &Context) {
    context.all_styles_mut(|style| {
      style.text_styles.insert(
        TextStyle::Body,
        FontId::proportional(FONT_SIZE),
      );

      style.text_styles.insert(
        TextStyle::Button,
        FontId::proportional(FONT_SIZE),
      );

      style.spacing.interact_size.y = SLIDER_HEIGHT;
      style.spacing.slider_rail_height = SLIDER_RAIL_HEIGHT;
    });
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
          let Some(history) = self.history.as_ref() else {
            return;
          };

          ui.allocate_ui(
            vec2(CURRENT_STEP_TEXT_WITDH, FOOTER_HEIGHT),
            |ui| {
              ui.centered_and_justified(|ui| {
                ui.label(format!(
                  "Nó visitado {:?}",
                  history[self.current_step].0
                ));
              });
            },
          );

          ui.with_layout(
            Layout::right_to_left(Align::Center),
            |ui| {
              ui.allocate_ui(
                vec2(STEP_TEXT_WIDTH, FOOTER_HEIGHT),
                |ui| {
                  ui.centered_and_justified(|ui| {
                    ui.label(format!(
                      "{:?}",
                      history[self.current_step].1
                    ));
                  });
                },
              );

              let font_width = FONT_SIZE * 4.5;
              let value_box_width = font_width.max(55.0);
              let gap = ui.spacing().item_spacing.x;

              let slider_witdth = ui.available_width()
                - value_box_width
                - gap;

              ui.spacing_mut().slider_width =
                slider_witdth.max(10.0);

              ui.add(Slider::new(
                &mut self.current_step,
                0..=(history.len().saturating_sub(1)),
              ));
            },
          );
        });
      },
    );
  }
}
