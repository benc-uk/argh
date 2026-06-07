// ==============================================================================================
// Module & file:   math / matrix3.rs
// Purpose:         3x3 transformation matrix, standard maths operations not to be used for transforms
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use crate::math::{Mat4, Vec3};
use core::fmt;
use std::fmt::Formatter;
use std::ops::{Mul, MulAssign};

#[cfg(test)]
#[path = "matrix3_tests.rs"]
mod matrix3_tests;

/// A pure 3x3 matrix, designed for math operations not general purpose transforms, use [Mat4] for that
/// Only really used for transforming normals
#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct Mat3 {
  ele: [[f32; 3]; 3],
}

impl Mat3 {
  /// New identity matrix
  pub fn new() -> Self {
    Self {
      ele: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
    }
  }

  /// Create a zero matrix which is of almost no use
  pub fn zero() -> Self {
    Self {
      ele: [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
    }
  }

  /// Create a Mat3 from the upper left of a Mat4
  pub fn from_mat4_upper(m: &Mat4) -> Self {
    let e = m.raw();
    Self {
      ele: [[e[0][0], e[0][1], e[0][2]], [e[1][0], e[1][1], e[1][2]], [e[2][0], e[2][1], e[2][2]]],
    }
  }

  /// Transpose: swap rows and columns.
  pub fn transpose(&self) -> Self {
    Self {
      ele: [
        [self.ele[0][0], self.ele[1][0], self.ele[2][0]],
        [self.ele[0][1], self.ele[1][1], self.ele[2][1]],
        [self.ele[0][2], self.ele[1][2], self.ele[2][2]],
      ],
    }
  }

  /// Determinant via cofactor expansion along the first row.
  pub fn determinant(&self) -> f32 {
    let a = self.ele[0][0];
    let b = self.ele[1][0];
    let c = self.ele[2][0];
    let d = self.ele[0][1];
    let e = self.ele[1][1];
    let f = self.ele[2][1];
    let g = self.ele[0][2];
    let h = self.ele[1][2];
    let i = self.ele[2][2];

    a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g)
  }

  /// Matrix inverse. Returns None if the matrix is singular (det ~= 0).
  pub fn inverse(&self) -> Option<Self> {
    let a = self.ele[0][0];
    let b = self.ele[1][0];
    let c = self.ele[2][0];
    let d = self.ele[0][1];
    let e = self.ele[1][1];
    let f = self.ele[2][1];
    let g = self.ele[0][2];
    let h = self.ele[1][2];
    let i = self.ele[2][2];

    let c00 = e * i - f * h;
    let c01 = -(d * i - f * g);
    let c02 = d * h - e * g;
    let c10 = -(b * i - c * h);
    let c11 = a * i - c * g;
    let c12 = -(a * h - b * g);
    let c20 = b * f - c * e;
    let c21 = -(a * f - c * d);
    let c22 = a * e - b * d;

    let det = a * c00 + b * c01 + c * c02;
    if det.abs() < 1e-12 {
      return None;
    }
    let inv_det = 1.0 / det;

    // Adjugate (transposed cofactor matrix) divided by determinant.
    // Stored column-major, so each inner array is one column.
    Some(Self {
      ele: [
        [c00 * inv_det, c01 * inv_det, c02 * inv_det],
        [c10 * inv_det, c11 * inv_det, c12 * inv_det],
        [c20 * inv_det, c21 * inv_det, c22 * inv_det],
      ],
    })
  }

  /// Combined inverse + transpose. The normal-transform matrix.
  /// Cheaper than `m.inverse()?.transpose()` because the transpose folds
  /// into the cofactor layout.
  pub fn inverse_transpose(&self) -> Option<Self> {
    let a = self.ele[0][0];
    let b = self.ele[1][0];
    let c = self.ele[2][0];
    let d = self.ele[0][1];
    let e = self.ele[1][1];
    let f = self.ele[2][1];
    let g = self.ele[0][2];
    let h = self.ele[1][2];
    let i = self.ele[2][2];

    let c00 = e * i - f * h;
    let c01 = -(d * i - f * g);
    let c02 = d * h - e * g;
    let c10 = -(b * i - c * h);
    let c11 = a * i - c * g;
    let c12 = -(a * h - b * g);
    let c20 = b * f - c * e;
    let c21 = -(a * f - c * d);
    let c22 = a * e - b * d;

    let det = a * c00 + b * c01 + c * c02;
    if det.abs() < 1e-12 {
      return None;
    }
    let inv_det = 1.0 / det;

    // Cofactor matrix stored column-major.
    // Cofactor row 0 becomes column 0, row 1 becomes column 1, row 2 becomes column 2.
    Some(Self {
      ele: [
        [c00 * inv_det, c10 * inv_det, c20 * inv_det],
        [c01 * inv_det, c11 * inv_det, c21 * inv_det],
        [c02 * inv_det, c12 * inv_det, c22 * inv_det],
      ],
    })
  }
}

impl fmt::Display for Mat3 {
  /// Output this matrix in readable form
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "[{}, {}, {}\n {}, {}, {}\n {}, {}, {}]",
      self.ele[0][0], self.ele[1][0], self.ele[2][0], self.ele[0][1], self.ele[1][1], self.ele[2][1], self.ele[0][2], self.ele[1][2], self.ele[2][2],
    )
  }
}

impl Mul<&Vec3> for Mat3 {
  type Output = Vec3;

  /// Linear transform of a 3D vector. No translation (Mat3 cannot express it).
  fn mul(self, v: &Vec3) -> Vec3 {
    Vec3 {
      x: self.ele[0][0] * v.x + self.ele[1][0] * v.y + self.ele[2][0] * v.z,
      y: self.ele[0][1] * v.x + self.ele[1][1] * v.y + self.ele[2][1] * v.z,
      z: self.ele[0][2] * v.x + self.ele[1][2] * v.y + self.ele[2][2] * v.z,
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
