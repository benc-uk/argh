// ==============================================================================================
// Module & file:   models.rs
// Purpose:         Module for 3D meshes, materials and higher level models & object
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use crate::{
  colour::Colour,
  math::{Mat4, Quat, VEC3_ZERO, Vec3},
};

// ===================================
// Textures
// ===================================

/// Texture abstraction around bitmap images or other types of procedural textures
pub trait Texture {
  fn get_colour_at(&self, u: f64, v: f64) -> Colour;
}

/// A flat uniform colour, handy for debugging
pub struct SimpleColourTexture {
  colour: Colour,
}

impl SimpleColourTexture {
  pub fn new(c: Colour) -> Self {
    Self { colour: c }
  }
}

impl Texture for SimpleColourTexture {
  fn get_colour_at(&self, _u: f64, _v: f64) -> Colour {
    self.colour
  }
}

// ===================================
// Material
// ===================================

/// Material holds parameters for rendering the surface of a mesh
pub struct Material {
  _diffuse: f64,
  _spec: f64,
  pub(crate) texture: Box<dyn Texture>,
}

impl Material {
  pub fn new(t: Box<dyn Texture>) -> Self {
    Self {
      _diffuse: 1.0,
      _spec: 1.0,
      texture: t,
    }
  }
}
// ===================================
// Mesh
// ===================================

/// Simple mesh
pub struct Mesh {
  pub(crate) verts: Vec<Vec3>,
  pub(crate) indices: Vec<i32>,
  pub(crate) material: Option<Material>,

  pub(crate) pos: Vec3,
  pub(crate) rot: Quat,
}

impl Mesh {
  /// Create a mesh for a unit cube
  pub fn new_cube() -> Self {
    // Unit cube at the origin
    let verts = vec![
      Vec3::new(-0.5, -0.5, -0.5), // 0: back  bottom left
      Vec3::new(0.5, -0.5, -0.5),  // 1: back  bottom right
      Vec3::new(0.5, 0.5, -0.5),   // 2: back  top    right
      Vec3::new(-0.5, 0.5, -0.5),  // 3: back  top    left
      Vec3::new(-0.5, -0.5, 0.5),  // 4: front bottom left
      Vec3::new(0.5, -0.5, 0.5),   // 5: front bottom right
      Vec3::new(0.5, 0.5, 0.5),    // 6: front top    right
      Vec3::new(-0.5, 0.5, 0.5),   // 7: front top    left
    ];

    // 12 triangles, CCW winding when viewed from outside the cube
    let indices = vec![
      4, 5, 6, 4, 6, 7, // front
      1, 0, 3, 1, 3, 2, // back
      0, 4, 7, 0, 7, 3, // left
      5, 1, 2, 5, 2, 6, // right
      7, 6, 2, 7, 2, 3, // top
      0, 1, 5, 0, 5, 4, // bottom
    ];

    Self {
      verts,
      indices,
      material: None,
      pos: VEC3_ZERO,
      rot: Quat::ident(),
    }
  }

  pub fn set_material(&mut self, m: Material) {
    self.material = Some(m);
  }

  pub fn set_pos(&mut self, pos: Vec3) {
    self.pos = pos
  }

  pub fn set_pos_xyz(&mut self, x: f64, y: f64, z: f64) {
    self.pos = Vec3 { x, y, z }
  }

  pub fn rot_x(&mut self, a: f64) {
    self.rot.rot_x(a);
  }

  pub fn rot_y(&mut self, a: f64) {
    self.rot.rot_y(a);
  }

  pub fn rot_z(&mut self, a: f64) {
    self.rot.rot_z(a);
  }

  /// Return the model matrix for this mesh, with scale, rotation and translation
  pub fn get_model_mat(&self) -> Mat4 {
    Mat4::new_scale_rot_trans(1.0, 1.0, 1.0, self.rot, self.pos.x, self.pos.y, self.pos.z)
  }
}
