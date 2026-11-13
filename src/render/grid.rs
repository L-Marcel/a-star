use std::rc::Rc;

use eframe::egui::{
  Color32, ColorImage, Context, TextureHandle,
  TextureOptions, Ui, vec2,
};

use crate::{
  graph::{
    matrix::{Matrix, Position, Step},
    nodes::NodeType,
  },
  render::theme::{
    FOOTER_HEIGHT, RESULT_COLOR, VISITED_COLOR, node_color,
  },
};

pub struct GridCanvas {
  matrix: Rc<Matrix<NodeType>>,
  result: Option<Vec<Position>>,
  history: Option<Vec<Step>>,
  last_step: Option<usize>,
  image: Option<ColorImage>,
  texture: Option<TextureHandle>,
}

impl GridCanvas {
  pub fn new(
    matrix: Rc<Matrix<NodeType>>,
    result: Option<Vec<Position>>,
    history: Option<Vec<Step>>,
  ) -> Self {
    Self {
      matrix,
      result,
      history,
      last_step: None,
      image: None,
      texture: None,
    }
  }

  pub fn history(&self) -> Option<&[Step]> {
    self.history.as_deref()
  }

  pub fn render(
    &mut self,
    ui: &mut Ui,
    current_step: usize,
  ) {
    self.ensure_texture(ui.ctx(), current_step);
    self.sync_texture(current_step);

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

          let display_size = tex_size * scale;

          ui.centered_and_justified(|ui| {
            ui.image((texture.id(), display_size));
          });
        }
      },
    );
  }

  fn set_pixel(
    &self,
    image: &mut ColorImage,
    (col, row): Position,
    color: Color32,
  ) {
    let width = self.matrix.get_width();
    image.pixels[row * width + col] = color;
  }

  fn paint_overlay(
    &self,
    image: &mut ColorImage,
    pos: Position,
    color: Color32,
  ) {
    if !matches!(
      self.matrix[pos],
      NodeType::START | NodeType::END
    ) {
      self.set_pixel(image, pos, color);
    }
  }

  fn paint_result(
    &self,
    image: &mut ColorImage,
    color: Color32,
  ) {
    if let Some(result) = self.result.as_ref() {
      for &pos in result {
        self.paint_overlay(image, pos, color);
      }
    }
  }

  fn ensure_texture(
    &mut self,
    ctx: &Context,
    current_step: usize,
  ) {
    if self.texture.is_some() {
      return;
    }

    let width = self.matrix.get_width();
    let height = self.matrix.get_height();
    if width == 0 || height == 0 {
      return;
    }

    let mut pixels = Vec::with_capacity(width * height);
    for row in 0..height {
      for col in 0..width {
        pixels.push(node_color(self.matrix[(col, row)]));
      }
    }

    let mut image =
      ColorImage::new([width, height], pixels);

    match self.history.as_ref() {
      None => {
        self.paint_result(&mut image, RESULT_COLOR);
      }
      Some(history) if !history.is_empty() => {
        let end_step = history.len() - 1;
        for &(pos, _) in
          &history[..=current_step.min(end_step)]
        {
          self.paint_overlay(
            &mut image,
            pos,
            VISITED_COLOR,
          );
        }
        if current_step >= end_step {
          self.paint_result(&mut image, RESULT_COLOR);
        }
        self.last_step = Some(current_step);
      }
      Some(_) => {}
    }

    let texture = ctx.load_texture(
      "grid",
      image.clone(),
      TextureOptions::NEAREST,
    );
    self.image = Some(image);
    self.texture = Some(texture);
  }

  fn sync_texture(&mut self, current_step: usize) {
    if self.last_step == Some(current_step) {
      return;
    }

    let Some(history) = self.history.as_ref() else {
      return;
    };

    if history.is_empty() {
      return;
    }

    let Some(mut image) = self.image.take() else {
      return;
    };

    let prev = self.last_step.unwrap_or(0);
    let final_step = history.len() - 1;

    if prev == final_step && current_step < final_step {
      self.paint_result(&mut image, VISITED_COLOR);
    }

    if current_step > prev {
      for &(pos, _) in &history[(prev + 1)..=current_step] {
        self.paint_overlay(&mut image, pos, VISITED_COLOR);
      }
    } else {
      for &(pos, _) in &history[(current_step + 1)..=prev] {
        self.set_pixel(
          &mut image,
          pos,
          node_color(self.matrix[pos]),
        );
      }
    }

    if current_step == final_step {
      self.paint_result(&mut image, RESULT_COLOR);
    }

    if let Some(texture) = self.texture.as_mut() {
      texture.set(image.clone(), TextureOptions::NEAREST);
    }

    self.image = Some(image);
    self.last_step = Some(current_step);
  }
}
