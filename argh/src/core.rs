// ==============================================================================================
// Module & file:   core.rs
// Purpose:         Holds common structures like Models, Textures, Meshes, Materials, Instances
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:           Split this up once it makes sense
// ==============================================================================================

use image::{ImageError, ImageReader};
use slotmap::SlotMap;
use std::{collections::HashMap, io, rc::Rc};

use crate::{
  colour::{BLACK, Colour, INV_255, WHITE},
  engine::{InstanceHandle, LightHandle, ModelHandle},
  light::Light,
  math::{Mat4, Quat, Vec2, Vec3},
};

// ===================================
// Textures
// ===================================

/// Errors that can occur when loading or decoding a [Texture]
#[derive(thiserror::Error)]
pub enum TextureError {
  /// The underlying image crate failed to decode the image data
  #[error("image error: {0}")]
  ImageError(#[from] ImageError),
  /// An I/O error occurred reading the texture file from disk
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
  pub(crate) alpha_cutout: bool,
}

// In Rust enums can have methods and an implementation, which is kinda wild
impl Texture {
  /// Load a texture from an image file on disk. Supports any format the `image` crate can decode (PNG, JPEG, BMP, etc).
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

  /// Load a texture from a byte buffer (e.g. an embedded asset via `include_bytes!`).
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

  /// Enable or disable alpha cutout
  pub fn enable_cutout(&mut self, cutout: bool) {
    self.alpha_cutout = cutout
  }
}

// ===================================
// Material
// ===================================

/// Material holds parameters for rendering the surface of a mesh, can be textured or flat
#[derive(Clone)]
pub struct Material {
  /// Diffuse or base colour of the object
  pub diffuse: Colour,

  /// How this object reflects specular lighting, nearly always WHITE
  pub specular: Colour,

  /// Size of specular highlights, higher > smaller
  pub hardness: f32,

  // Internal texture, might be None
  pub(crate) texture: Option<Rc<Texture>>,
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
      texture: Some(Rc::new(tex)),
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
    self.texture = Some(Rc::new(tex))
  }

  /// Get the internal texture
  pub fn texture(&self) -> Option<&Rc<Texture>> {
    self.texture.as_ref()
  }
}

// ===================================
// Mesh
// ===================================

/// Triangle based 3D mesh of verts + material
pub(crate) struct Mesh {
  pub(crate) material: Material,
  pub(crate) verts: Vec<Vec3>,   // Vert position
  pub(crate) normals: Vec<Vec3>, // Normal per vert
  pub(crate) uvs: Vec<Vec2>,     // Texture coords
  pub(crate) indices: Vec<i32>,  // Indices are pointers to verts, in groups of three
  pub(crate) name: String,
  pub(crate) tri_count: u32,
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
      tri_count: 0,
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
      tri_count: 0,
    }
  }
}

// ===================================
// Model
// ===================================

/// Model holds multiple meshes
pub struct Model {
  pub(crate) meshes: Vec<Mesh>,
  pub(crate) name: String,
  pub(crate) tri_count: u32,
}

impl Model {
  /// Create an empty model with no meshes
  pub(crate) fn new(name: &str) -> Self {
    Self {
      meshes: vec![],
      name: name.to_string(),
      tri_count: 0,
    }
  }

  /// Convenience to wrap a single [Mesh] in a [Model]
  pub(crate) fn from_mesh(mesh: Mesh, name: &str) -> Self {
    let mut model = Self {
      meshes: vec![],
      name: name.to_string(),
      tri_count: 0,
    };

    model.add_mesh(mesh);
    model
  }

  /// Add a mesh to a model
  pub(crate) fn add_mesh(&mut self, mesh: Mesh) {
    debug_assert_eq!(mesh.uvs.len(), mesh.verts.len(), "UVs must match vert count");
    debug_assert_eq!(mesh.normals.len(), mesh.verts.len(), "normals must match vert count");

    self.tri_count += &mesh.tri_count;
    self.meshes.push(mesh);
  }

  /// Get model's name typically this is generated when it's loaded/parsed from a file
  pub fn name(&self) -> &str {
    &self.name
  }

  /// Get a map of every mesh in this model, keyed by mesh name, with the value being the
  /// mesh's index (position) in the model's mesh list. If two meshes share the same name
  /// the later one in the model wins, since [`HashMap`] keys are unique.
  pub fn mesh_info(&self) -> HashMap<&str, usize> {
    self.meshes.iter().enumerate().map(|(i, mesh)| (mesh.name.as_str(), i)).collect()
  }

  /// Update a meshes within this model and override it's material
  pub fn set_mesh_material(&mut self, index: usize, mat: Material) {
    self.meshes[index].material = mat;
  }

  /// Update ALL meshes within this model with a new material
  pub fn set_all_material(&mut self, mat: Material) {
    for mesh in &mut self.meshes {
      mesh.material = mat.clone();
    }
  }
}

// ===================================
// Instance
// ===================================

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

// ===================================
// Baked Mesh
// ===================================

pub(crate) struct BakedMesh {
  pub(crate) material: Material,
  pub(crate) verts: Vec<Vec3>,   // already in WORLD space
  pub(crate) normals: Vec<Vec3>, // already in WORLD space, normalised
  pub(crate) uvs: Vec<Vec2>,
  pub(crate) indices: Vec<i32>,
  pub(crate) baked_lighting: Vec<Colour>, // Baked lighting
}

impl BakedMesh {
  pub(crate) fn bake_lighting(&mut self, lights: &SlotMap<LightHandle, Light>, ambient: Colour) {
    self.baked_lighting.clear();
    self.baked_lighting.reserve(self.verts.len());

    for (vert, normal) in self.verts.iter().zip(&self.normals) {
      let mut diffuse = BLACK;

      for light in lights.values() {
        // Important, only is_static lights are used for baking
        if !light.is_static {
          continue;
        }

        let l_raw = light.pos - *vert;
        let d = l_raw.len();
        let l = l_raw.normalize_new();
        let atten = 1.0 / (1.0 + light.atten_linear * d + light.atten_quad * d * d);
        let n_dot_l = normal.dot(l).max(0.0);
        diffuse += light.colour * light.brightness * n_dot_l * atten;
      }

      let amb = ambient * self.material.diffuse;
      self.baked_lighting.push(diffuse * self.material.diffuse + amb);
    }
  }
}
