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

/// Standard 3D vector with x, y & z coords
#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct Vec3 {
  /// X component
  pub x: f32,
  /// Y component
  pub y: f32,
  /// Z component
  pub z: f32,
}

/// A Vec3 pointing along the x axis [1.0, 0.0, 0.0]
pub static V3_AXIS_X: Vec3 = Vec3 { x: 1.0, y: 0.0, z: 0.0 };
/// A Vec3 pointing along the y axis [0.0, 1.0, 0.0]
pub static V3_AXIS_Y: Vec3 = Vec3 { x: 0.0, y: 1.0, z: 0.0 };
/// A Vec3 pointing along the z axis [0.0, 0.0, 1.0]
pub static V3_AXIS_Z: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 1.0 };
/// A Vec3 = [0.0, 0.0, 0.0]
pub static V3_ZERO: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
/// A Vec3 = [1.0, 1.0, 1.0]
pub static V3_ONE: Vec3 = Vec3 { x: 1.0, y: 1.0, z: 1.0 };

impl Vec3 {
  /// Construct a new vector, slightly shorter than writing Vec3 { x:1.0, y:2.0, z:3.0 }
  pub fn new(x: f32, y: f32, z: f32) -> Self {
    Self { x, y, z }
  }

  /// Construct a new normalized vector
  pub fn new_normalized(x: f32, y: f32, z: f32) -> Self {
    let mut out = Self { x, y, z };
    out.normalize();
    out
  }

  /// Construct a [0, 0, 0] vector at the origin
  pub fn zero() -> Self {
    Self { x: 0.0, y: 0.0, z: 0.0 }
  }

  /// Construct a [1.0, 1.0, 1.0] vector
  pub fn ident() -> Self {
    Self { x: 1.0, y: 1.0, z: 1.0 }
  }

  /// Return the length of this vector
  pub fn len(self) -> f32 {
    f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
  }

  /// Calculate the dot product between this Vec3 and another
  pub fn dot(self, v: Self) -> f32 {
    self.x * v.x + self.y * v.y + self.z * v.z
  }

  /// Calculate the cross product between this Vec3 and another
  pub fn cross(self, v: Self) -> Self {
    Self {
      x: self.y * v.z - self.z * v.y,
      y: self.z * v.x - self.x * v.z,
      z: self.x * v.y - self.y * v.x,
    }
  }

  /// The distance between this Vec3 and another
  pub fn dist(self, v: Self) -> f32 {
    let a = v.x - self.x;
    let b = v.y - self.y;
    let c = v.z - self.z;

    f32::sqrt(a * a + b * b + c * c)
  }

  /// Normalize this Vec3 in place
  pub fn normalize(&mut self) {
    let len = self.len();
    self.x /= len;
    self.y /= len;
    self.z /= len;
  }

  /// Normalize this Vec3 in to a new Vec3
  pub fn normalize_new(&self) -> Self {
    let len = self.len();
    Self {
      x: self.x / len,
      y: self.y / len,
      z: self.z / len,
    }
  }

  /// Reflect this vector about a (unit) normal.
  /// Uses the "incident points into surface" convention:
  ///   R = I - 2(N·I)N
  pub fn reflect(&self, n: Self) -> Self {
    *self - n * (2.0 * self.dot(n))
  }

  /// Invert direction of this vector
  pub fn invert(self) -> Self {
    Self {
      x: -self.x,
      y: -self.y,
      z: -self.z,
    }
  }
}

impl fmt::Display for Vec3 {
  /// Human readable form [x, y, z]
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
  }
}

impl Index<usize> for Vec3 {
  type Output = f32;

  /// Get the value of the element at given index
  fn index(&self, i: usize) -> &f32 {
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

impl Mul<f32> for Vec3 {
  type Output = Self;

  /// Multiply by a float (scale) and return as new value
  fn mul(self, s: f32) -> Self {
    Self {
      x: self.x * s,
      y: self.y * s,
      z: self.z * s,
    }
  }
}

impl MulAssign<f32> for Vec3 {
  /// Multiply by a float (scale) and mutate in place
  fn mul_assign(&mut self, s: f32) {
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

impl Div<f32> for Vec3 {
  type Output = Self;

  /// Divide by a scalar and return as new value
  fn div(self, s: f32) -> Self {
    Self {
      x: self.x / s,
      y: self.y / s,
      z: self.z / s,
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
