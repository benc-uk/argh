// ==============================================================================================
// Module & file:   helpers.rs
// Purpose:         INTERNAL - Helper functions
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use crate::{
  math::{V3_ZERO, Vec3, Vec4},
  prelude::v3,
};

#[cfg(test)]
#[path = "tests/helpers_tests.rs"]
mod helpers_tests;

// One bit per frustum plane
pub(crate) const OUT_LEFT: u8 = 1 << 0;
pub(crate) const OUT_RIGHT: u8 = 1 << 1;
pub(crate) const OUT_BOTTOM: u8 = 1 << 2;
pub(crate) const OUT_TOP: u8 = 1 << 3;
pub(crate) const OUT_NEAR: u8 = 1 << 4;
pub(crate) const OUT_FAR: u8 = 1 << 5;

/// Don't ask me to explain this one!
#[inline(always)]
pub(crate) fn compute_outcode(v: &Vec4) -> u8 {
  let mut code = 0u8;
  if v.x + v.w < 0.0 {
    code |= OUT_LEFT;
  }
  if v.w - v.x < 0.0 {
    code |= OUT_RIGHT;
  }
  if v.y + v.w < 0.0 {
    code |= OUT_BOTTOM;
  }
  if v.w - v.y < 0.0 {
    code |= OUT_TOP;
  }
  if v.z < 0.0 {
    code |= OUT_FAR;
  }
  if v.w - v.z < 0.0 {
    code |= OUT_NEAR;
  }
  code
}

pub(crate) struct FpsAveragerEight {
  samples: [f32; 8],
  index: usize,
  count: usize,
  sum: f32,
}

impl FpsAveragerEight {
  pub(crate) const fn new() -> Self {
    Self {
      samples: [0.0; 8],
      index: 0,
      count: 0,
      sum: 0.0,
    }
  }

  pub(crate) fn add_fps(&mut self, fps: f32) {
    if self.count == 8 {
      self.sum -= self.samples[self.index];
    } else {
      self.count += 1;
    }

    self.samples[self.index] = fps;
    self.sum += fps;

    // Bitwise wrapping: Automatically cycles 0->7->0
    self.index = (self.index + 1) & 7;
  }

  #[inline]
  pub(crate) fn avg_fps(&self) -> f32 {
    if self.count == 8 {
      // Highly optimized by the compiler into a fast multiplication (* 0.125)
      self.sum / 8.0
    } else if self.count > 0 {
      self.sum / self.count as f32
    } else {
      0.0
    }
  }
}

#[derive(Debug)]
pub struct Aabb {
  pub min: Vec3,
  pub max: Vec3,
}

impl Aabb {
  /// Construct from a vec of Vec3 positions
  pub fn from_points(positions: &[Vec3]) -> Self {
    let mut min = v3(f32::INFINITY, f32::INFINITY, f32::INFINITY);
    let mut max = v3(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);
    for &p in positions {
      min.x = min.x.min(p.x);
      min.y = min.y.min(p.y);
      min.z = min.z.min(p.z);
      max.x = max.x.max(p.x);
      max.y = max.y.max(p.y);
      max.z = max.z.max(p.z);
    }

    Self { min, max }
  }

  /// Build an empty AABB
  pub fn empty() -> Self {
    Self { min: V3_ZERO, max: V3_ZERO }
  }

  /// Midpoint of the box. Cheap representative point for sorting.
  #[inline]
  pub fn centroid(&self) -> Vec3 {
    (self.min + self.max) * 0.5
  }

  /// Computes the smallest AABB enclosing both boxes.
  pub fn union(&self, other: &Self) -> Self {
    Self {
      min: Vec3 {
        x: self.min.x.min(other.min.x),
        y: self.min.y.min(other.min.y),
        z: self.min.z.min(other.min.z),
      },
      max: Vec3 {
        x: self.max.x.max(other.max.x),
        y: self.max.y.max(other.max.y),
        z: self.max.z.max(other.max.z),
      },
    }
  }
}
