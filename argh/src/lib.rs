//! Argh (Another Rust Graphics Helper) is a software rendering library using minifb. It supports 2D and 3D graphics functions and has been developed without AI assistance
//!
//! # Examples
//! Minimal usage and getting started
//! ```no_run
//! use argh::prelude::*;
//!
//! struct MyApp {}
//!
//! impl App for MyApp {
//!   fn update(&mut self, e: &mut Engine, _dt: f64, _t: f64) {
//!     e.clear(BLUE);
//!   }
//! }
//!
//! fn main() {
//!   let mut eng = Engine::new(800, 600);
//!   let mut app = MyApp {};
//!   eng.start_window(&mut app, "Hello World", 1);
//! }
//! ```

// === Crate level modules, remain internal

/// Buffer holds the pixel video buffer to be drawn each frame
pub(crate) mod buffer;

/// Some helpers for various graphics routines
pub(crate) mod helpers;

/// Very basic text rendering
pub(crate) mod text;

// === Public modules
pub mod math; // This has it's own docs in math/mod.rs

/// Module for working with 8-bit RGB colour
pub mod colour;

pub mod engine; // This has it's own docs in engine/mod.rs

/// Models, meshes, textures, materials and 3D objects
pub mod models;

/// Static methods for creating meshes of simple primatives
pub mod primitives;

/// Positionable Camera with look-at and FOV
pub mod camera;

/// Light source with position & colour
pub mod light;

/// App trait
pub mod app;

/// Scenes hold instances of objects to be rendered
pub mod scene;

// Prelude
pub mod prelude {
  pub use crate::app::App;
  pub use crate::camera::Camera;
  pub use crate::colour::*;
  pub use crate::engine::{Engine, InstanceHandle, LightHandle, ModelHandle};
  pub use crate::light::Light;
  pub use crate::math::{Affine2, Mat4, Quat, V3_ZERO, Vec2, Vec3, Vec4};
  pub use crate::models::{MATERIAL_PLACEHOLDER, Material, Texture};
  pub use crate::primitives;
  pub use crate::scene::Scene;

  #[inline]
  pub fn v3(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3::new(x, y, z)
  }

  #[inline]
  pub fn v2(x: f64, y: f64) -> Vec2 {
    Vec2::new(x, y)
  }

  #[inline]
  pub fn col(r: f32, g: f32, b: f32) -> Colour {
    Colour::new(r, g, b)
  }

  #[inline]
  pub fn col8(r: u8, g: u8, b: u8) -> Colour {
    Colour::from_rgb8(r, g, b)
  }
}
