// ==============================================================================================
// Module & file:   helpers.rs
// Purpose:         Helper functions for generating geometry such as regular polygons
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use crate::math::Vec2;
use std::f64::consts::PI;

/// Construct a basic regular polygon, triangle, square, pentagon, hexagon etc
pub fn simple_poly(count: u32, size: f64) -> Vec<Vec2> {
  let mut out = vec![];
  if count < 3 {
    return out;
  }

  for i in 0..count {
    let mut p = Vec2::new(size, 0.0);
    p.rotate(((2.0 * PI) / count as f64) * i as f64);
    out.push(p);
  }
  out.push(Vec2::new(size, 0.0));

  out
}
