// ==============================================================================================
// Module & file:   engine.rs
// Purpose:         Core rendering engine, window management and drawing operations
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use crate::colour::Colour;
use crate::math::Vec2;
use crate::{buffer::Buffer, helpers};
use minifb::{Key, Window, WindowOptions};
use std::time::Instant;

/// All users of argh are expected to provide their own Scene implementation
pub trait Scene {
  /// This method will be called every frame by the main loop, use it to draw and render your scene
  fn update(&mut self, engine: &mut Engine, dt: f64);
}

/// This is the heart of argh, create an instance of the Engine to use the library
pub struct Engine {
  win_size: (usize, usize),
  win_title: String,
  buffer: Buffer,
  t: f64,
  scale: minifb::Scale,
  fps: f64,
  pub target_fps: usize,
  pub debug: bool,
}

impl Engine {
  /// Constructor for a new Engine
  /// # Arguments
  /// * `w` - Width of the window in pixels
  /// * `h` - Height of the window in pixels
  /// * `title` - Title of the window
  pub fn new(w: i32, h: i32, title: String, scale: i32) -> Self {
    let scl = match scale {
      n if n <= 0 => minifb::Scale::FitScreen,
      1 => minifb::Scale::X1,
      2 => minifb::Scale::X2,
      4 => minifb::Scale::X4,
      8 => minifb::Scale::X8,
      _ => minifb::Scale::X1,
    };

    Self {
      win_size: (w as usize, h as usize),
      win_title: title,
      buffer: Buffer::new(w as usize, h as usize),
      t: 0.0,
      scale: scl,
      fps: 0.0,
      debug: false,
      target_fps: 60,
    }
  }

  /// Clear the entire window and buffer with the given colour
  pub fn clear(&mut self, colour: Colour) {
    self.buffer.clear(colour);
  }

  /// Return the width & height of the window
  pub fn get_size(&self) -> (usize, usize) {
    (self.win_size.0, self.win_size.1)
  }

  /// Get the current time in seconds
  pub fn t(&self) -> f64 {
    self.t
  }

  /// Set the colour of a
  /// # Arguments
  /// * `x` - X position of pixel
  /// * `y` - Y position of pixel
  /// * `colour` - New colour of the pixel
  pub fn set_pixel(&mut self, x: usize, y: usize, colour: Colour) {
    self.buffer.set_pixel(x, y, colour);
  }

  /// Begin the main loop, open the window and blocks until the window is closed or escape is pressed
  /// # Arguments
  /// * `scene` - Implementation of Scene with your own `update()` function
  pub fn start<S: Scene>(mut self, mut scene: S) {
    let opt = WindowOptions {
      scale_mode: minifb::ScaleMode::Stretch,
      topmost: true,
      resize: true,
      scale: self.scale,
      ..Default::default()
    };

    let mut window = Window::new(&self.win_title, self.win_size.0, self.win_size.1, opt).unwrap_or_else(|e| {
      panic!("{}", e);
    });

    if self.target_fps > 0 {
      window.set_target_fps(self.target_fps);
    }
    let mut last_time = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
      let now = Instant::now();
      let dt = now.duration_since(last_time).as_secs_f64();
      self.t += dt;
      self.fps = 1.0 / dt;
      last_time = now;

      scene.update(&mut self, dt);

      if self.debug {
        self.draw_string(&format!("FPS: {:.2}", self.fps), 2, 2, crate::colour::BLACK);
        self.draw_string(&format!("FPS: {:.2}", self.fps), 1, 1, crate::colour::WHITE);
      }

      let res = window.update_with_buffer(&self.buffer.pixels, self.win_size.0, self.win_size.1);
      if res.is_err() {
        println!("Error updating buffer: {}", res.err().unwrap());
      }
    }
  }

  /// Draw text onto the screen
  /// # Arguments
  /// * `s` - String to draw
  /// * `x` - X position of pixel
  /// * `y` - Y position of pixel
  /// * `colour` - New colour of the pixel
  pub fn draw_string(&mut self, s: &str, x: usize, y: usize, colour: Colour) {
    for (i, ch) in s.chars().enumerate() {
      self.buffer.draw_char(ch, x + (i * (crate::text::glyph_size().0 + 1)), y, colour);
    }
  }

  pub fn draw_rect(&mut self, x: usize, y: usize, w: usize, h: usize, colour: Colour) {
    self.buffer.fill_rect(x, y, w, h, colour);
  }

  pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, colour: Colour) {
    let mut x0 = x0;
    let mut y0 = y0;

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

  pub fn draw_poly_line(&mut self, points: Vec<Vec2>, colour: Colour) {
    for p in 0..points.len() {
      if p + 1 >= points.len() {
        break;
      }

      self.draw_line(points[p].x as i32, points[p].y as i32, points[p + 1].x as i32, points[p + 1].y as i32, colour);
    }
  }

  pub fn fill_triangle(&mut self, v0: Vec2, v1: Vec2, v2: Vec2, colour: Colour) {
    let min_x = (v0.x.min(v1.x).min(v2.x).max(0.0)) as i32;
    let min_y = (v0.y.min(v1.y).min(v2.y).max(0.0)) as i32;
    let max_x = (v0.x.max(v1.x).max(v2.x).min(self.buffer.w() as f64 - 1.0)) as i32;
    let max_y = (v0.y.max(v1.y).max(v2.y).min(self.buffer.h() as f64 - 1.0)) as i32;

    let p0 = Vec2 { x: min_x as f64, y: min_y as f64 };

    // Edge values at top-left of bounding box
    let mut w0_row = helpers::edge_function(&v1, &v2, &p0);
    let mut w1_row = helpers::edge_function(&v2, &v0, &p0);
    let mut w2_row = helpers::edge_function(&v0, &v1, &p0);

    // Step amounts: how much each edge value changes per pixel
    let dx0 = (v1.y - v2.y) as i32;
    let dx1 = (v2.y - v0.y) as i32;
    let dx2 = (v0.y - v1.y) as i32;

    let dy0 = (v2.x - v1.x) as i32;
    let dy1 = (v0.x - v2.x) as i32;
    let dy2 = (v1.x - v0.x) as i32;

    for y in min_y..=max_y {
      let mut w0 = w0_row;
      let mut w1 = w1_row;
      let mut w2 = w2_row;

      for x in min_x..=max_x {
        if (w0 | w1 | w2) >= 0 {
          self.buffer.set_pixel(x as usize, y as usize, colour);
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
}
