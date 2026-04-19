// ==============================================================================================
// Module & file:   math / matrix3_tests.rs
// Purpose:         Tests for Mat3 3x3 transformation matrix
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:           AI generated
// ==============================================================================================

use super::*;

const EPSILON: f64 = 1e-10;

fn approx_eq(a: f64, b: f64) -> bool {
  (a - b).abs() < EPSILON
}

fn mat3_approx_eq(a: &Mat3, b: &Mat3) -> bool {
  for col in 0..3 {
    for row in 0..3 {
      if !approx_eq(a.ele[col][row], b.ele[col][row]) {
        return false;
      }
    }
  }
  true
}

fn vec2_approx_eq(a: &Vec2, b: &Vec2) -> bool {
  approx_eq(a.x, b.x) && approx_eq(a.y, b.y)
}

// ============================================================================
// Constructors
// ============================================================================

#[test]
fn test_new_is_identity() {
  let m = Mat3::new();
  assert_eq!(
    m,
    Mat3 {
      ele: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
    }
  );
}

#[test]
fn test_default_is_zero() {
  let m: Mat3 = Default::default();
  assert_eq!(m, Mat3::zero());
}

#[test]
fn test_zero() {
  let m = Mat3::zero();
  for col in 0..3 {
    for row in 0..3 {
      assert_eq!(m.ele[col][row], 0.0);
    }
  }
}

#[test]
fn test_new_scale() {
  let m = Mat3::new_scale(2.0, 3.0);
  assert_eq!(m.ele[0][0], 2.0);
  assert_eq!(m.ele[1][1], 3.0);
  assert_eq!(m.ele[2][2], 1.0);
  // Off-diagonals should be zero
  assert_eq!(m.ele[0][1], 0.0);
  assert_eq!(m.ele[1][0], 0.0);
  assert_eq!(m.ele[2][0], 0.0);
  assert_eq!(m.ele[2][1], 0.0);
}

#[test]
fn test_new_scale_uniform() {
  let m = Mat3::new_scale(5.0, 5.0);
  assert_eq!(m.ele[0][0], 5.0);
  assert_eq!(m.ele[1][1], 5.0);
}

#[test]
fn test_new_scale_one_is_identity_like() {
  let m = Mat3::new_scale(1.0, 1.0);
  assert_eq!(m, Mat3::new());
}

#[test]
fn test_new_scale_negative() {
  let m = Mat3::new_scale(-1.0, -1.0);
  assert_eq!(m.ele[0][0], -1.0);
  assert_eq!(m.ele[1][1], -1.0);
}

#[test]
fn test_new_rot_zero() {
  let m = Mat3::new_rot(0.0);
  assert!(mat3_approx_eq(&m, &Mat3::new()));
}

#[test]
fn test_new_rot_90() {
  let m = Mat3::new_rot(std::f64::consts::FRAC_PI_2);
  // ele layout: ele[col][row], new_rot stores [[c, -s, 0], [s, c, 0], ...]
  assert!(approx_eq(m.ele[0][0], 0.0)); // cos(90)
  assert!(approx_eq(m.ele[0][1], -1.0)); // -sin(90)
  assert!(approx_eq(m.ele[1][0], 1.0)); // sin(90)
  assert!(approx_eq(m.ele[1][1], 0.0)); // cos(90)
}

#[test]
fn test_new_rot_180() {
  let m = Mat3::new_rot(std::f64::consts::PI);
  assert!(approx_eq(m.ele[0][0], -1.0));
  assert!(approx_eq(m.ele[1][1], -1.0));
  assert!(approx_eq(m.ele[1][0], 0.0));
  assert!(approx_eq(m.ele[0][1], 0.0));
}

#[test]
fn test_new_rot_360_is_identity() {
  let m = Mat3::new_rot(std::f64::consts::TAU);
  assert!(mat3_approx_eq(&m, &Mat3::new()));
}

#[test]
fn test_new_rot_negative() {
  let m_pos = Mat3::new_rot(std::f64::consts::FRAC_PI_4);
  let m_neg = Mat3::new_rot(-std::f64::consts::FRAC_PI_4);
  // cos is even, sin is odd, so [0][0] should match, [0][1] should negate
  assert!(approx_eq(m_pos.ele[0][0], m_neg.ele[0][0]));
  assert!(approx_eq(m_pos.ele[0][1], -m_neg.ele[0][1]));
}

#[test]
fn test_new_trans() {
  let m = Mat3::new_trans(10.0, 20.0);
  assert_eq!(m.ele[2][0], 10.0);
  assert_eq!(m.ele[2][1], 20.0);
  // Upper 2x2 should be identity
  assert_eq!(m.ele[0][0], 1.0);
  assert_eq!(m.ele[1][1], 1.0);
  assert_eq!(m.ele[0][1], 0.0);
  assert_eq!(m.ele[1][0], 0.0);
}

#[test]
fn test_new_trans_zero() {
  let m = Mat3::new_trans(0.0, 0.0);
  assert_eq!(m, Mat3::new());
}

#[test]
fn test_new_trans_negative() {
  let m = Mat3::new_trans(-5.0, -10.0);
  assert_eq!(m.ele[2][0], -5.0);
  assert_eq!(m.ele[2][1], -10.0);
}

#[test]
fn test_new_scale_rot_trans_no_rotation() {
  let m = Mat3::new_scale_rot_trans(2.0, 3.0, 0.0, 10.0, 20.0);
  assert!(approx_eq(m.ele[0][0], 2.0));
  assert!(approx_eq(m.ele[1][1], 3.0));
  assert_eq!(m.ele[2][0], 10.0);
  assert_eq!(m.ele[2][1], 20.0);
}

#[test]
fn test_new_scale_rot_trans_identity_values() {
  let m = Mat3::new_scale_rot_trans(1.0, 1.0, 0.0, 0.0, 0.0);
  assert!(mat3_approx_eq(&m, &Mat3::new()));
}

#[test]
fn test_new_scale_rot_trans_rotation_only() {
  let a = std::f64::consts::FRAC_PI_2;
  let m = Mat3::new_scale_rot_trans(1.0, 1.0, a, 0.0, 0.0);
  let expected = Mat3::new_rot(a);
  assert!(mat3_approx_eq(&m, &expected));
}

// ============================================================================
// Mutating methods
// ============================================================================

#[test]
fn test_trans_sets_translation() {
  let mut m = Mat3::new();
  m.trans(5.0, 10.0);
  assert_eq!(m.ele[2][0], 5.0);
  assert_eq!(m.ele[2][1], 10.0);
  // Upper 2x2 unchanged
  assert_eq!(m.ele[0][0], 1.0);
  assert_eq!(m.ele[1][1], 1.0);
}

#[test]
fn test_trans_overwrites_previous() {
  let mut m = Mat3::new_trans(100.0, 200.0);
  m.trans(1.0, 2.0);
  assert_eq!(m.ele[2][0], 1.0);
  assert_eq!(m.ele[2][1], 2.0);
}

#[test]
fn test_rot_sets_rotation() {
  let mut m = Mat3::new();
  m.rot(std::f64::consts::FRAC_PI_2);
  assert!(approx_eq(m.ele[0][0], 0.0));
  assert!(approx_eq(m.ele[1][0], -1.0));
  assert!(approx_eq(m.ele[0][1], 1.0));
  assert!(approx_eq(m.ele[1][1], 0.0));
}

#[test]
fn test_rot_zero_keeps_identity_rotation() {
  let mut m = Mat3::new();
  m.rot(0.0);
  assert!(approx_eq(m.ele[0][0], 1.0));
  assert!(approx_eq(m.ele[1][1], 1.0));
}

#[test]
fn test_scale_sets_scale() {
  let mut m = Mat3::new();
  m.scale(3.0, 4.0);
  assert_eq!(m.ele[0][0], 3.0);
  assert_eq!(m.ele[1][1], 4.0);
}

#[test]
fn test_scale_overwrites_previous() {
  let mut m = Mat3::new_scale(10.0, 20.0);
  m.scale(2.0, 3.0);
  assert_eq!(m.ele[0][0], 2.0);
  assert_eq!(m.ele[1][1], 3.0);
}

#[test]
fn test_scale_zero() {
  let mut m = Mat3::new();
  m.scale(0.0, 0.0);
  assert_eq!(m.ele[0][0], 0.0);
  assert_eq!(m.ele[1][1], 0.0);
}

// ============================================================================
// Display
// ============================================================================

#[test]
fn test_display_identity() {
  let m = Mat3::new();
  let s = format!("{}", m);
  assert!(s.contains("1"));
  assert!(s.contains("0"));
}

#[test]
fn test_display_contains_all_elements() {
  let m = Mat3::new_trans(42.0, 99.0);
  let s = format!("{}", m);
  assert!(s.contains("42"));
  assert!(s.contains("99"));
}

// ============================================================================
// Copy & Clone
// ============================================================================

#[test]
fn test_copy_semantics() {
  let a = Mat3::new_trans(5.0, 10.0);
  let b = a; // copy
  assert_eq!(a, b);
}

#[test]
fn test_clone() {
  let a = Mat3::new_scale(2.0, 3.0);
  let b = a.clone();
  assert_eq!(a, b);
}

// ============================================================================
// Mul<&Vec2> for Mat3 (point transformation)
// ============================================================================

#[test]
fn test_mul_vec2_identity() {
  let m = Mat3::new();
  let v = Vec2::new(3.0, 4.0);
  let result = m * &v;
  assert_eq!(result, Vec2::new(3.0, 4.0));
}

#[test]
fn test_mul_vec2_translation() {
  let m = Mat3::new_trans(10.0, 20.0);
  let v = Vec2::new(1.0, 2.0);
  let result = m * &v;
  assert_eq!(result, Vec2::new(11.0, 22.0));
}

#[test]
fn test_mul_vec2_scale() {
  let m = Mat3::new_scale(2.0, 3.0);
  let v = Vec2::new(4.0, 5.0);
  let result = m * &v;
  assert_eq!(result, Vec2::new(8.0, 15.0));
}

#[test]
fn test_mul_vec2_rotation_90() {
  let m = Mat3::new_rot(std::f64::consts::FRAC_PI_2);
  let v = Vec2::new(1.0, 0.0);
  let result = m * &v;
  // new_rot uses clockwise rotation: (1,0) at 90° CW -> (0, -1)
  assert!(approx_eq(result.x, 0.0));
  assert!(approx_eq(result.y, -1.0));
}

#[test]
fn test_mul_vec2_rotation_180() {
  let m = Mat3::new_rot(std::f64::consts::PI);
  let v = Vec2::new(1.0, 0.0);
  let result = m * &v;
  assert!(approx_eq(result.x, -1.0));
  assert!(approx_eq(result.y, 0.0));
}

#[test]
fn test_mul_vec2_zero_vector() {
  let m = Mat3::new_trans(5.0, 10.0);
  let v = Vec2::zero();
  let result = m * &v;
  // Translation applied to origin gives the translation values
  assert_eq!(result, Vec2::new(5.0, 10.0));
}

#[test]
fn test_mul_vec2_zero_matrix() {
  let m = Mat3::zero();
  let v = Vec2::new(3.0, 4.0);
  let result = m * &v;
  assert_eq!(result, Vec2::zero());
}

#[test]
fn test_mul_vec2_scale_and_translate() {
  // A * B * v means "apply B first, then A" (column-vector convention)
  // To scale first then translate: m = t * s
  let s = Mat3::new_scale(2.0, 2.0);
  let t = Mat3::new_trans(10.0, 10.0);
  let m = t * s; // scale first, then translate
  let v = Vec2::new(1.0, 1.0);
  let result = m * &v;
  // scale(1,1) = (2,2), then translate = (12,12)
  assert!(vec2_approx_eq(&result, &Vec2::new(12.0, 12.0)));
}

#[test]
fn test_mul_vec2_negative_scale() {
  let m = Mat3::new_scale(-1.0, -1.0);
  let v = Vec2::new(3.0, 4.0);
  let result = m * &v;
  assert_eq!(result, Vec2::new(-3.0, -4.0));
}

#[test]
fn test_mul_vec2_negative_translation() {
  let m = Mat3::new_trans(-5.0, -10.0);
  let v = Vec2::new(5.0, 10.0);
  let result = m * &v;
  assert_eq!(result, Vec2::zero());
}

// ============================================================================
// Mul<Mat3> for Mat3 (matrix multiplication)
// ============================================================================

#[test]
fn test_mul_mat3_identity_left() {
  let m = Mat3::new_scale(2.0, 3.0);
  let result = Mat3::new() * m;
  assert_eq!(result, m);
}

#[test]
fn test_mul_mat3_identity_right() {
  let m = Mat3::new_scale(2.0, 3.0);
  let result = m * Mat3::new();
  assert_eq!(result, m);
}

#[test]
fn test_mul_mat3_zero_left() {
  let m = Mat3::new_scale(2.0, 3.0);
  let result = Mat3::zero() * m;
  assert_eq!(result, Mat3::zero());
}

#[test]
fn test_mul_mat3_zero_right() {
  let m = Mat3::new_scale(2.0, 3.0);
  let result = m * Mat3::zero();
  assert_eq!(result, Mat3::zero());
}

#[test]
fn test_mul_mat3_two_translations() {
  let a = Mat3::new_trans(3.0, 4.0);
  let b = Mat3::new_trans(10.0, 20.0);
  let result = a * b;
  // Combined translation should add
  assert!(approx_eq(result.ele[2][0], 13.0));
  assert!(approx_eq(result.ele[2][1], 24.0));
}

#[test]
fn test_mul_mat3_two_scales() {
  let a = Mat3::new_scale(2.0, 3.0);
  let b = Mat3::new_scale(4.0, 5.0);
  let result = a * b;
  assert!(approx_eq(result.ele[0][0], 8.0));
  assert!(approx_eq(result.ele[1][1], 15.0));
}

#[test]
fn test_mul_mat3_two_rotations() {
  let a = Mat3::new_rot(std::f64::consts::FRAC_PI_4);
  let b = Mat3::new_rot(std::f64::consts::FRAC_PI_4);
  let result = a * b;
  let expected = Mat3::new_rot(std::f64::consts::FRAC_PI_2);
  assert!(mat3_approx_eq(&result, &expected));
}

#[test]
fn test_mul_mat3_associativity() {
  let a = Mat3::new_scale(2.0, 3.0);
  let b = Mat3::new_rot(0.5);
  let c = Mat3::new_trans(10.0, 20.0);
  let ab_c = (a * b) * c;
  let a_bc = a * (b * c);
  assert!(mat3_approx_eq(&ab_c, &a_bc));
}

#[test]
fn test_mul_mat3_not_commutative() {
  let s = Mat3::new_scale(2.0, 2.0);
  let t = Mat3::new_trans(10.0, 10.0);
  let st = s * t;
  let ts = t * s;
  // Scale * Translate != Translate * Scale (generally)
  // Actually for these specific matrices let's verify they produce different results on a point
  let v = Vec2::new(1.0, 1.0);
  let r1 = st * &v;
  let r2 = ts * &v;
  // s*t: scale(1,1)=(2,2), then translate=(12,12)
  // t*s: translate(1,1)=(11,11), then scale=(22,22)
  assert!(!vec2_approx_eq(&r1, &r2));
}

// ============================================================================
// MulAssign<Mat3>
// ============================================================================

#[test]
fn test_mul_assign_identity() {
  let mut m = Mat3::new_scale(2.0, 3.0);
  let original = m;
  m *= Mat3::new();
  assert_eq!(m, original);
}

#[test]
fn test_mul_assign_equivalent_to_mul() {
  let a = Mat3::new_scale(2.0, 3.0);
  let b = Mat3::new_trans(5.0, 10.0);
  let expected = a * b;
  let mut m = a;
  m *= b;
  assert_eq!(m, expected);
}

#[test]
fn test_mul_assign_chained() {
  let s = Mat3::new_scale(2.0, 2.0);
  let r = Mat3::new_rot(std::f64::consts::FRAC_PI_4);
  let t = Mat3::new_trans(10.0, 20.0);

  let mut m = s;
  m *= r;
  m *= t;

  let expected = s * r * t;
  assert!(mat3_approx_eq(&m, &expected));
}

#[test]
fn test_mul_assign_zero() {
  let mut m = Mat3::new_scale(5.0, 5.0);
  m *= Mat3::zero();
  assert_eq!(m, Mat3::zero());
}

// ============================================================================
// Mul<&Vec<Vec2>> for Mat3 (transform list of points)
// ============================================================================

#[test]
fn test_mul_vec_of_vec2_identity() {
  let m = Mat3::new();
  let points = vec![Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0)];
  let result = m * &points;
  assert_eq!(result, points);
}

#[test]
fn test_mul_vec_of_vec2_translation() {
  let m = Mat3::new_trans(10.0, 20.0);
  let points = vec![Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0)];
  let result = m * &points;
  assert_eq!(result[0], Vec2::new(11.0, 22.0));
  assert_eq!(result[1], Vec2::new(13.0, 24.0));
}

#[test]
fn test_mul_vec_of_vec2_scale() {
  let m = Mat3::new_scale(2.0, 3.0);
  let points = vec![Vec2::new(1.0, 1.0), Vec2::new(2.0, 2.0), Vec2::new(3.0, 3.0)];
  let result = m * &points;
  assert_eq!(result[0], Vec2::new(2.0, 3.0));
  assert_eq!(result[1], Vec2::new(4.0, 6.0));
  assert_eq!(result[2], Vec2::new(6.0, 9.0));
}

#[test]
fn test_mul_vec_of_vec2_empty() {
  let m = Mat3::new_trans(10.0, 20.0);
  let points: Vec<Vec2> = vec![];
  let result = m * &points;
  assert!(result.is_empty());
}

#[test]
fn test_mul_vec_of_vec2_preserves_length() {
  let m = Mat3::new_scale(2.0, 2.0);
  let points = vec![Vec2::zero(); 100];
  let result = m * &points;
  assert_eq!(result.len(), 100);
}

#[test]
fn test_mul_vec_of_vec2_consistent_with_single() {
  let m = Mat3::new_scale_rot_trans(2.0, 3.0, 0.7, 10.0, 20.0);
  let points = vec![Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0), Vec2::new(-1.0, -2.0)];
  let batch_result = m * &points;
  for (i, p) in points.iter().enumerate() {
    let single_result = m * p;
    assert!(vec2_approx_eq(&batch_result[i], &single_result));
  }
}

// ============================================================================
// Combined / integration style tests
// ============================================================================

#[test]
fn test_scale_rot_trans_convenience_matches_manual() {
  let sx = 2.0;
  let sy = 3.0;
  let angle = std::f64::consts::FRAC_PI_6;
  let tx = 10.0;
  let ty = 20.0;

  let combined = Mat3::new_scale_rot_trans(sx, sy, angle, tx, ty);

  // Test with a point, verify against the actual ele layout
  let v = Vec2::new(5.0, 7.0);
  let result = combined * &v;

  // The mul implementation reads: x = ele[0][0]*vx + ele[1][0]*vy + ele[2][0]
  let expected_x = combined.ele[0][0] * v.x + combined.ele[1][0] * v.y + combined.ele[2][0];
  let expected_y = combined.ele[0][1] * v.x + combined.ele[1][1] * v.y + combined.ele[2][1];
  assert!(approx_eq(result.x, expected_x));
  assert!(approx_eq(result.y, expected_y));

  // Also verify the ele values are as documented
  let ca = f64::cos(angle);
  let sa = f64::sin(angle);
  assert!(approx_eq(combined.ele[0][0], ca * sx));
  assert!(approx_eq(combined.ele[0][1], -sa * sy));
  assert!(approx_eq(combined.ele[1][0], sa * sx));
  assert!(approx_eq(combined.ele[1][1], ca * sy));
  assert_eq!(combined.ele[2][0], tx);
  assert_eq!(combined.ele[2][1], ty);
}

#[test]
fn test_rotate_point_around_origin() {
  // new_rot does CW rotation: (1, 0) by 90° CW -> (0, -1)
  let m = Mat3::new_rot(std::f64::consts::FRAC_PI_2);
  let v = Vec2::new(1.0, 0.0);
  let result = m * &v;
  assert!(vec2_approx_eq(&result, &Vec2::new(0.0, -1.0)));
}

#[test]
fn test_translate_then_scale() {
  // A * B * v = A(B(v)), so B applied first
  // To translate first then scale: m = s * t (t applied first)
  let t = Mat3::new_trans(5.0, 5.0);
  let s = Mat3::new_scale(2.0, 2.0);
  let m = s * t; // t(translate) applied first, then s(scale)
  let v = Vec2::new(1.0, 1.0);
  let result = m * &v;
  // t(1,1)=(6,6) then s(6,6)=(12,12)
  assert!(vec2_approx_eq(&result, &Vec2::new(12.0, 12.0)));
}

#[test]
fn test_scale_then_translate() {
  // To scale first then translate: m = t * s (s applied first)
  let s = Mat3::new_scale(2.0, 2.0);
  let t = Mat3::new_trans(5.0, 5.0);
  let m = t * s; // s(scale) applied first, then t(translate)
  let v = Vec2::new(1.0, 1.0);
  let result = m * &v;
  // s(1,1)=(2,2) then t(2,2)=(7,7)
  assert!(vec2_approx_eq(&result, &Vec2::new(7.0, 7.0)));
}

#[test]
fn test_double_rotation() {
  let m = Mat3::new_rot(std::f64::consts::FRAC_PI_4);
  let composed = m * m;
  let v = Vec2::new(1.0, 0.0);
  let result = composed * &v;
  // 45 + 45 = 90 degrees CW: (1,0) -> (0,-1)
  assert!(vec2_approx_eq(&result, &Vec2::new(0.0, -1.0)));
}

#[test]
fn test_identity_mul_preserves_all_transforms() {
  let m = Mat3::new_scale_rot_trans(2.0, 3.0, 1.0, 10.0, 20.0);
  let result = Mat3::new() * m;
  assert!(mat3_approx_eq(&result, &m));
}

#[test]
fn test_partial_eq() {
  let a = Mat3::new_scale(2.0, 3.0);
  let b = Mat3::new_scale(2.0, 3.0);
  let c = Mat3::new_scale(2.0, 4.0);
  assert_eq!(a, b);
  assert_ne!(a, c);
}

#[test]
fn test_debug_trait() {
  let m = Mat3::new();
  let debug = format!("{:?}", m);
  assert!(debug.contains("Mat3"));
}
