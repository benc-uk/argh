// ==============================================================================================
// Module & file:   models.rs
// Purpose:         Module for 3D meshes, materials and higher level models & object
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use crate::{
  colour::{Colour, WHITE},
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
  /// Create a texture with a single flat colour
  pub fn new(c: Colour) -> Self {
    Self { colour: c }
  }

  /// Modify the colour of this texture
  pub fn set_colour(&mut self, c: Colour) {
    self.colour = c
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
  pub(crate) diffuse: f64,
  _spec: f64,
  pub(crate) texture: Box<dyn Texture>,
}

impl Material {
  pub fn new<T: Texture + 'static>(t: T) -> Self {
    Self {
      diffuse: 1.0,
      _spec: 1.0,
      texture: Box::new(t),
    }
  }
}

// ===================================
// Mesh
// ===================================

/// Simple 3D mesh of triangles using index
pub struct Mesh {
  pub(crate) material: Material, // Surface material, colour etc
  pub(crate) pos: Vec3,          // Position
  pub(crate) rot: Quat,          // Rotation held as a Quat

  pub(crate) verts: Vec<Vec3>,   // Internal mesh vert position
  pub(crate) normals: Vec<Vec3>, // Normal per vert
  pub(crate) indices: Vec<i32>,  // Indices are pointers to verts, in groups of three

  pub smooth: bool,
}

impl Mesh {
  pub(crate) fn new() -> Self {
    let tex = SimpleColourTexture::new(WHITE);
    let mat = Material::new(tex);
    Self {
      material: mat,
      pos: VEC3_ZERO,
      rot: Quat::ident(),
      verts: vec![],
      indices: vec![],
      normals: vec![],
      smooth: true,
    }
  }

  pub fn set_material(&mut self, m: Material) {
    self.material = m;
  }

  pub fn get_material(&self) -> &Material {
    &self.material
  }

  pub fn set_pos(&mut self, pos: Vec3) {
    self.pos = pos;
  }

  pub fn set_pos_xyz(&mut self, x: f64, y: f64, z: f64) {
    self.pos = Vec3 { x, y, z };
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
