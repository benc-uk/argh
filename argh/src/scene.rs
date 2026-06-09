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
  core::Instance,
  engine::{InstanceHandle, LightHandle, ModelHandle},
  light::Light,
  math::{Quat, V3_ONE, V3_ZERO, Vec3},
};

/// Scene holds instances, lights etc to be rendered on demand
pub struct Scene {
  // Things tracked & cached by the engine
  pub(super) lights: SlotMap<LightHandle, Light>,
  pub(super) instances: SlotMap<InstanceHandle, Instance>,
  // Used to speed up looping, minimise copies in the render loops
  pub(super) instance_keys: Vec<InstanceHandle>,
  pub(super) light_keys: Vec<LightHandle>,

  // Static geometry held in chunks
  // pub(super) chunks: Vec<Chunk>,
  /// Ambient light colour, beware setting this too high it will look washed out
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
      //chunks: vec![],
      ambient_light: Colour::new(0.008, 0.008, 0.008),
    }
  }

  // ===== Lights ======================================================================================================

  /// Add a light to the scene
  pub fn add_light(&mut self, light: Light) -> LightHandle {
    let h = self.lights.insert(light);
    self.light_keys.push(h);
    h
  }

  /// Remove a light from the scene
  pub fn remove_light(&mut self, h: LightHandle) {
    self.lights.remove(h);
    self.light_keys.retain(|&k| k != h);
  }

  /// Get a mutable light from it's handle
  pub fn light_mut(&mut self, h: LightHandle) -> &mut Light {
    self.lights.get_mut(h).expect("light not found")
  }

  /// Get a mutable light from it's handle
  pub fn light(&self, h: LightHandle) -> &Light {
    self.lights.get(h).expect("light not found")
  }

  // ===== Instances ======================================================================================================

  /// Create an instance of a mesh with given name, using the material
  pub fn add_instance(&mut self, model_handle: ModelHandle) -> InstanceHandle {
    let i = Instance {
      model_handle,
      pos: V3_ZERO,
      scale: V3_ONE,
      rot: Quat::ident(),
      smooth: true,
    };

    let h = self.instances.insert(i);
    self.instance_keys.push(h);
    h
  }

  /// Create an instance of a mesh with given name, using the material transformed into the world
  pub fn add_instance_trans(&mut self, model_handle: ModelHandle, pos: Vec3, rot: Vec3, scale: Vec3) -> InstanceHandle {
    let mut i = Instance {
      model_handle,
      pos,
      scale,
      rot: Quat::ident(),
      smooth: true,
    };

    i.rot.rot_x(rot.x);
    i.rot.rot_y(rot.y);
    i.rot.rot_z(rot.z);

    let h = self.instances.insert(i);
    self.instance_keys.push(h);
    h
  }

  /// Get an mutable instance from its handle
  pub fn instance_mut(&mut self, h: InstanceHandle) -> &mut Instance {
    debug_assert!(self.instances.contains_key(h));

    self.instances.get_mut(h).expect("instance not found")
  }

  /// Get an instance from it's handle
  pub fn instance(&self, h: InstanceHandle) -> &Instance {
    self.instances.get(h).expect("instance not found")
  }

  /// Remove an instance from a scene
  pub fn remove_instance(&mut self, h: InstanceHandle) {
    self.instances.remove(h);
    self.instance_keys.retain(|&k| k != h);
  }

  /// A list of instances in the scene
  pub fn list_instances(&self) -> impl Iterator<Item = &Instance> {
    self.instances.values()
  }

  /// A mutable list of instances in the scene
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
