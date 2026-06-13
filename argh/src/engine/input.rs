// ==============================================================================================
// Module & file:   engine / input.rs
// Purpose:         Handles mouse and keyboard (maybe joypad?)
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use minifb::Window;

use super::Engine;
use super::Key;

pub(super) struct Inputs {
  pub(super) keys: Vec<Key>,
  pub(super) keys_pressed: Vec<Key>,
  pub(super) mouse_buttons: [bool; 3],
  pub(super) mouse_pos: Option<(f32, f32)>,
}

impl Inputs {
  pub(super) fn new() -> Self {
    Self {
      keys: vec![],
      keys_pressed: vec![],
      mouse_buttons: [false, false, false],
      mouse_pos: None,
    }
  }

  pub(super) fn scrape(&mut self, window: &Window) {
    self.keys = window.get_keys();
    self.keys_pressed = window.get_keys_pressed(minifb::KeyRepeat::Yes);
    self.mouse_buttons[0] = window.get_mouse_down(minifb::MouseButton::Left);
    self.mouse_buttons[1] = window.get_mouse_down(minifb::MouseButton::Middle);
    self.mouse_buttons[2] = window.get_mouse_down(minifb::MouseButton::Right);
    self.mouse_pos = None;
    self.mouse_pos = window.get_mouse_pos(minifb::MouseMode::Discard);
  }
}

impl Engine {
  /// Returns the keys held down this frame. Snapshot taken once per frame before scene.update().
  pub fn keys(&self) -> &[Key] {
    &self.desktop.inputs.keys
  }

  /// Returns the keys recently pressed. Snapshot taken once per frame before scene.update().
  pub fn keys_pressed(&self) -> &[Key] {
    &self.desktop.inputs.keys_pressed
  }

  /// Returns the mouse buttons
  pub fn mouse_buttons(&self) -> [bool; 3] {
    self.desktop.inputs.mouse_buttons
  }

  /// Helper to check if a key is pressed
  pub fn is_pressed(&self, k: Key) -> bool {
    self.desktop.inputs.keys.contains(&k)
  }

  /// Helper to check if left mouse button is pressed
  pub fn is_mouse_down_left(&self) -> bool {
    self.desktop.inputs.mouse_buttons[0]
  }

  /// Helper to check if middle mouse button is pressed
  pub fn is_mouse_down_middle(&self) -> bool {
    self.desktop.inputs.mouse_buttons[1]
  }

  /// Helper to check if right mouse button is pressed
  pub fn is_mouse_down_right(&self) -> bool {
    self.desktop.inputs.mouse_buttons[2]
  }

  /// Helper to check if right mouse button is pressed
  pub fn mouse_pos(&self) -> Option<(f32, f32)> {
    self.desktop.inputs.mouse_pos
  }
}
