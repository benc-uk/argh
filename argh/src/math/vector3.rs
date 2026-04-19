// ==============================================================================================
// Module & file:   math / vector3.rs
// Purpose:         General purpose 3D vector maths library & operations
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use core::fmt;
use std::fmt::Formatter;

/// Simple standard 3D vector with x, y & z coords
#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct Vec3 {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl Vec3 {
  /// Construct a new vector, slightly shorter than writing Vec3 { x: 1.0, y:2.0 }
  pub fn new(x: f64, y: f64, z: f64) -> Self {
    Vec3 { x, y, z }
  }

  /// Construct a [0, 0, 0] vector at the origin
  pub fn zero() -> Self {
    Vec3 { x: 0.0, y: 0.0, z: 0.0 }
  }
}

impl fmt::Display for Vec3 {
  /// Human readable form [x, y, z]
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
  }
}
