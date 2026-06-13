// ==============================================================================================
// Module & file:   mesh.rs
// Purpose:         Triangle based 3D mesh of verts plus a material
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use crate::{
  material::{MATERIAL_PLACEHOLDER, Material},
  math::{Vec2, Vec3},
};

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
