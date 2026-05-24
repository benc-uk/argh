// ==============================================================================================
// Module & file:   engine / mod.rs
// Purpose:         Module root for the core argh engine, splitting it across multiple files
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

//! The engine is the core construct of argh, used to hold everything being rendered (meshes, instances, textures),
//! the window & frame buffer, and to carry out rendering

mod draw2d;
mod input;
mod render;
mod resources;

use minifb::{Window, WindowOptions};
use slotmap::{SlotMap, new_key_type};
use std::time::Instant;

use crate::{
  buffer::Buffer,
  colour::*,
  light::Light,
  models::{Instance, Material, Mesh},
};

// Re-export some of the minifb enums for inputs
pub use minifb::{Key, MouseButton};

new_key_type! {
  /// A handle to reference instances held by the engine
  pub struct InstanceHandle;
  /// A handle to reference materials held by the engine
  pub struct MaterialHandle;
  /// A handle to reference meshes held by the engine
  pub struct MeshHandle;
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
  win_size: (usize, usize),
  win_title: String,
  aspect: f64,
  buffer: Buffer,
  t: f64,
  scale: minifb::Scale,
  fps: f64,
  lights: Vec<Light>,
  exit: bool,

  // Things tracked & cached by the engine
  meshes: SlotMap<MeshHandle, Mesh>,
  materials: SlotMap<MaterialHandle, Material>,
  instances: SlotMap<InstanceHandle, Instance>,

  // Inputs
  keys: Vec<Key>,
  keys_pressed: Vec<Key>,

  // Public fields...
  /// Rate to try to update the buffer, used at engine start only
  pub target_fps: usize,

  /// Output debug info like FPS onto the top right of the screen
  pub debug: bool,

  /// Ambient light colour, defaults to [0.1, 0.1, 0.1], beware setting this too high it will look washed out
  pub ambient_light: Colour,
}

/// All users of argh are expected to provide their own Scene implementation
pub trait Scene {
  /// This method will be called every frame by the main loop, use it to draw and render your scene
  fn update(&mut self, engine: &mut Engine, dt: f64, t: f64);
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
      buffer: Buffer::new(w as usize, h as usize),
      win_title: title,
      t: 0.0,
      scale: scl,
      fps: 0.0,
      debug: false,
      target_fps: 60,
      aspect: w as f64 / h as f64,
      exit: false,

      meshes: SlotMap::with_key(),
      materials: SlotMap::with_key(),
      instances: SlotMap::with_key(),

      lights: vec![],
      ambient_light: Colour::new(0.1, 0.1, 0.1),

      keys: vec![],
      keys_pressed: vec![],
    }
  }

  /// Begin the main loop, open the window and blocks until the window is closed or escape is pressed
  /// # Arguments
  /// * `scene` - Implementation of Scene with your own `update()` function
  pub fn start<S: Scene>(mut self, mut scene: S) {
    let opt = WindowOptions {
      scale_mode: minifb::ScaleMode::Stretch,
      resize: false,
      scale: self.scale,
      ..Default::default()
    };

    let mut window = Window::new(&self.win_title, self.win_size.0, self.win_size.1, opt).expect("failed to create window");

    if self.target_fps > 0 {
      window.set_target_fps(self.target_fps);
    }

    // Lights check
    if self.lights.is_empty() {
      println!("You have added no lights, there will be no shading just flat colours!")
    }

    let mut last_time = Instant::now();

    while window.is_open() {
      if self.exit {
        break;
      }

      let now = Instant::now();
      let dt = now.duration_since(last_time).as_secs_f64();
      self.t += dt;
      self.fps = 1.0 / dt;
      last_time = now;

      self.keys = window.get_keys();
      self.keys_pressed = window.get_keys_pressed(minifb::KeyRepeat::No);

      // This is the hook, the user does their rendering here.
      let t = self.t;
      scene.update(&mut self, dt, t);

      if self.debug {
        self.draw_string(&format!("FPS: {:.2}", self.fps), 2, 2, BLACK);
        self.draw_string(&format!("FPS: {:.2}", self.fps), 1, 1, WHITE);
      }

      if let Err(e) = window.update_with_buffer(&self.buffer.pixels, self.win_size.0, self.win_size.1) {
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
    (self.win_size.0, self.win_size.1)
  }

  /// Get the aspect ratio of the viewport and window
  pub fn get_aspect(&self) -> f64 {
    self.aspect
  }
}
