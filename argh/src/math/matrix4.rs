// ==============================================================================================
// Module & file:   math / matrix4.rs
// Purpose:         4x4 affine transformation matrix for 3D graphics
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use crate::math::{Quat, Vec3};
use core::fmt;
use std::fmt::Formatter;
use std::ops::{Mul, MulAssign};

#[cfg(test)]
#[path = "matrix4_tests.rs"]
mod matrix4_tests;

/// A classic 4x4 affine transformation matrix, designed for transformations on [Vec2]
/// Rotations are based on quaternions
#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct Mat4 {
  ele: [[f64; 4]; 4],
}

impl Mat4 {
  /// New identity matrix
  pub fn new() -> Self {
    Self {
      ele: [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]],
    }
  }

  /// New matrix with scale transform set
  pub fn new_scale(sx: f64, sy: f64, sz: f64) -> Self {
    Self {
      ele: [[sx, 0.0, 0.0, 0.0], [0.0, sy, 0.0, 0.0], [0.0, 0.0, sz, 0.0], [0.0, 0.0, 0.0, 1.0]],
    }
  }

  /// New matrix with rotation transform set
  pub fn new_rot(q: Quat) -> Self {
    let x = q.x;
    let y = q.y;
    let z = q.z;
    let w = q.w;

    let xx = x * x;
    let yy = y * y;
    let zz = z * z;
    let xy = x * y;
    let xz = x * z;
    let yz = y * z;
    let wx = w * x;
    let wy = w * y;
    let wz = w * z;

    Self {
      ele: [
        [1.0 - 2.0 * (yy + zz), 2.0 * (xy + wz), 2.0 * (xz - wy), 0.0],
        [2.0 * (xy - wz), 1.0 - 2.0 * (xx + zz), 2.0 * (yz + wx), 0.0],
        [2.0 * (xz + wy), 2.0 * (yz - wx), 1.0 - 2.0 * (xx + yy), 0.0],
        [0.0, 0.0, 0.0, 1.0],
      ],
    }
  }

  /// New matrix with translation transform set
  pub fn new_trans(x: f64, y: f64, z: f64) -> Self {
    Self {
      ele: [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [x, y, z, 1.0]],
    }
  }

  /// Convenience & optimisation method - new matrix with scale, rotate, and translation transform set
  pub fn new_scale_rot_trans(sx: f64, sy: f64, sz: f64, rot: Quat, x: f64, y: f64, z: f64) -> Self {
    let mut out = Self::new_rot(rot);
    out.scale(sx, sy, sz);
    out.trans(x, y, z);
    out
  }

  /// Create a zero matrix which is of almost no use
  pub fn zero() -> Self {
    Self {
      ele: [[0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0]],
    }
  }

  /// Matrix will translate by the given x & y amounts
  pub fn trans(&mut self, x: f64, y: f64, z: f64) {
    self.ele[3][0] = x;
    self.ele[3][1] = y;
    self.ele[3][2] = z;
  }

  /// Matrix will scale by the given x & y scaling factors
  pub fn scale(&mut self, sx: f64, sy: f64, sz: f64) {
    self.ele[0][0] = sx;
    self.ele[1][1] = sy;
    self.ele[2][2] = sz;
  }
}

impl fmt::Display for Mat4 {
  /// Output this matrix in readable form
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "[{}, {}, {}, {}\n {}, {}, {}, {}\n {}, {}, {}, {},\n {}, {}, {}, {}]",
      self.ele[0][0],
      self.ele[1][0],
      self.ele[2][0],
      self.ele[3][0],
      self.ele[0][1],
      self.ele[1][1],
      self.ele[2][1],
      self.ele[3][1],
      self.ele[0][2],
      self.ele[1][2],
      self.ele[2][2],
      self.ele[3][2],
      self.ele[0][3],
      self.ele[1][3],
      self.ele[2][3],
      self.ele[3][3]
    )
  }
}

impl Mul<&Vec3> for Mat4 {
  type Output = Vec3;

  /// Multiply and transform given Vec3 by this matrix, assumes that w = 1, so will be treated like a point and translated
  fn mul(self, v: &Vec3) -> Vec3 {
    Vec3 {
      x: self.ele[0][0] * v.x + self.ele[1][0] * v.y + self.ele[2][0] * v.z + self.ele[3][0], // implicit * 1 removed
      y: self.ele[0][1] * v.x + self.ele[1][1] * v.y + self.ele[2][1] * v.z + self.ele[3][1], // implicit * 1 removed
      z: self.ele[0][2] * v.x + self.ele[1][2] * v.y + self.ele[2][2] * v.z + self.ele[3][2], // implicit * 1 removed
    }
  }
}

impl Mul<Self> for Mat4 {
  type Output = Self;

  /// Multiply together two Mat4 to combine or compose them
  fn mul(self, m: Self) -> Self {
    let mut r = Self::zero();
    for col in 0..4 {
      for row in 0..4 {
        r.ele[col][row] = self.ele[0][row] * m.ele[col][0] + self.ele[1][row] * m.ele[col][1] + self.ele[2][row] * m.ele[col][2] + self.ele[3][row] * m.ele[col][3];
      }
    }
    r
  }
}

impl MulAssign<Self> for Mat4 {
  /// Multiply together two Mat3 to combine or compose them, mutate & store in place
  fn mul_assign(&mut self, m: Self) {
    *self = *self * m;
  }
}

impl Mul<&Vec<Vec3>> for Mat4 {
  type Output = Vec<Vec3>;

  /// Multiply a list of points by a matrix
  fn mul(self, points: &Vec<Vec3>) -> Vec<Vec3> {
    let mut out = Vec::with_capacity(points.len());

    for p in points {
      out.push(self * p);
    }

    out
  }
}
