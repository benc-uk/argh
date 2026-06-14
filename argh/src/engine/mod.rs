// ==============================================================================================
// Module & file:   engine / mod.rs
// Purpose:         Module root for the core argh engine, splitting it across multiple files
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

//! The engine is the core construct of argh, used to carry out rendering via a [../Scene] & [../Camera],
//! Holds the internal framebuffer and other constructs for rendering

mod draw2d;
#[cfg(feature = "desktop")]
mod input;
mod parse_gltf;
mod parse_obj;
mod render;

#[cfg(test)]
#[path = "../tests/engine_tests.rs"]
mod engine_tests;

#[cfg(feature = "desktop")]
use minifb::{Window, WindowOptions};
use slotmap::{SlotMap, new_key_type};
use web_time::Instant;

#[cfg(feature = "desktop")]
use crate::{app::App, engine::input::Inputs};
use crate::{buffer::Buffer, colour::*, engine::render::ProcessedVert, helpers::FpsAveragerEight, math::Vec3, model::Model};

#[cfg(feature = "desktop")]
pub use minifb::Key;

new_key_type! {
  /// A handle to reference instances held by the engine
  pub struct InstanceHandle;
  /// A handle to reference models held by the engine
  pub struct ModelHandle;
  /// A handle to reference lights held by the engine
  pub struct LightHandle;
}

// Convenience to group all desktop only fields and state in one place
#[cfg(feature = "desktop")]
struct DesktopState {
  inputs: input::Inputs, // Inputs, keyboard & mouse
  exit: bool,            // Flag for quit/exit
}

/// This is the heart of argh, create an instance of the Engine to use the library
pub struct Engine {
  size: (usize, usize),  // Framebuffer size: width & height
  aspect: f32,           // Easy access to the aspect ratio (w/h)
  buffer: Buffer,        // The framebuffer
  t: f64,                // Elapsed time
  last_time: Instant,    // For delta time calculation
  fps: FpsAveragerEight, // Holds an FPS average
  // log_level: usize,
  #[cfg(feature = "desktop")]
  desktop: DesktopState,

  // Stats
  stat_rend_tri_frame: u32,

  // Internal rendering perf cache kinda stuff
  verts: Vec<ProcessedVert>,
  normals: Vec<Vec3>,

  // Things tracked & cached by the engine
  models: SlotMap<ModelHandle, Model>,

  // Public fields...
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
      aspect: w as f32 / h as f32,
      buffer: Buffer::new(w as usize, h as usize),
      t: 0.0,
      last_time: Instant::now(),
      fps: FpsAveragerEight::new(),
      debug: false,
      verts: vec![],
      normals: vec![],
      stat_rend_tri_frame: 0,

      #[cfg(feature = "desktop")]
      desktop: DesktopState {
        exit: false,
        inputs: Inputs::new(),
      },

      models: SlotMap::with_key(),
    }
  }

  /// Begin the main loop, open the window and blocks until the window is closed or escape is pressed
  /// Only use this on desktop, WASM or other runtimes will not ever call this but use `tick()`
  /// # Arguments
  /// * `scene` - Implementation of Scene with your own `update()` function
  /// * `title` - Title of the window
  /// * `scale` - Scale up the viewport; Values: 0,1,2,4,8
  #[cfg(feature = "desktop")]
  pub fn start_window<A: App>(&mut self, app: &mut A, title: &str, scale: u8, target_fps: usize) {
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

    if target_fps > 0 {
      window.set_target_fps(target_fps);
    }

    while window.is_open() {
      if self.desktop.exit {
        break;
      }

      let dt = Instant::now().duration_since(self.last_time).as_secs_f64();
      let t = self.tick(dt); // time bookkeeping
      app.update(self, dt, t); // call app update hook (normally renders everything)

      // Draw debug info, FPS and other stats
      if self.debug {
        self.draw_debug();
      }

      // Scrape the inputs from the minifb window
      self.desktop.inputs.scrape(&window);

      // Finally put the image/framebuffer into the window
      if let Err(e) = window.update_with_buffer(&self.buffer.pixels, self.size.0, self.size.1) {
        println!("Error updating buffer: {}", e);
      }
    }
  }

  /// Stop running and exit, only used in window/desktop mode
  #[cfg(feature = "desktop")]
  pub fn stop(&mut self) {
    self.desktop.exit = true;
  }

  /// Return the width & height of the window
  pub fn size(&self) -> (usize, usize) {
    (self.size.0, self.size.1)
  }

  /// Get the aspect ratio of the viewport and window
  pub fn aspect(&self) -> f32 {
    self.aspect
  }

  /// Advance engine bookkeeping for one frame, returns the accumulated time `t`.
  pub fn tick(&mut self, dt: f64) -> f64 {
    self.t += dt;
    let fps = if dt > 0.0 { 1.0 / dt } else { 0.0 };
    self.fps.add_fps(fps as f32);
    self.last_time = Instant::now();
    self.stat_rend_tri_frame = 0;

    self.t
  }

  /// Getter for elapsed time
  pub fn time(&self) -> f64 {
    self.t
  }

  /// Add a model to the engine cache
  pub fn add_model(&mut self, model: Model) -> ModelHandle {
    println!("Adding model '{}' to the engine cache", model.name);

    self.models.insert(model)
  }

  /// Get a [Model] from its handle
  pub fn model(&self, model_h: ModelHandle) -> &Model {
    self.models.get(model_h).unwrap()
  }

  /// Get a mutable [Model] from its handle
  pub fn model_mut(&mut self, model_h: ModelHandle) -> &mut Model {
    self.models.get_mut(model_h).unwrap()
  }

  /// Draw the debug overlay on top of the current frame. Call AFTER app.update.
  pub fn draw_debug(&mut self) {
    self.draw_string(&format!("FPS: {:.2}", self.fps.avg_fps()), 1, 1, WHITE);
    self.draw_string(&format!("TRI_REND: {:.2}", self.stat_rend_tri_frame), 1, 21, WHITE);
  }

  /// Get engine stats
  pub fn stats(&self) -> u32 {
    self.stat_rend_tri_frame
  }
}
