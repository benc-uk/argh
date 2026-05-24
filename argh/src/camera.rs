// ==============================================================================================
// Module & file:   camera.rs
// Purpose:         A camera holds view and projection matrix and has various conveniences
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use std::error::Error;

use crate::math::{Mat4, Vec3};

/// Camera is used to render a 3D scene (set of meshes) from a given position and pointing in a given direction
pub struct Camera {
  pos: Vec3,
  look_at: Vec3,
  pub(crate) pers_mat: Mat4,
  pub(crate) view_mat: Mat4,
}

impl Camera {
  /// Create a perspective projection based Camera
  /// # Arguments:
  /// * `aspect` - Aspect ratio of the camera view, typically you'd use `engine.get_aspect()` to match the output viewport
  /// * `pos` - Position of the camera
  /// * `look_at` - Point in space for the camera to orient towards
  /// * `fov` - Field of view in degrees
  /// * `near` - Near clipping plane, normally want this near zero, but NOT zero!
  /// * `far` - Far clipping plane, should be as far as your scene extends
  pub fn new_perspective(aspect: f64, pos: Vec3, look_at: Vec3, fov: f64, near: f64, far: f64) -> Result<Self, Box<dyn Error>> {
    Ok(Self {
      pos,
      look_at,
      pers_mat: Mat4::new_perspective(fov.to_radians(), aspect, near, far)?,
      view_mat: Mat4::new_look_at(pos, look_at, crate::math::AXIS_Y),
    })
  }

  /// Move the Camera to a new position
  pub fn set_pos(&mut self, pos: Vec3) {
    self.pos = pos;
    self.update();
  }

  /// Get the position of this camera
  pub fn get_pos(&self) -> Vec3 {
    self.pos
  }

  /// Change where the Camera is pointing
  pub fn set_look_at(&mut self, look_at: Vec3) {
    self.look_at = look_at;
    self.update();
  }

  // Internal method used to update the main view matrix
  fn update(&mut self) {
    self.view_mat = Mat4::new_look_at(self.pos, self.look_at, crate::math::AXIS_Y);
  }
}
