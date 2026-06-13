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

#![warn(missing_docs)]

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

/// Image-backed textures used by materials
pub mod texture;

/// Surface materials, flat or textured
pub mod material;

/// Triangle meshes (crate-internal)
pub(crate) mod mesh;

/// 3D models composed of one or more meshes
pub mod model;

/// World-space instances of models, with position, rotation and scale
pub mod instance;

/// World-space meshes with pre-baked lighting (crate-internal)
pub(crate) mod baked_mesh;

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

/// Convenience re-exports of the most commonly used types and helpers.
/// `use argh::prelude::*;` to pull in everything needed for a typical app.
pub mod prelude {
  pub use crate::app::App;
  pub use crate::camera::Camera;
  pub use crate::colour::*;
  pub use crate::engine::{Engine, InstanceHandle, LightHandle, ModelHandle};
  pub use crate::light::Light;
  pub use crate::material::{MATERIAL_PLACEHOLDER, Material};
  pub use crate::math::{Affine2, Mat4, Quat, V3_ZERO, Vec2, Vec3, Vec4};
  pub use crate::primitives;
  pub use crate::scene::Scene;
  pub use crate::texture::Texture;

  /// Shorthand for [`Vec3::new`]
  #[inline]
  pub fn v3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(x, y, z)
  }

  /// Shorthand for [`Vec2::new`]
  #[inline]
  pub fn v2(x: f32, y: f32) -> Vec2 {
    Vec2::new(x, y)
  }

  /// Shorthand for [`Colour::new`], components are floats in the 0.0-1.0 range
  #[inline]
  pub fn col(r: f32, g: f32, b: f32) -> Colour {
    Colour::new(r, g, b)
  }

  /// Shorthand for [`Colour::from_rgb8`], components are bytes in the 0-255 range
  #[inline]
  pub fn col8(r: u8, g: u8, b: u8) -> Colour {
    Colour::from_rgb8(r, g, b)
  }
}
