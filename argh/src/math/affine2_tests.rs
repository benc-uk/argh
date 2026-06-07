// ==============================================================================================
// Module & file:   math / matrix3_tests.rs
// Purpose:         Tests for Affine2 3x3 transformation matrix
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:           AI generated
// ==============================================================================================

use super::*;

const EPSILON: f32 = 1e-5;

fn approx_eq(a: f32, b: f32) -> bool {
  (a - b).abs() < EPSILON
}

fn affine2_approx_eq(a: &Affine2, b: &Affine2) -> bool {
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
  let m = Affine2::new();
  assert_eq!(
    m,
    Affine2 {
      ele: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
    }
  );
}

#[test]
fn test_default_is_zero() {
  let m: Affine2 = Default::default();
  assert_eq!(m, Affine2::zero());
}

#[test]
fn test_zero() {
  let m = Affine2::zero();
  for col in 0..3 {
    for row in 0..3 {
      assert_eq!(m.ele[col][row], 0.0);
    }
  }
}

#[test]
fn test_new_scale() {
  let m = Affine2::new_scale(2.0, 3.0);
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
  let m = Affine2::new_scale(5.0, 5.0);
  assert_eq!(m.ele[0][0], 5.0);
  assert_eq!(m.ele[1][1], 5.0);
}

#[test]
fn test_new_scale_one_is_identity_like() {
  let m = Affine2::new_scale(1.0, 1.0);
  assert_eq!(m, Affine2::new());
}

#[test]
fn test_new_scale_negative() {
  let m = Affine2::new_scale(-1.0, -1.0);
  assert_eq!(m.ele[0][0], -1.0);
  assert_eq!(m.ele[1][1], -1.0);
}

#[test]
fn test_new_rot_zero() {
  let m = Affine2::new_rot(0.0);
  assert!(affine2_approx_eq(&m, &Affine2::new()));
}

#[test]
fn test_new_rot_90() {
  let m = Affine2::new_rot(std::f32::consts::FRAC_PI_2);
  // ele layout: ele[col][row], new_rot stores [[c, -s, 0], [s, c, 0], ...]
  assert!(approx_eq(m.ele[0][0], 0.0)); // cos(90)
  assert!(approx_eq(m.ele[0][1], -1.0)); // -sin(90)
  assert!(approx_eq(m.ele[1][0], 1.0)); // sin(90)
  assert!(approx_eq(m.ele[1][1], 0.0)); // cos(90)
}

#[test]
fn test_new_rot_180() {
  let m = Affine2::new_rot(std::f32::consts::PI);
  assert!(approx_eq(m.ele[0][0], -1.0));
  assert!(approx_eq(m.ele[1][1], -1.0));
  assert!(approx_eq(m.ele[1][0], 0.0));
  assert!(approx_eq(m.ele[0][1], 0.0));
}

#[test]
fn test_new_rot_360_is_identity() {
  let m = Affine2::new_rot(std::f32::consts::TAU);
  assert!(affine2_approx_eq(&m, &Affine2::new()));
}

#[test]
fn test_new_rot_negative() {
  let m_pos = Affine2::new_rot(std::f32::consts::FRAC_PI_4);
  let m_neg = Affine2::new_rot(-std::f32::consts::FRAC_PI_4);
  // cos is even, sin is odd, so [0][0] should match, [0][1] should negate
  assert!(approx_eq(m_pos.ele[0][0], m_neg.ele[0][0]));
  assert!(approx_eq(m_pos.ele[0][1], -m_neg.ele[0][1]));
}

#[test]
fn test_new_trans() {
  let m = Affine2::new_trans(10.0, 20.0);
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
  let m = Affine2::new_trans(0.0, 0.0);
  assert_eq!(m, Affine2::new());
}

#[test]
fn test_new_trans_negative() {
  let m = Affine2::new_trans(-5.0, -10.0);
  assert_eq!(m.ele[2][0], -5.0);
  assert_eq!(m.ele[2][1], -10.0);
}

#[test]
fn test_new_scale_rot_trans_no_rotation() {
  let m = Affine2::new_scale_rot_trans(2.0, 3.0, 0.0, 10.0, 20.0);
  assert!(approx_eq(m.ele[0][0], 2.0));
  assert!(approx_eq(m.ele[1][1], 3.0));
  assert_eq!(m.ele[2][0], 10.0);
  assert_eq!(m.ele[2][1], 20.0);
}

#[test]
fn test_new_scale_rot_trans_identity_values() {
  let m = Affine2::new_scale_rot_trans(1.0, 1.0, 0.0, 0.0, 0.0);
  assert!(affine2_approx_eq(&m, &Affine2::new()));
}

#[test]
fn test_new_scale_rot_trans_rotation_only() {
  let a = std::f32::consts::FRAC_PI_2;
  let m = Affine2::new_scale_rot_trans(1.0, 1.0, a, 0.0, 0.0);
  let expected = Affine2::new_rot(a);
  assert!(affine2_approx_eq(&m, &expected));
}

// ============================================================================
// Mutating methods
// ============================================================================

#[test]
fn test_trans_sets_translation() {
  let mut m = Affine2::new();
  m.trans(5.0, 10.0);
  assert_eq!(m.ele[2][0], 5.0);
  assert_eq!(m.ele[2][1], 10.0);
  // Upper 2x2 unchanged
  assert_eq!(m.ele[0][0], 1.0);
  assert_eq!(m.ele[1][1], 1.0);
}

#[test]
fn test_trans_overwrites_previous() {
  let mut m = Affine2::new_trans(100.0, 200.0);
  m.trans(1.0, 2.0);
  assert_eq!(m.ele[2][0], 1.0);
  assert_eq!(m.ele[2][1], 2.0);
}

#[test]
fn test_rot_sets_rotation() {
  let mut m = Affine2::new();
  m.rot(std::f32::consts::FRAC_PI_2);
  assert!(approx_eq(m.ele[0][0], 0.0));
  assert!(approx_eq(m.ele[0][1], -1.0));
  assert!(approx_eq(m.ele[1][0], 1.0));
  assert!(approx_eq(m.ele[1][1], 0.0));
}

#[test]
fn test_rot_matches_new_rot() {
  let a = std::f32::consts::FRAC_PI_3; // any non-trivial angle
  let m1 = Affine2::new_rot(a);
  let mut m2 = Affine2::new();
  m2.rot(a);
  assert!(affine2_approx_eq(&m1, &m2));
}

#[test]
fn test_rot_zero_keeps_identity_rotation() {
  let mut m = Affine2::new();
  m.rot(0.0);
  assert!(approx_eq(m.ele[0][0], 1.0));
  assert!(approx_eq(m.ele[1][1], 1.0));
}

#[test]
fn test_scale_sets_scale() {
  let mut m = Affine2::new();
  m.scale(3.0, 4.0);
  assert_eq!(m.ele[0][0], 3.0);
  assert_eq!(m.ele[1][1], 4.0);
}

#[test]
fn test_scale_overwrites_previous() {
  let mut m = Affine2::new_scale(10.0, 20.0);
  m.scale(2.0, 3.0);
  assert_eq!(m.ele[0][0], 2.0);
  assert_eq!(m.ele[1][1], 3.0);
}

#[test]
fn test_scale_zero() {
  let mut m = Affine2::new();
  m.scale(0.0, 0.0);
  assert_eq!(m.ele[0][0], 0.0);
  assert_eq!(m.ele[1][1], 0.0);
}

// ============================================================================
// Display
// ============================================================================

#[test]
fn test_display_identity() {
  let m = Affine2::new();
  let s = format!("{}", m);
  assert!(s.contains("1"));
  assert!(s.contains("0"));
}

#[test]
fn test_display_contains_all_elements() {
  let m = Affine2::new_trans(42.0, 99.0);
  let s = format!("{}", m);
  assert!(s.contains("42"));
  assert!(s.contains("99"));
}

// ============================================================================
// Copy & Clone
// ============================================================================

#[test]
fn test_copy_semantics() {
  let a = Affine2::new_trans(5.0, 10.0);
  let b = a; // copy
  assert_eq!(a, b);
}

#[test]
fn test_clone() {
  let a = Affine2::new_scale(2.0, 3.0);
  let b = a.clone();
  assert_eq!(a, b);
}

// ============================================================================
// Mul<&Vec2> for Affine2 (point transformation)
// ============================================================================

#[test]
fn test_mul_vec2_identity() {
  let m = Affine2::new();
  let v = Vec2::new(3.0, 4.0);
  let result = m * &v;
  assert_eq!(result, Vec2::new(3.0, 4.0));
}

#[test]
fn test_mul_vec2_translation() {
  let m = Affine2::new_trans(10.0, 20.0);
  let v = Vec2::new(1.0, 2.0);
  let result = m * &v;
  assert_eq!(result, Vec2::new(11.0, 22.0));
}

#[test]
fn test_mul_vec2_scale() {
  let m = Affine2::new_scale(2.0, 3.0);
  let v = Vec2::new(4.0, 5.0);
  let result = m * &v;
  assert_eq!(result, Vec2::new(8.0, 15.0));
}

#[test]
fn test_mul_vec2_rotation_90() {
  let m = Affine2::new_rot(std::f32::consts::FRAC_PI_2);
  let v = Vec2::new(1.0, 0.0);
  let result = m * &v;
  // new_rot uses clockwise rotation: (1,0) at 90° CW -> (0, -1)
  assert!(approx_eq(result.x, 0.0));
  assert!(approx_eq(result.y, -1.0));
}

#[test]
fn test_mul_vec2_rotation_180() {
  let m = Affine2::new_rot(std::f32::consts::PI);
  let v = Vec2::new(1.0, 0.0);
  let result = m * &v;
  assert!(approx_eq(result.x, -1.0));
  assert!(approx_eq(result.y, 0.0));
}

#[test]
fn test_mul_vec2_zero_vector() {
  let m = Affine2::new_trans(5.0, 10.0);
  let v = Vec2::zero();
  let result = m * &v;
  // Translation applied to origin gives the translation values
  assert_eq!(result, Vec2::new(5.0, 10.0));
}

#[test]
fn test_mul_vec2_zero_matrix() {
  let m = Affine2::zero();
  let v = Vec2::new(3.0, 4.0);
  let result = m * &v;
  assert_eq!(result, Vec2::zero());
}

#[test]
fn test_mul_vec2_scale_and_translate() {
  // A * B * v means "apply B first, then A" (column-vector convention)
  // To scale first then translate: m = t * s
  let s = Affine2::new_scale(2.0, 2.0);
  let t = Affine2::new_trans(10.0, 10.0);
  let m = t * s; // scale first, then translate
  let v = Vec2::new(1.0, 1.0);
  let result = m * &v;
  // scale(1,1) = (2,2), then translate = (12,12)
  assert!(vec2_approx_eq(&result, &Vec2::new(12.0, 12.0)));
}

#[test]
fn test_mul_vec2_negative_scale() {
  let m = Affine2::new_scale(-1.0, -1.0);
  let v = Vec2::new(3.0, 4.0);
  let result = m * &v;
  assert_eq!(result, Vec2::new(-3.0, -4.0));
}

#[test]
fn test_mul_vec2_negative_translation() {
  let m = Affine2::new_trans(-5.0, -10.0);
  let v = Vec2::new(5.0, 10.0);
  let result = m * &v;
  assert_eq!(result, Vec2::zero());
}

// ============================================================================
// Mul<Affine2> for Affine2 (matrix multiplication)
// ============================================================================

#[test]
fn test_mul_affine2_identity_left() {
  let m = Affine2::new_scale(2.0, 3.0);
  let result = Affine2::new() * m;
  assert_eq!(result, m);
}

#[test]
fn test_mul_affine2_identity_right() {
  let m = Affine2::new_scale(2.0, 3.0);
  let result = m * Affine2::new();
  assert_eq!(result, m);
}

#[test]
fn test_mul_affine2_zero_left() {
  let m = Affine2::new_scale(2.0, 3.0);
  let result = Affine2::zero() * m;
  assert_eq!(result, Affine2::zero());
}

#[test]
fn test_mul_affine2_zero_right() {
  let m = Affine2::new_scale(2.0, 3.0);
  let result = m * Affine2::zero();
  assert_eq!(result, Affine2::zero());
}

#[test]
fn test_mul_affine2_two_translations() {
  let a = Affine2::new_trans(3.0, 4.0);
  let b = Affine2::new_trans(10.0, 20.0);
  let result = a * b;
  // Combined translation should add
  assert!(approx_eq(result.ele[2][0], 13.0));
  assert!(approx_eq(result.ele[2][1], 24.0));
}

#[test]
fn test_mul_affine2_two_scales() {
  let a = Affine2::new_scale(2.0, 3.0);
  let b = Affine2::new_scale(4.0, 5.0);
  let result = a * b;
  assert!(approx_eq(result.ele[0][0], 8.0));
  assert!(approx_eq(result.ele[1][1], 15.0));
}

#[test]
fn test_mul_affine2_two_rotations() {
  let a = Affine2::new_rot(std::f32::consts::FRAC_PI_4);
  let b = Affine2::new_rot(std::f32::consts::FRAC_PI_4);
  let result = a * b;
  let expected = Affine2::new_rot(std::f32::consts::FRAC_PI_2);
  assert!(affine2_approx_eq(&result, &expected));
}

#[test]
fn test_mul_affine2_associativity() {
  let a = Affine2::new_scale(2.0, 3.0);
  let b = Affine2::new_rot(0.5);
  let c = Affine2::new_trans(10.0, 20.0);
  let ab_c = (a * b) * c;
  let a_bc = a * (b * c);
  assert!(affine2_approx_eq(&ab_c, &a_bc));
}

#[test]
fn test_mul_affine2_not_commutative() {
  let s = Affine2::new_scale(2.0, 2.0);
  let t = Affine2::new_trans(10.0, 10.0);
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
// MulAssign<Affine2>
// ============================================================================

#[test]
fn test_mul_assign_identity() {
  let mut m = Affine2::new_scale(2.0, 3.0);
  let original = m;
  m *= Affine2::new();
  assert_eq!(m, original);
}

#[test]
fn test_mul_assign_equivalent_to_mul() {
  let a = Affine2::new_scale(2.0, 3.0);
  let b = Affine2::new_trans(5.0, 10.0);
  let expected = a * b;
  let mut m = a;
  m *= b;
  assert_eq!(m, expected);
}

#[test]
fn test_mul_assign_chained() {
  let s = Affine2::new_scale(2.0, 2.0);
  let r = Affine2::new_rot(std::f32::consts::FRAC_PI_4);
  let t = Affine2::new_trans(10.0, 20.0);

  let mut m = s;
  m *= r;
  m *= t;

  let expected = s * r * t;
  assert!(affine2_approx_eq(&m, &expected));
}

#[test]
fn test_mul_assign_zero() {
  let mut m = Affine2::new_scale(5.0, 5.0);
  m *= Affine2::zero();
  assert_eq!(m, Affine2::zero());
}

// ============================================================================
// Mul<&Vec<Vec2>> for Affine2 (transform list of points)
// ============================================================================

#[test]
fn test_mul_vec_of_vec2_identity() {
  let m = Affine2::new();
  let points = vec![Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0)];
  let result = m * &points;
  assert_eq!(result, points);
}

#[test]
fn test_mul_vec_of_vec2_translation() {
  let m = Affine2::new_trans(10.0, 20.0);
  let points = vec![Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0)];
  let result = m * &points;
  assert_eq!(result[0], Vec2::new(11.0, 22.0));
  assert_eq!(result[1], Vec2::new(13.0, 24.0));
}

#[test]
fn test_mul_vec_of_vec2_scale() {
  let m = Affine2::new_scale(2.0, 3.0);
  let points = vec![Vec2::new(1.0, 1.0), Vec2::new(2.0, 2.0), Vec2::new(3.0, 3.0)];
  let result = m * &points;
  assert_eq!(result[0], Vec2::new(2.0, 3.0));
  assert_eq!(result[1], Vec2::new(4.0, 6.0));
  assert_eq!(result[2], Vec2::new(6.0, 9.0));
}

#[test]
fn test_mul_vec_of_vec2_empty() {
  let m = Affine2::new_trans(10.0, 20.0);
  let points: Vec<Vec2> = vec![];
  let result = m * &points;
  assert!(result.is_empty());
}

#[test]
fn test_mul_vec_of_vec2_preserves_length() {
  let m = Affine2::new_scale(2.0, 2.0);
  let points = vec![Vec2::zero(); 100];
  let result = m * &points;
  assert_eq!(result.len(), 100);
}

#[test]
fn test_mul_vec_of_vec2_consistent_with_single() {
  let m = Affine2::new_scale_rot_trans(2.0, 3.0, 0.7, 10.0, 20.0);
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
  let angle = std::f32::consts::FRAC_PI_6;
  let tx = 10.0;
  let ty = 20.0;

  let combined = Affine2::new_scale_rot_trans(sx, sy, angle, tx, ty);

  // Test with a point, verify against the actual ele layout
  let v = Vec2::new(5.0, 7.0);
  let result = combined * &v;

  // The mul implementation reads: x = ele[0][0]*vx + ele[1][0]*vy + ele[2][0]
  let expected_x = combined.ele[0][0] * v.x + combined.ele[1][0] * v.y + combined.ele[2][0];
  let expected_y = combined.ele[0][1] * v.x + combined.ele[1][1] * v.y + combined.ele[2][1];
  assert!(approx_eq(result.x, expected_x));
  assert!(approx_eq(result.y, expected_y));

  // Also verify the ele values are as documented
  let ca = f32::cos(angle);
  let sa = f32::sin(angle);
  assert!(approx_eq(combined.ele[0][0], ca * sx));
  assert!(approx_eq(combined.ele[0][1], -sa * sx));
  assert!(approx_eq(combined.ele[1][0], sa * sy));
  assert!(approx_eq(combined.ele[1][1], ca * sy));
  assert_eq!(combined.ele[2][0], tx);
  assert_eq!(combined.ele[2][1], ty);
}

#[test]
fn test_rotate_point_around_origin() {
  // new_rot does CW rotation: (1, 0) by 90° CW -> (0, -1)
  let m = Affine2::new_rot(std::f32::consts::FRAC_PI_2);
  let v = Vec2::new(1.0, 0.0);
  let result = m * &v;
  assert!(vec2_approx_eq(&result, &Vec2::new(0.0, -1.0)));
}

#[test]
fn test_translate_then_scale() {
  // A * B * v = A(B(v)), so B applied first
  // To translate first then scale: m = s * t (t applied first)
  let t = Affine2::new_trans(5.0, 5.0);
  let s = Affine2::new_scale(2.0, 2.0);
  let m = s * t; // t(translate) applied first, then s(scale)
  let v = Vec2::new(1.0, 1.0);
  let result = m * &v;
  // t(1,1)=(6,6) then s(6,6)=(12,12)
  assert!(vec2_approx_eq(&result, &Vec2::new(12.0, 12.0)));
}

#[test]
fn test_scale_then_translate() {
  // To scale first then translate: m = t * s (s applied first)
  let s = Affine2::new_scale(2.0, 2.0);
  let t = Affine2::new_trans(5.0, 5.0);
  let m = t * s; // s(scale) applied first, then t(translate)
  let v = Vec2::new(1.0, 1.0);
  let result = m * &v;
  // s(1,1)=(2,2) then t(2,2)=(7,7)
  assert!(vec2_approx_eq(&result, &Vec2::new(7.0, 7.0)));
}

#[test]
fn test_double_rotation() {
  let m = Affine2::new_rot(std::f32::consts::FRAC_PI_4);
  let composed = m * m;
  let v = Vec2::new(1.0, 0.0);
  let result = composed * &v;
  // 45 + 45 = 90 degrees CW: (1,0) -> (0,-1)
  assert!(vec2_approx_eq(&result, &Vec2::new(0.0, -1.0)));
}

#[test]
fn test_identity_mul_preserves_all_transforms() {
  let m = Affine2::new_scale_rot_trans(2.0, 3.0, 1.0, 10.0, 20.0);
  let result = Affine2::new() * m;
  assert!(affine2_approx_eq(&result, &m));
}

#[test]
fn test_partial_eq() {
  let a = Affine2::new_scale(2.0, 3.0);
  let b = Affine2::new_scale(2.0, 3.0);
  let c = Affine2::new_scale(2.0, 4.0);
  assert_eq!(a, b);
  assert_ne!(a, c);
}

#[test]
fn test_debug_trait() {
  let m = Affine2::new();
  let debug = format!("{:?}", m);
  assert!(debug.contains("Affine2"));
}

// ============================================================================
// Tightening: storage convention, off-axis rotation, orthogonality
// ============================================================================

const FRAC_PI_2_LOCAL: f32 = std::f32::consts::FRAC_PI_2;

fn arbitrary_affine2_a() -> Affine2 {
  // A general non-symmetric matrix used to expose row/column index bugs.
  // Construct by composing scale, rotation, translation so it's a valid affine.
  Affine2::new_trans(7.0, -3.0) * Affine2::new_rot(0.7) * Affine2::new_scale(2.0, 3.0)
}

fn arbitrary_affine2_b() -> Affine2 {
  Affine2::new_trans(-2.0, 5.0) * Affine2::new_rot(-0.4) * Affine2::new_scale(0.5, -1.5)
}

#[test]
fn test_mul_distributes_over_vec_mul() {
  // (A * B) * v == A * (B * v). This is THE storage-convention regression test.
  let a = arbitrary_affine2_a();
  let b = arbitrary_affine2_b();
  let v = Vec2::new(1.7, -2.3);
  let lhs = (a * b) * &v;
  let rhs = a * &(b * &v);
  assert!(vec2_approx_eq(&lhs, &rhs));
}

#[test]
fn test_mul_associative_general_matrices() {
  let a = arbitrary_affine2_a();
  let b = arbitrary_affine2_b();
  let c = Affine2::new_rot(1.1) * Affine2::new_trans(3.0, -4.0);
  let lhs = (a * b) * c;
  let rhs = a * (b * c);
  assert!(affine2_approx_eq(&lhs, &rhs));
}

#[test]
fn test_off_axis_rotation_preserves_length() {
  let m = Affine2::new_rot(0.917);
  let v = Vec2::new(3.0, -4.0);
  let r = m * &v;
  assert!(approx_eq(v.len(), r.len()));
}

#[test]
fn test_rotation_times_inverse_is_identity_on_vec() {
  let m = Affine2::new_rot(0.6);
  let m_inv = Affine2::new_rot(-0.6);
  let v = Vec2::new(2.5, -7.1);
  let r = (m * m_inv) * &v;
  assert!(vec2_approx_eq(&r, &v));
}

#[test]
fn test_rotation_columns_are_orthonormal() {
  // For a 2D rotation Affine2 the upper-left 2x2 columns form an orthonormal basis.
  let m = Affine2::new_rot(0.93);
  let c0 = (m.ele[0][0], m.ele[0][1]);
  let c1 = (m.ele[1][0], m.ele[1][1]);
  let dot01 = c0.0 * c1.0 + c0.1 * c1.1;
  let len0 = (c0.0 * c0.0 + c0.1 * c0.1).sqrt();
  let len1 = (c1.0 * c1.0 + c1.1 * c1.1).sqrt();
  assert!(approx_eq(dot01, 0.0));
  assert!(approx_eq(len0, 1.0));
  assert!(approx_eq(len1, 1.0));
}

#[test]
fn test_rotate_asymmetric_vec_90() {
  // Catches off-diagonal sign errors. Pin the actual behaviour of this codebase:
  // with the current Affine2::new_rot convention (which appears to apply the
  // transpose of the conventional column-major rotation), (3,4) rotated by +π/2
  // ends up at (4, -3).
  let m = Affine2::new_rot(FRAC_PI_2_LOCAL);
  let r = m * &Vec2::new(3.0, 4.0);
  assert!(vec2_approx_eq(&r, &Vec2::new(4.0, -3.0)));
}

#[test]
fn test_display_translation_appears_in_last_column() {
  // For column-major storage, translation values for a translation matrix
  // should print in the last COLUMN of each row (not the last row).
  let m = Affine2::new_trans(7.0, 8.0);
  let s = format!("{}", m);
  let lines: Vec<&str> = s.lines().collect();
  assert_eq!(lines.len(), 3);
  assert!(lines[0].trim_end().ends_with("7"), "row 0 was: {}", lines[0]);
  assert!(lines[1].trim_end().ends_with("8"), "row 1 was: {}", lines[1]);
  assert!(lines[2].trim_end().ends_with("1]"), "row 2 was: {}", lines[2]);
}

#[test]
fn test_mul_general_vec_against_hand_computation() {
  // Hand-built non-affine matrix to catch index mistakes.
  // Build directly to avoid any reliance on the constructors.
  let m = Affine2 {
    ele: [[1.0, 2.0, 0.0], [3.0, 4.0, 0.0], [5.0, 6.0, 1.0]],
  };
  // For Vec2 v = (10, 20): result.x = 1*10 + 3*20 + 5 = 75
  //                       result.y = 2*10 + 4*20 + 6 = 106
  let r = m * &Vec2::new(10.0, 20.0);
  assert!(vec2_approx_eq(&r, &Vec2::new(75.0, 106.0)));
}
