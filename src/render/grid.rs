use std::rc::Rc;

use eframe::egui::{
  TextureHandle, TextureOptions, Ui, vec2,
};

use crate::{
  graph::{
    matrix::{Matrix, Position, Step},
    nodes::NodeType,
  },
  render::{image::GridImage, theme::FOOTER_HEIGHT},
};

pub struct GridCanvas {
  buffer: GridImage,
  last_step: Option<usize>,
  texture: Option<TextureHandle>,
}

impl GridCanvas {
  pub fn new(
    matrix: Rc<Matrix<NodeType>>,
    result: Option<Vec<Position>>,
    history: Vec<Step>,
  ) -> Self {
    Self {
      buffer: GridImage::new(matrix, result, history),
      last_step: None,
      texture: None,
    }
  }

  pub fn history(&self) -> &[Step] {
    self.buffer.history()
  }

  pub fn render(
    &mut self,
    ui: &mut Ui,
    current_step: usize,
  ) {
    if self.buffer.is_empty() {
      return;
    }

    self.sync_texture(ui, current_step);

    let grid_height = ui.available_height()
      - ui.spacing().item_spacing.y
      - FOOTER_HEIGHT;

    ui.allocate_ui(
      vec2(ui.available_width(), grid_height),
      |ui| {
        if let Some(texture) = self.texture.as_ref() {
          let available = ui.available_size();
          let tex_size = texture.size_vec2();
          let scale = (available.x / tex_size.x)
            .min(available.y / tex_size.y);

          ui.centered_and_justified(|ui| {
            ui.image((texture.id(), tex_size * scale));
          });
        }
      },
    );
  }

  fn sync_texture(&mut self, ui: &Ui, current_step: usize) {
    let changed =
      self.buffer.apply_step(self.last_step, current_step);

    match self.texture.as_mut() {
      None => {
        let texture = ui.ctx().load_texture(
          "grid",
          self.buffer.as_color_image().clone(),
          TextureOptions::NEAREST,
        );

        self.texture = Some(texture);
      }
      Some(texture) if changed => {
        texture.set(
          self.buffer.as_color_image().clone(),
          TextureOptions::NEAREST,
        );
      }
      _ => {}
    }

    self.last_step = Some(current_step);
  }
}
