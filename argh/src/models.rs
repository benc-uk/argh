// ==============================================================================================
// Module & file:   models.rs
// Purpose:         Module for 3D meshes, materials and higher level models & object
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use image::{DynamicImage, GenericImageView, ImageError, ImageReader};
use std::io;

use crate::{
  colour::Colour,
  engine::{MaterialHandle, MeshHandle},
  math::{Mat4, Quat, Vec2, Vec3},
};

// ===================================
// Textures
// ===================================

#[derive(thiserror::Error)]
pub enum TextureError {
  #[error("image error: {0}")]
  ImageError(#[from] ImageError),
  #[error("io error: {0}")]
  IoError(#[from] io::Error),
}

impl std::fmt::Debug for TextureError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self, f)
  }
}

/// Texture abstraction around bitmap images or other types of procedural textures
pub trait Texture {
  fn sample(&self, u: f64, v: f64) -> Colour;
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
  fn sample(&self, _u: f64, _v: f64) -> Colour {
    self.colour
  }
}

pub struct ImageTexture {
  image: DynamicImage,
  w: u32,
  h: u32,
}

impl ImageTexture {
  pub fn new(path: &str) -> Result<Self, TextureError> {
    let img = ImageReader::open(path)?.decode()?;
    let w = img.width();
    let h = img.height();
    Ok(Self { image: img, w, h })
  }
}

impl Texture for ImageTexture {
  fn sample(&self, u: f64, v: f64) -> Colour {
    let x = u * self.w as f64;
    let y = v * self.h as f64;
    let pix = self.image.get_pixel(x as u32, y as u32);

    Colour::from_rgb8(pix.0[0], pix.0[1], pix.0[2])
  }
}

// ===================================
// Material
// ===================================

/// Material holds parameters for rendering the surface of a mesh
pub struct Material {
  pub diffuse: f64,
  pub specular: f64,
  pub hardness: f64,
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

  pub fn set_texture<T: Texture + 'static>(&mut self, t: T) {
    self.texture = Box::new(t)
  }
}

// ===================================
// Mesh
// ===================================

/// Triangle based 3D mesh
pub struct Mesh {
  pub(crate) verts: Vec<Vec3>,   // Internal mesh vert position
  pub(crate) normals: Vec<Vec3>, // Normal per vert
  pub(crate) uvs: Vec<Vec2>,     // Texture coords
  pub(crate) indices: Vec<i32>,  // Indices are pointers to verts, in groups of three
}

impl Mesh {
  // Internal only method for creating an "empty" mesh
  pub(crate) fn new() -> Self {
    Self {
      verts: vec![],
      normals: vec![],
      uvs: vec![],
      indices: vec![],
    }
  }
}

// ===================================
// Instance
// ===================================

/// Instance of a mesh in the world, with position, scale and rotation
pub struct Instance {
  pub(crate) material_handle: MaterialHandle, // Surface material, colour etc
  pub(crate) pos: Vec3,                       // Position
  pub(crate) rot: Quat,                       // Rotation held as a Quat
  pub(crate) scale: Vec3,                     // Scaling factors
  pub(crate) mesh_handle: MeshHandle,         // Reference to mesh via handle

  pub smooth: bool, // Gouraud shading enabled
}

impl Instance {
  pub fn set_material(&mut self, m: MaterialHandle) -> &mut Self {
    self.material_handle = m;
    self
  }

  pub fn get_material(&self) -> MaterialHandle {
    self.material_handle
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
