// ==============================================================================================
// Module & file:   mesh_tests.rs
// Purpose:         Tests for Mesh struct
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:           AI generated
// ==============================================================================================

use super::*;
use crate::colour::RED;
use crate::material::Material as ArghMaterial;

// --- Constructors ---

#[test]
fn test_new_has_empty_buffers() {
  let m = Mesh::new();
  assert!(m.positions.is_empty());
  assert!(m.normals.is_empty());
  assert!(m.tex_coords.is_empty());
  assert!(m.indices.is_empty());
}

#[test]
fn test_new_has_empty_name() {
  let m = Mesh::new();
  assert_eq!(m.name, "");
}

#[test]
fn test_new_has_zero_tri_count() {
  let m = Mesh::new();
  assert_eq!(m.tri_count, 0);
}

#[test]
fn test_new_uses_placeholder_material() {
  let m = Mesh::new();
  // Placeholder is WHITE diffuse, WHITE spec, hardness 20, no texture.
  assert_eq!(m.material.diffuse.channels(), MATERIAL_PLACEHOLDER.diffuse.channels());
  assert_eq!(m.material.hardness, 20.0);
  assert!(m.material.texture.is_none());
}

#[test]
fn test_new_with_material_carries_material() {
  let mat = ArghMaterial::new_flat(RED);
  let m = Mesh::new_with_material(mat);
  let (r, g, b) = m.material.diffuse.channels();
  assert_eq!(r, 1.0);
  assert_eq!(g, 0.0);
  assert_eq!(b, 0.0);
}

#[test]
fn test_new_with_material_other_fields_empty() {
  let m = Mesh::new_with_material(ArghMaterial::new_flat(RED));
  assert!(m.positions.is_empty());
  assert!(m.normals.is_empty());
  assert!(m.tex_coords.is_empty());
  assert!(m.indices.is_empty());
  assert_eq!(m.tri_count, 0);
}

// --- Field independence ---

#[test]
fn test_buffers_grow_independently() {
  let mut m = Mesh::new();
  m.positions.push(Vec3::new(0.0, 0.0, 0.0));
  m.positions.push(Vec3::new(1.0, 0.0, 0.0));
  assert_eq!(m.positions.len(), 2);
  assert_eq!(m.normals.len(), 0);
  assert_eq!(m.tex_coords.len(), 0);
  assert_eq!(m.indices.len(), 0);
}

#[test]
fn test_name_mutation() {
  let mut m = Mesh::new();
  m.name = "test_mesh".to_string();
  assert_eq!(m.name, "test_mesh");
}

#[test]
fn test_tri_count_mutation() {
  let mut m = Mesh::new();
  m.tri_count = 42;
  assert_eq!(m.tri_count, 42);
}
