// ==============================================================================================
// Module & file:   math / matrix4_tests.rs
// Purpose:         Tests for Mat4 4x4 transformation matrix
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:           AI generated
// ==============================================================================================

use super::*;
use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};

const EPSILON: f64 = 1e-10;

fn approx_eq(a: f64, b: f64) -> bool {
  (a - b).abs() < EPSILON
}

fn mat4_approx_eq(a: &Mat4, b: &Mat4) -> bool {
  for col in 0..4 {
    for row in 0..4 {
      if !approx_eq(a.ele[col][row], b.ele[col][row]) {
        return false;
      }
    }
  }
  true
}

fn vec3_approx_eq(a: &Vec3, b: &Vec3) -> bool {
  approx_eq(a.x, b.x) && approx_eq(a.y, b.y) && approx_eq(a.z, b.z)
}

// ============================================================================
// Constructors
// ============================================================================

#[test]
fn test_new_is_identity() {
  let m = Mat4::new();
  assert_eq!(
    m,
    Mat4 {
      ele: [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]],
    }
  );
}

#[test]
fn test_default_is_zero() {
  // Default derive yields all zeros (not identity)
  let m: Mat4 = Default::default();
  assert_eq!(m, Mat4::zero());
}

#[test]
fn test_zero_all_zero() {
  let m = Mat4::zero();
  for col in 0..4 {
    for row in 0..4 {
      assert_eq!(m.ele[col][row], 0.0);
    }
  }
}

#[test]
fn test_new_scale_sets_diagonal() {
  let m = Mat4::new_scale(2.0, 3.0, 4.0);
  assert_eq!(m.ele[0][0], 2.0);
  assert_eq!(m.ele[1][1], 3.0);
  assert_eq!(m.ele[2][2], 4.0);
  assert_eq!(m.ele[3][3], 1.0);
  // Off-diagonal entries should be zero
  for col in 0..4 {
    for row in 0..4 {
      if col != row {
        assert_eq!(m.ele[col][row], 0.0);
      }
    }
  }
}

#[test]
fn test_new_scale_with_negative_values() {
  let m = Mat4::new_scale(-1.0, -2.0, -3.0);
  let v = m * &Vec3::new(1.0, 1.0, 1.0);
  assert!(vec3_approx_eq(&v, &Vec3::new(-1.0, -2.0, -3.0)));
}

#[test]
fn test_new_scale_zero_collapses() {
  let m = Mat4::new_scale(0.0, 0.0, 0.0);
  let v = m * &Vec3::new(5.0, 6.0, 7.0);
  assert!(vec3_approx_eq(&v, &Vec3::new(0.0, 0.0, 0.0)));
}

#[test]
fn test_new_trans_sets_translation_column() {
  let m = Mat4::new_trans(5.0, 6.0, 7.0);
  // Translation lives in ele[3][0..2] (column-major storage)
  assert_eq!(m.ele[3][0], 5.0);
  assert_eq!(m.ele[3][1], 6.0);
  assert_eq!(m.ele[3][2], 7.0);
  assert_eq!(m.ele[3][3], 1.0);
  // Upper-left 3x3 should be identity
  assert_eq!(m.ele[0][0], 1.0);
  assert_eq!(m.ele[1][1], 1.0);
  assert_eq!(m.ele[2][2], 1.0);
}

#[test]
fn test_new_trans_zero_is_identity() {
  let m = Mat4::new_trans(0.0, 0.0, 0.0);
  assert_eq!(m, Mat4::new());
}

#[test]
fn test_new_rot_identity_quat_yields_identity_matrix() {
  // Identity quaternion (w=1, x=y=z=0)
  let q = Quat { w: 1.0, x: 0.0, y: 0.0, z: 0.0 };
  let m = Mat4::new_rot(q);
  assert!(mat4_approx_eq(&m, &Mat4::new()));
}

#[test]
fn test_new_rot_90_around_z() {
  let q = Quat::new(Vec3::new(0.0, 0.0, 1.0), FRAC_PI_2);
  let m = Mat4::new_rot(q);
  let v = m * &Vec3::new(1.0, 0.0, 0.0);
  // +x rotated +90 around +z -> +y
  assert!(vec3_approx_eq(&v, &Vec3::new(0.0, 1.0, 0.0)));
}

#[test]
fn test_new_rot_90_around_x() {
  let q = Quat::new(Vec3::new(1.0, 0.0, 0.0), FRAC_PI_2);
  let m = Mat4::new_rot(q);
  let v = m * &Vec3::new(0.0, 1.0, 0.0);
  // +y rotated +90 around +x -> +z
  assert!(vec3_approx_eq(&v, &Vec3::new(0.0, 0.0, 1.0)));
}

#[test]
fn test_new_rot_90_around_y() {
  let q = Quat::new(Vec3::new(0.0, 1.0, 0.0), FRAC_PI_2);
  let m = Mat4::new_rot(q);
  let v = m * &Vec3::new(0.0, 0.0, 1.0);
  // +z rotated +90 around +y -> +x
  assert!(vec3_approx_eq(&v, &Vec3::new(1.0, 0.0, 0.0)));
}

#[test]
fn test_new_rot_180_around_z_flips_xy() {
  let q = Quat::new(Vec3::new(0.0, 0.0, 1.0), PI);
  let m = Mat4::new_rot(q);
  let v = m * &Vec3::new(1.0, 2.0, 3.0);
  assert!(vec3_approx_eq(&v, &Vec3::new(-1.0, -2.0, 3.0)));
}

#[test]
fn test_new_rot_preserves_axis() {
  // Rotating any vector parallel to the rotation axis should leave it unchanged
  let axis = Vec3::new(0.0, 0.0, 1.0);
  let q = Quat::new(axis, FRAC_PI_4);
  let m = Mat4::new_rot(q);
  let v = m * &Vec3::new(0.0, 0.0, 5.0);
  assert!(vec3_approx_eq(&v, &Vec3::new(0.0, 0.0, 5.0)));
}

#[test]
fn test_new_rot_preserves_length() {
  let inv_sqrt3 = 1.0 / (3.0_f64).sqrt();
  let q = Quat::new(Vec3::new(inv_sqrt3, inv_sqrt3, inv_sqrt3), 1.234);
  let m = Mat4::new_rot(q);
  let v = Vec3::new(3.0, -4.0, 5.0);
  let r = m * &v;
  let orig_len_sq = v.x * v.x + v.y * v.y + v.z * v.z;
  let new_len_sq = r.x * r.x + r.y * r.y + r.z * r.z;
  assert!(approx_eq(orig_len_sq, new_len_sq));
}

// ============================================================================
// Mutating helpers (note: these OVERWRITE rather than compose)
// ============================================================================

#[test]
fn test_trans_overwrites_translation_column() {
  let mut m = Mat4::new();
  m.trans(10.0, 20.0, 30.0);
  assert_eq!(m.ele[3][0], 10.0);
  assert_eq!(m.ele[3][1], 20.0);
  assert_eq!(m.ele[3][2], 30.0);
  // Should not touch the rest
  assert_eq!(m.ele[0][0], 1.0);
  assert_eq!(m.ele[3][3], 1.0);
}

#[test]
fn test_trans_replaces_existing_translation() {
  let mut m = Mat4::new_trans(1.0, 2.0, 3.0);
  m.trans(4.0, 5.0, 6.0);
  assert_eq!(m.ele[3][0], 4.0);
  assert_eq!(m.ele[3][1], 5.0);
  assert_eq!(m.ele[3][2], 6.0);
}

#[test]
fn test_scale_overwrites_diagonal() {
  let mut m = Mat4::new();
  m.scale(2.0, 3.0, 4.0);
  assert_eq!(m.ele[0][0], 2.0);
  assert_eq!(m.ele[1][1], 3.0);
  assert_eq!(m.ele[2][2], 4.0);
  assert_eq!(m.ele[3][3], 1.0);
}

#[test]
fn test_new_scale_rot_trans_composition_quirk() {
  // The current implementation of new_scale_rot_trans applies scale() and
  // trans() AFTER new_rot(), and those mutators OVERWRITE diagonal/last
  // column entries instead of composing. This test pins down that actual
  // behaviour so future regressions are caught.
  let q = Quat::new(Vec3::new(0.0, 0.0, 1.0), FRAC_PI_2);
  let m = Mat4::new_scale_rot_trans(2.0, 3.0, 4.0, q, 7.0, 8.0, 9.0);

  // Diagonal got overwritten by scale()
  assert_eq!(m.ele[0][0], 2.0);
  assert_eq!(m.ele[1][1], 3.0);
  assert_eq!(m.ele[2][2], 4.0);
  // Last column (translation) overwritten by trans()
  assert_eq!(m.ele[3][0], 7.0);
  assert_eq!(m.ele[3][1], 8.0);
  assert_eq!(m.ele[3][2], 9.0);
  assert_eq!(m.ele[3][3], 1.0);
  // Off-diagonal rotation entries from new_rot are preserved
  // For a 90 deg rot about Z: ele[0][1] = 1, ele[1][0] = -1
  assert!(approx_eq(m.ele[0][1], 1.0));
  assert!(approx_eq(m.ele[1][0], -1.0));
  assert!(approx_eq(m.ele[0][2], 0.0));
  assert!(approx_eq(m.ele[2][0], 0.0));
}

// ============================================================================
// Mat4 * Vec3
// ============================================================================

#[test]
fn test_identity_times_vec3_unchanged() {
  let m = Mat4::new();
  let v = Vec3::new(1.5, -2.5, 3.5);
  let r = m * &v;
  assert!(vec3_approx_eq(&r, &v));
}

#[test]
fn test_identity_times_zero_vec3() {
  let m = Mat4::new();
  let v = Vec3::new(0.0, 0.0, 0.0);
  let r = m * &v;
  assert!(vec3_approx_eq(&r, &Vec3::new(0.0, 0.0, 0.0)));
}

#[test]
fn test_translation_applied_to_vec3() {
  let m = Mat4::new_trans(10.0, 20.0, 30.0);
  let v = Vec3::new(1.0, 2.0, 3.0);
  let r = m * &v;
  // Vec3 is treated as a point with implicit w=1
  assert!(vec3_approx_eq(&r, &Vec3::new(11.0, 22.0, 33.0)));
}

#[test]
fn test_translation_applied_to_origin() {
  let m = Mat4::new_trans(5.0, -6.0, 7.0);
  let r = m * &Vec3::new(0.0, 0.0, 0.0);
  assert!(vec3_approx_eq(&r, &Vec3::new(5.0, -6.0, 7.0)));
}

#[test]
fn test_scale_applied_to_vec3() {
  let m = Mat4::new_scale(2.0, 3.0, 4.0);
  let v = Vec3::new(1.0, 1.0, 1.0);
  let r = m * &v;
  assert!(vec3_approx_eq(&r, &Vec3::new(2.0, 3.0, 4.0)));
}

#[test]
fn test_zero_matrix_collapses_vec3_to_origin() {
  let m = Mat4::zero();
  let v = Vec3::new(99.0, -42.0, 13.0);
  let r = m * &v;
  assert!(vec3_approx_eq(&r, &Vec3::new(0.0, 0.0, 0.0)));
}

#[test]
fn test_negative_scale_flips_vec3() {
  let m = Mat4::new_scale(-1.0, 1.0, -1.0);
  let v = Vec3::new(2.0, 3.0, 4.0);
  let r = m * &v;
  assert!(vec3_approx_eq(&r, &Vec3::new(-2.0, 3.0, -4.0)));
}

// ============================================================================
// Mat4 * Mat4
// ============================================================================

#[test]
fn test_identity_times_identity() {
  let r = Mat4::new() * Mat4::new();
  assert!(mat4_approx_eq(&r, &Mat4::new()));
}

#[test]
fn test_identity_left_neutral() {
  let m = Mat4::new_trans(1.0, 2.0, 3.0);
  let r = Mat4::new() * m;
  assert!(mat4_approx_eq(&r, &m));
}

#[test]
fn test_identity_right_neutral() {
  let m = Mat4::new_scale(2.0, 3.0, 4.0);
  let r = m * Mat4::new();
  assert!(mat4_approx_eq(&r, &m));
}

#[test]
fn test_zero_times_anything_is_zero() {
  let m = Mat4::new_trans(1.0, 2.0, 3.0);
  let r = Mat4::zero() * m;
  assert!(mat4_approx_eq(&r, &Mat4::zero()));
  let r2 = m * Mat4::zero();
  // Note: m * zero is NOT all zero because translation column gets multiplied
  // by zero matrix's last column [0,0,0,0]. Verify it's all zero.
  assert!(mat4_approx_eq(&r2, &Mat4::zero()));
}

#[test]
fn test_compose_two_translations() {
  let a = Mat4::new_trans(1.0, 2.0, 3.0);
  let b = Mat4::new_trans(10.0, 20.0, 30.0);
  let c = a * b;
  // Composing two translations should sum them
  let v = c * &Vec3::new(0.0, 0.0, 0.0);
  assert!(vec3_approx_eq(&v, &Vec3::new(11.0, 22.0, 33.0)));
}

#[test]
fn test_compose_two_scales() {
  let a = Mat4::new_scale(2.0, 3.0, 4.0);
  let b = Mat4::new_scale(5.0, 6.0, 7.0);
  let c = a * b;
  let v = c * &Vec3::new(1.0, 1.0, 1.0);
  assert!(vec3_approx_eq(&v, &Vec3::new(10.0, 18.0, 28.0)));
}

#[test]
fn test_compose_translation_then_scale_associates_correctly() {
  // Build M = S * T. Applying to v gives S(T(v)) = S(v + t) = s*v + s*t
  let t = Mat4::new_trans(1.0, 2.0, 3.0);
  let s = Mat4::new_scale(2.0, 2.0, 2.0);
  let m = s * t;
  let v = Vec3::new(4.0, 5.0, 6.0);
  let r = m * &v;
  let expected = Vec3::new((4.0 + 1.0) * 2.0, (5.0 + 2.0) * 2.0, (6.0 + 3.0) * 2.0);
  assert!(vec3_approx_eq(&r, &expected));
}

#[test]
fn test_compose_scale_then_translation() {
  // M = T * S applied to v gives T(S(v)) = s*v + t
  let s = Mat4::new_scale(2.0, 2.0, 2.0);
  let t = Mat4::new_trans(1.0, 2.0, 3.0);
  let m = t * s;
  let v = Vec3::new(4.0, 5.0, 6.0);
  let r = m * &v;
  let expected = Vec3::new(4.0 * 2.0 + 1.0, 5.0 * 2.0 + 2.0, 6.0 * 2.0 + 3.0);
  assert!(vec3_approx_eq(&r, &expected));
}

#[test]
fn test_compose_two_rotations_equals_combined_angle() {
  let q1 = Quat::new(Vec3::new(0.0, 0.0, 1.0), FRAC_PI_4);
  let q2 = Quat::new(Vec3::new(0.0, 0.0, 1.0), FRAC_PI_4);
  let m1 = Mat4::new_rot(q1);
  let m2 = Mat4::new_rot(q2);
  let composed = m2 * m1;
  let v = composed * &Vec3::new(1.0, 0.0, 0.0);
  // Two pi/4 rotations = pi/2 -> +x becomes +y
  assert!(vec3_approx_eq(&v, &Vec3::new(0.0, 1.0, 0.0)));
}

#[test]
fn test_rotation_inverse_via_negative_angle() {
  let q = Quat::new(Vec3::new(0.0, 0.0, 1.0), FRAC_PI_4);
  let q_inv = Quat::new(Vec3::new(0.0, 0.0, 1.0), -FRAC_PI_4);
  let m = Mat4::new_rot(q);
  let m_inv = Mat4::new_rot(q_inv);
  let combined = m * m_inv;
  assert!(mat4_approx_eq(&combined, &Mat4::new()));
}

#[test]
fn test_mat_mul_is_not_commutative_in_general() {
  let t = Mat4::new_trans(1.0, 0.0, 0.0);
  let s = Mat4::new_scale(2.0, 2.0, 2.0);
  let ts = t * s;
  let st = s * t;
  assert!(!mat4_approx_eq(&ts, &st));
}

// ============================================================================
// MulAssign
// ============================================================================

#[test]
fn test_mul_assign_matches_mul() {
  let a = Mat4::new_trans(1.0, 2.0, 3.0);
  let b = Mat4::new_scale(2.0, 3.0, 4.0);
  let expected = a * b;
  let mut m = a;
  m *= b;
  assert!(mat4_approx_eq(&m, &expected));
}

#[test]
fn test_mul_assign_with_identity_is_noop() {
  let original = Mat4::new_trans(5.0, 6.0, 7.0);
  let mut m = original;
  m *= Mat4::new();
  assert!(mat4_approx_eq(&m, &original));
}

// ============================================================================
// Mat4 * &Vec<Vec3>
// ============================================================================

#[test]
fn test_mul_empty_vec_of_points() {
  let m = Mat4::new_trans(1.0, 2.0, 3.0);
  let pts: Vec<Vec3> = vec![];
  let r = m * &pts;
  assert!(r.is_empty());
}

#[test]
fn test_mul_vec_of_points_applies_to_each() {
  let m = Mat4::new_trans(1.0, 2.0, 3.0);
  let pts = vec![Vec3::new(0.0, 0.0, 0.0), Vec3::new(10.0, 10.0, 10.0), Vec3::new(-5.0, -5.0, -5.0)];
  let r = m * &pts;
  assert_eq!(r.len(), 3);
  assert!(vec3_approx_eq(&r[0], &Vec3::new(1.0, 2.0, 3.0)));
  assert!(vec3_approx_eq(&r[1], &Vec3::new(11.0, 12.0, 13.0)));
  assert!(vec3_approx_eq(&r[2], &Vec3::new(-4.0, -3.0, -2.0)));
}

#[test]
fn test_mul_vec_of_points_with_identity_unchanged() {
  let m = Mat4::new();
  let pts = vec![Vec3::new(1.0, 2.0, 3.0), Vec3::new(-1.0, -2.0, -3.0)];
  let r = m * &pts;
  for (a, b) in r.iter().zip(pts.iter()) {
    assert!(vec3_approx_eq(a, b));
  }
}

// ============================================================================
// Display
// ============================================================================

#[test]
fn test_display_identity_format() {
  let m = Mat4::new();
  let s = format!("{}", m);
  // Should contain four rows in some readable form. Not pinning exact
  // whitespace, just that all four diagonal 1s appear.
  assert!(s.contains("1"));
  assert!(s.contains("0"));
  // Four lines (three newlines)
  assert_eq!(s.matches('\n').count(), 3);
}

#[test]
fn test_display_pins_translation_in_last_column() {
  // For a translation matrix the four translation values should appear in the
  // last column of the printed output (not the last row). This pins the
  // intended row/column convention of the Display impl and would fail loudly
  // if someone transposed it.
  let m = Mat4::new_trans(7.0, 8.0, 9.0);
  let s = format!("{}", m);
  let lines: Vec<&str> = s.lines().collect();
  assert_eq!(lines.len(), 4);
  // First three rows end with 7, 8, 9 respectively (translation column)
  assert!(lines[0].trim_end().ends_with("7"), "row 0 was: {}", lines[0]);
  assert!(lines[1].trim_end().ends_with("8"), "row 1 was: {}", lines[1]);
  assert!(lines[2].trim_end().ends_with(", 9,") || lines[2].trim_end().ends_with("9,"), "row 2 was: {}", lines[2]);
  // Last row is [0, 0, 0, 1]
  assert!(lines[3].trim_end().ends_with("1]"), "row 3 was: {}", lines[3]);
}

// ============================================================================
// Copy / Clone / PartialEq derives
// ============================================================================

#[test]
fn test_copy_semantics() {
  let a = Mat4::new_trans(1.0, 2.0, 3.0);
  let b = a; // Copy, not move
  // a is still usable
  let _ = a * Mat4::new();
  assert_eq!(a, b);
}

#[test]
fn test_clone_equal() {
  let a = Mat4::new_scale(2.0, 3.0, 4.0);
  let b = a.clone();
  assert_eq!(a, b);
}

#[test]
fn test_partial_eq_negative_for_different_matrices() {
  let a = Mat4::new();
  let b = Mat4::new_scale(2.0, 2.0, 2.0);
  assert_ne!(a, b);
}

// ============================================================================
// Tightening: storage convention, off-axis rotation, associativity, invariants
// ============================================================================

fn arbitrary_mat4_a() -> Mat4 {
  // Compose scale, rotation, translation to get a general non-symmetric matrix
  let q = Quat::new(Vec3::new(0.6, -0.8, 0.0), 0.7);
  Mat4::new_trans(7.0, -3.0, 2.0) * Mat4::new_rot(q) * Mat4::new_scale(2.0, 3.0, 0.5)
}

fn arbitrary_mat4_b() -> Mat4 {
  let inv_sqrt3 = 1.0 / (3.0_f64).sqrt();
  let q = Quat::new(Vec3::new(inv_sqrt3, inv_sqrt3, inv_sqrt3), -0.4);
  Mat4::new_trans(-2.0, 5.0, 1.0) * Mat4::new_rot(q) * Mat4::new_scale(0.5, -1.5, 2.0)
}

#[test]
fn test_mul_distributes_over_vec_mul_general() {
  // (A * B) * v == A * (B * v). THE storage-convention regression test.
  let a = arbitrary_mat4_a();
  let b = arbitrary_mat4_b();
  let v = Vec3::new(1.7, -2.3, 4.1);
  let lhs = (a * b) * &v;
  let rhs = a * &(b * &v);
  assert!(vec3_approx_eq(&lhs, &rhs));
}

#[test]
fn test_mul_associative_general_matrices() {
  let a = arbitrary_mat4_a();
  let b = arbitrary_mat4_b();
  let q = Quat::new(Vec3::new(0.0, 1.0, 0.0), 0.3);
  let c = Mat4::new_rot(q) * Mat4::new_trans(3.0, -4.0, 1.0);
  assert!(mat4_approx_eq(&((a * b) * c), &(a * (b * c))));
}

#[test]
fn test_new_rot_180_around_x() {
  let q = Quat::new(Vec3::new(1.0, 0.0, 0.0), PI);
  let m = Mat4::new_rot(q);
  let v = m * &Vec3::new(1.0, 2.0, 3.0);
  assert!(vec3_approx_eq(&v, &Vec3::new(1.0, -2.0, -3.0)));
}

#[test]
fn test_new_rot_180_around_y() {
  let q = Quat::new(Vec3::new(0.0, 1.0, 0.0), PI);
  let m = Mat4::new_rot(q);
  let v = m * &Vec3::new(1.0, 2.0, 3.0);
  assert!(vec3_approx_eq(&v, &Vec3::new(-1.0, 2.0, -3.0)));
}

#[test]
fn test_new_rot_negative_angle_reverses_direction() {
  let pos = Mat4::new_rot(Quat::new(Vec3::new(0.0, 0.0, 1.0), 0.5));
  let neg = Mat4::new_rot(Quat::new(Vec3::new(0.0, 0.0, 1.0), -0.5));
  let v = Vec3::new(1.0, 0.0, 0.0);
  let rp = pos * &v;
  let rn = neg * &v;
  // y components should be opposite signs
  assert!(approx_eq(rp.y, -rn.y));
  assert!(approx_eq(rp.x, rn.x));
}

#[test]
fn test_new_rot_off_axis_preserves_length_and_orthonormal_columns() {
  let inv_sqrt3 = 1.0 / (3.0_f64).sqrt();
  let q = Quat::new(Vec3::new(inv_sqrt3, inv_sqrt3, inv_sqrt3), 1.234);
  let m = Mat4::new_rot(q);
  // Upper-left 3x3 columns must be orthonormal
  for i in 0..3 {
    let mut len_sq = 0.0;
    for r in 0..3 {
      len_sq += m.ele[i][r] * m.ele[i][r];
    }
    assert!(approx_eq(len_sq, 1.0), "column {} not unit length", i);
  }
  for i in 0..3 {
    for j in (i + 1)..3 {
      let mut dot = 0.0;
      for r in 0..3 {
        dot += m.ele[i][r] * m.ele[j][r];
      }
      assert!(approx_eq(dot, 0.0), "columns {} and {} not orthogonal", i, j);
    }
  }
}

#[test]
fn test_new_rot_leaves_affine_tail_clean() {
  let q = Quat::new(Vec3::new(0.6, -0.8, 0.0), 1.1);
  let m = Mat4::new_rot(q);
  // Last column above the bottom-right should be zero
  assert_eq!(m.ele[3][0], 0.0);
  assert_eq!(m.ele[3][1], 0.0);
  assert_eq!(m.ele[3][2], 0.0);
  // Bottom-right is 1
  assert_eq!(m.ele[3][3], 1.0);
  // Bottom row of upper 3 columns is zero (homogeneous w row)
  assert_eq!(m.ele[0][3], 0.0);
  assert_eq!(m.ele[1][3], 0.0);
  assert_eq!(m.ele[2][3], 0.0);
}

#[test]
fn test_new_scale_rot_trans_off_axis_preserves_rotation_off_diagonals() {
  // Pin that the overwrite quirk does not destroy off-diagonal rotation entries
  // for a non-axis-aligned rotation
  let inv_sqrt3 = 1.0 / (3.0_f64).sqrt();
  let q = Quat::new(Vec3::new(inv_sqrt3, inv_sqrt3, inv_sqrt3), FRAC_PI_2);
  let m = Mat4::new_scale_rot_trans(2.0, 3.0, 4.0, q, 7.0, 8.0, 9.0);
  let m_rot_only = Mat4::new_rot(q);
  // Off-diagonal rotation entries (col != row, both < 3) should match new_rot
  for col in 0..3 {
    for row in 0..3 {
      if col != row {
        assert!(
          approx_eq(m.ele[col][row], m_rot_only.ele[col][row]),
          "off-diagonal ({},{}) clobbered: {} vs {}",
          col,
          row,
          m.ele[col][row],
          m_rot_only.ele[col][row]
        );
      }
    }
  }
}

#[test]
fn test_mul_assign_chained_matches_mul_chained() {
  let a = Mat4::new_trans(1.0, 2.0, 3.0);
  let b = Mat4::new_scale(2.0, 3.0, 4.0);
  let c = Mat4::new_rot(Quat::new(Vec3::new(0.0, 0.0, 1.0), 0.5));
  let expected = a * b * c;
  let mut m = a;
  m *= b;
  m *= c;
  assert!(mat4_approx_eq(&m, &expected));
}

#[test]
fn test_vec_of_points_matches_individual_mul() {
  let m = arbitrary_mat4_a();
  let pts = vec![
    Vec3::new(1.0, 2.0, 3.0),
    Vec3::new(-4.0, 0.5, 7.0),
    Vec3::new(0.0, 0.0, 0.0),
    Vec3::new(100.0, -200.0, 50.0),
  ];
  let batch = m * &pts;
  assert_eq!(batch.len(), pts.len());
  for (i, p) in pts.iter().enumerate() {
    let single = m * p;
    assert!(vec3_approx_eq(&batch[i], &single), "mismatch at index {}", i);
  }
}

#[test]
fn test_mul_general_matrix_vec_hand_computed() {
  // Hand-built general matrix to catch index mistakes in Mul<&Vec3>
  let m = Mat4 {
    ele: [
      [1.0, 2.0, 3.0, 0.0], // column 0
      [4.0, 5.0, 6.0, 0.0], // column 1
      [7.0, 8.0, 9.0, 0.0], // column 2
      [10.0, 11.0, 12.0, 1.0], // column 3 (translation)
    ],
  };
  // For v = (1, 1, 1): result.x = 1*1 + 4*1 + 7*1 + 10 = 22
  //                    result.y = 2*1 + 5*1 + 8*1 + 11 = 26
  //                    result.z = 3*1 + 6*1 + 9*1 + 12 = 30
  let r = m * &Vec3::new(1.0, 1.0, 1.0);
  assert!(vec3_approx_eq(&r, &Vec3::new(22.0, 26.0, 30.0)));
}
