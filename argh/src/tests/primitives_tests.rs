// ==============================================================================================
// Module & file:   primitives_tests.rs
// Purpose:         Tests for the generated primitive shape models (cube, sphere, cylinder, cone).
// Author & Date:   AI generated, 2026
// License:         MIT
// Notes:           See misc/testing-spec.md for the house style and conventions.
// ==============================================================================================

use super::*;
use crate::material::Material;
use crate::math::Vec3;

const EPS_LEN: f32 = 1e-4;

// --- Cube ---

#[test]
fn test_cube_has_24_verts() {
  let m = new_cube(Material::default_placeholder());
  assert_eq!(m.meshes[0].positions.len(), 24);
}

#[test]
fn test_cube_has_24_normals() {
  let m = new_cube(Material::default_placeholder());
  assert_eq!(m.meshes[0].normals.len(), 24);
}

#[test]
fn test_cube_has_24_uvs() {
  let m = new_cube(Material::default_placeholder());
  assert_eq!(m.meshes[0].tex_coords.len(), 24);
}

#[test]
fn test_cube_has_36_indices() {
  let m = new_cube(Material::default_placeholder());
  assert_eq!(m.meshes[0].indices.len(), 36);
}

#[test]
fn test_cube_tri_count_is_12() {
  let m = new_cube(Material::default_placeholder());
  assert_eq!(m.tri_count, 12);
}

#[test]
fn test_cube_positions_in_unit_range() {
  let m = new_cube(Material::default_placeholder());
  for p in &m.meshes[0].positions {
    assert!(p.x >= -0.5 - EPS_LEN && p.x <= 0.5 + EPS_LEN);
    assert!(p.y >= -0.5 - EPS_LEN && p.y <= 0.5 + EPS_LEN);
    assert!(p.z >= -0.5 - EPS_LEN && p.z <= 0.5 + EPS_LEN);
  }
}

#[test]
fn test_cube_normals_are_unit_axis_aligned() {
  let m = new_cube(Material::default_placeholder());
  for n in &m.meshes[0].normals {
    let abs = (n.x.abs(), n.y.abs(), n.z.abs());
    // Exactly one component should be 1, the other two 0.
    let ones = [abs.0, abs.1, abs.2].iter().filter(|v| (**v - 1.0).abs() < EPS_LEN).count();
    let zeros = [abs.0, abs.1, abs.2].iter().filter(|v| v.abs() < EPS_LEN).count();
    assert_eq!(ones, 1);
    assert_eq!(zeros, 2);
  }
}

#[test]
fn test_cube_front_face_normal_is_plus_z() {
  // The first 4 verts are the +Z (front) face per the source layout.
  let m = new_cube(Material::default_placeholder());
  for i in 0..4 {
    let n = m.meshes[0].normals[i];
    assert!((n.z - 1.0).abs() < EPS_LEN);
  }
}

// --- Sphere ---

#[test]
fn test_sphere_vert_count_formula() {
  let m = new_sphere(Material::default_placeholder(), 8, 12);
  assert_eq!(m.meshes[0].positions.len(), (8 + 1) * (12 + 1));
}

#[test]
fn test_sphere_tri_count_formula() {
  let m = new_sphere(Material::default_placeholder(), 8, 12);
  assert_eq!(m.tri_count, 2 * (8 - 1) * 12);
}

#[test]
fn test_sphere_normals_unit_length() {
  let m = new_sphere(Material::default_placeholder(), 10, 16);
  for n in &m.meshes[0].normals {
    let len = (n.x * n.x + n.y * n.y + n.z * n.z).sqrt();
    assert!((len - 1.0).abs() < EPS_LEN, "normal length should be 1, got {len}");
  }
}

#[test]
fn test_sphere_positions_within_radius() {
  let m = new_sphere(Material::default_placeholder(), 10, 16);
  for p in &m.meshes[0].positions {
    let len = (p.x * p.x + p.y * p.y + p.z * p.z).sqrt();
    assert!((len - 0.5).abs() < EPS_LEN, "position should be radius 0.5, got {len}");
  }
}

#[test]
fn test_sphere_uvs_in_unit_range() {
  let m = new_sphere(Material::default_placeholder(), 6, 6);
  for uv in &m.meshes[0].tex_coords {
    assert!(uv.x >= 0.0 - EPS_LEN && uv.x <= 1.0 + EPS_LEN);
    assert!(uv.y >= 0.0 - EPS_LEN && uv.y <= 1.0 + EPS_LEN);
  }
}

#[test]
fn test_sphere_stacks_clamped_to_two() {
  let m = new_sphere(Material::default_placeholder(), 0, 5);
  // stacks clamped to 2, sectors=5 → verts = (2+1)*(5+1) = 18
  assert_eq!(m.meshes[0].positions.len(), 18);
}

#[test]
fn test_sphere_sectors_clamped_to_three() {
  let m = new_sphere(Material::default_placeholder(), 4, 0);
  // sectors clamped to 3 → verts = (4+1)*(3+1) = 20
  assert_eq!(m.meshes[0].positions.len(), 20);
}

#[test]
fn test_sphere_name_includes_stacks_and_sectors() {
  let m = new_sphere(Material::default_placeholder(), 8, 12);
  assert_eq!(m.name(), "sphere_8_12");
}

// --- Cylinder ---

#[test]
fn test_cylinder_open_vert_count() {
  let m = new_cylinder(Material::default_placeholder(), 12, false);
  assert_eq!(m.meshes[0].positions.len(), 2 * (12 + 1));
}

#[test]
fn test_cylinder_capped_vert_count() {
  let m = new_cylinder(Material::default_placeholder(), 12, true);
  let expected = 2 * (12 + 1) + 2 * (1 + 12);
  assert_eq!(m.meshes[0].positions.len(), expected);
}

#[test]
fn test_cylinder_open_tri_count() {
  let m = new_cylinder(Material::default_placeholder(), 12, false);
  assert_eq!(m.tri_count, 2 * 12);
}

#[test]
fn test_cylinder_capped_tri_count() {
  let m = new_cylinder(Material::default_placeholder(), 12, true);
  assert_eq!(m.tri_count, 4 * 12);
}

#[test]
fn test_cylinder_sectors_clamped_to_three() {
  let m = new_cylinder(Material::default_placeholder(), 0, true);
  // 4 * 3 = 12 tris when caps on
  assert_eq!(m.tri_count, 12);
}

#[test]
fn test_cylinder_side_normals_horizontal() {
  // First 2*(sectors+1) verts are the side ring with radial normals (y ≈ 0).
  let m = new_cylinder(Material::default_placeholder(), 8, false);
  for n in &m.meshes[0].normals {
    assert!(n.y.abs() < EPS_LEN, "side normal should be horizontal");
  }
}

#[test]
fn test_cylinder_positions_within_unit_height() {
  let m = new_cylinder(Material::default_placeholder(), 8, true);
  for p in &m.meshes[0].positions {
    assert!(p.y >= -0.5 - EPS_LEN && p.y <= 0.5 + EPS_LEN);
    let r = (p.x * p.x + p.z * p.z).sqrt();
    assert!(r <= 0.5 + EPS_LEN, "radius should be <= 0.5");
  }
}

// --- Cone ---

#[test]
fn test_cone_open_vert_count() {
  let m = new_cone(Material::default_placeholder(), 10, false);
  assert_eq!(m.meshes[0].positions.len(), 2 * (10 + 1));
}

#[test]
fn test_cone_capped_vert_count() {
  let m = new_cone(Material::default_placeholder(), 10, true);
  let expected = 2 * (10 + 1) + (1 + 10);
  assert_eq!(m.meshes[0].positions.len(), expected);
}

#[test]
fn test_cone_open_tri_count() {
  let m = new_cone(Material::default_placeholder(), 10, false);
  assert_eq!(m.tri_count, 10);
}

#[test]
fn test_cone_capped_tri_count() {
  let m = new_cone(Material::default_placeholder(), 10, true);
  assert_eq!(m.tri_count, 2 * 10);
}

#[test]
fn test_cone_apex_collapses_to_single_point() {
  // The apex ring is verts [sectors+1 .. 2*(sectors+1)), all at (0, +0.5, 0).
  let m = new_cone(Material::default_placeholder(), 8, false);
  let apex_target = Vec3::new(0.0, 0.5, 0.0);
  for i in 9..18 {
    let p = m.meshes[0].positions[i];
    assert!((p.x - apex_target.x).abs() < EPS_LEN);
    assert!((p.y - apex_target.y).abs() < EPS_LEN);
    assert!((p.z - apex_target.z).abs() < EPS_LEN);
  }
}

#[test]
fn test_cone_side_normals_have_positive_y() {
  let m = new_cone(Material::default_placeholder(), 8, false);
  for n in &m.meshes[0].normals {
    assert!(n.y > 0.0, "side normal should tilt upward, got {}", n.y);
  }
}

#[test]
fn test_cone_cap_normals_strictly_negative_y() {
  // The cap verts start after the side block; their normals all point -Y.
  let m = new_cone(Material::default_placeholder(), 8, true);
  let side_count = 2 * (8 + 1);
  for n in &m.meshes[0].normals[side_count..] {
    assert!((n.y + 1.0).abs() < EPS_LEN, "cap normal should be -Y, got {}", n.y);
  }
}

#[test]
fn test_cone_sectors_clamped_to_three() {
  let m = new_cone(Material::default_placeholder(), 0, false);
  assert_eq!(m.tri_count, 3);
}

// --- Material default helper for tests ---

// Helper trait to avoid building a Material from scratch in every test.
trait MatTest {
  fn default_placeholder() -> Self;
}
impl MatTest for Material {
  fn default_placeholder() -> Self {
    crate::material::MATERIAL_PLACEHOLDER.clone()
  }
}
