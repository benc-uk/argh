// ==============================================================================================
// Module & file:   math / vector2.rs
// Purpose:         General purpose 2D vector maths library & operations
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use crate::{colour::Colour, engine::Engine};
use core::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Sub, SubAssign};

#[cfg(test)]
#[path = "vector2_tests.rs"]
mod vector2_tests;

/// Standard 2D vector with a pair of x, y coords
#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct Vec2 {
  pub x: f64,
  pub y: f64,
}

impl Vec2 {
  /// Construct a new vector, slightly shorter than writing Vec2 { x: 1.0, y:2.0 }
  pub fn new(x: f64, y: f64) -> Self {
    Self { x, y }
  }

  /// Construct a [0, 0] vector
  pub fn zero() -> Self {
    Self { x: 0.0, y: 0.0 }
  }

  /// Construct a [1.0, 1.0] vector
  pub fn ident() -> Self {
    Self { x: 1.0, y: 1.0 }
  }

  /// Return the length of this vector
  pub fn len(self) -> f64 {
    f64::sqrt(self.x * self.x + self.y * self.y)
  }

  /// Rotate by angle and store in place, convenience method to do this without a Mat3
  pub fn rotate(&mut self, angle_rad: f64) {
    let cos_theta = angle_rad.cos();
    let sin_theta = angle_rad.sin();

    let new_x = self.x * cos_theta - self.y * sin_theta;
    let new_y = self.x * sin_theta + self.y * cos_theta;
    self.x = new_x;
    self.y = new_y;
  }

  /// Rotate by angle, return result in a new vector, convenience method to do this without a Mat3
  pub fn rotate_new(&mut self, angle_rad: f64) -> Self {
    let cos_theta = angle_rad.cos();
    let sin_theta = angle_rad.sin();

    Self {
      x: self.x * cos_theta - self.y * sin_theta,
      y: self.x * sin_theta + self.y * cos_theta,
    }
  }

  /// Convenience to draw a 2D vector as a point/pixel on the screen
  pub fn draw(self, e: &mut Engine, colour: Colour) {
    e.set_pixel(self.x as usize, self.y as usize, colour);
  }

  /// Calculate the dot product between this Vec2 and another
  pub fn dot(self, v: Self) -> f64 {
    self.x * v.x + self.y * v.y
  }
}

impl Display for Vec2 {
  /// Display as [x, y]
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "[{}, {}]", self.x, self.y)
  }
}

impl Index<usize> for Vec2 {
  type Output = f64;

  fn index(&self, i: usize) -> &f64 {
    match i {
      0 => &self.x,
      1 => &self.y,
      _ => panic!("Vec2 index must be 0 or 1"),
    }
  }
}

impl Mul<Self> for Vec2 {
  type Output = Self;

  /// Multiply by another Vec2 and return as new value
  fn mul(self, v: Self) -> Self {
    Self { x: self.x * v.x, y: self.y * v.y }
  }
}

impl MulAssign<Self> for Vec2 {
  /// Multiply by another Vec2 and mutate in place
  fn mul_assign(&mut self, v: Self) {
    self.x *= v.x;
    self.y *= v.y;
  }
}

impl Mul<f64> for Vec2 {
  type Output = Self;

  /// Multiply by float (scale) and return as new value
  fn mul(self, s: f64) -> Self {
    Self { x: self.x * s, y: self.y * s }
  }
}

impl MulAssign<f64> for Vec2 {
  /// Multiply by float (scale) and mutate in place
  fn mul_assign(&mut self, s: f64) {
    self.x *= s;
    self.y *= s;
  }
}

impl Add<Self> for Vec2 {
  type Output = Self;

  /// Add another Vec2 and return as new value
  fn add(self, v: Self) -> Self {
    Self { x: self.x + v.x, y: self.y + v.y }
  }
}

impl AddAssign<Self> for Vec2 {
  /// Add another Vec2 and mutate in place
  fn add_assign(&mut self, v: Self) {
    self.x += v.x;
    self.y += v.y;
  }
}

impl Sub<Self> for Vec2 {
  type Output = Self;

  /// Subtract another Vec2 and return as new value
  fn sub(self, v: Self) -> Self {
    Self { x: self.x - v.x, y: self.y - v.y }
  }
}

impl SubAssign<Self> for Vec2 {
  /// Subtract another Vec2 and mutate in place
  fn sub_assign(&mut self, v: Self) {
    self.x -= v.x;
    self.y -= v.y;
  }
}

impl Div<Self> for Vec2 {
  type Output = Self;

  /// Divide by Vec2 and return as new value
  fn div(self, v: Self) -> Self {
    Self { x: self.x / v.x, y: self.y / v.y }
  }
}

impl DivAssign<Self> for Vec2 {
  /// Divide by Vec2 and mutate in place
  fn div_assign(&mut self, v: Self) {
    self.x /= v.x;
    self.y /= v.y;
  }
}
