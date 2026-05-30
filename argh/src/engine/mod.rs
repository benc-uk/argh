// ==============================================================================================
// Module & file:   engine / mod.rs
// Purpose:         Module root for the core argh engine, splitting it across multiple files
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

//! The engine is the core construct of argh, used to hold everything being rendered (meshes, instances, textures),
//! the window & frame buffer, and to carry out rendering

mod app_scene;
mod draw2d;
#[cfg(feature = "desktop")]
mod input;
mod render;
mod resources;

#[cfg(feature = "desktop")]
use minifb::{Window, WindowOptions};
use slotmap::{SlotMap, new_key_type};
use web_time::Instant;

use crate::{
  buffer::Buffer,
  colour::*,
  models::{Material, Mesh},
};
pub use app_scene::App;
pub use app_scene::Scene;

#[cfg(feature = "desktop")]
pub use minifb::Key;

new_key_type! {
  /// A handle to reference instances held by the engine
  pub struct InstanceHandle;
  /// A handle to reference materials held by the engine
  pub struct MaterialHandle;
  /// A handle to reference meshes held by the engine
  pub struct MeshHandle;
  /// A handle to reference lights held by the engine
  pub struct LightHandle;
}

#[derive(thiserror::Error)]
pub enum EngineError {
  #[error("no mesh registered with the name: '{0}'")]
  MeshNotFound(String),
}

impl std::fmt::Debug for EngineError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self, f)
  }
}

/// This is the heart of argh, create an instance of the Engine to use the library
pub struct Engine {
  size: (usize, usize),
  aspect: f64,
  buffer: Buffer,
  t: f64,
  last_time: Instant,
  fps: f64,
  exit: bool,

  // Things tracked & cached by the engine
  meshes: SlotMap<MeshHandle, Mesh>,
  materials: SlotMap<MaterialHandle, Material>,

  // Inputs - Gated to desktop only not web/wasm
  #[cfg(feature = "desktop")]
  keys: Vec<Key>,
  #[cfg(feature = "desktop")]
  keys_pressed: Vec<Key>,

  // Public fields...
  /// Rate to try to update the buffer, used at engine start only
  pub target_fps: usize,

  /// Output debug info like FPS onto the top right of the screen
  pub debug: bool,
}

impl Engine {
  /// Constructor for a new Engine
  /// # Arguments
  /// * `w` - Width of the window in pixels
  /// * `h` - Height of the window in pixels
  pub fn new(w: i32, h: i32) -> Self {
    Self {
      size: (w as usize, h as usize),
      buffer: Buffer::new(w as usize, h as usize),
      t: 0.0,
      last_time: Instant::now(),
      fps: 0.0,
      target_fps: 60,
      aspect: w as f64 / h as f64,

      exit: false,
      debug: false,

      meshes: SlotMap::with_key(),
      materials: SlotMap::with_key(),

      #[cfg(feature = "desktop")]
      keys: vec![],
      #[cfg(feature = "desktop")]
      keys_pressed: vec![],
    }
  }

  /// Begin the main loop, open the window and blocks until the window is closed or escape is pressed
  /// Only use this on desktop, WASM or other runtimes will not ever call this but use `tick()`
  /// # Arguments
  /// * `scene` - Implementation of Scene with your own `update()` function
  /// * `title` - Title of the window
  /// * `scale` - Scale up the viewport; Values: 0,1,2,4,8
  #[cfg(feature = "desktop")]
  pub fn start_window<A: App>(&mut self, app: &mut A, title: &str, scale: u8) {
    let scl = match scale {
      0 => minifb::Scale::FitScreen,
      1 => minifb::Scale::X1,
      2 => minifb::Scale::X2,
      4 => minifb::Scale::X4,
      8 => minifb::Scale::X8,
      _ => minifb::Scale::X1,
    };

    let opt = WindowOptions {
      scale_mode: minifb::ScaleMode::Stretch,
      resize: false,
      scale: scl,
      ..Default::default()
    };

    let mut window = Window::new(title, self.size.0, self.size.1, opt).expect("failed to create window");

    if self.target_fps > 0 {
      window.set_target_fps(self.target_fps);
    }

    while window.is_open() {
      if self.exit {
        break;
      }

      let dt = Instant::now().duration_since(self.last_time).as_secs_f64();
      let t = self.tick(dt); // time bookkeeping
      app.update(self, dt, t); // app paints the world

      if self.debug {
        self.draw_debug();
      }

      self.keys = window.get_keys();
      self.keys_pressed = window.get_keys_pressed(minifb::KeyRepeat::No);

      if let Err(e) = window.update_with_buffer(&self.buffer.pixels, self.size.0, self.size.1) {
        println!("Error updating buffer: {}", e);
      }
    }
  }

  /// Stop running and exit the running process
  pub fn stop(&mut self) {
    self.exit = true;
  }

  /// Return the width & height of the window
  pub fn get_size(&self) -> (usize, usize) {
    (self.size.0, self.size.1)
  }

  /// Get the aspect ratio of the viewport and window
  pub fn get_aspect(&self) -> f64 {
    self.aspect
  }

  /// Advance engine bookkeeping for one frame, returns the accumulated time `t`.
  pub fn tick(&mut self, dt: f64) -> f64 {
    self.t += dt;
    self.fps = if dt > 0.0 { 1.0 / dt } else { 0.0 };
    self.last_time = Instant::now();

    self.t
  }

  /// Draw the debug overlay on top of the current frame. Call AFTER app.update.
  pub fn draw_debug(&mut self) {
    self.draw_string(&format!("FPS: {:.2}", self.fps), 2, 2, BLACK);
    self.draw_string(&format!("FPS: {:.2}", self.fps), 1, 1, WHITE);
  }
}
