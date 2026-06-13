// ==============================================================================================
// Module & file:   model.rs
// Purpose:         Model is a collection of meshes, typically loaded from a single asset file
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use std::collections::HashMap;

use crate::{material::Material, mesh::Mesh};

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
