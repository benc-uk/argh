use crate::buffer::Buffer;
use crate::colour::Colour;
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

  // pub fn draw_text(&mut self, str: String, x: usize, y: usize, colour: Colour) {
  //   self.buffer
  // }

  /// Begin the main loop, open the window and blocks until the window is closed or escape is pressed
  /// # Arguments
  /// * `scene` - Implementation of Scene with your own `update()` function
  pub fn start<S: Scene>(mut self, mut scene: S) {
    let mut opt = WindowOptions::default();
    opt.scale_mode = minifb::ScaleMode::Stretch;
    opt.topmost = true;
    opt.resize = true;
    opt.scale = self.scale;

    let mut window = Window::new(&self.win_title, self.win_size.0, self.win_size.1, opt).unwrap_or_else(|e| {
      panic!("{}", e);
    });

    window.set_target_fps(60);
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
    let mut i = 0;
    for ch in s.chars() {
      self.buffer.draw_char(ch, x + (i * (crate::text::glyph_size().0 + 1)), y, colour);
      i += 1;
    }
  }

  pub fn draw_rect(&mut self, x: usize, y: usize, w: usize, h: usize, colour: Colour) {
    self.buffer.fill_rect(x, y, w, h, colour);
  }
}
