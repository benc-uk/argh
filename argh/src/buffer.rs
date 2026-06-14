// ==============================================================================================
// Module & file:   buffer.rs
// Purpose:         Internal pixel & depth buffer
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use crate::colour::Colour;

#[cfg(test)]
#[path = "tests/buffer_tests.rs"]
mod buffer_tests;

// Internal frame & depth buffer
// The encoding for each pixel is 32bits 0RGB: The upper 8-bits are ignored, the next 24 bits are 8-bits of RGB each
// Depth buffer is f32
pub(crate) struct Buffer {
  pub(crate) pixels: Vec<u32>,
  pub(crate) depth: Vec<f32>,
  pub(crate) w: usize,
  pub(crate) h: usize,
}

impl Buffer {
  pub(crate) fn new(w: usize, h: usize) -> Self {
    Self {
      pixels: vec![0; w * h],
      w,
      h,
      depth: vec![0.0; w * h],
    }
  }

  #[inline(always)]
  pub(crate) fn clear(&mut self, colour: Colour) {
    self.pixels.fill(colour.to_packed_0rgb());
    self.clear_depth();
  }

  #[inline(always)]
  pub(crate) fn clear_depth(&mut self) {
    // As we reverse our depth buffer a cleared "all far" buffer is filled with zeros
    self.depth.fill(0.0);
  }

  #[inline(always)]
  pub(crate) fn set_pixel(&mut self, x: usize, y: usize, c: Colour) {
    if x < self.w && y < self.h {
      self.pixels[y * self.w + x] = c.to_packed_0rgb();
    }
  }

  #[inline(always)]
  pub(crate) fn set_pixel_depth(&mut self, x: usize, y: usize, c: Colour, z: f32) {
    let idx = y * self.w + x;
    // No bounds check as we don't actually need them
    if z > self.depth[idx] {
      self.pixels[idx] = c.to_packed_0rgb();
      self.depth[idx] = z;
    }
  }

  #[inline(always)]
  pub(crate) fn fill_rect(&mut self, x: usize, y: usize, w: usize, h: usize, c: Colour) {
    for row in y..((y + h).min(self.h)) {
      let start = row * self.w + x.min(self.w);
      let end = row * self.w + (x + w).min(self.w);
      self.pixels[start..end].fill(c.to_packed_0rgb());
    }
  }

  /// Internal method for rendering characters to the buffer
  pub(crate) fn draw_char(&mut self, ch: char, x: usize, y: usize, c: Colour) {
    let (w, h) = crate::text::glyph_size();
    if let Some(rows) = crate::text::glyph(ch) {
      for ri in 0..h {
        let row = rows[ri];
        for ci in 0..w {
          // Check each bit

          if row & (1 << (w - ci - 1)) != 0 {
            if ci + x < self.w {
              self.pixels[(ri + y) * self.w + ci + x] = c.to_packed_0rgb();
            }
          }
        }
      }
    }
  }
}
