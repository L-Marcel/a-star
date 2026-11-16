use std::rc::Rc;

use eframe::egui::{Color32, ColorImage};

use crate::{
  graph::{
    matrix::{Matrix, Position, Step},
    nodes::{Node, NodeType},
  },
  render::theme::{
    BACKWARD_VISITED_COLOR, FORWARD_VISITED_COLOR,
    RESULT_COLOR, VISITED_ALPHA, node_color,
  },
};

pub struct GridImage {
  matrix: Rc<Matrix<NodeType>>,
  result: Option<Vec<Node>>,
  history: Vec<Step>,
  forward_counts: Vec<u16>,
  backward_counts: Vec<u16>,
  image: ColorImage,
}

impl GridImage {
  pub fn new(
    matrix: Rc<Matrix<NodeType>>,
    result: Option<Vec<Node>>,
    history: Vec<Step>,
  ) -> Self {
    let image = build_base_image(&matrix);
    let size = matrix.get_size();

    Self {
      matrix,
      result,
      history,
      forward_counts: vec![0; size],
      backward_counts: vec![0; size],
      image,
    }
  }

  pub fn history(&self) -> &[Step] {
    &self.history
  }

  pub fn as_color_image(&self) -> &ColorImage {
    &self.image
  }

  pub fn is_empty(&self) -> bool {
    self.image.width() == 0 || self.image.height() == 0
  }

  pub fn apply_step(
    &mut self,
    from: Option<usize>,
    to: usize,
  ) -> bool {
    if self.history.is_empty() || from == Some(to) {
      return false;
    }

    let to = to.min(self.final_step());

    match from {
      None => self.visit_range(0, to),
      Some(previous) if to > previous => {
        self.visit_range(previous + 1, to);
      }
      Some(previous) => {
        self.unvisit_range(to + 1, previous)
      }
    }

    self.sync_result_path(from, to);
    true
  }

  fn final_step(&self) -> usize {
    self.history.len().saturating_sub(1)
  }

  fn visit_range(&mut self, start: usize, end: usize) {
    for step in start..=end {
      let (position, _, is_forward) = self.history[step];
      self.mark_visited(position, is_forward);
    }
  }

  fn unvisit_range(&mut self, start: usize, end: usize) {
    for step in start..=end {
      let (position, _, is_forward) = self.history[step];
      self.unmark_visited(position, is_forward);
    }
  }

  fn mark_visited(
    &mut self,
    position: Position,
    is_forward: bool,
  ) {
    let width = self.matrix.get_width();
    let index = pixel_index(width, position);

    let counts = if is_forward {
      &mut self.forward_counts
    } else {
      &mut self.backward_counts
    };

    counts[index] += 1;

    if counts[index] == 1 {
      let color = self.visited_color(position);
      paint_overlay(
        &mut self.image,
        &self.matrix,
        position,
        color,
      );
    }
  }

  fn unmark_visited(
    &mut self,
    position: Position,
    is_forward: bool,
  ) {
    let width = self.matrix.get_width();
    let index = pixel_index(width, position);

    let counts = if is_forward {
      &mut self.forward_counts
    } else {
      &mut self.backward_counts
    };

    counts[index] -= 1;

    if counts[index] == 0 {
      let color = self.visited_color(position);
      paint_overlay(
        &mut self.image,
        &self.matrix,
        position,
        color,
      );
    }
  }

  fn visited_color(&self, position: Position) -> Color32 {
    let index =
      pixel_index(self.matrix.get_width(), position);

    let mut color = node_color(self.matrix[position]);

    if self.forward_counts[index] > 0 {
      color =
        blend(color, FORWARD_VISITED_COLOR, VISITED_ALPHA);
    }

    if self.backward_counts[index] > 0 {
      color =
        blend(color, BACKWARD_VISITED_COLOR, VISITED_ALPHA);
    }

    color
  }

  fn sync_result_path(
    &mut self,
    from: Option<usize>,
    to: usize,
  ) {
    let Some(path) = self.result.as_ref() else {
      return;
    };

    let final_step = self.final_step();
    if to == final_step {
      paint_path(
        &mut self.image,
        &self.matrix,
        path,
        RESULT_COLOR,
      );
      return;
    }

    if from != Some(final_step) {
      return;
    }

    for &node in path {
      let position = node.0;
      let color = self.visited_color(position);

      paint_overlay(
        &mut self.image,
        &self.matrix,
        position,
        color,
      );
    }
  }
}

fn build_base_image(
  matrix: &Matrix<NodeType>,
) -> ColorImage {
  let width = matrix.get_width();
  let height = matrix.get_height();
  let mut pixels = Vec::with_capacity(width * height);

  for row in 0..height {
    for column in 0..width {
      pixels.push(node_color(matrix[(column, row)]));
    }
  }

  ColorImage::new([width, height], pixels)
}

fn paint_path(
  image: &mut ColorImage,
  matrix: &Matrix<NodeType>,
  path: &[Node],
  color: Color32,
) {
  for &node in path {
    let position = node.0;
    paint_overlay(image, matrix, position, color);
  }
}

fn paint_overlay(
  image: &mut ColorImage,
  matrix: &Matrix<NodeType>,
  position: Position,
  color: Color32,
) {
  if !matrix[position].is_princial() {
    set_pixel(image, matrix.get_width(), position, color);
  }
}

fn pixel_index(
  width: usize,
  (column, row): Position,
) -> usize {
  row * width + column
}

fn blend(
  base: Color32,
  overlay: Color32,
  alpha: u8,
) -> Color32 {
  let alpha = alpha as u16;
  let inverse = 255 - alpha;

  let red = (base.r() as u16 * inverse
    + overlay.r() as u16 * alpha)
    / 255;

  let green = (base.g() as u16 * inverse
    + overlay.g() as u16 * alpha)
    / 255;

  let blue = (base.b() as u16 * inverse
    + overlay.b() as u16 * alpha)
    / 255;

  Color32::from_rgb(red as u8, green as u8, blue as u8)
}

fn set_pixel(
  image: &mut ColorImage,
  width: usize,
  position: Position,
  color: Color32,
) {
  image.pixels[pixel_index(width, position)] = color;
}
