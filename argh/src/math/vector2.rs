// ==============================================================================================
// 2D Vectors
// ==============================================================================================

use crate::{colour::Colour, engine::Engine};
use core::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Sub, SubAssign};

#[cfg(test)]
#[path = "vector2_tests.rs"]
mod vector2_tests;

/// Simple standard 2D vector with x, y coords
#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct Vec2 {
  pub x: f64,
  pub y: f64,
}

impl Vec2 {
  /// Construct a new vector, slightly shorter than writing Vec2 { x: 1.0, y:2.0 }
  pub fn new(x: f64, y: f64) -> Self {
    Vec2 { x, y }
  }

  /// Construct a [0, 0] vector
  pub fn zero() -> Self {
    Vec2 { x: 0.0, y: 0.0 }
  }

  /// Construct a [1.0, 1.0] vector
  pub fn ident() -> Self {
    Vec2 { x: 1.0, y: 1.0 }
  }

  /// Return the length of this vector
  pub fn len(self) -> f64 {
    f64::sqrt(self.x * self.x + self.y * self.y)
  }

  /// Rotate by angle and store in place
  pub fn rotate(&mut self, angle_rad: f64) {
    let cos_theta = angle_rad.cos();
    let sin_theta = angle_rad.sin();

    let new_x = self.x * cos_theta - self.y * sin_theta;
    let new_y = self.x * sin_theta + self.y * cos_theta;
    self.x = new_x;
    self.y = new_y;
  }

  /// Rotate by angle, return result in a new vector
  pub fn rotate_new(&mut self, angle_rad: f64) -> Vec2 {
    let cos_theta = angle_rad.cos();
    let sin_theta = angle_rad.sin();

    Vec2 {
      x: self.x * cos_theta - self.y * sin_theta,
      y: self.x * sin_theta + self.y * cos_theta,
    }
  }

  /// Convenience to draw a 2D vector as a point/pixel on the screen
  pub fn draw(self, e: &mut Engine, colour: Colour) {
    e.set_pixel(self.x as usize, self.y as usize, colour);
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

impl Mul<Vec2> for Vec2 {
  type Output = Vec2;

  /// Multiply by Vec2 and return as new value
  fn mul(self, v: Vec2) -> Vec2 {
    Vec2 { x: self.x * v.x, y: self.y * v.y }
  }
}

impl MulAssign<Vec2> for Vec2 {
  /// Multiply by Vec2 and mutate in place
  fn mul_assign(&mut self, v: Vec2) {
    self.x *= v.x;
    self.y *= v.y;
  }
}

impl Mul<f64> for Vec2 {
  type Output = Vec2;

  /// Multiply by float (scale) and return as new value
  fn mul(self, s: f64) -> Vec2 {
    Vec2 { x: self.x * s, y: self.y * s }
  }
}

impl MulAssign<f64> for Vec2 {
  /// Multiply by float (scale) and mutate in place
  fn mul_assign(&mut self, s: f64) {
    self.x *= s;
    self.y *= s;
  }
}

impl Add<Vec2> for Vec2 {
  type Output = Vec2;

  /// Add another Vec2 and return as new value
  fn add(self, v: Vec2) -> Vec2 {
    Vec2 { x: self.x + v.x, y: self.y + v.y }
  }
}

impl AddAssign<Vec2> for Vec2 {
  /// Add another Vec2 and mutate in place
  fn add_assign(&mut self, v: Vec2) {
    self.x += v.x;
    self.y += v.y;
  }
}

impl Sub<Vec2> for Vec2 {
  type Output = Vec2;

  /// Subtract another Vec2 and return as new value
  fn sub(self, v: Vec2) -> Vec2 {
    Vec2 { x: self.x - v.x, y: self.y - v.y }
  }
}

impl SubAssign<Vec2> for Vec2 {
  /// Subtract another Vec2 and mutate in place
  fn sub_assign(&mut self, v: Vec2) {
    self.x -= v.x;
    self.y -= v.y;
  }
}

impl Div<Vec2> for Vec2 {
  type Output = Vec2;

  /// Divide by Vec2 and return as new value
  fn div(self, v: Vec2) -> Vec2 {
    Vec2 { x: self.x / v.x, y: self.y / v.y }
  }
}

impl DivAssign<Vec2> for Vec2 {
  /// Divide by Vec2 and mutate in place
  fn div_assign(&mut self, v: Vec2) {
    self.x /= v.x;
    self.y /= v.y;
  }
}
