// ==============================================================================================
// Module & file:   colour.rs
// Purpose:         Colour type wrapping u32 with named constants and conversion helpers
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

/// A RGB colour tuple, backed with u32 for use with [Engine](crate::engine::Engine)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Colour(u32);

impl Colour {
  /// Create a new Colour from given RGB values (0 - 255)
  pub const fn new(r: u8, g: u8, b: u8) -> Self {
    Self((r as u32) << 16 | (g as u32) << 8 | b as u32)
  }

  /// Get the R component
  pub fn r(self) -> u8 {
    (self.0 >> 16) as u8
  }

  /// Get the G component
  pub fn g(self) -> u8 {
    (self.0 >> 8) as u8
  }

  /// Get the B component
  pub fn b(self) -> u8 {
    self.0 as u8
  }

  /// Return the internal u32 representation of the colour
  pub fn as_u32(self) -> u32 {
    self.0
  }

  /// Scale colour by some amount (darker or brighter)
  pub fn scale(&mut self, amount: f64) {
    let r = ((self.r() as f64) * amount).clamp(0.0, 255.0) as u8;
    let g = ((self.g() as f64) * amount).clamp(0.0, 255.0) as u8;
    let b = ((self.b() as f64) * amount).clamp(0.0, 255.0) as u8;
    *self = Self::new(r, g, b);
  }

  /// Create a random RGB colour
  pub fn rand() -> Colour {
    let r = rand::random_range(0..255);
    let g = rand::random_range(0..255);
    let b = rand::random_range(0..255);
    Self::new(r, g, b)
  }
}

// Helper static colours

pub const BLACK: Colour = Colour::new(0, 0, 0);
pub const WHITE: Colour = Colour::new(255, 255, 255);
pub const RED: Colour = Colour::new(255, 0, 0);
pub const GREEN: Colour = Colour::new(0, 255, 0);
pub const BLUE: Colour = Colour::new(0, 0, 255);
