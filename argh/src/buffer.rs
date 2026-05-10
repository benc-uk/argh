// ==============================================================================================
// Module & file:   buffer.rs
// Purpose:         Internal pixel buffer wrapping Vec<u32> for use with minifb
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use crate::{colour::Colour, engine::ScreenVertex, helpers};

/// Internal struct wrapping a Vec<u32> to be used with minifb update_with_buffer(), each u32 is a single pixel
/// The encoding for each pixel is 0RGB: The upper 8-bits are ignored, the next 8-bits are for the red channel, the next 8-bits afterwards for the green channel, and the lower 8-bits for the blue channel.
/// This includes a f32 depth buffer for z-buffering
pub struct Buffer {
  pub pixels: Vec<u32>,
  pub depth: Vec<f32>,
  w: usize,
  h: usize,
}

impl Buffer {
  pub fn new(w: usize, h: usize) -> Self {
    Self {
      pixels: vec![0; w * h],
      w,
      h,
      depth: vec![f32::INFINITY; w * h],
    }
  }

  #[inline(always)]
  pub fn clear(&mut self, colour: Colour) {
    self.pixels.fill(colour.to_packed_0rgb());
    self.clear_depth();
  }

  #[inline(always)]
  pub fn clear_depth(&mut self) {
    self.depth.fill(f32::INFINITY);
  }

  #[inline(always)]
  pub fn set_pixel(&mut self, x: usize, y: usize, c: Colour) {
    if x < self.w && y < self.h {
      self.pixels[y * self.w + x] = c.to_packed_0rgb();
    }
  }

  #[inline(always)]
  pub fn set_pixel_depth(&mut self, x: usize, y: usize, c: Colour, z: f32) {
    let idx = y * self.w + x;
    if x < self.w && y < self.h && z < self.depth[idx] {
      self.pixels[idx] = c.to_packed_0rgb();
      self.depth[idx] = z;
    }
  }

  #[inline(always)]
  pub fn fill_rect(&mut self, x: usize, y: usize, w: usize, h: usize, c: Colour) {
    for row in y..((y + h).min(self.h)) {
      let start = row * self.w + x.min(self.w);
      let end = row * self.w + (x + w).min(self.w);
      self.pixels[start..end].fill(c.to_packed_0rgb());
    }
  }

  /// Fill a 3D triangle between three ScreenVertex points which form a triangle
  /// Not public!
  #[inline(always)]
  pub fn fill_triangle(&mut self, v0: ScreenVertex, v1: ScreenVertex, v2: ScreenVertex, colour: Colour) {
    let area = helpers::edge_function(v1, v2, v0.x, v0.y);
    if area == 0.0 {
      return;
    } // degenerate triangle, save ourselves a NaN
    let inv_area = 1.0 / area;

    let min_x = v1.x.min(v0.x).min(v2.x).max(0.0);
    let min_y = v1.y.min(v0.y).min(v2.y).max(0.0);
    let max_x = v1.x.max(v0.x).max(v2.x).min(self.w as f64 - 1.0);
    let max_y = v1.y.max(v0.y).max(v2.y).min(self.h as f64 - 1.0);
    let min_xi = min_x.floor() as i32;
    let max_xi = max_x.ceil() as i32;
    let min_yi = min_y.floor() as i32;
    let max_yi = max_y.ceil() as i32;

    // Sample at the centre of the first pixel in the loop range
    let start_x = min_xi as f64 + 0.5;
    let start_y = min_yi as f64 + 0.5;

    // CONVENTION: triangles arrive here AFTER back-face cull, in screen space
    // (Y-down). Our cull keeps triangles with NEGATIVE screen-space signed area
    // (CW on screen, from CCW-in-world meshes after the viewport Y-flip).
    //
    // We use the textbook CCW edge setup. Because our triangles are CW on screen,
    // inside points produce NEGATIVE edge values, so the inside test is `w <= 0`.
    // If the cull/Y convention ever changes, flip all three test signs back to >=.

    // Edge values at the centre of the first pixel
    let mut w0_row = helpers::edge_function(v1, v2, start_x, start_y);
    let mut w1_row = helpers::edge_function(v2, v0, start_x, start_y);
    let mut w2_row = helpers::edge_function(v0, v1, start_x, start_y);

    // Step amounts: how much each edge value changes per pixel
    let dx0 = v1.y - v2.y;
    let dx1 = v2.y - v0.y;
    let dx2 = v0.y - v1.y;

    let dy0 = v2.x - v1.x;
    let dy1 = v0.x - v2.x;
    let dy2 = v1.x - v0.x;

    for y in min_yi..=max_yi {
      let mut w0 = w0_row;
      let mut w1 = w1_row;
      let mut w2 = w2_row;

      for x in min_xi..=max_xi {
        if w0 <= 0.0 && w1 <= 0.0 && w2 <= 0.0 {
          // Barycentric weights (positive, sum to 1)
          let b0 = w0 * inv_area;
          let b1 = w1 * inv_area;
          let b2 = w2 * inv_area;

          // Linear depth interpolation (correct in screen space, no /w needed)
          let z = b0 * v0.z + b1 * v1.z + b2 * v2.z;
          self.set_pixel_depth(x as usize, y as usize, colour, z as f32);
        }
        w0 += dx0;
        w1 += dx1;
        w2 += dx2;
      }

      w0_row += dy0;
      w1_row += dy1;
      w2_row += dy2;
    }
  }

  pub fn draw_char(&mut self, ch: char, x: usize, y: usize, c: Colour) {
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
