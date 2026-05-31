// ==============================================================================================
// Module & file:   colour.rs
// Purpose:         Standard RGB Colour type
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use std::ops::*;

/// A RGB colour tuple. Linear RGB colour, components in [0.0, 1.0] for normal use but range is not enforced
#[derive(Debug, Clone, Copy)]
pub struct Colour {
  r: f32,
  g: f32,
  b: f32,
}

const INV_255: f32 = 1.0 / 255.0;

// Helper static colours

pub const BLACK: Colour = Colour::new(0.0, 0.0, 0.0);
pub const WHITE: Colour = Colour::new(1.0, 1.0, 1.0);
pub const RED: Colour = Colour::new(1.0, 0.0, 0.0);
pub const GREEN: Colour = Colour::new(0.0, 1.0, 0.0);
pub const BLUE: Colour = Colour::new(0.0, 0.0, 1.0);
pub const MAGENTA: Colour = Colour::new(1.0, 0.0, 1.0);
pub const CYAN: Colour = Colour::new(0.0, 1.0, 1.0);
pub const YELLOW: Colour = Colour::new(1.0, 1.0, 0.0);

impl Colour {
  /// Create a new Colour from given RGB values
  pub const fn new(r: f32, g: f32, b: f32) -> Self {
    Self { r, g, b }
  }

  /// Create a new Colour from given u8 RGB values (0 - 255)
  pub const fn from_rgb8(r: u8, g: u8, b: u8) -> Self {
    Self {
      r: r as f32 / 255.0,
      g: g as f32 / 255.0,
      b: b as f32 / 255.0,
    }
  }

  /// Create a new Colour from a three tuple
  pub const fn from_slice(v: [f32; 3]) -> Self {
    Self { r: v[0], g: v[1], b: v[2] }
  }

  /// Return as packed 0rgb u32 representation of the colour for use with the internal `Buffer`. Alpha is always 0 as minifb ignores it
  #[inline(always)]
  pub fn to_packed_0rgb(self) -> u32 {
    let r = (self.r.clamp(0.0, 1.0) * 255.0 + 0.5) as u32;
    let g = (self.g.clamp(0.0, 1.0) * 255.0 + 0.5) as u32;
    let b = (self.b.clamp(0.0, 1.0) * 255.0 + 0.5) as u32;
    (r << 16) | (g << 8) | b
  }

  /// Take a packed 0rgb u32 and output as a Colour
  #[inline(always)]
  pub const fn from_packed_0rgb(p: u32) -> Self {
    Self {
      r: ((p >> 16) & 0xFF) as f32 * INV_255,
      g: ((p >> 8) & 0xFF) as f32 * INV_255,
      b: (p & 0xFF) as f32 * INV_255,
    }
  }

  /// Create a random RGB colour
  pub fn rand() -> Self {
    let r = rand::random_range(0.0..1.0);
    let g = rand::random_range(0.0..1.0);
    let b = rand::random_range(0.0..1.0);
    Self::new(r, g, b)
  }
}

impl Mul<f32> for Colour {
  type Output = Self;
  fn mul(self, s: f32) -> Self {
    Self::new(self.r * s, self.g * s, self.b * s)
  }
}

impl Mul<f64> for Colour {
  type Output = Self;
  fn mul(self, s: f64) -> Self {
    let sf32 = s as f32;
    Self::new(self.r * sf32, self.g * sf32, self.b * sf32)
  }
}

impl Mul<Self> for Colour {
  type Output = Self;
  fn mul(self, c: Self) -> Self {
    Self::new(self.r * c.r, self.g * c.g, self.b * c.b)
  }
}

impl Add for Colour {
  type Output = Self;
  fn add(self, c: Self) -> Self {
    Self::new(self.r + c.r, self.g + c.g, self.b + c.b)
  }
}

impl MulAssign<f32> for Colour {
  fn mul_assign(&mut self, s: f32) {
    *self = *self * s;
  }
}

impl MulAssign<f64> for Colour {
  fn mul_assign(&mut self, s: f64) {
    *self = *self * s;
  }
}

impl MulAssign<Self> for Colour {
  fn mul_assign(&mut self, c: Self) {
    *self = *self * c;
  }
}

impl AddAssign<Self> for Colour {
  fn add_assign(&mut self, c: Self) {
    *self = *self + c;
  }
}
