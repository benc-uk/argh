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
  pub(crate) diffuse: f64,
  _spec: f64,
  pub(crate) texture: Box<dyn Texture>,
}

impl Material {
  pub fn new(t: Box<dyn Texture>) -> Self {
    Self {
      diffuse: 1.0,
      _spec: 1.0,
      texture: t,
    }
  }
}
// ===================================
// Mesh
// ===================================

/// Simple 3D mesh of triangles using index
pub struct Mesh {
  pub(crate) verts: Vec<Vec3>,
  pub(crate) indices: Vec<i32>,
  pub(crate) material: Option<Material>,
  pub(crate) normals: Vec<Vec3>, // Currently one per triangle

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

    let normals = vec![
      Vec3::new(0.0, 0.0, 1.0),  // front
      Vec3::new(0.0, 0.0, 1.0),  // front
      Vec3::new(0.0, 0.0, -1.0), // back
      Vec3::new(0.0, 0.0, -1.0), // back
      Vec3::new(-1.0, 0.0, 0.0), // left
      Vec3::new(-1.0, 0.0, 0.0), // left
      Vec3::new(1.0, 0.0, 0.0),  // right
      Vec3::new(1.0, 0.0, 0.0),  // right
      Vec3::new(0.0, 1.0, 0.0),  // top
      Vec3::new(0.0, 1.0, 0.0),  // top
      Vec3::new(0.0, -1.0, 0.0), // bottom
      Vec3::new(0.0, -1.0, 0.0), // bottom
    ];

    Self {
      verts,
      indices,
      normals,
      material: None,
      pos: VEC3_ZERO,
      rot: Quat::ident(),
    }
  }

  /// Create a UV sphere mesh of unit diameter (radius 0.5) at the origin.
  ///
  /// # Arguments
  /// * `stacks` - Latitude divisions (rings between the poles). Min 2.
  /// * `sectors` - Longitude divisions (slices around the Y axis). Min 3.
  ///
  /// Higher counts mean smoother silhouette and softer flat-shaded facets,
  /// at the cost of triangle count. A reasonable default is (16, 24).
  /// Triangle count is roughly 2 * stacks * sectors (less at the poles).
  pub fn new_sphere(stacks: usize, sectors: usize) -> Self {
    let stacks = stacks.max(2);
    let sectors = sectors.max(3);
    let radius = 0.5;

    let pi = std::f64::consts::PI;

    // --- 1. Generate verts: (stacks+1) rings of (sectors+1) verts.
    // Seam and poles are intentionally duplicated to keep indexing uniform.
    let mut verts: Vec<Vec3> = Vec::with_capacity((stacks + 1) * (sectors + 1));
    for i in 0..=stacks {
      let phi = pi * (i as f64) / (stacks as f64); // 0 at +Y pole, pi at -Y pole
      let sin_phi = phi.sin();
      let cos_phi = phi.cos();

      for j in 0..=sectors {
        let theta = 2.0 * pi * (j as f64) / (sectors as f64);
        let x = sin_phi * theta.cos();
        let y = cos_phi;
        let z = sin_phi * theta.sin();
        verts.push(Vec3::new(x * radius, y * radius, z * radius));
      }
    }

    // --- 2. Build triangles + per-face normals.
    // Each quad cell (i,j) -> (i+1,j+1) emits two triangles, except at the
    // poles where one of them is degenerate and is skipped.
    let row_stride = sectors + 1;
    let mut indices: Vec<i32> = Vec::with_capacity(stacks * sectors * 6);
    let mut normals: Vec<Vec3> = Vec::with_capacity(stacks * sectors * 2);

    let face_normal = |verts: &[Vec3], a: i32, b: i32, c: i32| -> Vec3 {
      let v0 = verts[a as usize];
      let v1 = verts[b as usize];
      let v2 = verts[c as usize];
      (v1 - v0).cross(v2 - v0).normalize_new()
    };

    for i in 0..stacks {
      for j in 0..sectors {
        let v00 = (i * row_stride + j) as i32; // top-left
        let v01 = (i * row_stride + j + 1) as i32; // top-right
        let v10 = ((i + 1) * row_stride + j) as i32; // bottom-left
        let v11 = ((i + 1) * row_stride + j + 1) as i32; // bottom-right

        // Tri A: WAS v00, v10, v11   ->  NOW v00, v11, v10
        if i != stacks - 1 {
          indices.extend_from_slice(&[v00, v11, v10]);
          normals.push(face_normal(&verts, v00, v11, v10));
        }

        // Tri B: WAS v00, v11, v01   ->  NOW v00, v01, v11
        if i != 0 {
          indices.extend_from_slice(&[v00, v01, v11]);
          normals.push(face_normal(&verts, v00, v01, v11));
        }
      }
    }

    Self {
      verts,
      indices,
      normals,
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
