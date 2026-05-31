// ==============================================================================================
// Module & file:   engine / resources.rs
// Purpose:         Operations for engine managed resources like meshes & materials
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use crate::models::{Material, Mesh};

use super::{Engine, MaterialHandle, MeshHandle};

impl Engine {
  // ===== Meshes ======================================================================================================

  /// Add a mesh to the cache and give it a name
  pub fn add_mesh(&mut self, mesh: Mesh) -> MeshHandle {
    println!("Adding mesh '{}' to the engine cache", mesh.name);
    self.meshes.insert(mesh)
  }

  // ===== Materials ======================================================================================================

  pub fn add_material(&mut self, mat: Material) -> MaterialHandle {
    self.materials.insert(mat)
  }

  pub fn material_mut(&mut self, h: MaterialHandle) -> &mut Material {
    self.materials.get_mut(h).expect("material not found")
  }

  pub fn material(&self, h: MaterialHandle) -> &Material {
    self.materials.get(h).expect("material not found")
  }

  pub fn material_placeholder(&self) -> MaterialHandle {
    self.mat_placeholder
  }
}
