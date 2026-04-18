//

use crate::colour::Colour;

/// Internal struct wrapping a Vec<u32> to be used with minifb update_with_buffer(), each u32 is a single pixel
/// The encoding for each pixel is 0RGB: The upper 8-bits are ignored, the next 8-bits are for the red channel, the next 8-bits afterwards for the green channel, and the lower 8-bits for the blue channel.
pub struct Buffer {
  pub pixels: Vec<u32>,
  w: usize,
  h: usize,
}

impl Buffer {
  pub fn new(w: usize, h: usize) -> Self {
    Self { pixels: vec![0; w * h], w, h }
  }

  #[inline(always)]
  pub fn clear(&mut self, colour: Colour) {
    self.pixels.fill(colour.as_u32());
  }

  #[inline(always)]
  pub fn set_pixel(&mut self, x: usize, y: usize, c: Colour) {
    if x < self.w && y < self.h {
      self.pixels[y * self.w + x] = c.as_u32();
    }
  }

  // #[inline(always)]
  // pub unsafe fn set_pixel_unchecked(&mut self, x: usize, y: usize, c: u32) {
  //   unsafe {
  //     *self.pixels.get_unchecked_mut(y * self.w + x) = c;
  //   }
  // }

  // pub fn fill_scanline(&mut self, y: usize, x_start: usize, x_end: usize, c: u32) {
  //   if y >= self.h {
  //     return;
  //   }
  //   let start = y * self.w + x_start.min(self.w);
  //   let end = y * self.w + x_end.min(self.w);
  //   self.pixels[start..end].fill(c);
  // }

  pub fn fill_rect(&mut self, x: usize, y: usize, w: usize, h: usize, c: Colour) {
    let colour = c.as_u32();
    for row in y..((y + h).min(self.h)) {
      let start = row * self.w + x.min(self.w);
      let end = row * self.w + (x + w).min(self.w);
      self.pixels[start..end].fill(colour);
    }
  }

  pub fn draw_char(&mut self, ch: char, x: usize, y: usize, c: Colour) {
    let (w, h) = crate::text::glyph_size();
    if let Some(rows) = crate::text::glyph(ch) {
      for ri in 0..h {
        let row = rows[ri];
        for ci in 0..w {
          // Check each bit
          if row & (1 << w - ci - 1) != 0 {
            if ci + x < self.w {
              self.pixels[(ri + y) * self.w + ci + x] = c.as_u32();
            }
          }
        }
      }
    }
  }
}
