use std::{
  ops::{Index, IndexMut},
  time::Duration,
  vec,
};

pub type Cost = usize;
pub type Column = usize;
pub type Row = usize;
pub type Position = (Column, Row);
pub type Step = (Position, Duration);

pub fn position_to_index(
  (column, row): Position,
  width: usize,
) -> usize {
  (row * width) + column
}

#[derive(Debug)]
pub struct Matrix<T: Clone> {
  nodes: Vec<T>,
  width: usize,
  height: usize,
}

impl<T: Clone> Matrix<T> {
  pub fn new(
    width: usize,
    height: usize,
    default: T,
  ) -> Self {
    Self {
      nodes: vec![default; width * height],
      width,
      height,
    }
  }

  fn get_index(&self, position: Position) -> usize {
    position_to_index(position, self.width)
  }

  pub fn get_size(&self) -> usize {
    self.nodes.len()
  }

  pub fn get_width(&self) -> usize {
    self.width
  }

  pub fn get_height(&self) -> usize {
    self.height
  }

  pub fn get_neightbor_of(
    &self,
    (column, row): Position,
  ) -> Vec<Position> {
    let mut neighbors = Vec::with_capacity(4);

    if row > 0 {
      neighbors.push((column, row - 1));
    }

    if row + 1 < self.height {
      neighbors.push((column, row + 1));
    }

    if column > 0 {
      neighbors.push((column - 1, row));
    }

    if column + 1 < self.width {
      neighbors.push((column + 1, row));
    }

    neighbors
  }
}

impl<T: Clone + PartialEq> Matrix<T> {
  pub fn contains(&self, value: &T) -> bool {
    self.nodes.contains(value)
  }
}

impl<T: Clone> Index<Position> for Matrix<T> {
  type Output = T;
  fn index(&self, position: Position) -> &Self::Output {
    &self.nodes[self.get_index(position)]
  }
}

impl<T: Clone> IndexMut<Position> for Matrix<T> {
  fn index_mut(
    &mut self,
    position: Position,
  ) -> &mut Self::Output {
    let index = self.get_index(position);
    &mut self.nodes[index]
  }
}
