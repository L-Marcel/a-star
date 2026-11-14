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
    let visit_counts =
      vec![0; matrix.get_width() * matrix.get_height()];

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
    if from == Some(to) {
      return false;
    };

    let width = self.matrix.get_width();
    let final_step = self.history.len() - 1;
    let to = to.min(final_step);

    match from {
      None => {
        for &(position, _) in &self.history[..=to] {
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
      }
      Some(previous) => {
        if to > previous {
          for &(position, _) in
            &self.history[(previous + 1)..=to]
          {
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
        } else {
          for &(position, _) in
            &self.history[(to + 1)..=previous]
          {
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
        }

        if previous == final_step
          && to < final_step
          && let Some(path) = self.result.as_ref()
        {
          for &position in path {
            let color = if self.visit_counts
              [pixel_index(width, position)]
              > 0
            {
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
    }

    if to == final_step
      && let Some(path) = self.result.as_ref()
    {
      paint_path(
        &mut self.image,
        &self.matrix,
        path,
        RESULT_COLOR,
      );
    }

    true
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
