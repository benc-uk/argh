// ==============================================================================================
// Module & file:   engine / resources.rs
// Purpose:         Operations for engine managed resources like lights, meshes, instances & materials
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use crate::{
  engine::LightHandle,
  light::Light,
  math::{Quat, VEC3_ONE, VEC3_ZERO, Vec3},
  models::{Instance, Material, Mesh},
};

use super::{Engine, InstanceHandle, MaterialHandle, MeshHandle};

impl Engine {
  // ===== Meshes ======================================================================================================

  /// Add a mesh to the cache and give it a name
  pub fn add_mesh(&mut self, mesh: Mesh) -> MeshHandle {
    self.meshes.insert(mesh)
  }

  // ===== Lights ======================================================================================================

  /// Add a light to the scene
  pub fn add_light(&mut self, light: Light) {
    let h = self.lights.insert(light);
    self.light_keys.push(h);
  }

  /// Remove a light from the scene
  pub fn remove_light(&mut self, h: LightHandle) {
    self.lights.remove(h);
    self.light_keys.retain(|&k| k != h);
  }

  pub fn light_mut(&mut self, h: LightHandle) -> &mut Light {
    self.lights.get_mut(h).expect("light not found")
  }

  pub fn light(&self, h: LightHandle) -> &Light {
    self.lights.get(h).expect("light not found")
  }

  // ===== Instances ======================================================================================================

  /// Create an instance of a mesh with given name, using the material
  pub fn add_instance(&mut self, mesh_handle: MeshHandle, mat_handle: MaterialHandle) -> InstanceHandle {
    let i = Instance {
      material_handle: mat_handle,
      pos: VEC3_ZERO,
      scale: VEC3_ONE,
      rot: Quat::ident(),
      smooth: true,
      mesh_handle,
    };

    let h = self.instances.insert(i);
    self.instance_keys.push(h);
    h
  }

  /// Create an instance of a mesh with given name, using the material transformed into the world
  pub fn add_instance_trans(&mut self, mesh_handle: MeshHandle, material_handle: MaterialHandle, pos: Vec3, rot: Vec3, scale: Vec3) -> InstanceHandle {
    let mut i = Instance {
      material_handle,
      pos: VEC3_ZERO,
      scale: VEC3_ONE,
      rot: Quat::ident(),
      smooth: true,
      mesh_handle,
    };
    i.scale = scale;
    i.pos = pos;
    i.rot = Quat::ident();
    i.rot.rot_x(rot.x);
    i.rot.rot_y(rot.y);
    i.rot.rot_z(rot.z);

    let h = self.instances.insert(i);
    self.instance_keys.push(h);
    h
  }

  pub fn instance_mut(&mut self, h: InstanceHandle) -> &mut Instance {
    self.instances.get_mut(h).expect("instance not found")
  }

  pub fn instance(&self, h: InstanceHandle) -> &Instance {
    self.instances.get(h).expect("instance not found")
  }

  pub fn remove_instance(&mut self, h: InstanceHandle) {
    self.instances.remove(h);
    self.instance_keys.retain(|&k| k != h);
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
}
