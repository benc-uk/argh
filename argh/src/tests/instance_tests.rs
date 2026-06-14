// ==============================================================================================
// Module & file:   instance_tests.rs
// Purpose:         Tests for the Instance builder API and model matrix composition.
// Author & Date:   AI generated, 2026
// License:         MIT
// Notes:           Instances cannot be constructed directly; tests go through Scene.
//                  See misc/testing-spec.md for the house style and conventions.
// ==============================================================================================

use crate::engine::Engine;
use crate::math::{V3_ZERO, Vec2, Vec3};
use crate::mesh::Mesh;
use crate::model::Model;
use crate::scene::Scene;
use crate::test_helpers::{EPS_TRIG, assert_mat4_near};

fn dummy_engine_with_triangle() -> (Engine, crate::engine::ModelHandle) {
  let mut e = Engine::new(64, 64);
  let mut m = Mesh::new();
  m.positions = vec![Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)];
  m.normals = vec![Vec3::new(0.0, 0.0, 1.0); 3];
  m.tex_coords = vec![Vec2::new(0.0, 0.0); 3];
  m.indices = vec![0, 1, 2];
  m.tri_count = 1;
  let h = e.add_model(Model::from_mesh(m, "tri"));
  (e, h)
}

// --- Builder defaults ---

#[test]
fn test_instance_default_pos_zero() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h = s.add_instance(mh);
  assert_eq!(s.instance(h).pos, V3_ZERO);
}

#[test]
fn test_instance_default_scale_one() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h = s.add_instance(mh);
  assert_eq!(s.instance(h).scale, Vec3::new(1.0, 1.0, 1.0));
}

#[test]
fn test_instance_default_smooth_true() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h = s.add_instance(mh);
  assert!(s.instance(h).smooth);
}

// --- pos and pos_xyz ---

#[test]
fn test_pos_sets_position() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h = s.add_instance(mh);
  s.instance_mut(h).pos(Vec3::new(1.0, 2.0, 3.0));
  assert_eq!(s.instance(h).pos, Vec3::new(1.0, 2.0, 3.0));
}

#[test]
fn test_pos_xyz_sets_position() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h = s.add_instance(mh);
  s.instance_mut(h).pos_xyz(4.0, 5.0, 6.0);
  assert_eq!(s.instance(h).pos, Vec3::new(4.0, 5.0, 6.0));
}

// --- scale and per-axis ---

#[test]
fn test_scale_uniform_sets_all_three_axes() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h = s.add_instance(mh);
  s.instance_mut(h).scale(2.5);
  assert_eq!(s.instance(h).scale, Vec3::new(2.5, 2.5, 2.5));
}

#[test]
fn test_scale_x_sets_x_only() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h = s.add_instance(mh);
  s.instance_mut(h).scale_x(3.0);
  let scale = s.instance(h).scale;
  assert_eq!(scale.x, 3.0);
  assert_eq!(scale.y, 1.0);
  assert_eq!(scale.z, 1.0);
}

#[test]
fn test_scale_y_sets_y_only() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h = s.add_instance(mh);
  s.instance_mut(h).scale_y(3.0);
  let scale = s.instance(h).scale;
  assert_eq!(scale.x, 1.0);
  assert_eq!(scale.y, 3.0);
  assert_eq!(scale.z, 1.0);
}

#[test]
fn test_scale_z_sets_z_only() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h = s.add_instance(mh);
  s.instance_mut(h).scale_z(3.0);
  let scale = s.instance(h).scale;
  assert_eq!(scale.x, 1.0);
  assert_eq!(scale.y, 1.0);
  assert_eq!(scale.z, 3.0);
}

// --- smooth ---

#[test]
fn test_smooth_toggle() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h = s.add_instance(mh);
  s.instance_mut(h).smooth(false);
  assert!(!s.instance(h).smooth);
  s.instance_mut(h).smooth(true);
  assert!(s.instance(h).smooth);
}

// --- model_mat ---

#[test]
fn test_model_mat_default_is_identity_like() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h = s.add_instance(mh);
  let mat = s.instance(h).model_mat();
  // With identity rot, scale=1, pos=0 the matrix should be the identity.
  let raw = mat.raw_for_test();
  // Diagonal ~1
  for i in 0..4 {
    assert!((raw[i][i] - 1.0).abs() < EPS_TRIG);
  }
}

#[test]
fn test_model_mat_translation_visible() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h = s.add_instance(mh);
  s.instance_mut(h).pos_xyz(5.0, 6.0, 7.0);

  // Identity matrix at translation 5,6,7. Mat4::new_scale_rot_trans uses column-major
  // storage in this engine, so the translation lives in row index 3 of the column-vector
  // mat[col][row]; we just look for the magnitudes anywhere along the boundary.
  let mat = s.instance(h).model_mat();
  let raw = mat.raw_for_test();
  let flat: Vec<f32> = raw.iter().flatten().copied().collect();
  assert!(flat.iter().any(|v| (v - 5.0).abs() < EPS_TRIG), "translation x missing");
  assert!(flat.iter().any(|v| (v - 6.0).abs() < EPS_TRIG), "translation y missing");
  assert!(flat.iter().any(|v| (v - 7.0).abs() < EPS_TRIG), "translation z missing");
}

#[test]
fn test_model_mat_scale_visible() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h = s.add_instance(mh);
  s.instance_mut(h).scale(4.0);
  let mat = s.instance(h).model_mat();
  let flat: Vec<f32> = mat.raw_for_test().iter().flatten().copied().collect();
  // Identity rotation + scale=4 must put a 4.0 somewhere on the diagonal of the
  // upper 3x3.
  let four_count = flat.iter().filter(|v| (**v - 4.0).abs() < EPS_TRIG).count();
  assert!(four_count >= 3, "expected at least 3 instances of 4.0, got {four_count}");
}

#[test]
fn test_model_mat_round_trip_unchanged() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h = s.add_instance(mh);
  let m1 = s.instance(h).model_mat();
  let m2 = s.instance(h).model_mat();
  assert_mat4_near(&m1, &m2, 1e-6);
}

// --- handle ---

#[test]
fn test_handle_returns_scene_handle() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h = s.add_instance(mh);
  assert_eq!(s.instance(h).handle(), h);
}

// --- Rotation accumulation ---

#[test]
fn test_rot_x_twice_differs_from_single() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h_single = s.add_instance(mh);
  let h_double = s.add_instance(mh);
  s.instance_mut(h_single).rot_x(0.5);
  s.instance_mut(h_double).rot_x(0.5);
  s.instance_mut(h_double).rot_x(0.5);
  let m1 = s.instance(h_single).model_mat();
  let m2 = s.instance(h_double).model_mat();
  let flat1: Vec<f32> = m1.raw_for_test().iter().flatten().copied().collect();
  let flat2: Vec<f32> = m2.raw_for_test().iter().flatten().copied().collect();
  let differs = flat1.iter().zip(flat2.iter()).any(|(a, b)| (a - b).abs() > EPS_TRIG);
  assert!(differs, "two rot_x should differ from one");
}

// --- Alternative constructors ---

#[test]
fn test_add_instance_mut_returns_default_instance() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let inst = s.add_instance_mut(mh);
  assert_eq!(inst.pos, V3_ZERO);
  assert_eq!(inst.scale, Vec3::new(1.0, 1.0, 1.0));
  assert!(inst.smooth);
}

#[test]
fn test_add_instance_mut_handle_matches_lookup() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h = s.add_instance_mut(mh).handle();
  // Same handle should round-trip through the scene.
  assert_eq!(s.instance(h).handle(), h);
}

#[test]
fn test_add_instance_world_sets_pos_and_scale() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h = s.add_instance_posed(mh, Vec3::new(1.0, 2.0, 3.0), V3_ZERO, Vec3::new(4.0, 5.0, 6.0));
  let i = s.instance(h);
  assert_eq!(i.pos, Vec3::new(1.0, 2.0, 3.0));
  assert_eq!(i.scale, Vec3::new(4.0, 5.0, 6.0));
}

#[test]
fn test_add_instance_world_with_zero_rot_acts_like_identity_for_a_point() {
  // pos=0, rot=0, scale=1 → identity transform of any point.
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h = s.add_instance_posed(mh, V3_ZERO, V3_ZERO, Vec3::new(1.0, 1.0, 1.0));
  let p = Vec3::new(7.0, -3.0, 2.5);
  let q = s.instance(h).model_mat().transform_point(&p);
  assert!((q.x - p.x).abs() < EPS_TRIG);
  assert!((q.y - p.y).abs() < EPS_TRIG);
  assert!((q.z - p.z).abs() < EPS_TRIG);
}

#[test]
fn test_add_instance_world_rotation_applied() {
  // Rotate 90 degrees around Y. The point (1,0,0) should become roughly (0,0,-1)
  // in a right-handed system with standard rotation.
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h = s.add_instance_posed(mh, V3_ZERO, Vec3::new(0.0, std::f32::consts::FRAC_PI_2, 0.0), Vec3::new(1.0, 1.0, 1.0));
  let q = s.instance(h).model_mat().transform_point(&Vec3::new(1.0, 0.0, 0.0));
  // The Y component must remain 0; the magnitude on the X and Z axes must equal 1.
  assert!(q.y.abs() < EPS_TRIG, "Y component should stay 0, got {}", q.y);
  let r = (q.x * q.x + q.z * q.z).sqrt();
  assert!((r - 1.0).abs() < EPS_TRIG, "length should be preserved");
}

// --- Builder chaining (fluent API) ---

#[test]
fn test_builder_chain_pos_scale_rot_returns_same_self() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h = s.add_instance(mh);
  // All builder methods return &mut Self, so they should chain in a single statement.
  s.instance_mut(h).pos_xyz(1.0, 2.0, 3.0).scale(2.0).rot_y(0.5).smooth(false);
  let i = s.instance(h);
  assert_eq!(i.pos, Vec3::new(1.0, 2.0, 3.0));
  assert_eq!(i.scale, Vec3::new(2.0, 2.0, 2.0));
  assert!(!i.smooth);
}

#[test]
fn test_builder_chain_three_rotations_compose() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h_chain = s.add_instance(mh);
  let h_seq = s.add_instance(mh);
  s.instance_mut(h_chain).rot_x(0.3).rot_y(0.4).rot_z(0.5);
  s.instance_mut(h_seq).rot_x(0.3);
  s.instance_mut(h_seq).rot_y(0.4);
  s.instance_mut(h_seq).rot_z(0.5);
  // Chained calls should match separate calls exactly.
  let m1 = s.instance(h_chain).model_mat();
  let m2 = s.instance(h_seq).model_mat();
  assert_mat4_near(&m1, &m2, 1e-5);
}

#[test]
fn test_builder_chain_pos_and_pos_xyz_equivalent() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h_a = s.add_instance(mh);
  let h_b = s.add_instance(mh);
  s.instance_mut(h_a).pos(Vec3::new(2.5, -1.0, 4.0));
  s.instance_mut(h_b).pos_xyz(2.5, -1.0, 4.0);
  assert_eq!(s.instance(h_a).pos, s.instance(h_b).pos);
}

// --- Instance independence ---

#[test]
fn test_two_instances_same_model_are_independent_pos() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h_a = s.add_instance(mh);
  let h_b = s.add_instance(mh);
  s.instance_mut(h_a).pos_xyz(10.0, 0.0, 0.0);
  // Mutating A should leave B untouched.
  assert_eq!(s.instance(h_a).pos, Vec3::new(10.0, 0.0, 0.0));
  assert_eq!(s.instance(h_b).pos, V3_ZERO);
}

#[test]
fn test_two_instances_same_model_have_distinct_handles() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h_a = s.add_instance(mh);
  let h_b = s.add_instance(mh);
  assert_ne!(h_a, h_b);
}

#[test]
fn test_two_instances_independent_smooth_and_scale() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h_a = s.add_instance(mh);
  let h_b = s.add_instance(mh);
  s.instance_mut(h_a).smooth(false).scale(2.0);
  assert!(!s.instance(h_a).smooth);
  assert_eq!(s.instance(h_a).scale, Vec3::new(2.0, 2.0, 2.0));
  assert!(s.instance(h_b).smooth);
  assert_eq!(s.instance(h_b).scale, Vec3::new(1.0, 1.0, 1.0));
}

// --- Removal ---

#[test]
fn test_remove_instance_reduces_iteration_count() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h_a = s.add_instance(mh);
  let _h_b = s.add_instance(mh);
  let _h_c = s.add_instance(mh);
  assert_eq!(s.instances().count(), 3);
  s.remove_instance(h_a);
  assert_eq!(s.instances().count(), 2);
}

#[test]
fn test_remove_instance_preserves_others_state() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h_a = s.add_instance(mh);
  let h_b = s.add_instance(mh);
  s.instance_mut(h_b).pos_xyz(9.0, 9.0, 9.0);
  s.remove_instance(h_a);
  // B's position must survive A's removal.
  assert_eq!(s.instance(h_b).pos, Vec3::new(9.0, 9.0, 9.0));
}

#[test]
#[should_panic(expected = "instance not found")]
fn test_access_removed_instance_panics() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h = s.add_instance(mh);
  s.remove_instance(h);
  let _ = s.instance(h);
}

#[test]
fn test_remove_then_add_yields_new_handle() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h1 = s.add_instance(mh);
  s.remove_instance(h1);
  let h2 = s.add_instance(mh);
  // Slotmap versioning means the new handle must not collide with the stale one.
  assert_ne!(h1, h2);
}

// --- Scale mutation semantics ---

#[test]
fn test_scale_uniform_overrides_previous_per_axis() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h = s.add_instance(mh);
  s.instance_mut(h).scale_x(7.0).scale_y(8.0).scale_z(9.0).scale(2.0);
  // The final uniform scale must clobber all three axes.
  assert_eq!(s.instance(h).scale, Vec3::new(2.0, 2.0, 2.0));
}

#[test]
fn test_scale_per_axis_independent_of_each_other() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h = s.add_instance(mh);
  s.instance_mut(h).scale_x(2.0).scale_y(3.0).scale_z(4.0);
  let sc = s.instance(h).scale;
  assert_eq!(sc, Vec3::new(2.0, 3.0, 4.0));
}

#[test]
fn test_scale_negative_allowed_and_stored() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h = s.add_instance(mh);
  s.instance_mut(h).scale(-1.0);
  // Negative scale is allowed (mirroring) and must be stored verbatim.
  assert_eq!(s.instance(h).scale, Vec3::new(-1.0, -1.0, -1.0));
}

#[test]
fn test_scale_zero_allowed_and_stored() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h = s.add_instance(mh);
  s.instance_mut(h).scale(0.0);
  assert_eq!(s.instance(h).scale, Vec3::new(0.0, 0.0, 0.0));
}

// --- model_mat point-transform semantics ---

#[test]
fn test_model_mat_default_transforms_point_to_itself() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h = s.add_instance(mh);
  let p = Vec3::new(1.0, 2.0, 3.0);
  let q = s.instance(h).model_mat().transform_point(&p);
  assert!((q.x - p.x).abs() < EPS_TRIG);
  assert!((q.y - p.y).abs() < EPS_TRIG);
  assert!((q.z - p.z).abs() < EPS_TRIG);
}

#[test]
fn test_model_mat_translation_applied_to_origin() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h = s.add_instance(mh);
  s.instance_mut(h).pos_xyz(5.0, 6.0, 7.0);
  let q = s.instance(h).model_mat().transform_point(&V3_ZERO);
  assert!((q.x - 5.0).abs() < EPS_TRIG);
  assert!((q.y - 6.0).abs() < EPS_TRIG);
  assert!((q.z - 7.0).abs() < EPS_TRIG);
}

#[test]
fn test_model_mat_uniform_scale_applied_to_unit_x() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h = s.add_instance(mh);
  s.instance_mut(h).scale(2.5);
  let q = s.instance(h).model_mat().transform_point(&Vec3::new(1.0, 0.0, 0.0));
  assert!((q.x - 2.5).abs() < EPS_TRIG);
  assert!(q.y.abs() < EPS_TRIG);
  assert!(q.z.abs() < EPS_TRIG);
}

#[test]
fn test_model_mat_per_axis_scale_applied_independently() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h = s.add_instance(mh);
  s.instance_mut(h).scale_x(2.0).scale_y(3.0).scale_z(4.0);
  let q = s.instance(h).model_mat().transform_point(&Vec3::new(1.0, 1.0, 1.0));
  assert!((q.x - 2.0).abs() < EPS_TRIG);
  assert!((q.y - 3.0).abs() < EPS_TRIG);
  assert!((q.z - 4.0).abs() < EPS_TRIG);
}

#[test]
fn test_model_mat_negative_scale_flips_point() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h = s.add_instance(mh);
  s.instance_mut(h).scale(-1.0);
  let q = s.instance(h).model_mat().transform_point(&Vec3::new(1.0, 2.0, 3.0));
  assert!((q.x + 1.0).abs() < EPS_TRIG);
  assert!((q.y + 2.0).abs() < EPS_TRIG);
  assert!((q.z + 3.0).abs() < EPS_TRIG);
}

#[test]
fn test_model_mat_rotation_y_half_pi_rotates_x_axis() {
  // Rotating (1,0,0) by 90 degrees around Y must produce a unit vector with Y=0.
  // The sign of Z depends on handedness; we only check magnitude and Y component.
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h = s.add_instance(mh);
  s.instance_mut(h).rot_y(std::f32::consts::FRAC_PI_2);
  let q = s.instance(h).model_mat().transform_point(&Vec3::new(1.0, 0.0, 0.0));
  assert!(q.y.abs() < EPS_TRIG);
  let len = (q.x * q.x + q.z * q.z).sqrt();
  assert!((len - 1.0).abs() < EPS_TRIG);
  assert!(q.x.abs() < EPS_TRIG, "X should rotate out, got {}", q.x);
}

#[test]
fn test_model_mat_full_circle_returns_point_to_origin() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h = s.add_instance(mh);
  s.instance_mut(h).rot_x(std::f32::consts::TAU);
  // A full rotation of 2π should leave any point essentially unchanged.
  let p = Vec3::new(1.0, 2.0, 3.0);
  let q = s.instance(h).model_mat().transform_point(&p);
  assert!((q.x - p.x).abs() < 1e-3);
  assert!((q.y - p.y).abs() < 1e-3);
  assert!((q.z - p.z).abs() < 1e-3);
}

#[test]
fn test_model_mat_opposite_rotations_cancel() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h = s.add_instance(mh);
  s.instance_mut(h).rot_z(1.2).rot_z(-1.2);
  let p = Vec3::new(3.0, -2.0, 1.0);
  let q = s.instance(h).model_mat().transform_point(&p);
  assert!((q.x - p.x).abs() < EPS_TRIG);
  assert!((q.y - p.y).abs() < EPS_TRIG);
  assert!((q.z - p.z).abs() < EPS_TRIG);
}

#[test]
fn test_model_mat_combined_scale_translate_applied_in_correct_order() {
  // Standard SRT order: scale then translate. Transforming (1,0,0) with scale=2
  // and pos=(10,0,0) should give (12,0,0), not (11,0,0) (which would be translate
  // before scale).
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h = s.add_instance(mh);
  s.instance_mut(h).scale(2.0).pos_xyz(10.0, 0.0, 0.0);
  let q = s.instance(h).model_mat().transform_point(&Vec3::new(1.0, 0.0, 0.0));
  assert!((q.x - 12.0).abs() < EPS_TRIG, "expected x=12 (scale then translate), got {}", q.x);
}

// --- Local vs world rotation ---

#[test]
fn test_local_and_world_rotation_match_from_identity() {
  // From the identity orientation there is no difference between local and world
  // rotations, so the resulting matrices should be identical.
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h_local = s.add_instance(mh);
  let h_world = s.add_instance(mh);
  s.instance_mut(h_local).rot_y(0.7);
  s.instance_mut(h_world).rot_y_world(0.7);
  let m1 = s.instance(h_local).model_mat();
  let m2 = s.instance(h_world).model_mat();
  assert_mat4_near(&m1, &m2, 1e-5);
}

#[test]
fn test_local_and_world_rotation_diverge_after_first_rotation() {
  // Once the instance is tilted, local-axis and world-axis rotations differ.
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h_local = s.add_instance(mh);
  let h_world = s.add_instance(mh);
  s.instance_mut(h_local).rot_x(0.5).rot_y(0.5);
  s.instance_mut(h_world).rot_x(0.5).rot_y_world(0.5);
  let m1 = s.instance(h_local).model_mat();
  let m2 = s.instance(h_world).model_mat();
  let f1: Vec<f32> = m1.raw_for_test().iter().flatten().copied().collect();
  let f2: Vec<f32> = m2.raw_for_test().iter().flatten().copied().collect();
  let differs = f1.iter().zip(f2.iter()).any(|(a, b)| (a - b).abs() > 1e-4);
  assert!(differs, "local and world rotations should differ after a non-identity first rotation");
}

#[test]
fn test_rot_x_and_rot_y_are_not_commutative() {
  // Composing rot_x then rot_y is generally different from rot_y then rot_x.
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h_xy = s.add_instance(mh);
  let h_yx = s.add_instance(mh);
  s.instance_mut(h_xy).rot_x(0.8).rot_y(0.6);
  s.instance_mut(h_yx).rot_y(0.6).rot_x(0.8);
  let f1: Vec<f32> = s.instance(h_xy).model_mat().raw_for_test().iter().flatten().copied().collect();
  let f2: Vec<f32> = s.instance(h_yx).model_mat().raw_for_test().iter().flatten().copied().collect();
  let differs = f1.iter().zip(f2.iter()).any(|(a, b)| (a - b).abs() > 1e-4);
  assert!(differs, "rot_x;rot_y should not equal rot_y;rot_x for non-trivial angles");
}

// --- smooth flag isolation ---

#[test]
fn test_smooth_flag_does_not_affect_model_mat() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h = s.add_instance(mh);
  let m_smooth = s.instance(h).model_mat();
  s.instance_mut(h).smooth(false);
  let m_flat = s.instance(h).model_mat();
  assert_mat4_near(&m_smooth, &m_flat, 1e-7);
}

// --- handle stability ---

#[test]
fn test_handle_stable_across_mutations() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h = s.add_instance(mh);
  let h_before = s.instance(h).handle();
  s.instance_mut(h).pos_xyz(1.0, 1.0, 1.0).scale(3.0).rot_y(1.0).smooth(false);
  let h_after = s.instance(h).handle();
  assert_eq!(h_before, h_after);
}

// --- Iteration ---

#[test]
fn test_instances_iter_yields_all_added() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  for _ in 0..5 {
    s.add_instance(mh);
  }
  assert_eq!(s.instances().count(), 5);
}

#[test]
fn test_instances_mut_can_bulk_update_position() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  for _ in 0..4 {
    s.add_instance(mh);
  }
  for inst in s.instances_mut() {
    inst.pos_xyz(7.0, 0.0, 0.0);
  }
  for inst in s.instances() {
    assert_eq!(inst.pos, Vec3::new(7.0, 0.0, 0.0));
  }
}

// --- Extreme values ---

#[test]
fn test_large_translation_preserved() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h = s.add_instance(mh);
  s.instance_mut(h).pos_xyz(1.0e6, -1.0e6, 1.0e6);
  let q = s.instance(h).model_mat().transform_point(&V3_ZERO);
  assert!((q.x - 1.0e6).abs() < 1.0);
  assert!((q.y + 1.0e6).abs() < 1.0);
  assert!((q.z - 1.0e6).abs() < 1.0);
}

#[test]
fn test_setting_pos_overwrites_previous_pos() {
  let (mut e, mh) = dummy_engine_with_triangle();
  let _ = &mut e;
  let mut s = Scene::new();
  let h = s.add_instance(mh);
  s.instance_mut(h).pos_xyz(1.0, 2.0, 3.0);
  s.instance_mut(h).pos_xyz(-1.0, -2.0, -3.0);
  // Position setters replace, not accumulate.
  assert_eq!(s.instance(h).pos, Vec3::new(-1.0, -2.0, -3.0));
}
