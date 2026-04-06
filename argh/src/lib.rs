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
//!     let eng = Engine::new(800, 600, String::from("Hello World"));
//!     eng.start(MyScene {});
//! }
//! ```
pub(crate) mod buffer;
pub mod colour;
pub mod core;
