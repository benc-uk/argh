// ==============================================================================================
// Module & file:   models.rs
// Purpose:         Holds common structures like Models, Textures, Meshes, Materials, Instances
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:           Split this up once it makes sense
// ==============================================================================================

use image::{ImageError, ImageReader};
use std::io;

use crate::{
  colour::{Colour, INV_255, WHITE},
  engine::ModelHandle,
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

/// Holds a pixels of an image and not much else
pub struct Texture {
  pixels: Vec<u32>, // packed 0RGB to match buffer format
  w: u32,
  h: u32,

  /// Treat alpha transparent pixels as invisible (cut them out). Defaults to true
  pub alpha_cutout: bool,
}

// In Rust enums can have methods and an implementation, which is kinda wild
impl Texture {
  // Private only called vis Texture enum methods
  pub fn new(path: &str) -> Result<Self, TextureError> {
    println!("Trying to load texture image file: {}", path);

    let img = ImageReader::open(path)?.decode()?.to_rgba8(); // was to_rgb8
    let (w, h) = img.dimensions();
    let pixels = img
      .pixels()
      .map(|p| {
        ((p[3] as u32) << 24) |   // alpha in top byte
        ((p[0] as u32) << 16) |
        ((p[1] as u32) << 8)  |
        (p[2] as u32)
      })
      .collect();
    Ok(Self { pixels, w, h, alpha_cutout: true })
  }

  pub fn from_bytes(bytes: &[u8]) -> Result<Self, TextureError> {
    let img = image::load_from_memory(bytes)?.to_rgba8();
    let (w, h) = img.dimensions();
    let pixels = img
      .pixels()
      .map(|p| {
        ((p[3] as u32) << 24) |   // alpha in top byte
        ((p[0] as u32) << 16) |
        ((p[1] as u32) << 8)  |
        (p[2] as u32)
      })
      .collect();
    Ok(Self { pixels, w, h, alpha_cutout: true })
  }

  /// Sample the texture with wrap-around addressing.
  /// Uses floor() to fold any UV into [0, 1] before scaling to texel space.
  /// Works for any texture size (pow2 or not)
  #[inline(always)]
  pub(crate) fn sample(&self, u: f32, v: f32) -> (Colour, f32) {
    let uf = u - u.floor();
    let vf = v - v.floor();
    let x = (uf * self.w as f32) as u32;
    let y = (vf * self.h as f32) as u32;
    let p = unsafe { *self.pixels.get_unchecked((y * self.w + x) as usize) };
    let a = ((p >> 24) & 0xFF) as f32 * INV_255;

    (Colour::from_packed_0rgb(p), a)
  }
}

// ===================================
// Material
// ===================================

/// Material holds parameters for rendering the surface of a mesh, can be textured or flat
pub struct Material {
  /// Diffuse or base colour of the object
  pub diffuse: Colour,

  /// How this object reflects specular lighting, nearly always WHITE
  pub specular: Colour,

  /// Size of specular highlights, higher > smaller
  pub hardness: f32,

  // Internal texture, might be None
  pub(crate) texture: Option<Texture>,
}

/// Most basic Material possible
pub const MATERIAL_PLACEHOLDER: Material = Material {
  diffuse: WHITE,
  hardness: 20.0,
  specular: WHITE,
  texture: None,
};

impl Material {
  /// Create a textured material from given texture
  pub fn new_textured(tex: Texture) -> Self {
    Self {
      diffuse: WHITE,
      specular: WHITE,
      hardness: 20.0,
      texture: Some(tex),
    }
  }

  /// Create a flat or solid coloured material
  pub fn new_flat(colour: Colour) -> Self {
    Self {
      diffuse: colour,
      specular: WHITE,
      hardness: 20.0,
      texture: None,
    }
  }

  /// Simple setter for the texture
  pub fn set_texture(&mut self, tex: Texture) {
    self.texture = Some(tex)
  }

  /// Get the internal texture
  pub fn get_texture(&mut self) -> &Option<Texture> {
    &self.texture
  }
}

// ===================================
// Mesh
// ===================================

/// Triangle based 3D mesh of verts + material
pub struct Mesh {
  pub(crate) material: Material,
  pub(crate) verts: Vec<Vec3>,   // Vert position
  pub(crate) normals: Vec<Vec3>, // Normal per vert
  pub(crate) uvs: Vec<Vec2>,     // Texture coords
  pub(crate) indices: Vec<i32>,  // Indices are pointers to verts, in groups of three
  pub(crate) name: String,
}

impl Mesh {
  // Internal only method for creating an "empty" mesh with placeholder material
  pub(crate) fn new() -> Self {
    Self {
      material: MATERIAL_PLACEHOLDER,
      verts: vec![],
      normals: vec![],
      uvs: vec![],
      indices: vec![],
      name: "".to_string(),
    }
  }

  // Internal only method for creating an "empty" mesh with placeholder material
  pub(crate) fn new_with_material(mat: Material) -> Self {
    Self {
      material: mat,
      verts: vec![],
      normals: vec![],
      uvs: vec![],
      indices: vec![],
      name: "".to_string(),
    }
  }

  pub fn set_material(&mut self, mat: Material) {
    self.material = mat
  }
}

// ===================================
// Model
// ===================================

/// Model holds multiple meshes
pub struct Model {
  pub(super) meshes: Vec<Mesh>,
  pub(super) name: String,
}

impl Model {
  /// Create an empty model with no meshes
  pub fn new(name: &str) -> Self {
    Self {
      meshes: vec![],
      name: name.to_string(),
    }
  }

  /// Convenience to wrap a single [Mesh] in a [Model]
  pub fn from_mesh(mesh: Mesh, name: &str) -> Self {
    let mut model = Self {
      meshes: vec![],
      name: name.to_string(),
    };

    model.add_mesh(mesh);
    model
  }

  /// Add a mesh to a model
  pub fn add_mesh(&mut self, mesh: Mesh) {
    debug_assert_eq!(mesh.uvs.len(), mesh.verts.len(), "UVs must match vert count");
    debug_assert_eq!(mesh.normals.len(), mesh.verts.len(), "normals must match vert count");

    self.meshes.push(mesh);
  }

  /// Get the meshes
  pub fn get_meshes(&self) -> &Vec<Mesh> {
    &self.meshes
  }
}

// ===================================
// Instance
// ===================================

/// Instance of a model in the world, with position, scale and rotation
pub struct Instance {
  pub(crate) pos: Vec3,                 // Position
  pub(crate) rot: Quat,                 // Rotation held as a Quat
  pub(crate) scale: Vec3,               // Scaling factors
  pub(crate) model_handle: ModelHandle, // Reference to mesh via handle

  pub smooth: bool, // Gouraud shading enabled
}

impl Instance {
  pub fn set_pos(&mut self, pos: Vec3) -> &mut Self {
    self.pos = pos;
    self
  }

  pub fn set_pos_xyz(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
    self.pos = Vec3 { x, y, z };
    self
  }

  pub fn rot_x(&mut self, a: f32) -> &mut Self {
    self.rot.rot_x(a);
    self
  }

  pub fn rot_y(&mut self, a: f32) -> &mut Self {
    self.rot.rot_y(a);
    self
  }

  pub fn rot_z(&mut self, a: f32) -> &mut Self {
    self.rot.rot_z(a);
    self
  }

  pub fn rot_x_world(&mut self, a: f32) -> &mut Self {
    self.rot.rot_x_world(a);
    self
  }

  pub fn rot_y_world(&mut self, a: f32) -> &mut Self {
    self.rot.rot_y_world(a);
    self
  }

  pub fn rot_z_world(&mut self, a: f32) -> &mut Self {
    self.rot.rot_z_world(a);
    self
  }

  pub fn scale(&mut self, s: f32) -> &mut Self {
    self.scale = Vec3 { x: s, y: s, z: s };
    self
  }

  pub fn scale_x(&mut self, s: f32) -> &mut Self {
    self.scale.x = s;
    self
  }

  pub fn scale_y(&mut self, s: f32) -> &mut Self {
    self.scale.y = s;
    self
  }

  pub fn scale_z(&mut self, s: f32) -> &mut Self {
    self.scale.z = s;
    self
  }

  /// Return the model matrix for this mesh, with scale, rotation and translation
  pub fn get_model_mat(&self) -> Mat4 {
    Mat4::new_scale_rot_trans(self.scale.x, self.scale.y, self.scale.z, self.rot, self.pos.x, self.pos.y, self.pos.z)
  }
}

// ===================================
// Static Mesh
// ===================================

// pub struct StaticMesh {
//   pub(crate) material: Material, // owned by value, same as Mesh today
//   pub(crate) verts: Vec<Vec3>,   // already in WORLD space
//   pub(crate) normals: Vec<Vec3>, // already in WORLD space, normalised
//   pub(crate) uvs: Vec<Vec2>,
//   pub(crate) indices: Vec<i32>,
//   // pub(crate) bounds: Aabb, // for frustum culling
//   pub(crate) baked_lighting: Vec<Colour>, // Baked lighting happening later
// }

// // ===================================
// // Chunk
// // ===================================

// pub struct Chunk {
//   bounds: Aabb,
//   meshes: Vec<StaticMesh>, // one per source mesh that landed in this chunk
// }
