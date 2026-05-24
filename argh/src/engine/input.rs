// ==============================================================================================
// Module & file:   engine / input.rs
// Purpose:         Handles mouse and keyboard (maybe joypad?)
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use minifb::Key;

use super::Engine;

impl Engine {
  /// Returns the keys held down this frame. Snapshot taken once per frame before scene.update().
  pub fn get_keys(&self) -> &[Key] {
    &self.keys
  }

  /// Returns the keys recently pressed. Snapshot taken once per frame before scene.update().
  pub fn get_keys_pressed(&self) -> &[Key] {
    &self.keys_pressed
  }
}
