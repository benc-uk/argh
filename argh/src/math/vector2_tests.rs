// ==============================================================================================
// Module & file:   math / vector2_tests.rs
// Purpose:         Tests for Vec2 2D vector operations and Mat3 interactions
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:           AI generated
// ==============================================================================================

use super::*;

// --- Constructors ---

#[test]
fn test_new() {
  let v = Vec2::new(3.0, 4.0);
  assert_eq!(v, Vec2 { x: 3.0, y: 4.0 });
}

#[test]
fn test_new_with_negatives() {
  let v = Vec2::new(-1.5, -2.5);
  assert_eq!(v, Vec2 { x: -1.5, y: -2.5 });
}

#[test]
fn test_zero() {
  let v = Vec2::zero();
  assert_eq!(v, Vec2 { x: 0.0, y: 0.0 });
}

#[test]
fn test_ident() {
  let v = Vec2::ident();
  assert_eq!(v, Vec2 { x: 1.0, y: 1.0 });
}

#[test]
fn test_default_trait() {
  let v: Vec2 = Default::default();
  assert_eq!(v, Vec2 { x: 0.0, y: 0.0 });
}

// --- add (mutating) ---

#[test]
fn test_add_mutates_in_place() {
  let mut a = Vec2::new(1.0, 2.0);
  a.add_assign(Vec2::new(3.0, 4.0));
  assert_eq!(a, Vec2 { x: 4.0, y: 6.0 });
}

#[test]
fn test_add_with_negatives() {
  let mut a = Vec2::new(5.0, 10.0);
  a.add_assign(Vec2::new(-3.0, -7.0));
  assert_eq!(a, Vec2 { x: 2.0, y: 3.0 });
}

#[test]
fn test_add_with_zero() {
  let mut a = Vec2::new(1.0, 2.0);
  a.add_assign(Vec2::zero());
  assert_eq!(a, Vec2 { x: 1.0, y: 2.0 });
}

// --- add_new (non-mutating) ---

#[test]
fn test_add_new() {
  let result = Vec2::new(1.0, 2.0).add(Vec2::new(3.0, 4.0));
  assert_eq!(result, Vec2 { x: 4.0, y: 6.0 });
}

#[test]
fn test_add_new_with_negatives() {
  let result = Vec2::new(-1.0, -2.0).add(Vec2::new(-3.0, -4.0));
  assert_eq!(result, Vec2 { x: -4.0, y: -6.0 });
}

#[test]
fn test_add_new_with_zeros() {
  let result = Vec2::zero().add(Vec2::zero());
  assert_eq!(result, Vec2 { x: 0.0, y: 0.0 });
}

// --- sub (mutating via -=) ---

#[test]
fn test_sub_mutates_in_place() {
  let mut a = Vec2::new(5.0, 7.0);
  a -= Vec2::new(2.0, 3.0);
  assert_eq!(a, Vec2 { x: 3.0, y: 4.0 });
}

#[test]
fn test_sub_with_negatives() {
  let mut a = Vec2::new(1.0, 1.0);
  a -= Vec2::new(-2.0, -3.0);
  assert_eq!(a, Vec2 { x: 3.0, y: 4.0 });
}

#[test]
fn test_sub_with_zero() {
  let mut a = Vec2::new(4.0, 5.0);
  a -= Vec2::zero();
  assert_eq!(a, Vec2 { x: 4.0, y: 5.0 });
}

#[test]
fn test_sub_self_gives_zero() {
  let mut a = Vec2::new(3.0, 7.0);
  a -= Vec2::new(3.0, 7.0);
  assert_eq!(a, Vec2 { x: 0.0, y: 0.0 });
}

// --- sub_new (non-mutating via -) ---

#[test]
fn test_sub_new() {
  let result = Vec2::new(5.0, 7.0) - Vec2::new(2.0, 3.0);
  assert_eq!(result, Vec2 { x: 3.0, y: 4.0 });
}

#[test]
fn test_sub_new_with_negatives() {
  let result = Vec2::new(-1.0, -2.0) - Vec2::new(-4.0, -6.0);
  assert_eq!(result, Vec2 { x: 3.0, y: 4.0 });
}

#[test]
fn test_sub_new_self_gives_zero() {
  let result = Vec2::new(3.0, 7.0) - Vec2::new(3.0, 7.0);
  assert_eq!(result, Vec2::zero());
}

// --- len ---

#[test]
fn test_len_3_4_triangle() {
  let v = Vec2::new(3.0, 4.0);
  assert!((v.len() - 5.0).abs() < f64::EPSILON);
}

#[test]
fn test_len_zero_vector() {
  let v = Vec2::zero();
  assert!((v.len() - 0.0).abs() < f64::EPSILON);
}

#[test]
fn test_len_unit_x() {
  let v = Vec2::new(1.0, 0.0);
  assert!((v.len() - 1.0).abs() < f64::EPSILON);
}

#[test]
fn test_len_unit_y() {
  let v = Vec2::new(0.0, 1.0);
  assert!((v.len() - 1.0).abs() < f64::EPSILON);
}

#[test]
fn test_len_negative_components() {
  let v = Vec2::new(-3.0, -4.0);
  assert!((v.len() - 5.0).abs() < f64::EPSILON);
}

// --- mult (mutating) ---

#[test]
fn test_mult_by_identity() {
  let mut v = Vec2::new(1.0, 1.0);
  v.mul_assign(Vec2::new(1.0, 1.0));
  assert_eq!(v, Vec2::new(1.0, 1.0));
}

#[test]
fn test_mult_mutates_in_place() {
  let mut v = Vec2::new(2.0, 3.0);
  v.mul_assign(Vec2::new(3.0, 4.0));
  assert_eq!(v, Vec2::new(6.0, 12.0));
}

#[test]
fn test_mult_by_zero() {
  let mut v = Vec2::new(5.0, 7.0);
  v.mul_assign(Vec2::zero());
  assert_eq!(v, Vec2::zero());
}

#[test]
fn test_mult_with_negatives() {
  let mut v = Vec2::new(3.0, -2.0);
  v.mul_assign(Vec2::new(-4.0, 5.0));
  assert_eq!(v, Vec2::new(-12.0, -10.0));
}

// --- mult_new (non-mutating) ---

#[test]
fn test_mult_new() {
  let v = Vec2::new(2.0, 3.0);
  let result = v.mul(Vec2::new(4.0, 5.0));
  assert_eq!(result, Vec2::new(8.0, 15.0));
}

#[test]
fn test_mult_new_by_zero() {
  let v = Vec2::new(5.0, 7.0);
  let result = v.mul(Vec2::zero());
  assert_eq!(result, Vec2::zero());
}

#[test]
fn test_mult_new_with_negatives() {
  let v = Vec2::new(-3.0, 2.0);
  let result = v.mul(Vec2::new(2.0, -3.0));
  assert_eq!(result, Vec2::new(-6.0, -6.0));
}

// --- div (mutating via /=) ---

#[test]
fn test_div_mutates_in_place() {
  let mut v = Vec2::new(10.0, 20.0);
  v /= Vec2::new(2.0, 5.0);
  assert_eq!(v, Vec2::new(5.0, 4.0));
}

#[test]
fn test_div_by_self_gives_ident() {
  let mut v = Vec2::new(7.0, 3.0);
  v /= Vec2::new(7.0, 3.0);
  assert_eq!(v, Vec2::ident());
}

#[test]
fn test_div_with_negatives() {
  let mut v = Vec2::new(-10.0, 20.0);
  v /= Vec2::new(2.0, -4.0);
  assert_eq!(v, Vec2::new(-5.0, -5.0));
}

#[test]
fn test_div_by_one() {
  let mut v = Vec2::new(3.0, 4.0);
  v /= Vec2::ident();
  assert_eq!(v, Vec2::new(3.0, 4.0));
}

// --- div_new (non-mutating via /) ---

#[test]
fn test_div_new() {
  let result = Vec2::new(12.0, 20.0) / Vec2::new(3.0, 4.0);
  assert_eq!(result, Vec2::new(4.0, 5.0));
}

#[test]
fn test_div_new_by_self_gives_ident() {
  let result = Vec2::new(5.0, 9.0) / Vec2::new(5.0, 9.0);
  assert_eq!(result, Vec2::ident());
}

#[test]
fn test_div_new_with_negatives() {
  let result = Vec2::new(-8.0, 6.0) / Vec2::new(4.0, -3.0);
  assert_eq!(result, Vec2::new(-2.0, -2.0));
}

// --- scale ---

#[test]
fn test_scale_up() {
  let mut v = Vec2::new(2.0, 3.0);
  v.mul_assign(3.0);
  assert_eq!(v, Vec2::new(6.0, 9.0));
}

#[test]
fn test_scale_down() {
  let mut v = Vec2::new(10.0, 20.0);
  v.mul_assign(0.5);
  assert_eq!(v, Vec2::new(5.0, 10.0));
}

#[test]
fn test_scale_by_zero() {
  let mut v = Vec2::new(5.0, 7.0);
  v.mul_assign(0.0);
  assert_eq!(v, Vec2::zero());
}

#[test]
fn test_scale_by_one() {
  let mut v = Vec2::new(3.0, 4.0);
  v.mul_assign(1.0);
  assert_eq!(v, Vec2::new(3.0, 4.0));
}

#[test]
fn test_scale_negative() {
  let mut v = Vec2::new(2.0, 3.0);
  v.mul_assign(-2.0);
  assert_eq!(v, Vec2::new(-4.0, -6.0));
}

// --- scale_new (non-mutating) ---

#[test]
fn test_scale_new() {
  let v = Vec2::new(2.0, 3.0);
  let result = v.mul(3.0);
  assert_eq!(result, Vec2::new(6.0, 9.0));
}

#[test]
fn test_scale_new_by_zero() {
  let v = Vec2::new(5.0, 7.0);
  let result = v.mul(0.0);
  assert_eq!(result, Vec2::zero());
}

#[test]
fn test_scale_new_by_one() {
  let v = Vec2::new(3.0, 4.0);
  let result = v.mul(1.0);
  assert_eq!(result, Vec2::new(3.0, 4.0));
}

#[test]
fn test_scale_new_negative() {
  let v = Vec2::new(2.0, 3.0);
  let result = v.mul(-2.0);
  assert_eq!(result, Vec2::new(-4.0, -6.0));
}

// --- Mul operator (*) ---

#[test]
fn test_mul_operator() {
  let a = Vec2::new(2.0, 3.0);
  let b = Vec2::new(4.0, 5.0);
  let result = a * b;
  assert_eq!(result, Vec2::new(8.0, 15.0));
}

#[test]
fn test_mul_operator_by_zero() {
  let a = Vec2::new(5.0, 7.0);
  let result = a * Vec2::zero();
  assert_eq!(result, Vec2::zero());
}

#[test]
fn test_mul_operator_by_ident() {
  let a = Vec2::new(3.0, 4.0);
  let result = a * Vec2::ident();
  assert_eq!(result, Vec2::new(3.0, 4.0));
}

#[test]
fn test_mul_operator_with_negatives() {
  let a = Vec2::new(-3.0, 2.0);
  let b = Vec2::new(2.0, -3.0);
  assert_eq!(a * b, Vec2::new(-6.0, -6.0));
}

#[test]
fn test_mul_operator_both_negative() {
  let a = Vec2::new(-2.0, -3.0);
  let b = Vec2::new(-4.0, -5.0);
  assert_eq!(a * b, Vec2::new(8.0, 15.0));
}

// --- MulAssign operator (*=) ---

#[test]
fn test_mul_assign_operator() {
  let mut a = Vec2::new(2.0, 3.0);
  a *= Vec2::new(4.0, 5.0);
  assert_eq!(a, Vec2::new(8.0, 15.0));
}

#[test]
fn test_mul_assign_operator_by_zero() {
  let mut a = Vec2::new(5.0, 7.0);
  a *= Vec2::zero();
  assert_eq!(a, Vec2::zero());
}

#[test]
fn test_mul_assign_operator_by_ident() {
  let mut a = Vec2::new(3.0, 4.0);
  a *= Vec2::ident();
  assert_eq!(a, Vec2::new(3.0, 4.0));
}

#[test]
fn test_mul_assign_operator_with_negatives() {
  let mut a = Vec2::new(-3.0, 2.0);
  a *= Vec2::new(2.0, -3.0);
  assert_eq!(a, Vec2::new(-6.0, -6.0));
}

#[test]
fn test_mul_assign_mutates_original() {
  let mut a = Vec2::new(2.0, 3.0);
  a *= Vec2::new(3.0, 4.0);
  // Verify original is changed, not a copy
  assert_eq!(a, Vec2::new(6.0, 12.0));
}

// --- Chained Mul operators ---

#[test]
fn test_mul_operator_chained() {
  let a = Vec2::new(2.0, 3.0);
  let b = Vec2::new(4.0, 5.0);
  let c = Vec2::new(0.5, 2.0);
  let result = a * b * c;
  assert_eq!(result, Vec2::new(4.0, 30.0));
}

#[test]
fn test_mul_assign_with_chained_mul() {
  let mut a = Vec2::new(2.0, 3.0);
  let b = Vec2::new(4.0, 5.0);
  let c = Vec2::new(0.5, 2.0);
  a *= b * c;
  assert_eq!(a, Vec2::new(4.0, 30.0));
}

// --- Add operator (+) ---

#[test]
fn test_add_operator() {
  let a = Vec2::new(1.0, 2.0);
  let b = Vec2::new(3.0, 4.0);
  let result = a + b;
  assert_eq!(result, Vec2::new(4.0, 6.0));
}

#[test]
fn test_add_operator_with_zero() {
  let a = Vec2::new(5.0, 7.0);
  let result = a + Vec2::zero();
  assert_eq!(result, Vec2::new(5.0, 7.0));
}

#[test]
fn test_add_operator_with_negatives() {
  let a = Vec2::new(3.0, -2.0);
  let b = Vec2::new(-5.0, 4.0);
  assert_eq!(a + b, Vec2::new(-2.0, 2.0));
}

#[test]
fn test_add_operator_both_negative() {
  let a = Vec2::new(-1.0, -2.0);
  let b = Vec2::new(-3.0, -4.0);
  assert_eq!(a + b, Vec2::new(-4.0, -6.0));
}

#[test]
fn test_add_operator_chained() {
  let a = Vec2::new(1.0, 2.0);
  let b = Vec2::new(3.0, 4.0);
  let c = Vec2::new(5.0, 6.0);
  let result = a + b + c;
  assert_eq!(result, Vec2::new(9.0, 12.0));
}

// --- AddAssign operator (+=) ---

#[test]
fn test_add_assign_operator() {
  let mut a = Vec2::new(1.0, 2.0);
  a += Vec2::new(3.0, 4.0);
  assert_eq!(a, Vec2::new(4.0, 6.0));
}

#[test]
fn test_add_assign_operator_with_zero() {
  let mut a = Vec2::new(5.0, 7.0);
  a += Vec2::zero();
  assert_eq!(a, Vec2::new(5.0, 7.0));
}

#[test]
fn test_add_assign_operator_with_negatives() {
  let mut a = Vec2::new(3.0, -2.0);
  a += Vec2::new(-5.0, 4.0);
  assert_eq!(a, Vec2::new(-2.0, 2.0));
}

#[test]
fn test_add_assign_mutates_original() {
  let mut a = Vec2::new(1.0, 2.0);
  a += Vec2::new(10.0, 20.0);
  assert_eq!(a, Vec2::new(11.0, 22.0));
}

#[test]
fn test_add_assign_with_chained_add() {
  let mut a = Vec2::new(1.0, 2.0);
  let b = Vec2::new(3.0, 4.0);
  let c = Vec2::new(5.0, 6.0);
  a += b + c;
  assert_eq!(a, Vec2::new(9.0, 12.0));
}

// --- Sub operator (-) ---

#[test]
fn test_sub_operator() {
  let a = Vec2::new(5.0, 7.0);
  let b = Vec2::new(2.0, 3.0);
  let result = a - b;
  assert_eq!(result, Vec2::new(3.0, 4.0));
}

#[test]
fn test_sub_operator_with_zero() {
  let a = Vec2::new(5.0, 7.0);
  let result = a - Vec2::zero();
  assert_eq!(result, Vec2::new(5.0, 7.0));
}

#[test]
fn test_sub_operator_with_negatives() {
  let a = Vec2::new(3.0, -2.0);
  let b = Vec2::new(-5.0, 4.0);
  assert_eq!(a - b, Vec2::new(8.0, -6.0));
}

#[test]
fn test_sub_operator_both_negative() {
  let a = Vec2::new(-1.0, -2.0);
  let b = Vec2::new(-3.0, -4.0);
  assert_eq!(a - b, Vec2::new(2.0, 2.0));
}

#[test]
fn test_sub_operator_chained() {
  let a = Vec2::new(10.0, 20.0);
  let b = Vec2::new(3.0, 4.0);
  let c = Vec2::new(2.0, 6.0);
  let result = a - b - c;
  assert_eq!(result, Vec2::new(5.0, 10.0));
}

// --- SubAssign operator (-=) ---

#[test]
fn test_sub_assign_operator() {
  let mut a = Vec2::new(5.0, 7.0);
  a -= Vec2::new(2.0, 3.0);
  assert_eq!(a, Vec2::new(3.0, 4.0));
}

#[test]
fn test_sub_assign_operator_with_zero() {
  let mut a = Vec2::new(5.0, 7.0);
  a -= Vec2::zero();
  assert_eq!(a, Vec2::new(5.0, 7.0));
}

#[test]
fn test_sub_assign_operator_with_negatives() {
  let mut a = Vec2::new(3.0, -2.0);
  a -= Vec2::new(-5.0, 4.0);
  assert_eq!(a, Vec2::new(8.0, -6.0));
}

#[test]
fn test_sub_assign_mutates_original() {
  let mut a = Vec2::new(10.0, 20.0);
  a -= Vec2::new(3.0, 7.0);
  assert_eq!(a, Vec2::new(7.0, 13.0));
}

#[test]
fn test_sub_assign_with_chained_sub() {
  let mut a = Vec2::new(10.0, 20.0);
  let b = Vec2::new(3.0, 4.0);
  let c = Vec2::new(2.0, 6.0);
  a -= b + c;
  assert_eq!(a, Vec2::new(5.0, 10.0));
}

// --- Div operator (/) ---

#[test]
fn test_div_operator() {
  let a = Vec2::new(12.0, 20.0);
  let b = Vec2::new(3.0, 4.0);
  let result = a / b;
  assert_eq!(result, Vec2::new(4.0, 5.0));
}

#[test]
fn test_div_operator_by_ident() {
  let a = Vec2::new(3.0, 4.0);
  let result = a / Vec2::ident();
  assert_eq!(result, Vec2::new(3.0, 4.0));
}

#[test]
fn test_div_operator_with_negatives() {
  let a = Vec2::new(-8.0, 6.0);
  let b = Vec2::new(4.0, -3.0);
  assert_eq!(a / b, Vec2::new(-2.0, -2.0));
}

#[test]
fn test_div_operator_both_negative() {
  let a = Vec2::new(-10.0, -20.0);
  let b = Vec2::new(-2.0, -5.0);
  assert_eq!(a / b, Vec2::new(5.0, 4.0));
}

#[test]
fn test_div_operator_self_gives_ident() {
  let a = Vec2::new(7.0, 3.0);
  assert_eq!(a / a, Vec2::ident());
}

// --- DivAssign operator (/=) ---

#[test]
fn test_div_assign_operator() {
  let mut a = Vec2::new(12.0, 20.0);
  a /= Vec2::new(3.0, 4.0);
  assert_eq!(a, Vec2::new(4.0, 5.0));
}

#[test]
fn test_div_assign_operator_by_ident() {
  let mut a = Vec2::new(3.0, 4.0);
  a /= Vec2::ident();
  assert_eq!(a, Vec2::new(3.0, 4.0));
}

#[test]
fn test_div_assign_operator_with_negatives() {
  let mut a = Vec2::new(-8.0, 6.0);
  a /= Vec2::new(4.0, -3.0);
  assert_eq!(a, Vec2::new(-2.0, -2.0));
}

#[test]
fn test_div_assign_mutates_original() {
  let mut a = Vec2::new(20.0, 30.0);
  a /= Vec2::new(4.0, 5.0);
  assert_eq!(a, Vec2::new(5.0, 6.0));
}

// --- rotate ---

#[test]
fn test_rot_self_90() {
  let mut v = Vec2::new(0.0, 3.0);
  v.rotate(std::f64::consts::FRAC_PI_2);
  assert_eq!(v.x, -3.0); // x = 3.0
  assert!(v.y < 0.00000001); // y = 0.0
}

#[test]
fn test_rotate_zero_angle() {
  let mut v = Vec2::new(3.0, 4.0);
  v.rotate(0.0);
  assert!((v.x - 3.0).abs() < f64::EPSILON);
  assert!((v.y - 4.0).abs() < f64::EPSILON);
}

#[test]
fn test_rotate_180() {
  let mut v = Vec2::new(1.0, 0.0);
  v.rotate(std::f64::consts::PI);
  assert!((v.x - (-1.0)).abs() < 1e-10);
  assert!(v.y.abs() < 1e-10);
}

#[test]
fn test_rotate_new_returns_new_vector() {
  let mut v = Vec2::new(1.0, 0.0);
  let result = v.rotate_new(std::f64::consts::FRAC_PI_2);
  assert!((result.x).abs() < 1e-10);
  assert!((result.y - 1.0).abs() < 1e-10);
}

#[test]
fn test_rotate_new_zero_angle() {
  let mut v = Vec2::new(5.0, 7.0);
  let result = v.rotate_new(0.0);
  assert!((result.x - 5.0).abs() < f64::EPSILON);
  assert!((result.y - 7.0).abs() < f64::EPSILON);
}

// --- Index ---

#[test]
fn test_index_x() {
  let v = Vec2::new(3.0, 4.0);
  assert_eq!(v[0], 3.0);
}

#[test]
fn test_index_y() {
  let v = Vec2::new(3.0, 4.0);
  assert_eq!(v[1], 4.0);
}

#[test]
#[should_panic(expected = "Vec2 index must be 0 or 1")]
fn test_index_out_of_bounds() {
  let v = Vec2::new(3.0, 4.0);
  let _ = v[2];
}

// --- Display ---

#[test]
fn test_display() {
  let v = Vec2::new(3.5, 4.5);
  let s = format!("{}", v);
  assert_eq!(s, "[3.5, 4.5]");
}

#[test]
fn test_display_negative() {
  let v = Vec2::new(-1.0, -2.0);
  let s = format!("{}", v);
  assert_eq!(s, "[-1, -2]");
}

// ============================================================================
// Mat3 interactions with Vec2
// ============================================================================

use crate::math::Mat3;

const MAT_EPSILON: f64 = 1e-10;

fn approx(a: f64, b: f64) -> bool {
  (a - b).abs() < MAT_EPSILON
}

// --- Mat3 * &Vec2 (identity) ---

#[test]
fn test_mat3_identity_preserves_vec2() {
  let m = Mat3::new();
  let v = Vec2::new(42.0, 99.0);
  let result = m * &v;
  assert_eq!(result, v);
}

#[test]
fn test_mat3_identity_preserves_zero() {
  let m = Mat3::new();
  let v = Vec2::zero();
  let result = m * &v;
  assert_eq!(result, Vec2::zero());
}

// --- Mat3 translation on Vec2 ---

#[test]
fn test_mat3_translate_vec2() {
  let m = Mat3::new_trans(10.0, 20.0);
  let v = Vec2::new(1.0, 2.0);
  assert_eq!(m * &v, Vec2::new(11.0, 22.0));
}

#[test]
fn test_mat3_translate_zero_vec2() {
  let m = Mat3::new_trans(5.0, 10.0);
  let v = Vec2::zero();
  assert_eq!(m * &v, Vec2::new(5.0, 10.0));
}

#[test]
fn test_mat3_negative_translate_vec2() {
  let m = Mat3::new_trans(-3.0, -7.0);
  let v = Vec2::new(3.0, 7.0);
  assert_eq!(m * &v, Vec2::zero());
}

// --- Mat3 scale on Vec2 ---

#[test]
fn test_mat3_scale_vec2() {
  let m = Mat3::new_scale(2.0, 3.0);
  let v = Vec2::new(4.0, 5.0);
  assert_eq!(m * &v, Vec2::new(8.0, 15.0));
}

#[test]
fn test_mat3_scale_zero_vec2() {
  let m = Mat3::new_scale(100.0, 100.0);
  let v = Vec2::zero();
  assert_eq!(m * &v, Vec2::zero());
}

#[test]
fn test_mat3_scale_by_zero() {
  let m = Mat3::new_scale(0.0, 0.0);
  let v = Vec2::new(42.0, 99.0);
  assert_eq!(m * &v, Vec2::zero());
}

#[test]
fn test_mat3_non_uniform_scale() {
  let m = Mat3::new_scale(2.0, 0.5);
  let v = Vec2::new(3.0, 10.0);
  assert_eq!(m * &v, Vec2::new(6.0, 5.0));
}

// --- Mat3 rotation on Vec2 ---

#[test]
fn test_mat3_rotate_vec2_90() {
  let m = Mat3::new_rot(std::f64::consts::FRAC_PI_2);
  let v = Vec2::new(1.0, 0.0);
  let result = m * &v;
  // new_rot uses CW rotation: (1,0) at 90° -> (0, -1)
  assert!(approx(result.x, 0.0));
  assert!(approx(result.y, -1.0));
}

#[test]
fn test_mat3_rotate_vec2_180() {
  let m = Mat3::new_rot(std::f64::consts::PI);
  let v = Vec2::new(5.0, 0.0);
  let result = m * &v;
  assert!(approx(result.x, -5.0));
  assert!(approx(result.y, 0.0));
}

#[test]
fn test_mat3_rotate_vec2_360_is_identity() {
  let m = Mat3::new_rot(std::f64::consts::TAU);
  let v = Vec2::new(3.0, 4.0);
  let result = m * &v;
  assert!(approx(result.x, 3.0));
  assert!(approx(result.y, 4.0));
}

#[test]
fn test_mat3_rotate_preserves_length() {
  let m = Mat3::new_rot(1.234);
  let v = Vec2::new(3.0, 4.0);
  let result = m * &v;
  assert!(approx(result.len(), v.len()));
}

#[test]
fn test_mat3_rotate_zero_vec2() {
  let m = Mat3::new_rot(std::f64::consts::FRAC_PI_2);
  let v = Vec2::zero();
  let result = m * &v;
  assert!(approx(result.x, 0.0));
  assert!(approx(result.y, 0.0));
}

// --- Mat3 combined transforms on Vec2 ---

#[test]
fn test_mat3_scale_rot_trans_on_vec2() {
  let m = Mat3::new_scale_rot_trans(2.0, 2.0, 0.0, 10.0, 10.0);
  let v = Vec2::new(1.0, 1.0);
  let result = m * &v;
  assert!(approx(result.x, 12.0));
  assert!(approx(result.y, 12.0));
}

#[test]
fn test_mat3_composed_transform_on_vec2() {
  let s = Mat3::new_scale(3.0, 3.0);
  let t = Mat3::new_trans(10.0, 20.0);
  // A * B * v means B applied first; to scale first then translate: m = t * s
  let m = t * s;
  let v = Vec2::new(1.0, 1.0);
  let result = m * &v;
  // scale(1,1) = (3,3), then translate(3,3) = (13,23)
  assert!(approx(result.x, 13.0));
  assert!(approx(result.y, 23.0));
}

#[test]
fn test_mat3_rotate_matches_vec2_rotate_negative() {
  // new_rot rotates CW, Vec2::rotate rotates CCW
  // So new_rot(a) should match Vec2::rotate(-a)
  let angle = 1.5;
  let m = Mat3::new_rot(angle);
  let v = Vec2::new(3.0, 4.0);
  let mat_result = m * &v;

  let mut v2 = Vec2::new(3.0, 4.0);
  v2.rotate(-angle);

  assert!(approx(mat_result.x, v2.x));
  assert!(approx(mat_result.y, v2.y));
}

// --- Mat3 * &Vec<Vec2> (batch transform) ---

#[test]
fn test_mat3_transform_vec_of_points() {
  let m = Mat3::new_trans(1.0, 1.0);
  let points = vec![Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0)];
  let result = m * &points;
  assert_eq!(result[0], Vec2::new(1.0, 1.0));
  assert_eq!(result[1], Vec2::new(2.0, 1.0));
  assert_eq!(result[2], Vec2::new(1.0, 2.0));
}

#[test]
fn test_mat3_transform_empty_vec() {
  let m = Mat3::new_scale(2.0, 2.0);
  let points: Vec<Vec2> = vec![];
  let result = m * &points;
  assert!(result.is_empty());
}

#[test]
fn test_mat3_transform_single_point_vec() {
  let m = Mat3::new_scale(3.0, 4.0);
  let points = vec![Vec2::new(2.0, 5.0)];
  let result = m * &points;
  assert_eq!(result.len(), 1);
  assert_eq!(result[0], Vec2::new(6.0, 20.0));
}

#[test]
fn test_mat3_batch_consistent_with_individual() {
  let m = Mat3::new_scale_rot_trans(2.0, 0.5, 0.8, -3.0, 7.0);
  let points = vec![Vec2::new(1.0, 2.0), Vec2::new(-3.0, 4.0), Vec2::new(0.0, 0.0), Vec2::new(100.0, -50.0)];
  let batch = m * &points;
  for (i, p) in points.iter().enumerate() {
    let single = m * p;
    assert!(approx(batch[i].x, single.x));
    assert!(approx(batch[i].y, single.y));
  }
}

// --- Chained Mat3 transforms on Vec2 ---

#[test]
fn test_mat3_chained_transforms_on_vec2() {
  let s = Mat3::new_scale(2.0, 2.0);
  let r = Mat3::new_rot(std::f64::consts::FRAC_PI_2);
  let t = Mat3::new_trans(10.0, 10.0);
  // t * r * s * v: scale first, then rotate, then translate
  let m = t * r * s;
  let v = Vec2::new(1.0, 0.0);
  let result = m * &v;
  // scale(1,0)=(2,0), rot90 CW (2,0)=(0,-2), trans(0,-2)=(10,8)
  assert!(approx(result.x, 10.0));
  assert!(approx(result.y, 8.0));
}

#[test]
fn test_mat3_mul_assign_then_transform_vec2() {
  // To scale first then translate: m = t * s
  let mut m = Mat3::new_trans(5.0, 5.0);
  m *= Mat3::new_scale(2.0, 2.0);
  let v = Vec2::new(1.0, 1.0);
  let result = m * &v;
  // scale(1,1)=(2,2), then translate=(7,7)
  assert!(approx(result.x, 7.0));
  assert!(approx(result.y, 7.0));
}

// --- Dot product ---

#[test]
fn test_dot_basic() {
  let a = Vec2::new(1.0, 2.0);
  let b = Vec2::new(3.0, 4.0);
  assert_eq!(a.dot(b), 11.0); // 1*3 + 2*4
}

#[test]
fn test_dot_with_zero() {
  let a = Vec2::new(5.0, 7.0);
  assert_eq!(a.dot(Vec2::zero()), 0.0);
}

#[test]
fn test_dot_perpendicular() {
  let a = Vec2::new(1.0, 0.0);
  let b = Vec2::new(0.0, 1.0);
  assert_eq!(a.dot(b), 0.0);
}

#[test]
fn test_dot_parallel_same_direction() {
  let a = Vec2::new(2.0, 0.0);
  let b = Vec2::new(5.0, 0.0);
  assert_eq!(a.dot(b), 10.0);
}

#[test]
fn test_dot_parallel_opposite_direction() {
  let a = Vec2::new(3.0, 0.0);
  let b = Vec2::new(-3.0, 0.0);
  assert_eq!(a.dot(b), -9.0);
}

#[test]
fn test_dot_with_negatives() {
  let a = Vec2::new(-2.0, 3.0);
  let b = Vec2::new(4.0, -5.0);
  assert_eq!(a.dot(b), -23.0); // -2*4 + 3*-5
}

#[test]
fn test_dot_commutative() {
  let a = Vec2::new(3.0, 7.0);
  let b = Vec2::new(2.0, 5.0);
  assert_eq!(a.dot(b), b.dot(a));
}

#[test]
fn test_dot_self_equals_len_squared() {
  let v = Vec2::new(3.0, 4.0);
  assert!((v.dot(v) - v.len() * v.len()).abs() < f64::EPSILON);
}
