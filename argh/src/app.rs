// ==============================================================================================
// Module & file:   app.rs
// Purpose:         App is just a trait with a single method used to hook the rendering loop
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use crate::engine::Engine;

/// All users of argh are expected to provide their own App implementation
pub trait App {
  /// This is a callback, invoked per frame to be the main render/draw loop of your App
  /// Desktop mode:
  /// This method will be called every frame by the main window loop
  /// If you are using web or WASM, or other host:
  /// You will need to call your this update() method yourself
  fn update(&mut self, engine: &mut Engine, dt: f64, t: f64);
}
