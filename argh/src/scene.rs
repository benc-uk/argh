// ==============================================================================================
// Module & file:   scene.rs
// Purpose:         Scene holds instances, lights etc to be rendered on demand
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use slotmap::SlotMap;

use crate::{
  colour::Colour,
  engine::{InstanceHandle, LightHandle, MaterialHandle, MeshHandle},
  light::Light,
  math::{Quat, V3_ONE, V3_ZERO, Vec3},
  models::Instance,
};

/// Scene holds instances, lights etc to be rendered on demand
pub struct Scene {
  // Things tracked & cached by the engine
  pub(super) lights: SlotMap<LightHandle, Light>,
  pub(super) instances: SlotMap<InstanceHandle, Instance>,
  // Used to speed up looping, minimise copies in the render loops
  pub(super) instance_keys: Vec<InstanceHandle>,
  pub(super) light_keys: Vec<LightHandle>,

  /// Ambient light colour, defaults to [0.1, 0.1, 0.1], beware setting this too high it will look washed out
  pub ambient_light: Colour,
}

impl Scene {
  /// Create an empty scene
  pub fn new() -> Self {
    Self {
      lights: SlotMap::with_key(),
      instances: SlotMap::with_key(),
      instance_keys: vec![],
      light_keys: vec![],

      ambient_light: Colour::new(0.1, 0.1, 0.1),
    }
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
      pos: V3_ZERO,
      scale: V3_ONE,
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
      pos,
      scale,
      rot: Quat::ident(),
      smooth: true,
      mesh_handle,
    };

    i.rot.rot_x(rot.x);
    i.rot.rot_y(rot.y);
    i.rot.rot_z(rot.z);

    let h = self.instances.insert(i);
    self.instance_keys.push(h);
    h
  }

  pub fn instance_mut(&mut self, h: InstanceHandle) -> &mut Instance {
    debug_assert!(self.instances.contains_key(h));

    self.instances.get_mut(h).expect("instance not found")
  }

  pub fn instance(&self, h: InstanceHandle) -> &Instance {
    self.instances.get(h).expect("instance not found")
  }

  pub fn remove_instance(&mut self, h: InstanceHandle) {
    self.instances.remove(h);
    self.instance_keys.retain(|&k| k != h);
  }

  pub fn list_instances(&self) -> impl Iterator<Item = &Instance> {
    self.instances.values()
  }

  pub fn instances_mut(&mut self) -> impl Iterator<Item = &mut Instance> {
    self.instances.values_mut()
  }
}

/// Default to stop linter complaining
impl Default for Scene {
  fn default() -> Self {
    Self::new()
  }
}
