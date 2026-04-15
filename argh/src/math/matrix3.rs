// ==============================================================================================
// 3x3 Matrix
// ==============================================================================================

use crate::math::{Vec2, Vec3};
use core::fmt;
use std::fmt::Formatter;
use std::ops::Mul;

/// 3x3 matrix
#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct Mat3 {
  col0: Vec3,
  col1: Vec3,
  col2: Vec3,
}

impl Mat3 {
  pub fn new() -> Self {
    Mat3 {
      col0: Vec3::new(1.0, 0.0, 0.0),
      col1: Vec3::new(0.0, 1.0, 0.0),
      col2: Vec3::new(0.0, 0.0, 1.0),
    }
  }

  pub fn zero() -> Self {
    Mat3 {
      col0: Vec3::zero(),
      col1: Vec3::zero(),
      col2: Vec3::zero(),
    }
  }

  pub fn trans(&mut self, x: f64, y: f64) {
    self.col2.x = x;
    self.col2.y = y;
  }

  pub fn rot(&mut self, a: f64) {
    let c = f64::cos(a);
    let s = f64::sin(a);

    self.col0.x = c;
    self.col1.x = -s;
    self.col0.y = s;
    self.col1.y = c;
  }
}

impl fmt::Display for Mat3 {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "[{}, {}, {}\n {}, {}, {}\n {}, {}, {}]",
      self.col0.x, self.col1.x, self.col2.x, self.col0.y, self.col1.y, self.col2.y, self.col0.z, self.col1.z, self.col2.z
    )
  }
}

impl Mul<Vec2> for Mat3 {
  type Output = Vec2;

  fn mul(self, v: Vec2) -> Vec2 {
    Vec2 {
      x: self.col0.x * v.x + self.col1.x * v.y + self.col2.x * 1.0,
      y: self.col0.y * v.x + self.col1.y * v.y + self.col2.y * 1.0,
    }
  }
}
