// ==============================================================================================
// Module & file:   model_tests.rs
// Purpose:         Tests for the Model type and mesh aggregation.
// Author & Date:   AI generated, 2026
// License:         MIT
// Notes:           See misc/testing-spec.md for the house style and conventions.
// ==============================================================================================

use super::*;
use crate::colour::{BLUE, RED};
use crate::material::{MATERIAL_PLACEHOLDER, Material};
use crate::math::{Vec2, Vec3};
use crate::mesh::Mesh;

// --- Helpers ---

fn build_triangle_mesh() -> Mesh {
  let mut m = Mesh::new();
  m.positions = vec![Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)];
  m.normals = vec![Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 0.0, 1.0)];
  m.tex_coords = vec![Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0)];
  m.indices = vec![0, 1, 2];
  m.tri_count = 1;
  m
}

fn build_quad_mesh() -> Mesh {
  let mut m = Mesh::new();
  m.positions = vec![Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 0.0), Vec3::new(0.0, 1.0, 0.0)];
  m.normals = vec![Vec3::new(0.0, 0.0, 1.0); 4];
  m.tex_coords = vec![Vec2::new(0.0, 0.0); 4];
  m.indices = vec![0, 1, 2, 0, 2, 3];
  m.tri_count = 2;
  m
}

// --- Construction ---

#[test]
fn test_new_empty_model_has_no_meshes() {
  let model = Model::new("empty");
  assert!(model.meshes.is_empty());
  assert_eq!(model.tri_count, 0);
}

#[test]
fn test_new_stores_name() {
  let model = Model::new("teapot");
  assert_eq!(model.name(), "teapot");
}

#[test]
fn test_from_mesh_wraps_single_mesh() {
  let model = Model::from_mesh(build_triangle_mesh(), "tri");
  assert_eq!(model.meshes.len(), 1);
  assert_eq!(model.tri_count, 1);
}

#[test]
fn test_from_mesh_propagates_tri_count() {
  let model = Model::from_mesh(build_quad_mesh(), "quad");
  assert_eq!(model.tri_count, 2);
}

// --- add_mesh ---

#[test]
fn test_add_mesh_appends_and_accumulates_tri_count() {
  let mut model = Model::new("multi");
  model.add_mesh(build_triangle_mesh());
  model.add_mesh(build_quad_mesh());
  assert_eq!(model.meshes.len(), 2);
  assert_eq!(model.tri_count, 3);
}

#[test]
#[should_panic(expected = "UVs must match vert count")]
fn test_add_mesh_panics_on_uv_count_mismatch() {
  let mut model = Model::new("bad");
  let mut mesh = build_triangle_mesh();
  mesh.tex_coords.pop();
  model.add_mesh(mesh);
}

#[test]
#[should_panic(expected = "normals must match vert count")]
fn test_add_mesh_panics_on_normal_count_mismatch() {
  let mut model = Model::new("bad");
  let mut mesh = build_triangle_mesh();
  mesh.normals.pop();
  model.add_mesh(mesh);
}

// --- mesh_info ---

#[test]
fn test_mesh_info_empty_for_new_model() {
  let model = Model::new("empty");
  assert!(model.mesh_info().is_empty());
}

#[test]
fn test_mesh_info_maps_names_to_indices() {
  let mut model = Model::new("multi");
  let mut a = build_triangle_mesh();
  a.name = "a".into();
  let mut b = build_quad_mesh();
  b.name = "b".into();
  model.add_mesh(a);
  model.add_mesh(b);
  let info = model.mesh_info();
  assert_eq!(info.get("a"), Some(&0));
  assert_eq!(info.get("b"), Some(&1));
}

#[test]
fn test_mesh_info_duplicate_names_later_wins() {
  let mut model = Model::new("dups");
  let mut a = build_triangle_mesh();
  a.name = "shared".into();
  let mut b = build_quad_mesh();
  b.name = "shared".into();
  model.add_mesh(a);
  model.add_mesh(b);
  let info = model.mesh_info();
  // HashMap takes the later value: the b mesh at index 1.
  assert_eq!(info.get("shared"), Some(&1));
}

// --- set_mesh_material ---

#[test]
fn test_set_mesh_material_replaces_material() {
  let mut model = Model::new("m");
  model.add_mesh(build_triangle_mesh());
  let mat = Material::new_flat(RED);
  model.set_mesh_material(0, mat);
  assert_eq!(model.meshes[0].material.diffuse.channels(), RED.channels());
}

#[test]
#[should_panic]
fn test_set_mesh_material_out_of_bounds_panics() {
  let mut model = Model::new("m");
  model.add_mesh(build_triangle_mesh());
  model.set_mesh_material(99, MATERIAL_PLACEHOLDER.clone());
}

// --- set_all_material ---

#[test]
fn test_set_all_material_updates_every_mesh() {
  let mut model = Model::new("m");
  model.add_mesh(build_triangle_mesh());
  model.add_mesh(build_quad_mesh());
  let mat = Material::new_flat(BLUE);
  model.set_all_material(mat);
  for m in &model.meshes {
    assert_eq!(m.material.diffuse.channels(), BLUE.channels());
  }
}

#[test]
fn test_set_all_material_on_empty_model_is_noop() {
  let mut model = Model::new("empty");
  let mat = Material::new_flat(RED);
  model.set_all_material(mat);
  assert!(model.meshes.is_empty());
}

#[test]
fn test_set_all_material_then_add_mesh_does_not_overwrite_new_mesh() {
  // set_all_material only touches the meshes present at call time. A later add_mesh
  // keeps its own material. Locks the current behaviour.
  let mut model = Model::new("m");
  model.add_mesh(build_triangle_mesh());
  model.set_all_material(Material::new_flat(RED));
  model.add_mesh(build_quad_mesh());
  // First mesh: red. Second mesh: placeholder white.
  assert_eq!(model.meshes[0].material.diffuse.channels(), RED.channels());
  assert_eq!(model.meshes[1].material.diffuse.channels(), MATERIAL_PLACEHOLDER.diffuse.channels());
}
