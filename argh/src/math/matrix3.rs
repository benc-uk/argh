// ==============================================================================================
// Module & file:   math / matrix3.rs
// Purpose:         3x3 affine transformation matrix for 2D graphics
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use crate::math::Vec2;
use core::fmt;
use std::fmt::Formatter;
use std::ops::{Mul, MulAssign};

#[cfg(test)]
#[path = "matrix3_tests.rs"]
mod matrix3_tests;

/// A classic 3x3 affine transformation matrix, designed for transformations on [Vec2]
#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct Mat3 {
  ele: [[f64; 3]; 3],
}

impl Mat3 {
  /// New identity matrix
  pub fn new() -> Self {
    Self {
      ele: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
    }
  }

  /// New matrix with scale transform set
  pub fn new_scale(sx: f64, sy: f64) -> Self {
    Self {
      ele: [[sx, 0.0, 0.0], [0.0, sy, 0.0], [0.0, 0.0, 1.0]],
    }
  }

  /// New matrix with rotation transform set
  pub fn new_rot(a: f64) -> Self {
    let c = f64::cos(a);
    let s = f64::sin(a);
    Self {
      ele: [[c, -s, 0.0], [s, c, 0.0], [0.0, 0.0, 1.0]],
    }
  }

  /// New matrix with translation transform set
  pub fn new_trans(x: f64, y: f64) -> Self {
    Self {
      ele: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [x, y, 1.0]],
    }
  }

  /// Convenience & optimisation method - new matrix with scale, rotate, and translation transform set
  pub fn new_scale_rot_trans(sx: f64, sy: f64, a: f64, x: f64, y: f64) -> Self {
    let ca = f64::cos(a);
    let sa = f64::sin(a);
    Self {
      ele: [[ca * sx, -sa * sy, 0.0], [sa * sx, ca * sy, 0.0], [x, y, 1.0]],
    }
  }

  /// Create a zero matrix which is of almost no use
  pub fn zero() -> Self {
    Self {
      ele: [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
    }
  }

  /// Matrix will translate by the given x & y amounts
  pub fn trans(&mut self, x: f64, y: f64) {
    self.ele[2][0] = x;
    self.ele[2][1] = y;
  }

  /// Matrix will rotate by the given angle
  pub fn rot(&mut self, a: f64) {
    let c = f64::cos(a);
    let s = f64::sin(a);

    self.ele[0][0] = c;
    self.ele[1][0] = -s;
    self.ele[0][1] = s;
    self.ele[1][1] = c;
  }

  /// Matrix will scale by the given x & y scaling factors
  pub fn scale(&mut self, sx: f64, sy: f64) {
    self.ele[0][0] = sx;
    self.ele[1][1] = sy;
  }
}

impl fmt::Display for Mat3 {
  /// Output this matrix in readable form
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "[{}, {}, {}\n {}, {}, {}\n {}, {}, {}]",
      self.ele[0][0], self.ele[1][0], self.ele[2][0], self.ele[0][1], self.ele[1][1], self.ele[2][1], self.ele[0][2], self.ele[1][2], self.ele[2][2]
    )
  }
}

impl Mul<&Vec2> for Mat3 {
  type Output = Vec2;

  /// Multiply and transform given Vec2 by this matrix, assumes that w = 1, so will be treated like a point and translated
  fn mul(self, v: &Vec2) -> Vec2 {
    Vec2 {
      x: self.ele[0][0] * v.x + self.ele[1][0] * v.y + self.ele[2][0],
      y: self.ele[0][1] * v.x + self.ele[1][1] * v.y + self.ele[2][1],
    }
  }
}

impl Mul<Self> for Mat3 {
  type Output = Self;

  /// Multiply together two Mat3 to combine or compose them
  fn mul(self, m: Self) -> Self {
    let mut r = Self::zero();
    for col in 0..3 {
      for row in 0..3 {
        r.ele[col][row] = self.ele[0][row] * m.ele[col][0] + self.ele[1][row] * m.ele[col][1] + self.ele[2][row] * m.ele[col][2];
      }
    }
    r
  }
}

impl MulAssign<Self> for Mat3 {
  /// Multiply together two Mat3 to combine or compose them, mutate & store in place
  fn mul_assign(&mut self, m: Self) {
    *self = *self * m;
  }
}

impl Mul<&Vec<Vec2>> for Mat3 {
  type Output = Vec<Vec2>;

  /// Multiply a list of points by a matrix
  fn mul(self, points: &Vec<Vec2>) -> Vec<Vec2> {
    let mut out = Vec::with_capacity(points.len());

    for p in points {
      out.push(self * p);
    }

    out
  }
}
