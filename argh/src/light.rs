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

/// Light holds position, brightness and colour
#[derive(Debug, Clone, Copy)]
pub struct Light {
  pub pos: Vec3,
  pub brightness: f64,
  pub colour: Colour,
}

impl Light {
  pub fn new(pos: Vec3, brightness: f64, colour: Colour) -> Self {
    Self { pos, brightness, colour }
  }

  pub fn new_default() -> Self {
    Self {
      pos: VEC3_ZERO,
      brightness: 1.0,
      colour: WHITE,
    }
  }
}
