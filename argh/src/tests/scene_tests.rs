// ==============================================================================================
// Module & file:   scene_tests.rs
// Purpose:         Tests for the Scene type, lights, instances, statics and stats.
// Author & Date:   AI generated, 2026
// License:         MIT
// Notes:           See misc/testing-spec.md for the house style and conventions.
// ==============================================================================================

use super::*;
use crate::colour::{RED, WHITE};
use crate::engine::Engine;
use crate::light::Light;
use crate::material::Material;
use crate::math::{V3_ONE, V3_ZERO, Vec2, Vec3};
use crate::mesh::Mesh;
use crate::model::Model;

// --- Helpers ---

fn engine() -> Engine {
  Engine::new(64, 64)
}

fn triangle_model(name: &str) -> Model {
  let mut m = Mesh::new();
  m.positions = vec![Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)];
  m.normals = vec![Vec3::new(0.0, 0.0, 1.0); 3];
  m.tex_coords = vec![Vec2::new(0.0, 0.0); 3];
  m.indices = vec![0, 1, 2];
  m.tri_count = 1;
  Model::from_mesh(m, name)
}

fn quad_model(name: &str) -> Model {
  let mut m = Mesh::new();
  m.positions = vec![Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 0.0), Vec3::new(0.0, 1.0, 0.0)];
  m.normals = vec![Vec3::new(0.0, 0.0, 1.0); 4];
  m.tex_coords = vec![Vec2::new(0.0, 0.0); 4];
  m.indices = vec![0, 1, 2, 0, 2, 3];
  m.tri_count = 2;
  Model::from_mesh(m, name)
}

// --- Construction ---

#[test]
fn test_new_scene_is_empty() {
  let s = Scene::new();
  assert_eq!(s.lights.len(), 0);
  assert_eq!(s.instances.len(), 0);
  assert!(s.instance_keys.is_empty());
  assert!(s.light_keys.is_empty());
  assert!(s.baked_meshes.is_empty());
}

#[test]
fn test_default_matches_new() {
  let s_new = Scene::new();
  let s_def: Scene = Scene::default();
  assert_eq!(s_new.lights.len(), s_def.lights.len());
  assert_eq!(s_new.instances.len(), s_def.instances.len());
}

#[test]
fn test_new_scene_has_dim_ambient_light() {
  let s = Scene::new();
  let (r, g, b) = s.ambient_light.channels();
  assert!(r < 0.05 && g < 0.05 && b < 0.05, "ambient should be very dim");
}

// --- Lights ---

#[test]
fn test_add_light_returns_resolvable_handle() {
  let mut s = Scene::new();
  let h = s.add_light(Light::new_default());
  assert!(s.lights.contains_key(h));
}

#[test]
fn test_add_light_tracked_in_light_keys() {
  let mut s = Scene::new();
  let h = s.add_light(Light::new_default());
  assert!(s.light_keys.contains(&h));
}

#[test]
fn test_add_multiple_lights_independent_handles() {
  let mut s = Scene::new();
  let h1 = s.add_light(Light::new_default());
  let h2 = s.add_light(Light::new_default());
  assert_ne!(h1, h2);
  assert_eq!(s.lights.len(), 2);
}

#[test]
fn test_remove_light_drops_from_slotmap() {
  let mut s = Scene::new();
  let h = s.add_light(Light::new_default());
  s.remove_light(h);
  assert!(!s.lights.contains_key(h));
  assert!(!s.light_keys.contains(&h));
}

#[test]
#[should_panic(expected = "light not found")]
fn test_light_panics_after_remove() {
  let mut s = Scene::new();
  let h = s.add_light(Light::new_default());
  s.remove_light(h);
  let _ = s.light(h);
}

#[test]
fn test_light_mut_mutations_visible() {
  let mut s = Scene::new();
  let h = s.add_light(Light::new_default());
  s.light_mut(h).brightness = 0.5;
  assert!((s.light(h).brightness - 0.5).abs() < 1e-5);
}

#[test]
fn test_light_returns_constructor_values() {
  let mut s = Scene::new();
  let l = Light::new(Vec3::new(1.0, 2.0, 3.0), 0.75, RED, 0.1, 0.2, true, false);
  let h = s.add_light(l);
  let read = s.light(h);
  assert_eq!(read.pos, Vec3::new(1.0, 2.0, 3.0));
  assert!((read.brightness - 0.75).abs() < 1e-5);
}

// --- Instances ---

#[test]
fn test_add_instance_defaults() {
  let mut e = engine();
  let mh = e.add_model(triangle_model("tri"));
  let mut s = Scene::new();
  let ih = s.add_instance(mh);
  let inst = s.instance(ih);
  assert_eq!(inst.pos, V3_ZERO);
  assert_eq!(inst.scale, V3_ONE);
  assert!(inst.smooth);
}

#[test]
fn test_add_instance_tracked_in_instance_keys() {
  let mut e = engine();
  let mh = e.add_model(triangle_model("tri"));
  let mut s = Scene::new();
  let ih = s.add_instance(mh);
  assert!(s.instance_keys.contains(&ih));
}

#[test]
fn test_add_instance_mut_returns_same_handle() {
  let mut e = engine();
  let mh = e.add_model(triangle_model("tri"));
  let mut s = Scene::new();
  let inst = s.add_instance_mut(mh);
  let h_via_mut = inst.handle();
  assert!(s.instance_keys.contains(&h_via_mut));
}

#[test]
fn test_add_instance_world_sets_pos_and_scale() {
  let mut e = engine();
  let mh = e.add_model(triangle_model("tri"));
  let mut s = Scene::new();
  let pos = Vec3::new(1.0, 2.0, 3.0);
  let scale = Vec3::new(2.0, 3.0, 4.0);
  let ih = s.add_instance_posed(mh, pos, Vec3::new(0.0, 0.0, 0.0), scale);
  let inst = s.instance(ih);
  assert_eq!(inst.pos, pos);
  assert_eq!(inst.scale, scale);
}

#[test]
fn test_remove_instance_drops_from_slotmap_and_keys() {
  let mut e = engine();
  let mh = e.add_model(triangle_model("tri"));
  let mut s = Scene::new();
  let ih = s.add_instance(mh);
  s.remove_instance(ih);
  assert!(!s.instances.contains_key(ih));
  assert!(!s.instance_keys.contains(&ih));
}

#[test]
#[should_panic(expected = "instance not found")]
fn test_instance_panics_after_remove() {
  let mut e = engine();
  let mh = e.add_model(triangle_model("tri"));
  let mut s = Scene::new();
  let ih = s.add_instance(mh);
  s.remove_instance(ih);
  let _ = s.instance(ih);
}

#[test]
fn test_instances_iterator_yields_expected_count() {
  let mut e = engine();
  let mh = e.add_model(triangle_model("tri"));
  let mut s = Scene::new();
  for _ in 0..7 {
    s.add_instance(mh);
  }
  assert_eq!(s.instances().count(), 7);
}

#[test]
fn test_instances_mut_iterator_allows_mutation() {
  let mut e = engine();
  let mh = e.add_model(triangle_model("tri"));
  let mut s = Scene::new();
  for _ in 0..3 {
    s.add_instance(mh);
  }
  for inst in s.instances_mut() {
    inst.pos = Vec3::new(9.0, 0.0, 0.0);
  }
  for inst in s.instances() {
    assert_eq!(inst.pos, Vec3::new(9.0, 0.0, 0.0));
  }
}

// --- add_static + bake_static_lighting ---

#[test]
fn test_add_static_appends_baked_meshes_per_mesh() {
  let mut e = engine();
  let mh = e.add_model(triangle_model("tri"));
  let mut s = Scene::new();
  s.add_static(&e, mh, V3_ZERO, V3_ZERO, V3_ONE);
  assert_eq!(s.baked_meshes.len(), 1);
  assert_eq!(s.baked_meshes[0].verts.len(), 3);
}

#[test]
fn test_add_static_translates_verts() {
  let mut e = engine();
  let mh = e.add_model(triangle_model("tri"));
  let mut s = Scene::new();
  let offset = Vec3::new(10.0, 20.0, 30.0);
  s.add_static(&e, mh, offset, V3_ZERO, V3_ONE);
  // Original first vert was 0,0,0; after translation must equal `offset`.
  let v0 = s.baked_meshes[0].verts[0];
  assert!((v0.x - 10.0).abs() < 1e-4);
  assert!((v0.y - 20.0).abs() < 1e-4);
  assert!((v0.z - 30.0).abs() < 1e-4);
}

#[test]
fn test_bake_static_lighting_populates_per_vert_colours() {
  let mut e = engine();
  // Use a model with a +Z normal so a light at +Z lights it up.
  let mh = e.add_model(triangle_model("tri"));
  let mut s = Scene::new();
  // Static-only light (is_static=true).
  s.add_light(Light::new(Vec3::new(0.0, 0.0, 5.0), 1.0, WHITE, 0.0, 0.0, true, false));
  s.add_static(&e, mh, V3_ZERO, V3_ZERO, V3_ONE);
  s.bake_static_lighting();
  let baked = &s.baked_meshes[0];
  assert_eq!(baked.baked_lighting.len(), baked.verts.len());
}

// --- stats ---

#[test]
fn test_stats_empty_scene_zero() {
  let e = engine();
  let s = Scene::new();
  let (inst, baked, lights, tris) = s.stats(&e);
  assert_eq!(inst, 0);
  assert_eq!(baked, 0);
  assert_eq!(lights, 0);
  assert_eq!(tris, 0);
}

#[test]
fn test_stats_counts_instances_lights_tris() {
  let mut e = engine();
  let tri = e.add_model(triangle_model("tri"));
  let quad = e.add_model(quad_model("quad"));
  let mut s = Scene::new();
  s.add_instance(tri); // 1 tri
  s.add_instance(quad); // 2 tris
  s.add_instance(quad); // 2 tris
  s.add_light(Light::new_default());
  s.add_light(Light::new_default());

  let (instances, baked, lights, tris) = s.stats(&e);
  assert_eq!(instances, 3);
  assert_eq!(baked, 0);
  assert_eq!(lights, 2);
  assert_eq!(tris, 5);
}

#[test]
fn test_stats_includes_static_triangle_count() {
  let mut e = engine();
  let quad = e.add_model(quad_model("quad"));
  let mut s = Scene::new();
  s.add_static(&e, quad, V3_ZERO, V3_ZERO, V3_ONE);
  let (_, baked, _, tris) = s.stats(&e);
  assert_eq!(baked, 1);
  assert_eq!(tris, 2);
}

#[test]
fn test_stats_removing_instance_drops_its_tris() {
  let mut e = engine();
  let quad = e.add_model(quad_model("quad"));
  let mut s = Scene::new();
  let h = s.add_instance(quad);
  s.add_instance(quad);
  let (_, _, _, before) = s.stats(&e);
  s.remove_instance(h);
  let (_, _, _, after) = s.stats(&e);
  assert_eq!(before, 4);
  assert_eq!(after, 2);
}

// --- ambient_light field ---

#[test]
fn test_ambient_light_field_writable() {
  let mut s = Scene::new();
  s.ambient_light = RED;
  let (r, g, b) = s.ambient_light.channels();
  assert!((r - 1.0).abs() < 1e-5);
  assert!(g.abs() < 1e-5);
  assert!(b.abs() < 1e-5);
}

// --- Instance::handle() round-trip ---

#[test]
fn test_instance_handle_matches_returned_handle() {
  let mut e = engine();
  let mh = e.add_model(triangle_model("tri"));
  let mut s = Scene::new();
  let h = s.add_instance(mh);
  assert_eq!(s.instance(h).handle(), h);
}

// --- Use _ = material to silence unused-import-style false positives ---
#[test]
fn test_unused_imports_compile() {
  let _m = Material::new_flat(WHITE);
}
