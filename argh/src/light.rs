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
  /// Position of the light in world space
  pub pos: Vec3,
  /// Scales the brightness of the light, typically 0.0 - 1.0 but can go higher
  pub brightness: f32,
  /// Colour of the light, usually [WHITE]
  pub colour: Colour,
  /// Linear attenuation coefficient, controls how light falls off with distance
  pub atten_linear: f32,
  /// Quadratic attenuation coefficient, controls how light falls off with the square of distance
  pub atten_quad: f32,
  /// If true, this light contributes to baked lighting on static geometry
  pub is_static: bool,
  /// If true, this light is applied per-frame even to static geometry
  pub is_dynamic: bool,
}

impl Light {
  /// Create a light
  /// # Arguments:
  /// * `pos` - Position in world space of the light
  /// * `brightness` - Scales the brightness of the the light 0-1
  /// * `colour` - Light colour
  pub fn new(pos: Vec3, brightness: f32, colour: Colour, atten_linear: f32, atten_quad: f32, is_static: bool, is_dynamic: bool) -> Self {
    Self {
      pos,
      brightness,
      colour,
      atten_linear,
      atten_quad,
      is_static,
      is_dynamic,
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
      is_static: false,
      is_dynamic: false,
    }
  }
}
