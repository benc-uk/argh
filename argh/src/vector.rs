use std::f64;

#[cfg(test)]
#[path = "vector_test.rs"]
mod vector_test;

/// 2D vector with x,y coords
#[derive(Debug, PartialEq, Default)]
pub struct Vec2 {
  x: f64,
  y: f64,
}

impl Vec2 {
  pub fn new(x: f64, y: f64) -> Self {
    Vec2 { x, y }
  }

  pub fn zero() -> Self {
    Vec2 { x: 0.0, y: 0.0 }
  }

  pub fn ident() -> Self {
    Vec2 { x: 1.0, y: 1.0 }
  }

  pub fn add(&mut self, v: Vec2) {
    self.x += v.x;
    self.y += v.y;
  }

  pub fn add_new(self, v: Vec2) -> Vec2 {
    Vec2 { x: self.x + v.x, y: self.y + v.y }
  }

  pub fn len(self) -> f64 {
    f64::sqrt(self.x + self.y)
  }
}
