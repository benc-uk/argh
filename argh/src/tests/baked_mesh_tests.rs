// ==============================================================================================
// Module & file:   baked_mesh_tests.rs
// Purpose:         Tests for the internal BakedMesh type and static lighting bake.
// Author & Date:   AI generated, 2026
// License:         MIT
// Notes:           See misc/testing-spec.md for the house style and conventions.
// ==============================================================================================

use super::*;
use crate::colour::WHITE;
use crate::engine::LightHandle;
use crate::light::Light;
use crate::material::MATERIAL_PLACEHOLDER;
use slotmap::SlotMap;

// --- Helpers ---

fn empty_lights() -> SlotMap<LightHandle, Light> {
  SlotMap::with_key()
}

fn lights_with(light: Light) -> SlotMap<LightHandle, Light> {
  let mut sm: SlotMap<LightHandle, Light> = SlotMap::with_key();
  sm.insert(light);
  sm
}

fn small_baked() -> BakedMesh {
  BakedMesh {
    material: MATERIAL_PLACEHOLDER.clone(),
    positions: vec![Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)],
    normals: vec![Vec3::new(0.0, 0.0, 1.0); 3],
    tex_coords: vec![Vec2::new(0.0, 0.0); 3],
    indices: vec![0, 1, 2],
    lighting: vec![],
  }
}

// --- bake_lighting basics ---

#[test]
fn test_bake_lighting_populates_per_vert_entries() {
  let mut bm = small_baked();
  let lights = lights_with(Light::new_static(Vec3::new(0.0, 0.0, 5.0), 1.0, WHITE, 0.0, 0.0));
  bm.bake_lighting(&lights, Colour::new(0.1, 0.1, 0.1));
  assert_eq!(bm.lighting.len(), bm.positions.len());
}

#[test]
fn test_bake_lighting_with_no_lights_uses_ambient_only() {
  let mut bm = small_baked();
  let lights = empty_lights();
  let ambient = Colour::new(0.2, 0.2, 0.2);
  bm.bake_lighting(&lights, ambient);
  for c in &bm.lighting {
    let (r, g, b) = (c.r(), c.g(), c.b());
    // ambient * material.diffuse where diffuse is white = ambient itself.
    assert!((r - 0.2).abs() < 1e-5);
    assert!((g - 0.2).abs() < 1e-5);
    assert!((b - 0.2).abs() < 1e-5);
  }
}

#[test]
fn test_bake_lighting_clears_previous_results() {
  let mut bm = small_baked();
  bm.lighting = vec![Colour::new(99.0, 99.0, 99.0); 999]; // garbage
  let lights = empty_lights();
  bm.bake_lighting(&lights, Colour::new(0.0, 0.0, 0.0));
  assert_eq!(bm.lighting.len(), bm.positions.len());
}

// --- is_static filtering ---

#[test]
fn test_bake_lighting_ignores_non_static_lights() {
  let mut bm = small_baked();
  // is_static=false: should be skipped.
  let lights = lights_with(Light::new_dynamic(Vec3::new(0.0, 0.0, 5.0), 1.0, WHITE, 0.0, 0.0));
  bm.bake_lighting(&lights, Colour::new(0.0, 0.0, 0.0));
  for c in &bm.lighting {
    let (r, g, b) = (c.r(), c.g(), c.b());
    assert!(r.abs() < 1e-5 && g.abs() < 1e-5 && b.abs() < 1e-5);
  }
}

#[test]
fn test_bake_lighting_includes_static_lights() {
  let mut bm = small_baked();
  let lights = lights_with(Light::new_static(Vec3::new(0.0, 0.0, 5.0), 1.0, WHITE, 0.0, 0.0));
  bm.bake_lighting(&lights, Colour::new(0.0, 0.0, 0.0));
  let (r, _, _) = (bm.lighting[0].r(), bm.lighting[0].g(), bm.lighting[0].b());
  assert!(r > 0.0);
}

// --- Attenuation ---

#[test]
fn test_bake_lighting_attenuates_with_distance() {
  let mut near_bm = small_baked();
  let mut far_bm = small_baked();
  let near_lights = lights_with(Light::new_static(Vec3::new(0.0, 0.0, 1.0), 1.0, WHITE, 0.1, 0.1));
  let far_lights = lights_with(Light::new_static(Vec3::new(0.0, 0.0, 100.0), 1.0, WHITE, 0.1, 0.1));
  near_bm.bake_lighting(&near_lights, Colour::new(0.0, 0.0, 0.0));
  far_bm.bake_lighting(&far_lights, Colour::new(0.0, 0.0, 0.0));
  let near_r = near_bm.lighting[0].r();
  let far_r = far_bm.lighting[0].r();
  assert!(near_r > far_r, "near light should produce brighter bake than far light");
}

// --- Back-facing normal ---

#[test]
fn test_bake_lighting_clamps_back_facing_to_zero() {
  let mut bm = small_baked();
  // Flip the normals to face away from the light.
  bm.normals = vec![Vec3::new(0.0, 0.0, -1.0); 3];
  let lights = lights_with(Light::new_static(Vec3::new(0.0, 0.0, 5.0), 1.0, WHITE, 0.0, 0.0));
  bm.bake_lighting(&lights, Colour::new(0.0, 0.0, 0.0));
  for c in &bm.lighting {
    let (r, g, b) = (c.r(), c.g(), c.b());
    assert!(r.abs() < 1e-5 && g.abs() < 1e-5 && b.abs() < 1e-5);
  }
}

// --- Material diffuse modulates the result ---

#[test]
fn test_bake_lighting_modulates_by_material_diffuse() {
  let mut bm = small_baked();
  bm.material.diffuse = Colour::new(0.5, 0.0, 0.0); // red material
  let lights = empty_lights();
  bm.bake_lighting(&lights, Colour::new(1.0, 1.0, 1.0));
  // ambient * diffuse => (0.5, 0, 0)
  for c in &bm.lighting {
    let (r, g, b) = (c.r(), c.g(), c.b());
    assert!((r - 0.5).abs() < 1e-5);
    assert!(g.abs() < 1e-5);
    assert!(b.abs() < 1e-5);
  }
}
