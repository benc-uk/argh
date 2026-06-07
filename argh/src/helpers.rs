// ==============================================================================================
// Module & file:   helpers.rs
// Purpose:         INTERNAL - Helper functions
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use slotmap::SlotMap;

use crate::{
  colour::*,
  engine::LightHandle,
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
    code |= OUT_FAR;
  }
  if v.w - v.z < 0.0 {
    code |= OUT_NEAR;
  }
  code
}

// Internal function for calculating the light at a vertex in world space
// We return light values (as RGB Colours) falling on that vert, NOT the colour of the surface
#[inline(always)]
pub fn shade_vert(lights: &SlotMap<LightHandle, Light>, world: Vec3, n: Vec3, eye: Vec3, hardness: f32) -> (Colour, Colour) {
  // Shading & lighting over multiple lights
  let mut diff_sum = BLACK;
  let mut spec_sum = BLACK;

  for light in lights.values() {
    // Vectors to and from the surface and the light
    let l_raw = light.pos - world;
    let d = l_raw.len();
    let l = l_raw.normalize_new();
    let li = l.invert();

    // Add attenuation
    let atten = 1.0 / (1.0 + light.atten_linear * d + light.atten_quad * d * d);

    // Diffuse lighting
    let n_dot_l = n.dot(l).max(0.0);
    let diff_col = light.colour * light.brightness * n_dot_l * atten;
    diff_sum += diff_col;

    // Specular
    if n_dot_l > 0.0 {
      let v = (eye - world).normalize_new();
      let r = li.reflect(n);
      let v_dot_r = v.dot(r).max(0.0);
      let spec = v_dot_r.powf(hardness);
      let spec_col = light.colour * spec * light.brightness * atten;
      spec_sum += spec_col;
    }
  }

  (diff_sum, spec_sum)
}

pub struct FpsAveragerEight {
  samples: [f32; 8],
  index: usize,
  count: usize,
  sum: f32,
}

impl FpsAveragerEight {
  pub const fn new() -> Self {
    Self {
      samples: [0.0; 8],
      index: 0,
      count: 0,
      sum: 0.0,
    }
  }

  pub fn add_fps(&mut self, fps: f32) {
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
  pub fn avg_fps(&self) -> f32 {
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
