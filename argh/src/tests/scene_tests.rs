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
  let (r, g, b) = (s.ambient_light.r(), s.ambient_light.g(), s.ambient_light.b());
  assert!(r < 0.05 && g < 0.05 && b < 0.05, "ambient should be very dim");
}

// --- Lights ---

#[test]
fn test_add_light_returns_resolvable_handle() {
  let mut s = Scene::new();
  let h = s.add_light(Light::default());
  assert!(s.lights.contains_key(h));
}

#[test]
fn test_add_light_tracked_in_light_keys() {
  let mut s = Scene::new();
  let h = s.add_light(Light::default());
  assert!(s.light_keys.contains(&h));
}

#[test]
fn test_add_multiple_lights_independent_handles() {
  let mut s = Scene::new();
  let h1 = s.add_light(Light::default());
  let h2 = s.add_light(Light::default());
  assert_ne!(h1, h2);
  assert_eq!(s.lights.len(), 2);
}

#[test]
fn test_remove_light_drops_from_slotmap() {
  let mut s = Scene::new();
  let h = s.add_light(Light::default());
  s.remove_light(h);
  assert!(!s.lights.contains_key(h));
  assert!(!s.light_keys.contains(&h));
}

#[test]
#[should_panic(expected = "light not found")]
fn test_light_panics_after_remove() {
  let mut s = Scene::new();
  let h = s.add_light(Light::default());
  s.remove_light(h);
  let _ = s.light(h);
}

#[test]
fn test_light_mut_mutations_visible() {
  let mut s = Scene::new();
  let h = s.add_light(Light::default());
  s.light_mut(h).brightness = 0.5;
  assert!((s.light(h).brightness - 0.5).abs() < 1e-5);
}

#[test]
fn test_light_returns_constructor_values() {
  let mut s = Scene::new();
  let l = Light::new_static(Vec3::new(1.0, 2.0, 3.0), 0.75, RED, 0.1, 0.2);
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
  assert_eq!(s.baked_meshes[0].positions.len(), 3);
}

#[test]
fn test_add_static_translates_verts() {
  let mut e = engine();
  let mh = e.add_model(triangle_model("tri"));
  let mut s = Scene::new();
  let offset = Vec3::new(10.0, 20.0, 30.0);
  s.add_static(&e, mh, offset, V3_ZERO, V3_ONE);
  // Original first vert was 0,0,0; after translation must equal `offset`.
  let v0 = s.baked_meshes[0].positions[0];
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
  s.add_light(Light::new_static(Vec3::new(0.0, 0.0, 5.0), 1.0, WHITE, 0.0, 0.0));
  s.add_static(&e, mh, V3_ZERO, V3_ZERO, V3_ONE);
  s.bake_static_lighting();
  let baked = &s.baked_meshes[0];
  assert_eq!(baked.lighting.len(), baked.positions.len());
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
  s.add_light(Light::default());
  s.add_light(Light::default());

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
  let (r, g, b) = (s.ambient_light.r(), s.ambient_light.g(), s.ambient_light.b());
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

// --- Helpers for multi-mesh testing ---

fn two_mesh_model(name: &str) -> Model {
  let mut m = Model::new(name);
  let mut a = Mesh::new();
  a.positions = vec![Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)];
  a.normals = vec![Vec3::new(0.0, 0.0, 1.0); 3];
  a.tex_coords = vec![Vec2::new(0.0, 0.0); 3];
  a.indices = vec![0, 1, 2];
  a.tri_count = 1;
  m.add_mesh(a);
  let mut b = Mesh::new();
  b.positions = vec![Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 1.0)];
  b.normals = vec![Vec3::new(1.0, 0.0, 0.0); 3];
  b.tex_coords = vec![Vec2::new(0.5, 0.5); 3];
  b.indices = vec![0, 1, 2];
  b.tri_count = 1;
  m.add_mesh(b);
  m
}

// --- Light removal & accessor contracts ---

#[test]
fn test_remove_light_unknown_handle_is_silent_no_panic() {
  let mut s = Scene::new();
  let h_real = s.add_light(Light::default());
  let h_stale = s.add_light(Light::default());
  s.remove_light(h_stale);
  // Removing the same stale handle a second time must be a silent no-op.
  s.remove_light(h_stale);
  assert_eq!(s.lights.len(), 1);
  assert!(s.lights.contains_key(h_real));
}

#[test]
#[should_panic(expected = "light not found")]
fn test_light_mut_panics_after_remove() {
  let mut s = Scene::new();
  let h = s.add_light(Light::default());
  s.remove_light(h);
  let _ = s.light_mut(h);
}

#[test]
fn test_remove_light_preserves_other_lights_state() {
  let mut s = Scene::new();
  let h_keep = s.add_light(Light::new_static(Vec3::new(1.0, 2.0, 3.0), 0.42, RED, 0.0, 0.0));
  let h_drop = s.add_light(Light::default());
  s.remove_light(h_drop);
  let l = s.light(h_keep);
  assert_eq!(l.pos, Vec3::new(1.0, 2.0, 3.0));
  assert!((l.brightness - 0.42).abs() < 1e-5);
}

// --- Instance removal & accessor contracts ---

#[test]
fn test_remove_instance_unknown_handle_is_silent_no_panic() {
  let mut e = engine();
  let mh = e.add_model(triangle_model("tri"));
  let mut s = Scene::new();
  let h_real = s.add_instance(mh);
  let h_stale = s.add_instance(mh);
  s.remove_instance(h_stale);
  s.remove_instance(h_stale); // double-remove must not panic
  assert_eq!(s.instances.len(), 1);
  assert_eq!(s.instance(h_real).handle(), h_real);
}

#[test]
#[should_panic]
fn test_instance_mut_panics_after_remove() {
  let mut e = engine();
  let mh = e.add_model(triangle_model("tri"));
  let mut s = Scene::new();
  let h = s.add_instance(mh);
  s.remove_instance(h);
  let _ = s.instance_mut(h);
}

#[test]
fn test_remove_instance_preserves_other_instance_state() {
  let mut e = engine();
  let mh = e.add_model(triangle_model("tri"));
  let mut s = Scene::new();
  let h_keep = s.add_instance(mh);
  s.instance_mut(h_keep).pos_xyz(7.0, 8.0, 9.0).scale(3.0);
  let h_drop = s.add_instance(mh);
  s.remove_instance(h_drop);
  let i = s.instance(h_keep);
  assert_eq!(i.pos, Vec3::new(7.0, 8.0, 9.0));
  assert_eq!(i.scale, Vec3::new(3.0, 3.0, 3.0));
}

#[test]
fn test_add_instance_mut_fluent_chain_persists_in_scene() {
  // The mutable reference returned by add_instance_mut must support a full builder
  // chain and the resulting state must be visible via a subsequent lookup.
  let mut e = engine();
  let mh = e.add_model(triangle_model("tri"));
  let mut s = Scene::new();
  let h = s.add_instance_mut(mh).pos_xyz(1.0, 2.0, 3.0).scale(5.0).handle();
  let i = s.instance(h);
  assert_eq!(i.pos, Vec3::new(1.0, 2.0, 3.0));
  assert_eq!(i.scale, Vec3::new(5.0, 5.0, 5.0));
}

// --- add_static deep semantics ---

#[test]
fn test_add_static_multi_mesh_model_appends_one_baked_per_mesh() {
  let mut e = engine();
  let mh = e.add_model(two_mesh_model("two"));
  let mut s = Scene::new();
  s.add_static(&e, mh, V3_ZERO, V3_ZERO, V3_ONE);
  // Each Mesh inside the Model becomes its own BakedMesh.
  assert_eq!(s.baked_meshes.len(), 2);
  assert_eq!(s.baked_meshes[0].positions.len(), 3);
  assert_eq!(s.baked_meshes[1].positions.len(), 3);
}

#[test]
fn test_add_static_scale_applied_to_verts() {
  let mut e = engine();
  let mh = e.add_model(triangle_model("tri"));
  let mut s = Scene::new();
  s.add_static(&e, mh, V3_ZERO, V3_ZERO, Vec3::new(5.0, 1.0, 1.0));
  // The original (1,0,0) vert must be scaled to (5,0,0) in world space.
  let v1 = s.baked_meshes[0].positions[1];
  assert!((v1.x - 5.0).abs() < 1e-4, "expected x=5, got {}", v1.x);
  assert!(v1.y.abs() < 1e-4);
  assert!(v1.z.abs() < 1e-4);
}

#[test]
fn test_add_static_rotation_applied_to_verts() {
  // 90 degrees around Y rotates (1,0,0) onto the XZ plane (X→0).
  let mut e = engine();
  let mh = e.add_model(triangle_model("tri"));
  let mut s = Scene::new();
  s.add_static(&e, mh, V3_ZERO, Vec3::new(0.0, std::f32::consts::FRAC_PI_2, 0.0), V3_ONE);
  let v1 = s.baked_meshes[0].positions[1];
  assert!(v1.x.abs() < 1e-4, "expected x≈0 after Y rotation, got {}", v1.x);
  let len = (v1.x * v1.x + v1.z * v1.z).sqrt();
  assert!((len - 1.0).abs() < 1e-4);
}

#[test]
fn test_add_static_normals_remain_unit_length() {
  // The inverse-transpose path used for normals must still produce unit vectors
  // under a non-uniform scale.
  let mut e = engine();
  let mh = e.add_model(triangle_model("tri"));
  let mut s = Scene::new();
  s.add_static(&e, mh, V3_ZERO, V3_ZERO, Vec3::new(2.0, 1.0, 0.5));
  for n in &s.baked_meshes[0].normals {
    let len = (n.x * n.x + n.y * n.y + n.z * n.z).sqrt();
    assert!((len - 1.0).abs() < 1e-4, "normal not unit length: {}", len);
  }
}

#[test]
fn test_add_static_copies_indices_and_uvs() {
  let mut e = engine();
  let mh = e.add_model(quad_model("quad"));
  let mut s = Scene::new();
  s.add_static(&e, mh, V3_ZERO, V3_ZERO, V3_ONE);
  // Indices and UV counts must be preserved verbatim from the source mesh.
  assert_eq!(s.baked_meshes[0].indices, vec![0, 1, 2, 0, 2, 3]);
  assert_eq!(s.baked_meshes[0].tex_coords.len(), 4);
}

#[test]
fn test_multiple_add_static_calls_accumulate() {
  let mut e = engine();
  let mh = e.add_model(triangle_model("tri"));
  let mut s = Scene::new();
  s.add_static(&e, mh, V3_ZERO, V3_ZERO, V3_ONE);
  s.add_static(&e, mh, Vec3::new(10.0, 0.0, 0.0), V3_ZERO, V3_ONE);
  s.add_static(&e, mh, Vec3::new(0.0, 10.0, 0.0), V3_ZERO, V3_ONE);
  assert_eq!(s.baked_meshes.len(), 3);
}

// --- bake_static_lighting deep semantics ---

#[test]
fn test_bake_static_lighting_with_no_lights_still_populates_with_ambient() {
  // Even with zero lights, baking must produce a per-vert colour entry; the
  // ambient component alone (>0) must show up.
  let mut e = engine();
  let mh = e.add_model(triangle_model("tri"));
  let mut s = Scene::new();
  s.ambient_light = WHITE;
  s.add_static(&e, mh, V3_ZERO, V3_ZERO, V3_ONE);
  s.bake_static_lighting();
  let baked = &s.baked_meshes[0];
  assert_eq!(baked.lighting.len(), baked.positions.len());
  let (r, g, b) = (baked.lighting[0].r(), baked.lighting[0].g(), baked.lighting[0].b());
  assert!(r > 0.0 || g > 0.0 || b > 0.0, "ambient should contribute, got {} {} {}", r, g, b);
}

#[test]
fn test_bake_static_lighting_skips_non_static_lights() {
  // A non-static light must NOT contribute to the bake; the result with only
  // a dynamic light should match an unlit (ambient-only) bake.
  let mut e = engine();
  let mh = e.add_model(triangle_model("tri"));

  let mut s_ambient = Scene::new();
  s_ambient.ambient_light = WHITE;
  s_ambient.add_static(&e, mh, V3_ZERO, V3_ZERO, V3_ONE);
  s_ambient.bake_static_lighting();

  let mut s_dynonly = Scene::new();
  s_dynonly.ambient_light = WHITE;
  s_dynonly.add_light(Light::new_dynamic(Vec3::new(0.0, 0.0, 5.0), 1.0, WHITE, 0.0, 0.0));
  s_dynonly.add_static(&e, mh, V3_ZERO, V3_ZERO, V3_ONE);
  s_dynonly.bake_static_lighting();

  let (ra, ga, ba) = (
    s_ambient.baked_meshes[0].lighting[0].r(),
    s_ambient.baked_meshes[0].lighting[0].g(),
    s_ambient.baked_meshes[0].lighting[0].b(),
  );
  let (rd, gd, bd) = (
    s_dynonly.baked_meshes[0].lighting[0].r(),
    s_dynonly.baked_meshes[0].lighting[0].g(),
    s_dynonly.baked_meshes[0].lighting[0].b(),
  );
  assert!((ra - rd).abs() < 1e-5, "non-static light leaked into bake (r): {} vs {}", ra, rd);
  assert!((ga - gd).abs() < 1e-5);
  assert!((ba - bd).abs() < 1e-5);
}

#[test]
fn test_bake_static_lighting_applies_to_all_baked_meshes() {
  // Every BakedMesh in the scene must get its baked_lighting populated.
  let mut e = engine();
  let mh = e.add_model(triangle_model("tri"));
  let mut s = Scene::new();
  s.ambient_light = WHITE;
  for _ in 0..3 {
    s.add_static(&e, mh, V3_ZERO, V3_ZERO, V3_ONE);
  }
  s.bake_static_lighting();
  for bm in &s.baked_meshes {
    assert_eq!(bm.lighting.len(), bm.positions.len());
  }
}

#[test]
fn test_bake_static_lighting_is_idempotent() {
  // Calling bake twice in a row should produce identical results; the second
  // call must not accumulate or duplicate the per-vert lighting.
  let mut e = engine();
  let mh = e.add_model(triangle_model("tri"));
  let mut s = Scene::new();
  s.ambient_light = WHITE;
  s.add_light(Light::new_static(Vec3::new(0.0, 0.0, 5.0), 1.0, WHITE, 0.0, 0.0));
  s.add_static(&e, mh, V3_ZERO, V3_ZERO, V3_ONE);
  s.bake_static_lighting();
  let first: Vec<_> = s.baked_meshes[0].lighting.iter().map(|c| (c.r(), c.g(), c.b())).collect();
  s.bake_static_lighting();
  let second: Vec<_> = s.baked_meshes[0].lighting.iter().map(|c| (c.r(), c.g(), c.b())).collect();
  assert_eq!(first.len(), second.len());
  for (a, b) in first.iter().zip(&second) {
    assert!((a.0 - b.0).abs() < 1e-5);
    assert!((a.1 - b.1).abs() < 1e-5);
    assert!((a.2 - b.2).abs() < 1e-5);
  }
}

// --- Mixed stats ---

#[test]
fn test_stats_counts_instance_and_static_tris_together() {
  let mut e = engine();
  let tri = e.add_model(triangle_model("tri"));
  let quad = e.add_model(quad_model("quad"));
  let mut s = Scene::new();
  s.add_instance(tri); // 1 dynamic tri
  s.add_instance(quad); // 2 dynamic tris
  s.add_static(&e, quad, V3_ZERO, V3_ZERO, V3_ONE); // 2 static tris
  let (instances, baked, _, tris) = s.stats(&e);
  assert_eq!(instances, 2);
  assert_eq!(baked, 1);
  assert_eq!(tris, 5);
}
