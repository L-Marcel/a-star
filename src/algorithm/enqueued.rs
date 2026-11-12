use std::cmp::Ordering;

use crate::matrix::Position;

pub struct Enqueued(pub Position, pub usize);

impl PartialEq for Enqueued {
  fn eq(&self, other: &Self) -> bool {
    self.1 == other.1 && self.0 == other.0
  }
}

impl Eq for Enqueued {}

impl Ord for Enqueued {
  fn cmp(&self, other: &Self) -> Ordering {
    self.1.cmp(&other.1).then_with(|| self.0.cmp(&other.0))
  }
}

impl PartialOrd for Enqueued {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}
