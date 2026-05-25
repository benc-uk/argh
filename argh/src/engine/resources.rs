// ==============================================================================================
// Module & file:   engine / resources.rs
// Purpose:         Engine operations for resources like meshes, instances & materials
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use crate::{
  light::Light,
  math::{Quat, VEC3_ONE, VEC3_ZERO, Vec3},
  models::{Instance, Material, Mesh},
};

use super::{Engine, InstanceHandle, MaterialHandle, MeshHandle};

impl Engine {
  /// Add a mesh to the cache and give it a name
  pub fn add_mesh(&mut self, mesh: Mesh) -> MeshHandle {
    self.meshes.insert(mesh)
  }

  /// Add a light to the scene, used by 3D rendering
  pub fn add_light(&mut self, light: Light) {
    self.lights.push(light);
  }

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

    self.instances.insert(i)
  }

  /// Create an instance of a mesh with given name, using the material transformed into the world
  pub fn add_instance_trans(&mut self, mesh_handle: MeshHandle, mat_handle: MaterialHandle, pos: Vec3, rot: Vec3, scale: Vec3) -> InstanceHandle {
    let mut i = Instance {
      material_handle: mat_handle,
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

    self.instances.insert(i)
  }

  pub fn instance_mut(&mut self, h: InstanceHandle) -> &mut Instance {
    self.instances.get_mut(h).expect("instance not found")
  }

  pub fn instance(&self, h: InstanceHandle) -> &Instance {
    self.instances.get(h).expect("instance not found")
  }

  pub fn remove_instance(&mut self, h: InstanceHandle) {
    self.instances.remove(h);
  }

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
