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
fn test_new_scale_rot_trans_matches_t_r_s() {
  let q = Quat::new(Vec3::new(0.0, 0.0, 1.0), FRAC_PI_2);
  let combined = Mat4::new_scale_rot_trans(2.0, 3.0, 4.0, q, 7.0, 8.0, 9.0);
  let separate = Mat4::new_trans(7.0, 8.0, 9.0) * Mat4::new_rot(q) * Mat4::new_scale(2.0, 3.0, 4.0);
  // approx compare element-wise rather than == (floating point)
  for c in 0..4 {
    for r in 0..4 {
      assert!(
        approx_eq(combined.ele[c][r], separate.ele[c][r]),
        "mismatch at ele[{}][{}]: {} vs {}",
        c,
        r,
        combined.ele[c][r],
        separate.ele[c][r]
      );
    }
  }
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
      [1.0, 2.0, 3.0, 0.0],    // column 0
      [4.0, 5.0, 6.0, 0.0],    // column 1
      [7.0, 8.0, 9.0, 0.0],    // column 2
      [10.0, 11.0, 12.0, 1.0], // column 3 (translation)
    ],
  };
  // For v = (1, 1, 1): result.x = 1*1 + 4*1 + 7*1 + 10 = 22
  //                    result.y = 2*1 + 5*1 + 8*1 + 11 = 26
  //                    result.z = 3*1 + 6*1 + 9*1 + 12 = 30
  let r = m * &Vec3::new(1.0, 1.0, 1.0);
  assert!(vec3_approx_eq(&r, &Vec3::new(22.0, 26.0, 30.0)));
}

// ============================================================================
// new_perspective
//
// Right-handed perspective with camera looking down -Z. Maps view-space z in
// [-near, -far] to NDC z in [-1, +1] after the perspective divide by clip.w.
// Top two rows project x/y; near/far only affect depth and clip.w.
// ============================================================================

fn vec4_approx_eq(a: &Vec4, b: &Vec4) -> bool {
  approx_eq(a.x, b.x) && approx_eq(a.y, b.y) && approx_eq(a.z, b.z) && approx_eq(a.w, b.w)
}

#[test]
fn test_new_perspective_layout_for_right_handed_minus_z() {
  let p = Mat4::new_perspective(FRAC_PI_2, 1.0, 1.0, 100.0);
  // [2][3] should be -1 (this drives clip.w = -z_view)
  assert!(approx_eq(p.ele[2][3], -1.0));
  // [3][3] should be 0 (a true projection, not affine)
  assert!(approx_eq(p.ele[3][3], 0.0));
  // No translation in x/y rows
  assert!(approx_eq(p.ele[3][0], 0.0));
  assert!(approx_eq(p.ele[3][1], 0.0));
}

#[test]
fn test_new_perspective_fov_90_unit_aspect_xy_scale_is_one() {
  // tan(45) = 1, so cotangent is 1. For 90deg fovy with aspect=1, [0][0] and [1][1] should both be 1
  let p = Mat4::new_perspective(FRAC_PI_2, 1.0, 0.1, 100.0);
  assert!(approx_eq(p.ele[0][0], 1.0));
  assert!(approx_eq(p.ele[1][1], 1.0));
}

#[test]
fn test_new_perspective_aspect_scales_x_only() {
  // Aspect 2.0 (wide): [0][0] = f/aspect, [1][1] = f. So x-scale is half y-scale.
  let p = Mat4::new_perspective(FRAC_PI_2, 2.0, 0.1, 100.0);
  assert!(approx_eq(p.ele[0][0], 0.5));
  assert!(approx_eq(p.ele[1][1], 1.0));
}

#[test]
fn test_new_perspective_z_near_maps_to_minus_one_in_ndc() {
  // A point at view-space z = -near should land on the near plane (NDC z = -1)
  let near = 0.5;
  let far = 100.0;
  let p = Mat4::new_perspective(FRAC_PI_2, 1.0, near, far);
  // View-space point on the camera's forward ray, at z = -near
  let v_view = Vec4::new(0.0, 0.0, -near, 1.0);
  let clip = p * &v_view;
  // After perspective divide
  let ndc_z = clip.z / clip.w;
  assert!(approx_eq(ndc_z, -1.0));
}

#[test]
fn test_new_perspective_z_far_maps_to_plus_one_in_ndc() {
  let near = 0.5;
  let far = 100.0;
  let p = Mat4::new_perspective(FRAC_PI_2, 1.0, near, far);
  let v_view = Vec4::new(0.0, 0.0, -far, 1.0);
  let clip = p * &v_view;
  let ndc_z = clip.z / clip.w;
  assert!(approx_eq(ndc_z, 1.0));
}

#[test]
fn test_new_perspective_clip_w_equals_minus_z_view() {
  // The classic perspective trick: clip.w receives -z_view via the [2][3] = -1 entry.
  // For an arbitrary view-space point this must hold.
  let p = Mat4::new_perspective(FRAC_PI_2, 1.6, 0.1, 100.0);
  let v = Vec4::new(2.0, -3.0, -7.5, 1.0);
  let clip = p * &v;
  assert!(approx_eq(clip.w, -v.z));
}

#[test]
fn test_new_perspective_centre_axis_projects_to_origin() {
  // A point on the camera's forward ray (x=y=0) projects to the centre of the
  // screen no matter how far it is.
  let p = Mat4::new_perspective(FRAC_PI_2, 1.0, 0.1, 100.0);
  for &z in &[-0.2, -1.0, -10.0, -50.0] {
    let clip = p * &Vec4::new(0.0, 0.0, z, 1.0);
    let inv_w = 1.0 / clip.w;
    let ndc_x = clip.x * inv_w;
    let ndc_y = clip.y * inv_w;
    assert!(approx_eq(ndc_x, 0.0), "ndc_x at z={} was {}", z, ndc_x);
    assert!(approx_eq(ndc_y, 0.0), "ndc_y at z={} was {}", z, ndc_y);
  }
}

#[test]
fn test_new_perspective_foreshortening_scales_with_distance() {
  // Same x at farther z should produce smaller ndc_x. Specifically with fov=90
  // and aspect=1, ndc_x = x / -z. Verify the ratio.
  let p = Mat4::new_perspective(FRAC_PI_2, 1.0, 0.1, 100.0);
  let near_pt = p * &Vec4::new(1.0, 0.0, -1.0, 1.0);
  let far_pt = p * &Vec4::new(1.0, 0.0, -10.0, 1.0);
  let ndc_near_x = near_pt.x / near_pt.w;
  let ndc_far_x = far_pt.x / far_pt.w;
  // 10x further away -> 1/10th the screen-space x
  assert!(approx_eq(ndc_near_x, 1.0));
  assert!(approx_eq(ndc_far_x, 0.1));
}

#[test]
fn test_new_perspective_changing_far_does_not_change_xy_projection() {
  // Pin: x/y outputs depend only on fovy and aspect, not on near/far
  let p1 = Mat4::new_perspective(FRAC_PI_4, 1.6, 0.1, 50.0);
  let p2 = Mat4::new_perspective(FRAC_PI_4, 1.6, 0.1, 5000.0);
  let v = Vec4::new(0.7, -0.3, -2.0, 1.0);
  let c1 = p1 * &v;
  let c2 = p2 * &v;
  assert!(approx_eq(c1.x / c1.w, c2.x / c2.w));
  assert!(approx_eq(c1.y / c1.w, c2.y / c2.w));
}

#[test]
fn test_new_perspective_changing_near_does_not_change_xy_projection() {
  let p1 = Mat4::new_perspective(FRAC_PI_4, 1.6, 0.01, 100.0);
  let p2 = Mat4::new_perspective(FRAC_PI_4, 1.6, 5.0, 100.0);
  let v = Vec4::new(0.7, -0.3, -10.0, 1.0);
  let c1 = p1 * &v;
  let c2 = p2 * &v;
  assert!(approx_eq(c1.x / c1.w, c2.x / c2.w));
  assert!(approx_eq(c1.y / c1.w, c2.y / c2.w));
}

#[test]
fn test_new_perspective_off_axis_xy_scaling() {
  // With fovy = 90 deg, a point at (1, 1, -1) should land at ndc (1, 1).
  let p = Mat4::new_perspective(FRAC_PI_2, 1.0, 0.1, 100.0);
  let clip = p * &Vec4::new(1.0, 1.0, -1.0, 1.0);
  let ndc_x = clip.x / clip.w;
  let ndc_y = clip.y / clip.w;
  assert!(approx_eq(ndc_x, 1.0));
  assert!(approx_eq(ndc_y, 1.0));
}

// ============================================================================
// new_look_at
//
// Right-handed view matrix. Camera at `eye` pointing at `target`, with
// `up_hint` providing the rough world-up. Output transforms world space into
// view space (camera at origin, looking down -Z, +X right, +Y up).
// ============================================================================

#[test]
fn test_look_at_camera_at_origin_looking_minus_z_is_identity() {
  // Camera at origin looking down -Z with world-up Y is the canonical view
  // orientation; it should produce the identity matrix.
  let m = Mat4::new_look_at(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 1.0, 0.0));
  assert!(mat4_approx_eq(&m, &Mat4::new()));
}

#[test]
fn test_look_at_translates_eye_to_origin() {
  // The eye should be transformed to the origin in view space, regardless of
  // where it is in the world.
  let eye = Vec3::new(3.0, 5.0, 8.0);
  let target = Vec3::new(0.0, 0.0, 0.0);
  let up = Vec3::new(0.0, 1.0, 0.0);
  let m = Mat4::new_look_at(eye, target, up);
  let r = m * &eye;
  assert!(vec3_approx_eq(&r, &Vec3::new(0.0, 0.0, 0.0)));
}

#[test]
fn test_look_at_target_lands_in_front_of_camera_minus_z() {
  // The target point should appear on the camera's forward axis (-Z in view space)
  // with negative z (i.e. in front of the camera).
  let eye = Vec3::new(0.0, 0.0, 5.0);
  let target = Vec3::new(0.0, 0.0, 0.0);
  let m = Mat4::new_look_at(eye, target, Vec3::new(0.0, 1.0, 0.0));
  let r = m * &target;
  // x and y should be 0, z should be negative (in front of camera)
  assert!(approx_eq(r.x, 0.0));
  assert!(approx_eq(r.y, 0.0));
  assert!(r.z < 0.0);
  // Specifically the distance from eye to target was 5, so z should be -5
  assert!(approx_eq(r.z, -5.0));
}

#[test]
fn test_look_at_pure_translation_when_axes_align() {
  // With camera at (0,0,5) looking at the origin and up=Y, the camera's local
  // axes already align with world axes, so the matrix should be a pure
  // translation by (0, 0, -5).
  let m = Mat4::new_look_at(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
  assert!(mat4_approx_eq(&m, &Mat4::new_trans(0.0, 0.0, -5.0)));
}

#[test]
fn test_look_at_rotation_columns_are_orthonormal() {
  // The upper-left 3x3 is the world->view rotation. It should be orthonormal
  // (its columns are unit length and mutually perpendicular).
  let eye = Vec3::new(4.0, 3.0, 2.0);
  let target = Vec3::new(-1.0, 0.5, -2.0);
  let up = Vec3::new(0.0, 1.0, 0.0);
  let m = Mat4::new_look_at(eye, target, up);
  for i in 0..3 {
    let mut len_sq = 0.0;
    for r in 0..3 {
      len_sq += m.ele[i][r] * m.ele[i][r];
    }
    assert!(approx_eq(len_sq, 1.0), "view-rot column {} not unit length", i);
  }
  for i in 0..3 {
    for j in (i + 1)..3 {
      let mut dot = 0.0;
      for r in 0..3 {
        dot += m.ele[i][r] * m.ele[j][r];
      }
      assert!(approx_eq(dot, 0.0), "view-rot columns {} and {} not orthogonal", i, j);
    }
  }
}

#[test]
fn test_look_at_preserves_distances() {
  // A view matrix is a rigid transform (rotation + translation). Distances
  // between any two world points should be preserved in view space.
  let eye = Vec3::new(5.0, -2.0, 3.0);
  let target = Vec3::new(0.0, 0.0, 0.0);
  let m = Mat4::new_look_at(eye, target, Vec3::new(0.0, 1.0, 0.0));
  let p1 = Vec3::new(1.0, 2.0, 3.0);
  let p2 = Vec3::new(-2.0, 1.0, 4.0);
  let r1 = m * &p1;
  let r2 = m * &p2;
  let dx_world = p2.x - p1.x;
  let dy_world = p2.y - p1.y;
  let dz_world = p2.z - p1.z;
  let world_dist_sq = dx_world * dx_world + dy_world * dy_world + dz_world * dz_world;
  let dx_view = r2.x - r1.x;
  let dy_view = r2.y - r1.y;
  let dz_view = r2.z - r1.z;
  let view_dist_sq = dx_view * dx_view + dy_view * dy_view + dz_view * dz_view;
  assert!(approx_eq(world_dist_sq, view_dist_sq));
}

#[test]
fn test_look_at_camera_above_looking_down_maps_axes() {
  // Camera at (0, 5, 0) looking at origin with up_hint = +Z.
  // The camera's local axes in world coords:
  //   forward (+camera-Z, away from target) = +Y world
  //   right = up_hint x forward = (0,0,1) x (0,1,0) = (-1, 0, 0) world
  //   up = forward x right = (0,1,0) x (-1,0,0) = (0, 0, -1)
  // Wait, but let's just test what we can verify: a world point on +X should
  // end up on -X in view space (because right is -X world).
  let eye = Vec3::new(0.0, 5.0, 0.0);
  let target = Vec3::new(0.0, 0.0, 0.0);
  let up_hint = Vec3::new(0.0, 0.0, 1.0);
  let m = Mat4::new_look_at(eye, target, up_hint);
  // Verify: world (1, 5, 0) is one unit to the right of the camera in world,
  // which should end up at view x = -1 (because right axis is world -X)
  let r = m * &Vec3::new(1.0, 5.0, 0.0);
  assert!(approx_eq(r.x, -1.0));
  assert!(approx_eq(r.y, 0.0));
  assert!(approx_eq(r.z, 0.0));
}

#[test]
fn test_look_at_point_behind_camera_lands_at_positive_z_view() {
  // World point further from camera in the look direction than the target
  // should appear deeper into -Z view. A point on the opposite side of the
  // camera from the target should appear at +Z view (behind camera).
  let eye = Vec3::new(0.0, 0.0, 5.0);
  let target = Vec3::new(0.0, 0.0, 0.0);
  let m = Mat4::new_look_at(eye, target, Vec3::new(0.0, 1.0, 0.0));
  // World z=10 is on the opposite side of the camera from target (z=0)
  let r = m * &Vec3::new(0.0, 0.0, 10.0);
  assert!(r.z > 0.0, "point behind camera should have view z > 0, got {}", r.z);
}

#[test]
fn test_look_at_world_origin_at_view_negative_distance() {
  // For an arbitrary eye looking at origin, the world origin should appear at
  // view space (0, 0, -|eye|).
  let eye = Vec3::new(2.0, 3.0, 6.0); // length 7
  let target = Vec3::new(0.0, 0.0, 0.0);
  let m = Mat4::new_look_at(eye, target, Vec3::new(0.0, 1.0, 0.0));
  let r = m * &target;
  assert!(approx_eq(r.x, 0.0));
  assert!(approx_eq(r.y, 0.0));
  assert!(approx_eq(r.z, -7.0));
}

#[test]
fn test_look_at_translation_column_matches_neg_dot_basis_eye() {
  // The translation column should be -(R^T * eye), i.e. element[3][i] should
  // equal -dot(basis_i, eye) for each row i.
  let eye = Vec3::new(2.5, -1.5, 4.0);
  let target = Vec3::new(0.0, 1.0, 0.0);
  let up = Vec3::new(0.0, 1.0, 0.0);
  let m = Mat4::new_look_at(eye, target, up);
  // Reconstruct the basis vectors from the matrix rows
  let right_basis = Vec3::new(m.ele[0][0], m.ele[1][0], m.ele[2][0]);
  let up_basis = Vec3::new(m.ele[0][1], m.ele[1][1], m.ele[2][1]);
  let forward_basis = Vec3::new(m.ele[0][2], m.ele[1][2], m.ele[2][2]);
  assert!(approx_eq(m.ele[3][0], -right_basis.dot(eye)));
  assert!(approx_eq(m.ele[3][1], -up_basis.dot(eye)));
  assert!(approx_eq(m.ele[3][2], -forward_basis.dot(eye)));
}

#[test]
fn test_look_at_homogeneous_w_row_is_clean() {
  // A view matrix is affine, not a projection. The bottom row should be
  // [0, 0, 0, 1] (in column-major: ele[0..2][3] = 0, ele[3][3] = 1).
  let m = Mat4::new_look_at(Vec3::new(1.0, 2.0, 3.0), Vec3::new(-1.0, 0.5, -2.0), Vec3::new(0.0, 1.0, 0.0));
  assert!(approx_eq(m.ele[0][3], 0.0));
  assert!(approx_eq(m.ele[1][3], 0.0));
  assert!(approx_eq(m.ele[2][3], 0.0));
  assert!(approx_eq(m.ele[3][3], 1.0));
}

#[test]
fn test_look_at_then_perspective_pipeline_smoke() {
  // End-to-end smoke test: a model-space point through view * projection
  // should produce sensible NDC coordinates in [-1, 1].
  let view = Mat4::new_look_at(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
  let proj = Mat4::new_perspective(FRAC_PI_2, 1.0, 0.1, 100.0);
  let mvp = proj * view;
  // A point at world origin: 5 units in front of the camera
  let clip = mvp * &Vec4::new(0.0, 0.0, 0.0, 1.0);
  let ndc_x = clip.x / clip.w;
  let ndc_y = clip.y / clip.w;
  let ndc_z = clip.z / clip.w;
  assert!(approx_eq(ndc_x, 0.0));
  assert!(approx_eq(ndc_y, 0.0));
  // 5 is between near (0.1) and far (100), so ndc_z should be in (-1, 1)
  assert!(ndc_z > -1.0 && ndc_z < 1.0, "ndc_z={} not in (-1, 1)", ndc_z);
}

#[test]
fn test_look_at_pipeline_off_centre_point() {
  // A point at world (1, 0, 0) seen from camera at (0, 0, 5) should appear
  // to the right of centre on screen (positive ndc_x).
  let view = Mat4::new_look_at(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
  let proj = Mat4::new_perspective(FRAC_PI_2, 1.0, 0.1, 100.0);
  let mvp = proj * view;
  let clip = mvp * &Vec4::new(1.0, 0.0, 0.0, 1.0);
  let ndc_x = clip.x / clip.w;
  assert!(ndc_x > 0.0, "off-centre +x point should give positive ndc_x, got {}", ndc_x);
}
