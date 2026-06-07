// ==============================================================================================
// Module & file:   light.rs
// Purpose:         Point light source
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use crate::{
  colour::{Colour, WHITE},
  math::*,
};

/// Light holds position, brightness and colour, plus attenuation (drop off)
#[derive(Debug, Clone, Copy)]
pub struct Light {
  pub pos: Vec3,
  pub brightness: f32,
  pub colour: Colour,
  pub atten_linear: f32,
  pub atten_quad: f32,
}

impl Light {
  /// Create a light
  /// # Arguments:
  /// * `pos` - Position in world space of the light
  /// * `brightness` - Scales the brightness of the the light 0-1
  /// * `colour` - Light colour
  pub fn new(pos: Vec3, brightness: f32, colour: Colour) -> Self {
    Self {
      pos,
      brightness,
      colour,
      atten_linear: 0.09,
      atten_quad: 0.032,
    }
  }

  /// Create a default light at (0, 0, 0) with white colour and full brightness
  /// Attenuation linear= 0.09, quad=0.032
  pub fn new_default() -> Self {
    Self {
      pos: V3_ZERO,
      brightness: 1.0,
      colour: WHITE,
      atten_linear: 0.09,
      atten_quad: 0.032,
    }
  }
}
