//! Argh (Another Rust Graphics Helper) is a software rendering library using minifb. It supports 2D and 3D graphics functions and has been developed without AI assistance
//!
//! # Examples
//! Minimal usage and getting started
//! ```
//! use argh::core::{Engine, Scene};
//!
//! struct MyScene {}
//! impl Scene for MyScene {
//!     fn update(&mut self, e: &mut Engine, _: f64) {
//!         // Update & draw here
//!     }
//! }
//!
//! fn main() {
//!     let eng = Engine::new(800, 600, String::from("Hello World"), 1);
//!     eng.start(MyScene {});
//! }
//! ```

// Crate level modules
pub(crate) mod buffer;

/// Some helpers for various graphics routines
pub(crate) mod helpers;

pub mod math;

/// Module for working with 8-bit RGB colour
pub mod colour;

/// The core of argh is the engine module
pub mod engine;

/// Models, meshes, textures, materials and 3D objects
pub mod models;

/// Models, meshes, textures, materials and 3D objects
pub mod primitives;

/// Very basic text rendering
pub mod text;

/// Positionable Camera with look-at and FOV
pub mod camera;

/// Light source with position & colour
pub mod light;
