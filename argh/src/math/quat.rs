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
  pub x: f64, // imaginary i
  pub y: f64, // imaginary j
  pub z: f64, // imaginary k
  pub w: f64, // scalar (real) part
}

impl Quat {
  /// Create a Quaternion with given angle, around given axis
  /// # Arguments
  /// * `axis` - Vector representing the axis, should be normalized
  /// * `a` - Angle in radians
  pub fn new(axis: Vec3, a: f64) -> Self {
    let half = a * 0.5;
    let s = half.sin();
    Self {
      x: axis.x * s,
      y: axis.y * s,
      z: axis.z * s,
      w: half.cos(),
    }
  }

  /// Create identity Quat
  pub fn ident() -> Self {
    Self { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }
  }

  /// Normalize this Quaternion
  pub fn normalise(&self) -> Self {
    let len = (self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
    let inv = 1.0 / len;
    Self {
      x: self.x * inv,
      y: self.y * inv,
      z: self.z * inv,
      w: self.w * inv,
    }
  }

  /// Rotate around X axis by given angle
  pub fn rot_x(&mut self, a: f64) {
    let half = a * 0.5;
    let s = f64::sin(half);
    let c = f64::cos(half);
    self.x = self.x * c + self.w * s;
    self.y = self.y * c + self.z * s;
    self.z = self.z * c - self.y * s;
    self.w = self.w * c - self.x * s;
  }

  /// Rotate around Y axis by given angle
  pub fn rot_y(&mut self, a: f64) {
    let half = a * 0.5;
    let s = f64::sin(half);
    let c = f64::cos(half);
    self.x = self.x * c - self.z * s;
    self.y = self.y * c + self.w * s;
    self.z = self.z * c + self.x * s;
    self.w = self.w * c - self.y * s;
  }

  /// Rotate around Z axis by given angle
  pub fn rot_z(&mut self, a: f64) {
    let half = a * 0.5;
    let s = f64::sin(half);
    let c = f64::cos(half);
    self.x = self.x * c + self.y * s;
    self.y = self.y * c - self.x * s;
    self.z = self.z * c + self.w * s;
    self.w = self.w * c - self.z * s;
  }
}

impl Mul for Quat {
  type Output = Self;

  /// Combine Quaternions using multiply
  fn mul(self, q: Self) -> Self {
    Self {
      x: q.w * self.x + q.x * self.w - q.y * self.z + q.z * self.y,
      y: q.w * self.y + q.x * self.z + q.y * self.w - q.z * self.x,
      z: q.w * self.z - q.x * self.y + q.y * self.x + q.z * self.w,
      w: q.w * self.w - q.x * self.x - q.y * self.y - q.z * self.z,
    }
  }
}
