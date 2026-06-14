// ==============================================================================================
// Module & file:   engine / draw2d.rs
// Purpose:         Some basic 2D drawing operations
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use crate::{colour::Colour, math::Vec2};

use super::Engine;

#[cfg(test)]
#[path = "../tests/draw2d_tests.rs"]
mod draw2d_tests;

impl Engine {
  /// Clear the entire window and buffer with the given colour
  /// Also clears the depth buffer
  pub fn clear(&mut self, colour: Colour) {
    self.buffer.clear(colour);
  }

  /// Draw text onto the screen
  /// # Arguments
  /// * `s` - String to draw
  /// * `x` - X position of start of text
  /// * `y` - Y position of text
  /// * `colour` - Colour to draw the text
  pub fn draw_string(&mut self, s: &str, x: usize, y: usize, colour: Colour) {
    for (i, ch) in s.chars().enumerate() {
      self.buffer.draw_char(ch, x + (i * (crate::text::glyph_size().0 + 1)), y, colour);
    }
  }

  /// Draw a filled 2D rectangle
  /// # Arguments
  /// * `x` - X coord of top left corner of rectangle
  /// * `y` - Y coord of top left corner of rectangle
  /// * `w` - Width of rectangle in pixels
  /// * `h` - Height of rectangle in pixels
  /// * `colour` - Colour to fill the rectangle
  pub fn draw_rect(&mut self, x: usize, y: usize, w: usize, h: usize, colour: Colour) {
    self.buffer.fill_rect(x, y, w, h, colour);
  }

  /// Draw a 2D line between two points
  pub fn draw_line(&mut self, mut x0: i32, mut y0: i32, x1: i32, y1: i32, colour: Colour) {
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut error = dx + dy;

    loop {
      self.buffer.set_pixel(x0 as usize, y0 as usize, colour);
      let e2 = 2 * error;
      if e2 >= dy {
        if x0 == x1 {
          break;
        }
        error += dy;
        x0 += sx;
      }
      if e2 <= dx {
        if y0 == y1 {
          break;
        }
        error += dx;
        y0 += sy;
      }
    }
  }

  /// Draw a series of 2D lines between a list of Vec2 points, designed for drawing a polygon but this method does not ensure the shape is closed
  pub fn draw_poly_line(&mut self, points: &[Vec2], colour: Colour) {
    for pair in points.windows(2) {
      let (a, b) = (pair[0], pair[1]);
      self.draw_line(a.x as i32, a.y as i32, b.x as i32, b.y as i32, colour);
    }
  }
}
