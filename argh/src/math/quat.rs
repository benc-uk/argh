// ==============================================================================================
// Module & file:   math / quat.rs
// Purpose:         Quaternion needed for 3D rotations
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use crate::math::{V3_AXIS_X, V3_AXIS_Y, V3_AXIS_Z, Vec3};
use std::ops::Mul;

#[cfg(test)]
#[path = "quat_tests.rs"]
mod quat_tests;

/// A minimal implementation of of a 4-tuple Quaternion (x, y, z, w) used for rotations
#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct Quat {
  /// Imaginary `i` component
  pub x: f32,
  /// Imaginary `j` component
  pub y: f32,
  /// Imaginary `k` component
  pub z: f32,
  /// Scalar (real) component
  pub w: f32,
}

impl Quat {
  /// Create a Quaternion with given angle, around given axis
  /// # Arguments
  /// * `axis` - Vector representing the axis, should be normalized
  /// * `a` - Angle in radians
  pub fn new(axis: Vec3, a: f32) -> Self {
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

  /// Rotate a vector by this quaternion. Assumes quat is unit length.
  pub fn rotate_vec3(&self, v: Vec3) -> Vec3 {
    // Optimised form: v' = v + 2*q.xyz × (q.xyz × v + q.w * v)
    // Avoids constructing q⁻¹ and doing two full quaternion multiplies.
    let q_xyz = Vec3 { x: self.x, y: self.y, z: self.z };
    let t = q_xyz.cross(v) * 2.0;
    v + t * self.w + q_xyz.cross(t)
  }

  /// Rotate around the local X axis by given angle (post-multiplies)
  pub fn rot_x(&mut self, a: f32) {
    self.rotate_local(V3_AXIS_X, a);
  }

  /// Rotate around the local Y axis by given angle (post-multiplies)
  pub fn rot_y(&mut self, a: f32) {
    self.rotate_local(V3_AXIS_Y, a);
  }

  /// Rotate around the local Z axis by given angle (post-multiplies)
  pub fn rot_z(&mut self, a: f32) {
    self.rotate_local(V3_AXIS_Z, a);
  }

  /// Rotate around the world X axis by given angle (pre-multiplies)
  pub fn rot_x_world(&mut self, a: f32) {
    self.rotate_world(V3_AXIS_X, a);
  }

  /// Rotate around the world Y axis by given angle (pre-multiplies)
  pub fn rot_y_world(&mut self, a: f32) {
    self.rotate_world(V3_AXIS_Y, a);
  }

  /// Rotate around the world Z axis by given angle (pre-multiplies)
  pub fn rot_z_world(&mut self, a: f32) {
    self.rotate_world(V3_AXIS_Z, a);
  }

  /// Rotate around an axis in world space
  pub fn rotate_world(&mut self, axis: Vec3, a: f32) {
    *self = Self::new(axis, a) * *self; // pre-multiply
  }

  /// Rotate around an axis in local object space
  pub fn rotate_local(&mut self, axis: Vec3, a: f32) {
    *self = *self * Self::new(axis, a); // post-multiply
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
