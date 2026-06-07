// ==============================================================================================
// Module & file:   math / affine2.rs
// Purpose:         Affine transformation matrix for 2D graphics
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use crate::math::Vec2;
use core::fmt;
use std::fmt::Formatter;
use std::ops::{Mul, MulAssign};

#[cfg(test)]
#[path = "affine2_tests.rs"]
mod affine2_tests;

/// A classic 3x3 affine transformation matrix, designed for transformations on [Vec2]
/// Rotations are based on Euler angles, which is ok for 2D
#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct Affine2 {
  ele: [[f32; 3]; 3],
}

impl Affine2 {
  /// New identity matrix
  pub fn new() -> Self {
    Self {
      ele: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
    }
  }

  /// New matrix with scale transform set
  pub fn new_scale(sx: f32, sy: f32) -> Self {
    Self {
      ele: [[sx, 0.0, 0.0], [0.0, sy, 0.0], [0.0, 0.0, 1.0]],
    }
  }

  /// New matrix with rotation transform set
  pub fn new_rot(a: f32) -> Self {
    let c = f32::cos(a);
    let s = f32::sin(a);
    Self {
      ele: [[c, -s, 0.0], [s, c, 0.0], [0.0, 0.0, 1.0]],
    }
  }

  /// New matrix with translation transform set
  pub fn new_trans(x: f32, y: f32) -> Self {
    Self {
      ele: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [x, y, 1.0]],
    }
  }

  /// Convenience & optimisation method - new matrix with scale, rotate, and translation transform set
  pub fn new_scale_rot_trans(sx: f32, sy: f32, a: f32, x: f32, y: f32) -> Self {
    let ca = f32::cos(a);
    let sa = f32::sin(a);
    Self {
      ele: [[ca * sx, -sa * sx, 0.0], [sa * sy, ca * sy, 0.0], [x, y, 1.0]],
    }
  }

  /// Create a zero matrix which is of almost no use
  pub fn zero() -> Self {
    Self {
      ele: [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
    }
  }

  /// Matrix will translate by the given x & y amounts
  pub fn trans(&mut self, x: f32, y: f32) {
    self.ele[2][0] = x;
    self.ele[2][1] = y;
  }

  /// Matrix will rotate by the given angle
  pub fn rot(&mut self, a: f32) {
    let c = f32::cos(a);
    let s = f32::sin(a);

    self.ele[0][0] = c;
    self.ele[0][1] = -s;
    self.ele[1][0] = s;
    self.ele[1][1] = c;
  }

  /// Matrix will scale by the given x & y scaling factors
  pub fn scale(&mut self, sx: f32, sy: f32) {
    self.ele[0][0] = sx;
    self.ele[1][1] = sy;
  }
}

impl fmt::Display for Affine2 {
  /// Output this matrix in readable form
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "[{}, {}, {}\n {}, {}, {}\n {}, {}, {}]",
      self.ele[0][0], self.ele[1][0], self.ele[2][0], self.ele[0][1], self.ele[1][1], self.ele[2][1], self.ele[0][2], self.ele[1][2], self.ele[2][2]
    )
  }
}

impl Mul<&Vec2> for Affine2 {
  type Output = Vec2;

  /// Multiply and transform given Vec2 by this matrix, assumes that w = 1, so will be treated like a point and translated
  fn mul(self, v: &Vec2) -> Vec2 {
    Vec2 {
      x: self.ele[0][0] * v.x + self.ele[1][0] * v.y + self.ele[2][0], // implicit * 1 removed
      y: self.ele[0][1] * v.x + self.ele[1][1] * v.y + self.ele[2][1], // implicit * 1 removed
    }
  }
}

impl Mul<Self> for Affine2 {
  type Output = Self;

  /// Multiply together two Affine2 to combine or compose them
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

impl MulAssign<Self> for Affine2 {
  /// Multiply together two Affine2 to combine or compose them, mutate & store in place
  fn mul_assign(&mut self, m: Self) {
    *self = *self * m;
  }
}

impl Mul<&Vec<Vec2>> for Affine2 {
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
