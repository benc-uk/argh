// ==============================================================================================
// Module & file:   model.rs
// Purpose:         Model is a collection of meshes, typically loaded from a single asset file
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use std::collections::HashMap;

use crate::{helpers::Aabb, material::Material, mesh::Mesh};

#[cfg(test)]
#[path = "tests/model_tests.rs"]
mod model_tests;

/// Model holds multiple meshes
pub struct Model {
  pub(crate) meshes: Vec<Mesh>,
  pub(crate) name: String,
  pub(crate) tri_count: u32,
  pub(crate) aabb: Aabb,
  pub(crate) is_opaque: bool,
}

impl Model {
  /// Create an empty model with no meshes
  pub(crate) fn new(name: &str) -> Self {
    Self {
      meshes: vec![],
      name: name.to_string(),
      tri_count: 0,
      aabb: Aabb::empty(),
      is_opaque: true,
    }
  }

  /// Convenience to wrap a single [Mesh] in a [Model]
  pub(crate) fn from_mesh(mesh: Mesh, name: &str) -> Self {
    let mut model = Self {
      meshes: vec![],
      name: name.to_string(),
      tri_count: 0,
      aabb: Aabb::empty(),
      is_opaque: true,
    };

    model.add_mesh(mesh);
    model
  }

  /// Add a mesh to a model
  pub(crate) fn add_mesh(&mut self, mut mesh: Mesh) {
    debug_assert_eq!(mesh.tex_coords.len(), mesh.positions.len(), "UVs must match vert count");
    debug_assert_eq!(mesh.normals.len(), mesh.positions.len(), "normals must match vert count");

    // Compute mesh AABB
    mesh.aabb = Aabb::from_points(mesh.positions.as_slice());

    // Update model AABB to union with new mesh
    self.aabb = self.aabb.union(&mesh.aabb);

    self.tri_count += mesh.tri_count;
    self.meshes.push(mesh);

    self.recompute_opaque()
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
    self.recompute_opaque()
  }

  /// Update ALL meshes within this model with a new material
  pub fn set_all_material(&mut self, mat: Material) {
    for mesh in &mut self.meshes {
      mesh.material = mat.clone();
    }
    self.recompute_opaque()
  }

  /// Split shared vertices across every mesh and assign per-face normals so
  /// the model renders as faceted (flat-shaded) under the Gouraud based triangle fill.
  pub fn flatten(&mut self) -> &mut Self {
    for mesh in &mut self.meshes {
      mesh.flatten();
    }
    self
  }

  // Internal method
  fn recompute_opaque(&mut self) {
    let mut opaque = true;

    for mesh in &self.meshes {
      if !mesh.material.is_opaque() {
        opaque = false
      }
    }

    self.is_opaque = opaque
  }
}
