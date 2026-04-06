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

  pub fn clear(&mut self, colour: Colour) {
    self.pixels.fill(colour.as_u32());
  }

  pub fn set_pixel(&mut self, x: usize, y: usize, c: Colour) {
    if x < self.w && y < self.h {
      self.pixels[y * self.w + x] = c.as_u32();
    }
  }
}
