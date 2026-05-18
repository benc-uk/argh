// ==============================================================================================
// Module & file:   models.rs
// Purpose:         Module for 3D meshes, materials and higher level models & object
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use crate::{
  colour::Colour,
  engine::MeshHandle,
  math::{Mat4, Quat, Vec3},
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
  pub(crate) specular: f64,
  pub(crate) hardness: f64,
  pub(crate) texture: Box<dyn Texture>,
}

impl Material {
  pub fn new<T: Texture + 'static>(t: T) -> Self {
    Self {
      diffuse: 1.0,
      specular: 1.0,
      hardness: 12.0,
      texture: Box::new(t),
    }
  }
}

// ===================================
// Mesh
// ===================================

/// Triangle based 3D mesh
pub struct Mesh {
  pub(crate) verts: Vec<Vec3>,   // Internal mesh vert position
  pub(crate) normals: Vec<Vec3>, // Normal per vert
  pub(crate) indices: Vec<i32>,  // Indices are pointers to verts, in groups of three
}

impl Mesh {
  // Internal only method for creating an "empty" mesh
  pub(crate) fn new() -> Self {
    Self {
      verts: vec![],
      normals: vec![],
      indices: vec![],
    }
  }
}

// ===================================
// Instance
// ===================================

/// Instance of a mesh in the world, with position, scale and rotation
pub struct Instance {
  pub(crate) material: Material, // Surface material, colour etc
  pub(crate) pos: Vec3,          // Position
  pub(crate) rot: Quat,          // Rotation held as a Quat
  pub(crate) scale: Vec3,        // Scaling factors
  pub(crate) mesh: MeshHandle,   // Reference to mesh via handle

  pub smooth: bool, // Gouraud shading enabled
}

impl Instance {
  pub fn set_material(&mut self, m: Material) -> &mut Self {
    self.material = m;
    self
  }

  pub fn get_material(&self) -> &Material {
    &self.material
  }

  pub fn set_pos(&mut self, pos: Vec3) -> &mut Self {
    self.pos = pos;
    self
  }

  pub fn set_pos_xyz(&mut self, x: f64, y: f64, z: f64) -> &mut Self {
    self.pos = Vec3 { x, y, z };
    self
  }

  pub fn rot_x(&mut self, a: f64) -> &mut Self {
    self.rot.rot_x(a);
    self
  }

  pub fn rot_y(&mut self, a: f64) -> &mut Self {
    self.rot.rot_y(a);
    self
  }

  pub fn rot_z(&mut self, a: f64) -> &mut Self {
    self.rot.rot_z(a);
    self
  }

  pub fn scale(&mut self, s: f64) -> &mut Self {
    self.scale = Vec3 { x: s, y: s, z: s };
    self
  }

  pub fn scale_x(&mut self, s: f64) -> &mut Self {
    self.scale.x = s;
    self
  }

  pub fn scale_y(&mut self, s: f64) -> &mut Self {
    self.scale.y = s;
    self
  }

  pub fn scale_z(&mut self, s: f64) -> &mut Self {
    self.scale.z = s;
    self
  }

  /// Return the model matrix for this mesh, with scale, rotation and translation
  pub fn get_model_mat(&self) -> Mat4 {
    Mat4::new_scale_rot_trans(self.scale.x, self.scale.y, self.scale.z, self.rot, self.pos.x, self.pos.y, self.pos.z)
  }
}
