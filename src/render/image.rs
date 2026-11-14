use std::rc::Rc;

use eframe::egui::{Color32, ColorImage};

use crate::{
  graph::{
    matrix::{Matrix, Position, Step},
    nodes::NodeType,
  },
  render::theme::{
    RESULT_COLOR, VISITED_COLOR, node_color,
  },
};

pub struct GridImage {
  matrix: Rc<Matrix<NodeType>>,
  result: Option<Vec<Position>>,
  history: Vec<Step>,
  visit_counts: Vec<u16>,
  image: ColorImage,
}

impl GridImage {
  pub fn new(
    matrix: Rc<Matrix<NodeType>>,
    result: Option<Vec<Position>>,
    history: Vec<Step>,
  ) -> Self {
    let image = build_base_image(&matrix);
    let visit_counts = vec![0; matrix.get_size()];

    Self {
      matrix,
      result,
      history,
      visit_counts,
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
      let (position, _) = self.history[step];
      self.mark_visited(position);
    }
  }

  fn unvisit_range(&mut self, start: usize, end: usize) {
    for step in start..=end {
      let (position, _) = self.history[step];
      self.unmark_visited(position);
    }
  }

  fn mark_visited(&mut self, position: Position) {
    let width = self.matrix.get_width();
    let index = pixel_index(width, position);
    self.visit_counts[index] += 1;

    if self.visit_counts[index] == 1 {
      paint_overlay(
        &mut self.image,
        &self.matrix,
        position,
        VISITED_COLOR,
      );
    }
  }

  fn unmark_visited(&mut self, position: Position) {
    let width = self.matrix.get_width();
    let index = pixel_index(width, position);
    self.visit_counts[index] -= 1;

    if self.visit_counts[index] == 0 {
      set_pixel(
        &mut self.image,
        width,
        position,
        node_color(self.matrix[position]),
      );
    }
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

    let width = self.matrix.get_width();
    for &position in path {
      let index = pixel_index(width, position);
      let visit_counts = self.visit_counts[index];

      let color = if visit_counts > 0 {
        VISITED_COLOR
      } else {
        node_color(self.matrix[position])
      };

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
  path: &[Position],
  color: Color32,
) {
  for &position in path {
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

fn set_pixel(
  image: &mut ColorImage,
  width: usize,
  position: Position,
  color: Color32,
) {
  image.pixels[pixel_index(width, position)] = color;
}
