// ==============================================================================================
// Module & file:   baked_mesh.rs
// Purpose:         Mesh pre-transformed to world space with baked per-vertex lighting
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use slotmap::SlotMap;

use crate::{
  colour::{BLACK, Colour},
  engine::LightHandle,
  light::Light,
  material::Material,
  math::{Vec2, Vec3},
};

#[cfg(test)]
#[path = "tests/baked_mesh_tests.rs"]
mod baked_mesh_tests;

pub(crate) struct BakedMesh {
  pub(crate) material: Material,
  pub(crate) verts: Vec<Vec3>,   // already in WORLD space
  pub(crate) normals: Vec<Vec3>, // already in WORLD space, normalised
  pub(crate) uvs: Vec<Vec2>,
  pub(crate) indices: Vec<u32>,
  pub(crate) baked_lighting: Vec<Colour>, // Baked lighting
}

impl BakedMesh {
  pub(crate) fn bake_lighting(&mut self, lights: &SlotMap<LightHandle, Light>, ambient: Colour) {
    self.baked_lighting.clear();
    self.baked_lighting.reserve(self.verts.len());

    for (vert, normal) in self.verts.iter().zip(&self.normals) {
      let mut diffuse = BLACK;

      for light in lights.values() {
        // Important, only is_static lights are used for baking
        if !light.is_static {
          continue;
        }

        let l_raw = light.pos - *vert;
        let d = l_raw.len();
        let l = l_raw.normalize_new();
        let atten = 1.0 / (1.0 + light.atten_linear * d + light.atten_quad * d * d);
        let n_dot_l = normal.dot(l).max(0.0);
        diffuse += light.colour * light.brightness * n_dot_l * atten;
      }

      let amb = ambient * self.material.diffuse;
      self.baked_lighting.push(diffuse * self.material.diffuse + amb);
    }
  }
}
