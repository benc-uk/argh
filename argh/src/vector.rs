use std::f64;

use crate::{colour::Colour, engine::Engine};

#[cfg(test)]
#[path = "vector_test.rs"]
mod vector_test;

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

  /// Add v to this vector, mutate and store in place
  pub fn add(&mut self, v: Vec2) {
    self.x += v.x;
    self.y += v.y;
  }

  /// Add v to this vector, return result in a new vector
  pub fn add_new(self, v: Vec2) -> Vec2 {
    Vec2 { x: self.x + v.x, y: self.y + v.y }
  }

  // Subtract v from this vector, mutate and store in place
  pub fn sub(&mut self, v: Vec2) {
    self.x -= v.x;
    self.y -= v.y;
  }

  /// Subtract v from this vector, return result in a new vector
  pub fn sub_new(self, v: Vec2) -> Vec2 {
    Vec2 { x: self.x - v.x, y: self.y - v.y }
  }

  /// Multiply this vector by v, mutate and store in place
  pub fn mult(&mut self, v: Vec2) {
    self.x *= v.x;
    self.y *= v.y;
  }

  /// Multiply this vector by v, return result in a new vector
  pub fn mult_new(&mut self, v: Vec2) -> Vec2 {
    Vec2 { x: self.x * v.x, y: self.y * v.y }
  }

  /// Divide this vector by v, mutate and store in place
  pub fn div(&mut self, v: Vec2) {
    self.x /= v.x;
    self.y /= v.y;
  }

  /// Divide this vector by v and return a new Vec2 with the result
  pub fn div_new(&mut self, v: Vec2) -> Vec2 {
    Vec2 { x: self.x / v.x, y: self.y / v.y }
  }

  /// Scale this vector in place
  pub fn scale(&mut self, s: f64) {
    self.x *= s;
    self.y *= s;
  }

  /// Scale this vector, return result in a new vector
  pub fn scale_new(&mut self, s: f64) -> Vec2 {
    Vec2 { x: self.x * s, y: self.y * s }
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

  pub fn draw(self, e: &mut Engine, colour: Colour) {
    e.set_pixel(self.x as usize, self.y as usize, colour);
  }
}
