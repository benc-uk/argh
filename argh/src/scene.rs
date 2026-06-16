// ==============================================================================================
// Module & file:   scene.rs
// Purpose:         Scene holds instances, lights etc to be rendered on demand
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use slotmap::SlotMap;

use crate::{
  baked_mesh::BakedMesh,
  colour::Colour,
  engine::{Engine, InstanceHandle, LightHandle, ModelHandle},
  instance::Instance,
  light::Light,
  math::{Mat3, Mat4, Quat, V3_ONE, V3_ZERO, Vec3},
};

#[cfg(test)]
#[path = "tests/scene_tests.rs"]
mod scene_tests;

/// Scene holds [Instances][crate::instance::Instance] and [Lights][crate::light::Light] etc to be rendered on demand in your main loop
pub struct Scene {
  // Things tracked & cached by the engine
  pub(crate) lights: SlotMap<LightHandle, Light>,
  pub(crate) instances: SlotMap<InstanceHandle, Instance>,
  // Used to speed up looping, minimise copies in the render loops
  pub(crate) instance_keys: Vec<InstanceHandle>,
  pub(crate) light_keys: Vec<LightHandle>,

  // Static geometry held in BakedMesh
  pub(crate) baked_meshes: Vec<BakedMesh>,

  /// Ambient light colour applied to all geometry, beware setting this too high it will look washed out
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
      baked_meshes: vec![],
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

  /// Create an instance of a model with this handle, returns the [InstanceHandle]
  pub fn add_instance(&mut self, model_handle: ModelHandle) -> InstanceHandle {
    // We use insert_with_key to get the key as it is being added to the slotmap
    let h = self.instances.insert_with_key(|key| Instance {
      handle: key,
      model_handle,
      pos: V3_ZERO,
      scale: V3_ONE,
      rot: Quat::ident(),
    });

    self.instance_keys.push(h);
    h
  }

  /// Create an instance of a model with this handle, returns a mutable [Instance]
  pub fn add_instance_mut(&mut self, model_handle: ModelHandle) -> &mut Instance {
    let hdl = self.add_instance(model_handle);
    self.instance_mut(hdl)
  }

  /// Create an instance of a model with this handle, returns the [InstanceHandle]. Posed into the world with modifiers:
  /// Arguments:
  /// * `pos` - Holds the X, Y and Z world position of the created instance
  /// * `rot` - Holds Euler rotation angles around the X, Y and Z axis, applied in order; rot-x, rot-y then rot-z
  /// * `scale` - Holds the X, Y and Z scaling factors of the created instance
  pub fn add_instance_posed(&mut self, model_handle: ModelHandle, pos: Vec3, rot: Vec3, scale: Vec3) -> InstanceHandle {
    // We use insert_with_key to get the key as it is being added to the slotmap
    let handle = self.instances.insert_with_key(|key| {
      let rot = Quat::new_euler(rot.x, rot.y, rot.z);

      Instance {
        handle: key,
        model_handle,
        pos,
        scale,
        rot,
      }
    });

    self.instance_keys.push(handle);
    handle
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
  pub fn instances(&self) -> impl Iterator<Item = &Instance> {
    self.instances.values()
  }

  /// A mutable list of instances in the scene
  pub fn instances_mut(&mut self) -> impl Iterator<Item = &mut Instance> {
    self.instances.values_mut()
  }

  // ===== BakedMesh ====================================

  /// Bake static lighting into all BakedMeshes in this scene.
  /// Important: Call once after adding all static geometry and lights, before rendering.
  pub fn bake_static_lighting(&mut self) {
    for sm in &mut self.baked_meshes {
      sm.bake_lighting(&self.lights, self.ambient_light);
    }
  }

  /// Create a static version of a [ModelHandle] will be stored as one or more BakedMesh internally
  pub fn add_static(&mut self, eng: &Engine, model_handle: ModelHandle, pos: Vec3, rot: Vec3, scale: Vec3) {
    let rot_q = Quat::new_euler(rot.x, rot.y, rot.z);

    let m = Mat4::new_scale_rot_trans(scale.x, scale.y, scale.z, rot_q, pos.x, pos.y, pos.z);
    let m_inv_t = Mat3::from_mat4_upper(&m).inverse_transpose().unwrap_or_default();

    let model = eng.model(model_handle);

    // Each Mesh in the Model becomes BakedMesh, internal data is copied
    for mesh in &model.meshes {
      let positions: Vec<Vec3> = mesh.positions.iter().map(|v| m.transform_point(v)).collect();
      let normals: Vec<Vec3> = mesh.normals.iter().map(|n: &Vec3| (m_inv_t * n).normalize_new()).collect();

      let baked = BakedMesh {
        material: mesh.material.clone(),
        positions,
        normals,
        tex_coords: mesh.tex_coords.clone(),
        indices: mesh.indices.clone(),
        lighting: vec![], // populated when scene.bake_static_lighting() is called
      };

      self.baked_meshes.push(baked);
    }
  }

  /// Get number of instances, statics, lights, & total triangles
  pub fn stats(&self, eng: &Engine) -> (usize, usize, usize, u32) {
    let inst_tris: u32 = self
      .instance_keys
      .iter()
      .map(|h| {
        let m = eng.model(self.instances[*h].model_handle);
        m.tri_count
      })
      .sum();

    let static_tris: u32 = self.baked_meshes.iter().map(|sm| sm.indices.len() as u32 / 3).sum();

    (self.instances.len(), self.baked_meshes.len(), self.lights.len(), inst_tris + static_tris)
  }
}

/// Default to stop dumb linter complaining
impl Default for Scene {
  fn default() -> Self {
    Self::new()
  }
}
