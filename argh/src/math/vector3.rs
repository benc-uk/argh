// ==============================================================================================
// Module & file:   math / vector3.rs
// Purpose:         General purpose 3D vector maths library & operations
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
#[path = "vector3_tests.rs"]
mod vector3_tests;

/// Simple standard 3D vector with x, y & z coords
#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct Vec3 {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl Vec3 {
  /// Construct a new vector, slightly shorter than writing Vec3 { x:1.0, y:2.0, z:3.0 }
  pub fn new(x: f64, y: f64, z: f64) -> Self {
    Self { x, y, z }
  }

  /// Construct a [0, 0, 0] vector at the origin
  pub fn zero() -> Self {
    Self { x: 0.0, y: 0.0, z: 0.0 }
  }

  /// Construct a [1.0, 1.0] vector
  pub fn ident() -> Self {
    Self { x: 1.0, y: 1.0, z: 1.0 }
  }

  /// Return the length of this vector
  pub fn len(self) -> f64 {
    f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
  }

  /// Calculate the dot product between this Vec3 and another
  pub fn dot(self, v: Self) -> f64 {
    self.x * v.x + self.y * v.y + self.z * v.z
  }
}

impl fmt::Display for Vec3 {
  /// Human readable form [x, y, z]
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
  }
}

impl Index<usize> for Vec3 {
  type Output = f64;

  fn index(&self, i: usize) -> &f64 {
    match i {
      0 => &self.x,
      1 => &self.y,
      2 => &self.z,
      _ => panic!("Vec3 index must be 0, 1 or 2"),
    }
  }
}

impl Mul<Self> for Vec3 {
  type Output = Self;

  /// Multiply by another Vec3 and return as new value
  fn mul(self, v: Self) -> Self {
    Self {
      x: self.x * v.x,
      y: self.y * v.y,
      z: self.z * v.z,
    }
  }
}

impl MulAssign<Self> for Vec3 {
  /// Multiply by another Vec3 and mutate in place
  fn mul_assign(&mut self, v: Self) {
    self.x *= v.x;
    self.y *= v.y;
    self.z *= v.z;
  }
}

impl Mul<f64> for Vec3 {
  type Output = Self;

  /// Multiply by a float (scale) and return as new value
  fn mul(self, s: f64) -> Self {
    Self {
      x: self.x * s,
      y: self.y * s,
      z: self.z * s,
    }
  }
}

impl MulAssign<f64> for Vec3 {
  /// Multiply by a float (scale) and mutate in place
  fn mul_assign(&mut self, s: f64) {
    self.x *= s;
    self.y *= s;
    self.z *= s;
  }
}

impl Add<Self> for Vec3 {
  type Output = Self;

  /// Add another Vec3 and return as new value
  fn add(self, v: Self) -> Self {
    Self {
      x: self.x + v.x,
      y: self.y + v.y,
      z: self.z + v.z,
    }
  }
}

impl AddAssign<Self> for Vec3 {
  /// Add another Vec3 and mutate in place
  fn add_assign(&mut self, v: Self) {
    self.x += v.x;
    self.y += v.y;
    self.z += v.z;
  }
}

impl Sub<Self> for Vec3 {
  type Output = Self;

  /// Subtract another Vec3 and return as new value
  fn sub(self, v: Self) -> Self {
    Self {
      x: self.x - v.x,
      y: self.y - v.y,
      z: self.z - v.z,
    }
  }
}

impl SubAssign<Self> for Vec3 {
  /// Subtract another Vec3 and mutate in place
  fn sub_assign(&mut self, v: Self) {
    self.x -= v.x;
    self.y -= v.y;
    self.z -= v.z;
  }
}

impl Div<Self> for Vec3 {
  type Output = Self;

  /// Divide by another Vec3 and return as new value
  fn div(self, v: Self) -> Self {
    Self {
      x: self.x / v.x,
      y: self.y / v.y,
      z: self.z / v.z,
    }
  }
}

impl DivAssign<Self> for Vec3 {
  /// Divide by another Vec3 and mutate in place
  fn div_assign(&mut self, v: Self) {
    self.x /= v.x;
    self.y /= v.y;
    self.z /= v.z;
  }
}
