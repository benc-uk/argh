// ==============================================================================================
// Module & file:   helpers.rs
// Purpose:         INTERNAL - Helper functions
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use crate::{
  colour::*,
  light::Light,
  math::{Vec3, Vec4},
};

// One bit per frustum plane
pub const OUT_LEFT: u8 = 1 << 0;
pub const OUT_RIGHT: u8 = 1 << 1;
pub const OUT_BOTTOM: u8 = 1 << 2;
pub const OUT_TOP: u8 = 1 << 3;
pub const OUT_NEAR: u8 = 1 << 4;
pub const OUT_FAR: u8 = 1 << 5;

/// Don't ask me to explain this one!
#[inline(always)]
pub fn compute_outcode(v: &Vec4) -> u8 {
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
    code |= OUT_NEAR;
  }
  if v.w - v.z < 0.0 {
    code |= OUT_FAR;
  }
  code
}

// Internal function for calculating the light at a vertex in world space
// We return light values (as RGB Colours) falling on that vert, NOT the colour of the surface
pub fn shade_vert(lights: &Vec<Light>, world: Vec3, n: Vec3, eye: Vec3, hardness: f64) -> (Colour, Colour) {
  // Shading & lighting over multiple lights
  let mut diff_sum = BLACK;
  let mut spec_sum = BLACK;

  for light in lights {
    // Vectors to and from the surface and the light
    let l = (light.pos - world).normalize_new();
    let li = l.invert();

    // Diffuse lighting
    let n_dot_l = n.dot(l).max(0.0);
    let diff_col = light.colour * light.brightness * n_dot_l;
    diff_sum += diff_col;

    // Specular
    if n_dot_l > 0.0 {
      let v = (eye - world).normalize_new();
      let r = li.reflect(n);
      let v_dot_r = v.dot(r).max(0.0);
      let spec = v_dot_r.powf(hardness);
      let spec_col = light.colour * spec * light.brightness;
      spec_sum += spec_col;
    }
  }

  (diff_sum, spec_sum)
}
