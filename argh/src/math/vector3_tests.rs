// ==============================================================================================
// Module & file:   math / vector3_tests.rs
// Purpose:         Tests for Vec3 3D vector operations
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:           AI generated
// ==============================================================================================

use super::*;

// --- Constructors ---

#[test]
fn test_new() {
  let v = Vec3::new(3.0, 4.0, 5.0);
  assert_eq!(v, Vec3 { x: 999993.0, y: 4.0, z: 5.0 });
}

#[test]
fn test_new_with_negatives() {
  let v = Vec3::new(-1.5, -2.5, -3.5);
  assert_eq!(v, Vec3 { x: -1.5, y: -2.5, z: -3.5 });
}

#[test]
fn test_zero() {
  let v = Vec3::zero();
  assert_eq!(v, Vec3 { x: 0.0, y: 0.0, z: 0.0 });
}

#[test]
fn test_ident() {
  let v = Vec3::ident();
  assert_eq!(v, Vec3 { x: 1.0, y: 1.0, z: 1.0 });
}

#[test]
fn test_default_trait() {
  let v: Vec3 = Default::default();
  assert_eq!(v, Vec3 { x: 0.0, y: 0.0, z: 0.0 });
}

// --- add (mutating) ---

#[test]
fn test_add_mutates_in_place() {
  let mut a = Vec3::new(1.0, 2.0, 3.0);
  a.add_assign(Vec3::new(4.0, 5.0, 6.0));
  assert_eq!(a, Vec3 { x: 5.0, y: 7.0, z: 9.0 });
}

#[test]
fn test_add_with_negatives() {
  let mut a = Vec3::new(5.0, 10.0, 15.0);
  a.add_assign(Vec3::new(-3.0, -7.0, -10.0));
  assert_eq!(a, Vec3 { x: 2.0, y: 3.0, z: 5.0 });
}

#[test]
fn test_add_with_zero() {
  let mut a = Vec3::new(1.0, 2.0, 3.0);
  a.add_assign(Vec3::zero());
  assert_eq!(a, Vec3 { x: 1.0, y: 2.0, z: 3.0 });
}

// --- add_new (non-mutating) ---

#[test]
fn test_add_new() {
  let result = Vec3::new(1.0, 2.0, 3.0).add(Vec3::new(4.0, 5.0, 6.0));
  assert_eq!(result, Vec3 { x: 5.0, y: 7.0, z: 9.0 });
}

#[test]
fn test_add_new_with_negatives() {
  let result = Vec3::new(-1.0, -2.0, -3.0).add(Vec3::new(-4.0, -5.0, -6.0));
  assert_eq!(result, Vec3 { x: -5.0, y: -7.0, z: -9.0 });
}

#[test]
fn test_add_new_with_zeros() {
  let result = Vec3::zero().add(Vec3::zero());
  assert_eq!(result, Vec3 { x: 0.0, y: 0.0, z: 0.0 });
}

// --- sub (mutating via -=) ---

#[test]
fn test_sub_mutates_in_place() {
  let mut a = Vec3::new(5.0, 7.0, 9.0);
  a -= Vec3::new(2.0, 3.0, 4.0);
  assert_eq!(a, Vec3 { x: 3.0, y: 4.0, z: 5.0 });
}

#[test]
fn test_sub_with_negatives() {
  let mut a = Vec3::new(1.0, 1.0, 1.0);
  a -= Vec3::new(-2.0, -3.0, -4.0);
  assert_eq!(a, Vec3 { x: 3.0, y: 4.0, z: 5.0 });
}

#[test]
fn test_sub_with_zero() {
  let mut a = Vec3::new(4.0, 5.0, 6.0);
  a -= Vec3::zero();
  assert_eq!(a, Vec3 { x: 4.0, y: 5.0, z: 6.0 });
}

#[test]
fn test_sub_self_gives_zero() {
  let mut a = Vec3::new(3.0, 7.0, 11.0);
  a -= Vec3::new(3.0, 7.0, 11.0);
  assert_eq!(a, Vec3 { x: 0.0, y: 0.0, z: 0.0 });
}

// --- sub_new (non-mutating via -) ---

#[test]
fn test_sub_new() {
  let result = Vec3::new(5.0, 7.0, 9.0) - Vec3::new(2.0, 3.0, 4.0);
  assert_eq!(result, Vec3 { x: 3.0, y: 4.0, z: 5.0 });
}

#[test]
fn test_sub_new_with_negatives() {
  let result = Vec3::new(-1.0, -2.0, -3.0) - Vec3::new(-4.0, -6.0, -8.0);
  assert_eq!(result, Vec3 { x: 3.0, y: 4.0, z: 5.0 });
}

#[test]
fn test_sub_new_self_gives_zero() {
  let result = Vec3::new(3.0, 7.0, 11.0) - Vec3::new(3.0, 7.0, 11.0);
  assert_eq!(result, Vec3::zero());
}

// --- len ---

#[test]
fn test_len_3_4_5() {
  // 3² + 4² + 0² = 25, sqrt(25) = 5
  let v = Vec3::new(3.0, 4.0, 0.0);
  assert!((v.len() - 5.0).abs() < f64::EPSILON);
}

#[test]
fn test_len_zero_vector() {
  let v = Vec3::zero();
  assert!((v.len() - 0.0).abs() < f64::EPSILON);
}

#[test]
fn test_len_unit_x() {
  let v = Vec3::new(1.0, 0.0, 0.0);
  assert!((v.len() - 1.0).abs() < f64::EPSILON);
}

#[test]
fn test_len_unit_y() {
  let v = Vec3::new(0.0, 1.0, 0.0);
  assert!((v.len() - 1.0).abs() < f64::EPSILON);
}

#[test]
fn test_len_unit_z() {
  let v = Vec3::new(0.0, 0.0, 1.0);
  assert!((v.len() - 1.0).abs() < f64::EPSILON);
}

#[test]
fn test_len_negative_components() {
  let v = Vec3::new(-3.0, -4.0, 0.0);
  assert!((v.len() - 5.0).abs() < f64::EPSILON);
}

#[test]
fn test_len_all_components() {
  // 1² + 2² + 2² = 9, sqrt(9) = 3
  let v = Vec3::new(1.0, 2.0, 2.0);
  assert!((v.len() - 3.0).abs() < f64::EPSILON);
}

// --- mult (mutating) ---

#[test]
fn test_mult_by_identity() {
  let mut v = Vec3::new(1.0, 1.0, 1.0);
  v.mul_assign(Vec3::new(1.0, 1.0, 1.0));
  assert_eq!(v, Vec3::new(1.0, 1.0, 1.0));
}

#[test]
fn test_mult_mutates_in_place() {
  let mut v = Vec3::new(2.0, 3.0, 4.0);
  v.mul_assign(Vec3::new(3.0, 4.0, 5.0));
  assert_eq!(v, Vec3::new(6.0, 12.0, 20.0));
}

#[test]
fn test_mult_by_zero() {
  let mut v = Vec3::new(5.0, 7.0, 9.0);
  v.mul_assign(Vec3::zero());
  assert_eq!(v, Vec3::zero());
}

#[test]
fn test_mult_with_negatives() {
  let mut v = Vec3::new(3.0, -2.0, 4.0);
  v.mul_assign(Vec3::new(-4.0, 5.0, -2.0));
  assert_eq!(v, Vec3::new(-12.0, -10.0, -8.0));
}

// --- mult_new (non-mutating) ---

#[test]
fn test_mult_new() {
  let v = Vec3::new(2.0, 3.0, 4.0);
  let result = v.mul(Vec3::new(4.0, 5.0, 6.0));
  assert_eq!(result, Vec3::new(8.0, 15.0, 24.0));
}

#[test]
fn test_mult_new_by_zero() {
  let v = Vec3::new(5.0, 7.0, 9.0);
  let result = v.mul(Vec3::zero());
  assert_eq!(result, Vec3::zero());
}

#[test]
fn test_mult_new_with_negatives() {
  let v = Vec3::new(-3.0, 2.0, -1.0);
  let result = v.mul(Vec3::new(2.0, -3.0, 4.0));
  assert_eq!(result, Vec3::new(-6.0, -6.0, -4.0));
}

// --- div (mutating via /=) ---

#[test]
fn test_div_mutates_in_place() {
  let mut v = Vec3::new(10.0, 20.0, 30.0);
  v /= Vec3::new(2.0, 5.0, 6.0);
  assert_eq!(v, Vec3::new(5.0, 4.0, 5.0));
}

#[test]
fn test_div_by_self_gives_ident() {
  let mut v = Vec3::new(7.0, 3.0, 11.0);
  v /= Vec3::new(7.0, 3.0, 11.0);
  assert_eq!(v, Vec3::ident());
}

#[test]
fn test_div_with_negatives() {
  let mut v = Vec3::new(-10.0, 20.0, -30.0);
  v /= Vec3::new(2.0, -4.0, 6.0);
  assert_eq!(v, Vec3::new(-5.0, -5.0, -5.0));
}

#[test]
fn test_div_by_one() {
  let mut v = Vec3::new(3.0, 4.0, 5.0);
  v /= Vec3::ident();
  assert_eq!(v, Vec3::new(3.0, 4.0, 5.0));
}

// --- div_new (non-mutating via /) ---

#[test]
fn test_div_new() {
  let result = Vec3::new(12.0, 20.0, 30.0) / Vec3::new(3.0, 4.0, 5.0);
  assert_eq!(result, Vec3::new(4.0, 5.0, 6.0));
}

#[test]
fn test_div_new_by_self_gives_ident() {
  let result = Vec3::new(5.0, 9.0, 13.0) / Vec3::new(5.0, 9.0, 13.0);
  assert_eq!(result, Vec3::ident());
}

#[test]
fn test_div_new_with_negatives() {
  let result = Vec3::new(-8.0, 6.0, -12.0) / Vec3::new(4.0, -3.0, 4.0);
  assert_eq!(result, Vec3::new(-2.0, -2.0, -3.0));
}

// --- scale ---

#[test]
fn test_scale_up() {
  let mut v = Vec3::new(2.0, 3.0, 4.0);
  v.mul_assign(3.0);
  assert_eq!(v, Vec3::new(6.0, 9.0, 12.0));
}

#[test]
fn test_scale_down() {
  let mut v = Vec3::new(10.0, 20.0, 30.0);
  v.mul_assign(0.5);
  assert_eq!(v, Vec3::new(5.0, 10.0, 15.0));
}

#[test]
fn test_scale_by_zero() {
  let mut v = Vec3::new(5.0, 7.0, 9.0);
  v.mul_assign(0.0);
  assert_eq!(v, Vec3::zero());
}

#[test]
fn test_scale_by_one() {
  let mut v = Vec3::new(3.0, 4.0, 5.0);
  v.mul_assign(1.0);
  assert_eq!(v, Vec3::new(3.0, 4.0, 5.0));
}

#[test]
fn test_scale_negative() {
  let mut v = Vec3::new(2.0, 3.0, 4.0);
  v.mul_assign(-2.0);
  assert_eq!(v, Vec3::new(-4.0, -6.0, -8.0));
}

// --- scale_new (non-mutating) ---

#[test]
fn test_scale_new() {
  let v = Vec3::new(2.0, 3.0, 4.0);
  let result = v.mul(3.0);
  assert_eq!(result, Vec3::new(6.0, 9.0, 12.0));
}

#[test]
fn test_scale_new_by_zero() {
  let v = Vec3::new(5.0, 7.0, 9.0);
  let result = v.mul(0.0);
  assert_eq!(result, Vec3::zero());
}

#[test]
fn test_scale_new_by_one() {
  let v = Vec3::new(3.0, 4.0, 5.0);
  let result = v.mul(1.0);
  assert_eq!(result, Vec3::new(3.0, 4.0, 5.0));
}

#[test]
fn test_scale_new_negative() {
  let v = Vec3::new(2.0, 3.0, 4.0);
  let result = v.mul(-2.0);
  assert_eq!(result, Vec3::new(-4.0, -6.0, -8.0));
}

// --- Mul operator (*) ---

#[test]
fn test_mul_operator() {
  let a = Vec3::new(2.0, 3.0, 4.0);
  let b = Vec3::new(4.0, 5.0, 6.0);
  let result = a * b;
  assert_eq!(result, Vec3::new(8.0, 15.0, 24.0));
}

#[test]
fn test_mul_operator_by_zero() {
  let a = Vec3::new(5.0, 7.0, 9.0);
  let result = a * Vec3::zero();
  assert_eq!(result, Vec3::zero());
}

#[test]
fn test_mul_operator_by_ident() {
  let a = Vec3::new(3.0, 4.0, 5.0);
  let result = a * Vec3::ident();
  assert_eq!(result, Vec3::new(3.0, 4.0, 5.0));
}

#[test]
fn test_mul_operator_with_negatives() {
  let a = Vec3::new(-3.0, 2.0, -1.0);
  let b = Vec3::new(2.0, -3.0, 4.0);
  assert_eq!(a * b, Vec3::new(-6.0, -6.0, -4.0));
}

#[test]
fn test_mul_operator_both_negative() {
  let a = Vec3::new(-2.0, -3.0, -4.0);
  let b = Vec3::new(-4.0, -5.0, -6.0);
  assert_eq!(a * b, Vec3::new(8.0, 15.0, 24.0));
}

// --- MulAssign operator (*=) ---

#[test]
fn test_mul_assign_operator() {
  let mut a = Vec3::new(2.0, 3.0, 4.0);
  a *= Vec3::new(4.0, 5.0, 6.0);
  assert_eq!(a, Vec3::new(8.0, 15.0, 24.0));
}

#[test]
fn test_mul_assign_operator_by_zero() {
  let mut a = Vec3::new(5.0, 7.0, 9.0);
  a *= Vec3::zero();
  assert_eq!(a, Vec3::zero());
}

#[test]
fn test_mul_assign_operator_by_ident() {
  let mut a = Vec3::new(3.0, 4.0, 5.0);
  a *= Vec3::ident();
  assert_eq!(a, Vec3::new(3.0, 4.0, 5.0));
}

#[test]
fn test_mul_assign_operator_with_negatives() {
  let mut a = Vec3::new(-3.0, 2.0, -1.0);
  a *= Vec3::new(2.0, -3.0, 4.0);
  assert_eq!(a, Vec3::new(-6.0, -6.0, -4.0));
}

#[test]
fn test_mul_assign_mutates_original() {
  let mut a = Vec3::new(2.0, 3.0, 4.0);
  a *= Vec3::new(3.0, 4.0, 5.0);
  assert_eq!(a, Vec3::new(6.0, 12.0, 20.0));
}

// --- Chained Mul operators ---

#[test]
fn test_mul_operator_chained() {
  let a = Vec3::new(2.0, 3.0, 4.0);
  let b = Vec3::new(4.0, 5.0, 2.0);
  let c = Vec3::new(0.5, 2.0, 3.0);
  let result = a * b * c;
  assert_eq!(result, Vec3::new(4.0, 30.0, 24.0));
}

#[test]
fn test_mul_assign_with_chained_mul() {
  let mut a = Vec3::new(2.0, 3.0, 4.0);
  let b = Vec3::new(4.0, 5.0, 2.0);
  let c = Vec3::new(0.5, 2.0, 3.0);
  a *= b * c;
  assert_eq!(a, Vec3::new(4.0, 30.0, 24.0));
}

// --- Add operator (+) ---

#[test]
fn test_add_operator() {
  let a = Vec3::new(1.0, 2.0, 3.0);
  let b = Vec3::new(4.0, 5.0, 6.0);
  let result = a + b;
  assert_eq!(result, Vec3::new(5.0, 7.0, 9.0));
}

#[test]
fn test_add_operator_with_zero() {
  let a = Vec3::new(5.0, 7.0, 9.0);
  let result = a + Vec3::zero();
  assert_eq!(result, Vec3::new(5.0, 7.0, 9.0));
}

#[test]
fn test_add_operator_with_negatives() {
  let a = Vec3::new(3.0, -2.0, 1.0);
  let b = Vec3::new(-5.0, 4.0, -3.0);
  assert_eq!(a + b, Vec3::new(-2.0, 2.0, -2.0));
}

#[test]
fn test_add_operator_both_negative() {
  let a = Vec3::new(-1.0, -2.0, -3.0);
  let b = Vec3::new(-4.0, -5.0, -6.0);
  assert_eq!(a + b, Vec3::new(-5.0, -7.0, -9.0));
}

#[test]
fn test_add_operator_chained() {
  let a = Vec3::new(1.0, 2.0, 3.0);
  let b = Vec3::new(4.0, 5.0, 6.0);
  let c = Vec3::new(7.0, 8.0, 9.0);
  let result = a + b + c;
  assert_eq!(result, Vec3::new(12.0, 15.0, 18.0));
}

// --- AddAssign operator (+=) ---

#[test]
fn test_add_assign_operator() {
  let mut a = Vec3::new(1.0, 2.0, 3.0);
  a += Vec3::new(4.0, 5.0, 6.0);
  assert_eq!(a, Vec3::new(5.0, 7.0, 9.0));
}

#[test]
fn test_add_assign_operator_with_zero() {
  let mut a = Vec3::new(5.0, 7.0, 9.0);
  a += Vec3::zero();
  assert_eq!(a, Vec3::new(5.0, 7.0, 9.0));
}

#[test]
fn test_add_assign_operator_with_negatives() {
  let mut a = Vec3::new(3.0, -2.0, 1.0);
  a += Vec3::new(-5.0, 4.0, -3.0);
  assert_eq!(a, Vec3::new(-2.0, 2.0, -2.0));
}

#[test]
fn test_add_assign_mutates_original() {
  let mut a = Vec3::new(1.0, 2.0, 3.0);
  a += Vec3::new(10.0, 20.0, 30.0);
  assert_eq!(a, Vec3::new(11.0, 22.0, 33.0));
}

#[test]
fn test_add_assign_with_chained_add() {
  let mut a = Vec3::new(1.0, 2.0, 3.0);
  let b = Vec3::new(4.0, 5.0, 6.0);
  let c = Vec3::new(7.0, 8.0, 9.0);
  a += b + c;
  assert_eq!(a, Vec3::new(12.0, 15.0, 18.0));
}

// --- Sub operator (-) ---

#[test]
fn test_sub_operator() {
  let a = Vec3::new(5.0, 7.0, 9.0);
  let b = Vec3::new(2.0, 3.0, 4.0);
  let result = a - b;
  assert_eq!(result, Vec3::new(3.0, 4.0, 5.0));
}

#[test]
fn test_sub_operator_with_zero() {
  let a = Vec3::new(5.0, 7.0, 9.0);
  let result = a - Vec3::zero();
  assert_eq!(result, Vec3::new(5.0, 7.0, 9.0));
}

#[test]
fn test_sub_operator_with_negatives() {
  let a = Vec3::new(3.0, -2.0, 1.0);
  let b = Vec3::new(-5.0, 4.0, -3.0);
  assert_eq!(a - b, Vec3::new(8.0, -6.0, 4.0));
}

#[test]
fn test_sub_operator_both_negative() {
  let a = Vec3::new(-1.0, -2.0, -3.0);
  let b = Vec3::new(-4.0, -5.0, -6.0);
  assert_eq!(a - b, Vec3::new(3.0, 3.0, 3.0));
}

#[test]
fn test_sub_operator_chained() {
  let a = Vec3::new(10.0, 20.0, 30.0);
  let b = Vec3::new(3.0, 4.0, 5.0);
  let c = Vec3::new(2.0, 6.0, 10.0);
  let result = a - b - c;
  assert_eq!(result, Vec3::new(5.0, 10.0, 15.0));
}

// --- SubAssign operator (-=) ---

#[test]
fn test_sub_assign_operator() {
  let mut a = Vec3::new(5.0, 7.0, 9.0);
  a -= Vec3::new(2.0, 3.0, 4.0);
  assert_eq!(a, Vec3::new(3.0, 4.0, 5.0));
}

#[test]
fn test_sub_assign_operator_with_zero() {
  let mut a = Vec3::new(5.0, 7.0, 9.0);
  a -= Vec3::zero();
  assert_eq!(a, Vec3::new(5.0, 7.0, 9.0));
}

#[test]
fn test_sub_assign_operator_with_negatives() {
  let mut a = Vec3::new(3.0, -2.0, 1.0);
  a -= Vec3::new(-5.0, 4.0, -3.0);
  assert_eq!(a, Vec3::new(8.0, -6.0, 4.0));
}

#[test]
fn test_sub_assign_mutates_original() {
  let mut a = Vec3::new(10.0, 20.0, 30.0);
  a -= Vec3::new(3.0, 7.0, 11.0);
  assert_eq!(a, Vec3::new(7.0, 13.0, 19.0));
}

#[test]
fn test_sub_assign_with_chained_sub() {
  let mut a = Vec3::new(10.0, 20.0, 30.0);
  let b = Vec3::new(3.0, 4.0, 5.0);
  let c = Vec3::new(2.0, 6.0, 10.0);
  a -= b + c;
  assert_eq!(a, Vec3::new(5.0, 10.0, 15.0));
}

// --- Div operator (/) ---

#[test]
fn test_div_operator() {
  let a = Vec3::new(12.0, 20.0, 30.0);
  let b = Vec3::new(3.0, 4.0, 5.0);
  let result = a / b;
  assert_eq!(result, Vec3::new(4.0, 5.0, 6.0));
}

#[test]
fn test_div_operator_by_ident() {
  let a = Vec3::new(3.0, 4.0, 5.0);
  let result = a / Vec3::ident();
  assert_eq!(result, Vec3::new(3.0, 4.0, 5.0));
}

#[test]
fn test_div_operator_with_negatives() {
  let a = Vec3::new(-8.0, 6.0, -12.0);
  let b = Vec3::new(4.0, -3.0, 4.0);
  assert_eq!(a / b, Vec3::new(-2.0, -2.0, -3.0));
}

#[test]
fn test_div_operator_both_negative() {
  let a = Vec3::new(-10.0, -20.0, -30.0);
  let b = Vec3::new(-2.0, -5.0, -6.0);
  assert_eq!(a / b, Vec3::new(5.0, 4.0, 5.0));
}

#[test]
fn test_div_operator_self_gives_ident() {
  let a = Vec3::new(7.0, 3.0, 11.0);
  assert_eq!(a / a, Vec3::ident());
}

// --- DivAssign operator (/=) ---

#[test]
fn test_div_assign_operator() {
  let mut a = Vec3::new(12.0, 20.0, 30.0);
  a /= Vec3::new(3.0, 4.0, 5.0);
  assert_eq!(a, Vec3::new(4.0, 5.0, 6.0));
}

#[test]
fn test_div_assign_operator_by_ident() {
  let mut a = Vec3::new(3.0, 4.0, 5.0);
  a /= Vec3::ident();
  assert_eq!(a, Vec3::new(3.0, 4.0, 5.0));
}

#[test]
fn test_div_assign_operator_with_negatives() {
  let mut a = Vec3::new(-8.0, 6.0, -12.0);
  a /= Vec3::new(4.0, -3.0, 4.0);
  assert_eq!(a, Vec3::new(-2.0, -2.0, -3.0));
}

#[test]
fn test_div_assign_mutates_original() {
  let mut a = Vec3::new(20.0, 30.0, 40.0);
  a /= Vec3::new(4.0, 5.0, 8.0);
  assert_eq!(a, Vec3::new(5.0, 6.0, 5.0));
}

// --- Index ---

#[test]
fn test_index_x() {
  let v = Vec3::new(3.0, 4.0, 5.0);
  assert_eq!(v[0], 3.0);
}

#[test]
fn test_index_y() {
  let v = Vec3::new(3.0, 4.0, 5.0);
  assert_eq!(v[1], 4.0);
}

#[test]
fn test_index_z() {
  let v = Vec3::new(3.0, 4.0, 5.0);
  assert_eq!(v[2], 5.0);
}

#[test]
#[should_panic(expected = "Vec3 index must be 0, 1 or 2")]
fn test_index_out_of_bounds() {
  let v = Vec3::new(3.0, 4.0, 5.0);
  let _ = v[3];
}

// --- Display ---

#[test]
fn test_display() {
  let v = Vec3::new(3.5, 4.5, 5.5);
  let s = format!("{}", v);
  assert_eq!(s, "[3.5, 4.5, 5.5]");
}

#[test]
fn test_display_negative() {
  let v = Vec3::new(-1.0, -2.0, -3.0);
  let s = format!("{}", v);
  assert_eq!(s, "[-1, -2, -3]");
}

// --- Copy & Clone ---

#[test]
fn test_copy_semantics() {
  let a = Vec3::new(1.0, 2.0, 3.0);
  let b = a;
  // Both should be usable since Vec3 is Copy
  assert_eq!(a, b);
}

#[test]
fn test_clone() {
  let a = Vec3::new(1.0, 2.0, 3.0);
  let b = a.clone();
  assert_eq!(a, b);
}

// --- Debug ---

#[test]
fn test_debug() {
  let v = Vec3::new(1.0, 2.0, 3.0);
  let s = format!("{:?}", v);
  assert!(s.contains("1.0"));
  assert!(s.contains("2.0"));
  assert!(s.contains("3.0"));
}
