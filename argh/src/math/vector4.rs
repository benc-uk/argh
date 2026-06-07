// ==============================================================================================
// Module & file:   math / vector4.rs
// Purpose:         General purpose 4D vector maths library & operations
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use core::fmt;
use std::{
  fmt::Formatter,
  ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Sub, SubAssign},
};

#[cfg(test)]
#[path = "vector4_tests.rs"]
mod vector4_tests;

/// Standard 4D vector with x, y, z & w coords
#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct Vec4 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
  pub w: f32,
}

impl Vec4 {
  /// Construct a new vector, slightly shorter than writing Vec3 { x:5.0, y:2.0, z:3.0, w:1.0 }
  pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
    Self { x, y, z, w }
  }

  /// Construct a [0, 0, 0, 0.0] vector at the origin
  pub fn zero() -> Self {
    Self { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }
  }

  /// Construct a [1.0, 1.0, 1.0, 1.0] vector
  pub fn ident() -> Self {
    Self { x: 1.0, y: 1.0, z: 1.0, w: 1.0 }
  }

  /// Return the length of this vector
  pub fn len(self) -> f32 {
    f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w)
  }

  /// Calculate the dot product between this Vec4 and another
  pub fn dot(self, v: Self) -> f32 {
    self.x * v.x + self.y * v.y + self.z * v.z + self.w * v.w
  }

  /// The distance between this Vec4 and another
  pub fn dist(self, v: Self) -> f32 {
    let a = v.x - self.x;
    let b = v.y - self.y;
    let c = v.z - self.z;
    let d = v.w - self.w;

    f32::sqrt(a * a + b * b + c * c + d * d)
  }

  /// Normalize this Vec4 in place
  pub fn normalize(&mut self) {
    let len = self.len();
    self.x /= len;
    self.y /= len;
    self.z /= len;
    self.w /= len;
  }
}

impl fmt::Display for Vec4 {
  /// Human readable form [x, y, z, w]
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "[{}, {}, {}, {}]", self.x, self.y, self.z, self.w)
  }
}

impl Index<usize> for Vec4 {
  type Output = f32;

  /// Get the value of the element at given index
  fn index(&self, i: usize) -> &f32 {
    match i {
      0 => &self.x,
      1 => &self.y,
      2 => &self.z,
      3 => &self.w,
      _ => panic!("Vec4 index must be in range: 0~3"),
    }
  }
}

impl Mul<Self> for Vec4 {
  type Output = Self;

  /// Multiply by another Vec4 and return as new value
  fn mul(self, v: Self) -> Self {
    Self {
      x: self.x * v.x,
      y: self.y * v.y,
      z: self.z * v.z,
      w: self.w * v.w,
    }
  }
}

impl MulAssign<Self> for Vec4 {
  /// Multiply by another Vec3 and mutate in place
  fn mul_assign(&mut self, v: Self) {
    self.x *= v.x;
    self.y *= v.y;
    self.z *= v.z;
    self.w *= v.w;
  }
}

impl Mul<f32> for Vec4 {
  type Output = Self;

  /// Multiply by a float (scale) and return as new value
  fn mul(self, s: f32) -> Self {
    Self {
      x: self.x * s,
      y: self.y * s,
      z: self.z * s,
      w: self.w * s,
    }
  }
}

impl MulAssign<f32> for Vec4 {
  /// Multiply by a float (scale) and mutate in place
  fn mul_assign(&mut self, s: f32) {
    self.x *= s;
    self.y *= s;
    self.z *= s;
    self.w *= s;
  }
}

impl Add<Self> for Vec4 {
  type Output = Self;

  /// Add another Vec3 and return as new value
  fn add(self, v: Self) -> Self {
    Self {
      x: self.x + v.x,
      y: self.y + v.y,
      z: self.z + v.z,
      w: self.w + v.w,
    }
  }
}

impl AddAssign<Self> for Vec4 {
  /// Add another Vec3 and mutate in place
  fn add_assign(&mut self, v: Self) {
    self.x += v.x;
    self.y += v.y;
    self.z += v.z;
    self.w += v.w;
  }
}

impl Sub<Self> for Vec4 {
  type Output = Self;

  /// Subtract another Vec3 and return as new value
  fn sub(self, v: Self) -> Self {
    Self {
      x: self.x - v.x,
      y: self.y - v.y,
      z: self.z - v.z,
      w: self.w - v.w,
    }
  }
}

impl SubAssign<Self> for Vec4 {
  /// Subtract another Vec3 and mutate in place
  fn sub_assign(&mut self, v: Self) {
    self.x -= v.x;
    self.y -= v.y;
    self.z -= v.z;
    self.w -= v.w;
  }
}

impl Div<Self> for Vec4 {
  type Output = Self;

  /// Divide by another Vec3 and return as new value
  fn div(self, v: Self) -> Self {
    Self {
      x: self.x / v.x,
      y: self.y / v.y,
      z: self.z / v.z,
      w: self.w / v.w,
    }
  }
}

impl DivAssign<Self> for Vec4 {
  /// Divide by another Vec3 and mutate in place
  fn div_assign(&mut self, v: Self) {
    self.x /= v.x;
    self.y /= v.y;
    self.z /= v.z;
    self.w /= v.w;
  }
}
