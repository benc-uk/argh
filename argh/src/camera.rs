// ==============================================================================================
// Module & file:   camera.rs
// Purpose:         A camera holds view and projection matrix and has various conveniences
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use crate::math::*;

pub struct Camera {
  pos: Vec3,
  look_at: Vec3,
  pub(crate) pers_mat: Mat4,
  pub(crate) view_mat: Mat4,
}

impl Camera {
  pub fn new_perspective(aspect: f64, pos: Vec3, look_at: Vec3, fov: f64, near: f64, far: f64) -> Self {
    Self {
      pos,
      look_at,
      pers_mat: Mat4::new_perspective(fov.to_radians(), aspect, near, far),
      view_mat: Mat4::new_look_at(pos, look_at, AXIS_Y),
    }
  }

  pub fn set_pos(&mut self, pos: Vec3) {
    self.pos = pos;
    self.update();
  }

  pub fn set_look_at(&mut self, look_at: Vec3) {
    self.look_at = look_at;
    self.update();
  }

  fn update(&mut self) {
    self.view_mat = Mat4::new_look_at(self.pos, self.look_at, AXIS_Y);
  }
}
