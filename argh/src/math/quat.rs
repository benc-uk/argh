// ==============================================================================================
// Module & file:   math / quat.rs
// Purpose:         Quaternion needed for 3D rotations
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use crate::math::Vec3;
use std::ops::Mul;

#[cfg(test)]
#[path = "quat_tests.rs"]
mod quat_tests;

#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct Quat {
  pub w: f64, // scalar (real) part
  pub x: f64, // imaginary i
  pub y: f64, // imaginary j
  pub z: f64, // imaginary k
}

impl Quat {
  pub fn new(axis: Vec3, a: f64) -> Self {
    let half = a * 0.5;
    let s = half.sin();
    Self {
      w: half.cos(),
      x: axis.x * s,
      y: axis.y * s,
      z: axis.z * s,
    }
  }

  /// Keep it unit-length (accumulated multiplies drift over time)
  pub fn normalise(&self) -> Quat {
    let len = (self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
    let inv = 1.0 / len;
    Quat {
      w: self.w * inv,
      x: self.x * inv,
      y: self.y * inv,
      z: self.z * inv,
    }
  }
}

impl Mul for Quat {
  type Output = Self;

  fn mul(self, q: Quat) -> Self {
    Quat {
      w: q.w * self.w - q.x * self.x - q.y * self.y - q.z * self.z,
      x: q.w * self.x + q.x * self.w - q.y * self.z + q.z * self.y,
      y: q.w * self.y + q.x * self.z + q.y * self.w - q.z * self.x,
      z: q.w * self.z - q.x * self.y + q.y * self.x + q.z * self.w,
    }
  }
}
