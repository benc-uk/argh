// ==============================================================================================
// Module & file:   helpers.rs
// Purpose:         INTERNAL - Helper functions
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use crate::{engine::*, math::*};

// One bit per frustum plane
pub(crate) const OUT_LEFT: u8 = 1 << 0;
pub(crate) const OUT_RIGHT: u8 = 1 << 1;
pub(crate) const OUT_BOTTOM: u8 = 1 << 2;
pub(crate) const OUT_TOP: u8 = 1 << 3;
pub(crate) const OUT_NEAR: u8 = 1 << 4;
pub(crate) const OUT_FAR: u8 = 1 << 5;

#[inline(always)]
pub fn edge_function(a: ScreenVert, b: ScreenVert, px: f64, py: f64) -> f64 {
  (b.x - a.x) * (py - a.y) - (b.y - a.y) * (px - a.x)
}

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
