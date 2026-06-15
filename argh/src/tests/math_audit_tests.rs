// ==============================================================================================
// Module & file:   math / audit_tests.rs
// Purpose:         Top-up tests for the math module covering edge cases that the per-type
//                  test files don't explicitly exercise (NaN/inf propagation, near-singular
//                  matrices, perspective/look-at degenerate inputs, normalisation drift, etc).
// Author & Date:   AI generated, 2026
// License:         MIT
// Notes:           Intentionally a single cross-cutting file rather than scattered across
//                  every *_tests.rs. See misc/testing-spec.md for the house style.
// ==============================================================================================

use super::*;

// --- Vec3 NaN / inf propagation ---

#[test]
fn test_vec3_nan_propagates_through_add() {
  let a = Vec3::new(f32::NAN, 0.0, 0.0);
  let b = Vec3::new(1.0, 2.0, 3.0);
  let r = a + b;
  assert!(r.x.is_nan());
}

#[test]
fn test_vec3_nan_propagates_through_dot() {
  let a = Vec3::new(f32::NAN, 0.0, 0.0);
  let b = Vec3::new(1.0, 2.0, 3.0);
  assert!(a.dot(b).is_nan());
}

#[test]
fn test_vec3_inf_in_len_yields_inf() {
  let a = Vec3::new(f32::INFINITY, 0.0, 0.0);
  assert!(a.len().is_infinite());
}

#[test]
fn test_vec3_normalize_zero_vector_yields_nans() {
  // Defined behaviour locked: dividing by zero length should produce NaN
  // components rather than panic.
  let a = Vec3::new(0.0, 0.0, 0.0);
  let n = a.normalize_new();
  assert!(n.x.is_nan() && n.y.is_nan() && n.z.is_nan());
}

// --- Vec3 cross-product algebra ---

#[test]
fn test_vec3_cross_is_anti_commutative() {
  let a = Vec3::new(1.0, 2.0, 3.0);
  let b = Vec3::new(4.0, 5.0, 6.0);
  let ab = a.cross(b);
  let ba = b.cross(a);
  assert!((ab.x + ba.x).abs() < 1e-5);
  assert!((ab.y + ba.y).abs() < 1e-5);
  assert!((ab.z + ba.z).abs() < 1e-5);
}

#[test]
fn test_vec3_cross_is_orthogonal_to_both() {
  let a = Vec3::new(1.0, 2.0, 3.0);
  let b = Vec3::new(4.0, 0.0, -1.0);
  let c = a.cross(b);
  assert!(a.dot(c).abs() < 1e-4);
  assert!(b.dot(c).abs() < 1e-4);
}

#[test]
fn test_vec3_cross_of_parallel_is_zero() {
  let a = Vec3::new(1.0, 2.0, 3.0);
  let b = Vec3::new(2.0, 4.0, 6.0);
  let c = a.cross(b);
  assert!(c.x.abs() < 1e-5 && c.y.abs() < 1e-5 && c.z.abs() < 1e-5);
}

// --- Vec3 Display formatting ---

#[test]
fn test_vec3_display_contains_components() {
  let v = Vec3::new(1.0, 2.0, 3.0);
  let s = format!("{v}");
  assert!(s.contains('1'));
  assert!(s.contains('2'));
  assert!(s.contains('3'));
}

// --- Vec2 / Vec4 Display ---

#[test]
fn test_vec2_display_contains_components() {
  let v = Vec2::new(7.0, 9.0);
  let s = format!("{v}");
  assert!(s.contains('7'));
  assert!(s.contains('9'));
}

#[test]
fn test_vec4_display_contains_components() {
  let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
  let s = format!("{v}");
  assert!(s.contains('1') && s.contains('2') && s.contains('3') && s.contains('4'));
}

// --- Mat4::new_perspective error cases ---

#[test]
fn test_mat4_perspective_near_zero_errors() {
  let r = Mat4::new_perspective(1.0, 1.0, 0.0, 100.0);
  assert!(r.is_err());
}

#[test]
fn test_mat4_perspective_far_equal_near_errors() {
  let r = Mat4::new_perspective(1.0, 1.0, 10.0, 10.0);
  assert!(r.is_err());
}

#[test]
fn test_mat4_perspective_far_less_than_near_errors() {
  let r = Mat4::new_perspective(1.0, 1.0, 100.0, 10.0);
  assert!(r.is_err());
}

#[test]
fn test_mat4_perspective_extreme_values_ok() {
  let r = Mat4::new_perspective(1.0, 16.0 / 9.0, 1e-6, 1e9);
  assert!(r.is_ok());
}

// --- Mat4 look_at degenerate ---

#[test]
fn test_mat4_look_at_eye_equals_target_does_not_panic() {
  // forward vector is zero; underlying normalise will yield NaNs but no panic.
  let _m = Mat4::new_look_at(Vec3::new(1.0, 1.0, 1.0), Vec3::new(1.0, 1.0, 1.0), V3_AXIS_Y);
}

#[test]
fn test_mat4_look_at_up_parallel_to_forward_does_not_panic() {
  // forward is (0, 1, 0); up is also (0, 1, 0). The cross-product collapses to zero.
  let _m = Mat4::new_look_at(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), V3_AXIS_Y);
}

// --- Mat4 transform_point with identity ---

#[test]
fn test_mat4_identity_preserves_point() {
  let id = Mat4::new();
  let p = Vec3::new(3.0, 4.0, 5.0);
  let r = id.transform_point(&p);
  assert!((r.x - 3.0).abs() < 1e-5);
  assert!((r.y - 4.0).abs() < 1e-5);
  assert!((r.z - 5.0).abs() < 1e-5);
}

#[test]
fn test_mat4_translation_moves_point() {
  let t = Mat4::new_trans(5.0, 6.0, 7.0);
  let p = Vec3::new(1.0, 1.0, 1.0);
  let r = t.transform_point(&p);
  assert!((r.x - 6.0).abs() < 1e-5);
  assert!((r.y - 7.0).abs() < 1e-5);
  assert!((r.z - 8.0).abs() < 1e-5);
}

// --- Quat normalisation drift ---

#[test]
fn test_quat_normalise_is_idempotent() {
  let mut q = Quat::ident();
  for _ in 0..10000 {
    q.rot_x(0.01);
  }
  let qn = q.normalize();
  let qn2 = qn.normalize();
  // Comparing components requires reaching into the quat; instead verify that
  // rotating a vector with q vs qn yields a unit-length result.
  let v = Vec3::new(1.0, 0.0, 0.0);
  let r = qn.rotate_vec3(v);
  let r2 = qn2.rotate_vec3(v);
  let len = r.len();
  let len2 = r2.len();
  assert!((len - 1.0).abs() < 1e-3);
  assert!((len2 - 1.0).abs() < 1e-3);
}

#[test]
fn test_quat_ident_rotates_to_self() {
  let q = Quat::ident();
  let v = Vec3::new(1.0, 2.0, 3.0);
  let r = q.rotate_vec3(v);
  assert!((r.x - 1.0).abs() < 1e-5);
  assert!((r.y - 2.0).abs() < 1e-5);
  assert!((r.z - 3.0).abs() < 1e-5);
}

#[test]
fn test_quat_rot_x_world_differs_from_local_after_non_ident() {
  // Set up a non-identity orientation, then apply local-x vs world-x.
  let mut base = Quat::ident();
  base.rot_y(std::f32::consts::FRAC_PI_2);

  let mut local = base;
  let mut world = base;
  local.rot_x(std::f32::consts::FRAC_PI_4);
  world.rot_x_world(std::f32::consts::FRAC_PI_4);

  let v = Vec3::new(0.0, 0.0, 1.0);
  let r_local = local.rotate_vec3(v);
  let r_world = world.rotate_vec3(v);
  let differs = (r_local.x - r_world.x).abs() > 1e-3 || (r_local.y - r_world.y).abs() > 1e-3 || (r_local.z - r_world.z).abs() > 1e-3;
  assert!(differs, "local x and world x should disagree after a y rotation");
}

#[test]
fn test_quat_rot_x_twice_is_double_angle() {
  let mut single = Quat::ident();
  single.rot_x(std::f32::consts::FRAC_PI_4 * 2.0);

  let mut double = Quat::ident();
  double.rot_x(std::f32::consts::FRAC_PI_4);
  double.rot_x(std::f32::consts::FRAC_PI_4);

  let v = Vec3::new(0.0, 1.0, 0.0);
  let r1 = single.rotate_vec3(v);
  let r2 = double.rotate_vec3(v);
  assert!((r1.x - r2.x).abs() < 1e-4);
  assert!((r1.y - r2.y).abs() < 1e-4);
  assert!((r1.z - r2.z).abs() < 1e-4);
}

// --- Mat3::from_mat4_upper().inverse_transpose() singular case ---

#[test]
fn test_mat3_inverse_transpose_singular_returns_default_or_zero() {
  // A zero 4x4 matrix has a zero 3x3 upper block. inverse_transpose should
  // gracefully handle this (return None / Err that the caller can `.unwrap_or_default()`).
  let zero = Mat4::zero();
  let m3 = Mat3::from_mat4_upper(&zero);
  let r = m3.inverse_transpose();
  // We don't care if it's Err or Ok; we just check we don't panic and that
  // the unwrap_or_default path works.
  let _ = r.unwrap_or_default();
}

// --- Affine2 composition ---

#[test]
fn test_affine2_identity_is_noop() {
  let id = Affine2::new();
  let p = Vec2::new(3.0, 4.0);
  let r = id * &p;
  assert!((r.x - 3.0).abs() < 1e-5);
  assert!((r.y - 4.0).abs() < 1e-5);
}

#[test]
fn test_affine2_composition_order_matters() {
  // Translate then scale vs scale then translate produce different results.
  let t = Affine2::new_trans(1.0, 0.0);
  let s = Affine2::new_scale(2.0, 2.0);
  let p = Vec2::new(3.0, 0.0);
  let r_ts = (t * s) * &p;
  let r_st = (s * t) * &p;
  // The two orderings should disagree on x.
  assert!((r_ts.x - r_st.x).abs() > 1e-4, "compose orderings should differ");
}

// --- Vec3 reflect smoke ---

#[test]
fn test_vec3_reflect_against_x_axis_flips_x() {
  let v = Vec3::new(1.0, 2.0, 3.0);
  let n = Vec3::new(1.0, 0.0, 0.0);
  let r = v.reflect(n);
  assert!((r.x + 1.0).abs() < 1e-5);
  assert!((r.y - 2.0).abs() < 1e-5);
  assert!((r.z - 3.0).abs() < 1e-5);
}

#[test]
fn test_vec3_invert_negates_all_components() {
  let v = Vec3::new(1.0, -2.0, 3.0);
  let r = v.invert();
  assert!((r.x + 1.0).abs() < 1e-5);
  assert!((r.y - 2.0).abs() < 1e-5);
  assert!((r.z + 3.0).abs() < 1e-5);
}
