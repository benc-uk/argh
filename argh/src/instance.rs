// ==============================================================================================
// Module & file:   instance.rs
// Purpose:         Instance of a model in the world, with position, scale and rotation
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use crate::{
  engine::{InstanceHandle, ModelHandle},
  math::{Mat4, Quat, Vec3},
};

/// Instance of a model in the world, with position, scale and rotation
/// Instances represent dynamic objects in the world which can be moved
pub struct Instance {
  // Transforms
  pub(crate) pos: Vec3,   // Position
  pub(crate) rot: Quat,   // Rotation held as a Quat
  pub(crate) scale: Vec3, // Scaling factors

  // References
  pub(crate) model_handle: ModelHandle, // Reference to the model this is an instance of
  pub(crate) handle: InstanceHandle,    // Self reference to the instance handle

  // Enable Gouraud (smooth) shading for this instance
  pub(crate) smooth: bool,
}

impl Instance {
  /// Set the position of this instance in world space
  pub fn pos(&mut self, pos: Vec3) -> &mut Self {
    self.pos = pos;
    self
  }

  /// Set the position of this instance from individual x, y, z components
  pub fn pos_xyz(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
    self.pos = Vec3 { x, y, z };
    self
  }

  /// Rotate around the instance's local X axis by `a` radians
  pub fn rot_x(&mut self, a: f32) -> &mut Self {
    self.rot.rot_x(a);
    self
  }

  /// Rotate around the instance's local Y axis by `a` radians
  pub fn rot_y(&mut self, a: f32) -> &mut Self {
    self.rot.rot_y(a);
    self
  }

  /// Rotate around the instance's local Z axis by `a` radians
  pub fn rot_z(&mut self, a: f32) -> &mut Self {
    self.rot.rot_z(a);
    self
  }

  /// Rotate around the world X axis by `a` radians
  pub fn rot_x_world(&mut self, a: f32) -> &mut Self {
    self.rot.rot_x_world(a);
    self
  }

  /// Rotate around the world Y axis by `a` radians
  pub fn rot_y_world(&mut self, a: f32) -> &mut Self {
    self.rot.rot_y_world(a);
    self
  }

  /// Rotate around the world Z axis by `a` radians
  pub fn rot_z_world(&mut self, a: f32) -> &mut Self {
    self.rot.rot_z_world(a);
    self
  }

  /// Set a uniform scale factor on all three axes
  pub fn scale(&mut self, s: f32) -> &mut Self {
    self.scale = Vec3 { x: s, y: s, z: s };
    self
  }

  /// Set the scale factor on the X axis only
  pub fn scale_x(&mut self, s: f32) -> &mut Self {
    self.scale.x = s;
    self
  }

  /// Set the scale factor on the Y axis only
  pub fn scale_y(&mut self, s: f32) -> &mut Self {
    self.scale.y = s;
    self
  }

  /// Set the scale factor on the Z axis only
  pub fn scale_z(&mut self, s: f32) -> &mut Self {
    self.scale.z = s;
    self
  }

  /// Set polygon surface smoothing
  pub fn smooth(&mut self, smooth: bool) -> &mut Self {
    self.smooth = smooth;
    self
  }

  /// Return the model matrix for this mesh, with scale, rotation and translation
  pub fn model_mat(&self) -> Mat4 {
    Mat4::new_scale_rot_trans(self.scale.x, self.scale.y, self.scale.z, self.rot, self.pos.x, self.pos.y, self.pos.z)
  }

  /// Get the handle of this instance
  pub fn handle(&self) -> InstanceHandle {
    self.handle
  }
}
