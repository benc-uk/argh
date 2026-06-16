// ==============================================================================================
// Module & file:   mesh.rs
// Purpose:         Triangle based 3D mesh (positions, normals, tex-coords) & Material
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use crate::{
  helpers::Aabb,
  material::{MATERIAL_PLACEHOLDER, Material},
  math::{Vec2, Vec3},
};

#[cfg(test)]
#[path = "tests/mesh_tests.rs"]
mod mesh_tests;

/// Triangle based 3D mesh of verts + material
pub(crate) struct Mesh {
  pub(crate) material: Material,    // Owned version of Material
  pub(crate) positions: Vec<Vec3>,  // Vert position
  pub(crate) normals: Vec<Vec3>,    // Normal per vert
  pub(crate) tex_coords: Vec<Vec2>, // Texture coords
  pub(crate) indices: Vec<u32>,     // Indices are pointers to verts, in groups of three
  pub(crate) name: String,          // From loaded/parsed models
  pub(crate) tri_count: u32,        // Stats only
  pub(crate) aabb: Aabb,
}

impl Mesh {
  // Internal only method for creating an "empty" mesh with placeholder material
  pub(crate) fn new() -> Self {
    Self {
      material: MATERIAL_PLACEHOLDER,
      positions: vec![],
      normals: vec![],
      tex_coords: vec![],
      indices: vec![],
      name: "".to_string(),
      tri_count: 0,
      aabb: Aabb::empty(),
    }
  }

  // Internal only method for creating an "empty" mesh with a material
  pub(crate) fn new_with_material(mat: Material) -> Self {
    Self {
      material: mat,
      positions: vec![],
      normals: vec![],
      tex_coords: vec![],
      indices: vec![],
      name: "".to_string(),
      tri_count: 0,
      aabb: Aabb::empty(),
    }
  }

  /// Split shared vertices and assign per-face normals so this mesh renders
  /// flat-shaded when fed through the pure Gouraud pipeline. Vertex count
  /// becomes 3 * tri_count and indices become 0..N sequential. Triangle count
  /// and surface topology are unchanged.
  pub(crate) fn flatten(&mut self) {
    let tri_count = self.indices.len() / 3;
    let mut new_positions = Vec::with_capacity(tri_count * 3);
    let mut new_normals = Vec::with_capacity(tri_count * 3);
    let mut new_tex_coords = Vec::with_capacity(tri_count * 3);

    for tri in self.indices.chunks_exact(3) {
      let i0 = tri[0] as usize;
      let i1 = tri[1] as usize;
      let i2 = tri[2] as usize;
      let p0 = self.positions[i0];
      let p1 = self.positions[i1];
      let p2 = self.positions[i2];

      // Face normal from the two triangle edges, kept consistent with the
      // winding used everywhere else in the renderer
      let face_normal = (p1 - p0).cross(p2 - p0).normalize_new();

      for &i in tri {
        let i = i as usize;
        new_positions.push(self.positions[i]);
        new_normals.push(face_normal);
        new_tex_coords.push(self.tex_coords[i]);
      }
    }

    self.positions = new_positions;
    self.normals = new_normals;
    self.tex_coords = new_tex_coords;
    self.indices = (0..(tri_count as u32 * 3)).collect();
  }
}
