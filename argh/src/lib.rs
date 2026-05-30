//! Argh (Another Rust Graphics Helper) is a software rendering library using minifb. It supports 2D and 3D graphics functions and has been developed without AI assistance
//!
//! # Examples
//! Minimal usage and getting started
//! ```no_run
//! use argh::colour::BLUE;
//! use argh::engine::{Engine, App};
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
