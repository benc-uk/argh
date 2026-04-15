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
