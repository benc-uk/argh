// ==============================================================================================
// Module & file:   math / vector4_tests.rs
// Purpose:         Tests for Vec4 4D vector operations
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:           AI generated
// ==============================================================================================

use super::*;

// --- Constructors ---

#[test]
fn test_new() {
  let v = Vec4::new(3.0, 4.0, 5.0, 6.0);
  assert_eq!(v, Vec4 { x: 3.0, y: 4.0, z: 5.0, w: 6.0 });
}

#[test]
fn test_new_with_negatives() {
  let v = Vec4::new(-1.5, -2.5, -3.5, -4.5);
  assert_eq!(
    v,
    Vec4 {
      x: -1.5,
      y: -2.5,
      z: -3.5,
      w: -4.5
    }
  );
}

#[test]
fn test_zero() {
  let v = Vec4::zero();
  assert_eq!(v, Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 });
}

#[test]
fn test_ident() {
  let v = Vec4::ident();
  assert_eq!(v, Vec4 { x: 1.0, y: 1.0, z: 1.0, w: 0.0 });
}

#[test]
fn test_default_trait() {
  let v: Vec4 = Default::default();
  assert_eq!(v, Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 });
}

// --- add_assign (mutating) ---

#[test]
fn test_add_mutates_in_place() {
  let mut a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  a.add_assign(Vec4::new(4.0, 5.0, 6.0, 7.0));
  assert_eq!(a, Vec4 { x: 5.0, y: 7.0, z: 9.0, w: 11.0 });
}

#[test]
fn test_add_with_negatives() {
  let mut a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  a.add_assign(Vec4::new(-1.0, -2.0, -3.0, -4.0));
  assert_eq!(a, Vec4::zero());
}

#[test]
fn test_add_with_zero() {
  let mut a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  a.add_assign(Vec4::zero());
  assert_eq!(a, Vec4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 });
}

// --- add (returning new) ---

#[test]
fn test_add_new() {
  let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  let b = a + Vec4::new(4.0, 5.0, 6.0, 7.0);
  assert_eq!(b, Vec4 { x: 5.0, y: 7.0, z: 9.0, w: 11.0 });
}

#[test]
fn test_add_new_with_negatives() {
  let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  let b = a + Vec4::new(-1.0, -2.0, -3.0, -4.0);
  assert_eq!(b, Vec4::zero());
}

#[test]
fn test_add_new_with_zeros() {
  let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  let b = a + Vec4::zero();
  assert_eq!(b, Vec4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 });
}

// --- sub_assign (mutating) ---

#[test]
fn test_sub_mutates_in_place() {
  let mut a = Vec4::new(5.0, 7.0, 9.0, 11.0);
  a.sub_assign(Vec4::new(1.0, 2.0, 3.0, 4.0));
  assert_eq!(a, Vec4 { x: 4.0, y: 5.0, z: 6.0, w: 7.0 });
}

#[test]
fn test_sub_with_negatives() {
  let mut a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  a.sub_assign(Vec4::new(-1.0, -2.0, -3.0, -4.0));
  assert_eq!(a, Vec4 { x: 2.0, y: 4.0, z: 6.0, w: 8.0 });
}

#[test]
fn test_sub_with_zero() {
  let mut a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  a.sub_assign(Vec4::zero());
  assert_eq!(a, Vec4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 });
}

#[test]
fn test_sub_self_gives_zero() {
  let mut a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  a.sub_assign(Vec4::new(1.0, 2.0, 3.0, 4.0));
  assert_eq!(a, Vec4::zero());
}

// --- sub (returning new) ---

#[test]
fn test_sub_new() {
  let a = Vec4::new(5.0, 7.0, 9.0, 11.0);
  let b = a - Vec4::new(1.0, 2.0, 3.0, 4.0);
  assert_eq!(b, Vec4 { x: 4.0, y: 5.0, z: 6.0, w: 7.0 });
}

#[test]
fn test_sub_new_with_negatives() {
  let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  let b = a - Vec4::new(-1.0, -2.0, -3.0, -4.0);
  assert_eq!(b, Vec4 { x: 2.0, y: 4.0, z: 6.0, w: 8.0 });
}

#[test]
fn test_sub_new_self_gives_zero() {
  let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  let b = a - a;
  assert_eq!(b, Vec4::zero());
}

// --- Length ---

#[test]
fn test_len_1_2_2_4() {
  // sqrt(1 + 4 + 4 + 16) = sqrt(25) = 5
  let v = Vec4::new(1.0, 2.0, 2.0, 4.0);
  assert_eq!(v.len(), 5.0);
}

#[test]
fn test_len_zero_vector() {
  let v = Vec4::zero();
  assert_eq!(v.len(), 0.0);
}

#[test]
fn test_len_unit_x() {
  let v = Vec4::new(1.0, 0.0, 0.0, 0.0);
  assert_eq!(v.len(), 1.0);
}

#[test]
fn test_len_unit_y() {
  let v = Vec4::new(0.0, 1.0, 0.0, 0.0);
  assert_eq!(v.len(), 1.0);
}

#[test]
fn test_len_unit_z() {
  let v = Vec4::new(0.0, 0.0, 1.0, 0.0);
  assert_eq!(v.len(), 1.0);
}

#[test]
fn test_len_unit_w() {
  let v = Vec4::new(0.0, 0.0, 0.0, 1.0);
  assert_eq!(v.len(), 1.0);
}

#[test]
fn test_len_negative_components() {
  let v = Vec4::new(-1.0, -2.0, -2.0, -4.0);
  assert_eq!(v.len(), 5.0);
}

#[test]
fn test_len_all_components() {
  // sqrt(2*2 + 3*3 + 6*6 + 0) = sqrt(49) = 7
  let v = Vec4::new(2.0, 3.0, 6.0, 0.0);
  assert_eq!(v.len(), 7.0);
}

// --- mul (component-wise) ---

#[test]
fn test_mult_by_identity_components() {
  let a = Vec4::new(3.0, 4.0, 5.0, 6.0);
  let b = a * Vec4::new(1.0, 1.0, 1.0, 1.0);
  assert_eq!(b, Vec4 { x: 3.0, y: 4.0, z: 5.0, w: 6.0 });
}

#[test]
fn test_mult_mutates_in_place() {
  let mut a = Vec4::new(2.0, 3.0, 4.0, 5.0);
  a.mul_assign(Vec4::new(2.0, 3.0, 4.0, 5.0));
  assert_eq!(a, Vec4 { x: 4.0, y: 9.0, z: 16.0, w: 25.0 });
}

#[test]
fn test_mult_by_zero() {
  let mut a = Vec4::new(2.0, 3.0, 4.0, 5.0);
  a.mul_assign(Vec4::zero());
  assert_eq!(a, Vec4::zero());
}

#[test]
fn test_mult_with_negatives() {
  let mut a = Vec4::new(2.0, 3.0, 4.0, 5.0);
  a.mul_assign(Vec4::new(-1.0, -2.0, -3.0, -4.0));
  assert_eq!(
    a,
    Vec4 {
      x: -2.0,
      y: -6.0,
      z: -12.0,
      w: -20.0
    }
  );
}

#[test]
fn test_mult_new() {
  let a = Vec4::new(2.0, 3.0, 4.0, 5.0);
  let b = a * Vec4::new(2.0, 3.0, 4.0, 5.0);
  assert_eq!(b, Vec4 { x: 4.0, y: 9.0, z: 16.0, w: 25.0 });
}

#[test]
fn test_mult_new_by_zero() {
  let a = Vec4::new(2.0, 3.0, 4.0, 5.0);
  let b = a * Vec4::zero();
  assert_eq!(b, Vec4::zero());
}

#[test]
fn test_mult_new_with_negatives() {
  let a = Vec4::new(2.0, 3.0, 4.0, 5.0);
  let b = a * Vec4::new(-1.0, -2.0, -3.0, -4.0);
  assert_eq!(
    b,
    Vec4 {
      x: -2.0,
      y: -6.0,
      z: -12.0,
      w: -20.0
    }
  );
}

// --- div (component-wise) ---

#[test]
fn test_div_mutates_in_place() {
  let mut a = Vec4::new(8.0, 9.0, 16.0, 25.0);
  a.div_assign(Vec4::new(2.0, 3.0, 4.0, 5.0));
  assert_eq!(a, Vec4 { x: 4.0, y: 3.0, z: 4.0, w: 5.0 });
}

#[test]
fn test_div_by_self_gives_ident_components() {
  let mut a = Vec4::new(2.0, 3.0, 4.0, 5.0);
  a.div_assign(Vec4::new(2.0, 3.0, 4.0, 5.0));
  assert_eq!(a, Vec4 { x: 1.0, y: 1.0, z: 1.0, w: 1.0 });
}

#[test]
fn test_div_with_negatives() {
  let mut a = Vec4::new(8.0, 9.0, 16.0, 25.0);
  a.div_assign(Vec4::new(-2.0, -3.0, -4.0, -5.0));
  assert_eq!(
    a,
    Vec4 {
      x: -4.0,
      y: -3.0,
      z: -4.0,
      w: -5.0
    }
  );
}

#[test]
fn test_div_by_one() {
  let mut a = Vec4::new(2.0, 3.0, 4.0, 5.0);
  a.div_assign(Vec4::new(1.0, 1.0, 1.0, 1.0));
  assert_eq!(a, Vec4 { x: 2.0, y: 3.0, z: 4.0, w: 5.0 });
}

#[test]
fn test_div_new() {
  let a = Vec4::new(8.0, 9.0, 16.0, 25.0);
  let b = a / Vec4::new(2.0, 3.0, 4.0, 5.0);
  assert_eq!(b, Vec4 { x: 4.0, y: 3.0, z: 4.0, w: 5.0 });
}

#[test]
fn test_div_new_by_self_gives_ident_components() {
  let a = Vec4::new(2.0, 3.0, 4.0, 5.0);
  let b = a / a;
  assert_eq!(b, Vec4 { x: 1.0, y: 1.0, z: 1.0, w: 1.0 });
}

#[test]
fn test_div_new_with_negatives() {
  let a = Vec4::new(8.0, 9.0, 16.0, 25.0);
  let b = a / Vec4::new(-2.0, -3.0, -4.0, -5.0);
  assert_eq!(
    b,
    Vec4 {
      x: -4.0,
      y: -3.0,
      z: -4.0,
      w: -5.0
    }
  );
}

// --- scale (mul by f64) ---

#[test]
fn test_scale_up() {
  let mut a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  a.mul_assign(2.0);
  assert_eq!(a, Vec4 { x: 2.0, y: 4.0, z: 6.0, w: 8.0 });
}

#[test]
fn test_scale_down() {
  let mut a = Vec4::new(2.0, 4.0, 6.0, 8.0);
  a.mul_assign(0.5);
  assert_eq!(a, Vec4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 });
}

#[test]
fn test_scale_by_zero() {
  let mut a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  a.mul_assign(0.0);
  assert_eq!(a, Vec4::zero());
}

#[test]
fn test_scale_by_one() {
  let mut a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  a.mul_assign(1.0);
  assert_eq!(a, Vec4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 });
}

#[test]
fn test_scale_negative() {
  let mut a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  a.mul_assign(-1.0);
  assert_eq!(
    a,
    Vec4 {
      x: -1.0,
      y: -2.0,
      z: -3.0,
      w: -4.0
    }
  );
}

#[test]
fn test_scale_new() {
  let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  let b = a * 3.0;
  assert_eq!(b, Vec4 { x: 3.0, y: 6.0, z: 9.0, w: 12.0 });
}

#[test]
fn test_scale_new_by_zero() {
  let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  let b = a * 0.0;
  assert_eq!(b, Vec4::zero());
}

#[test]
fn test_scale_new_by_one() {
  let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  let b = a * 1.0;
  assert_eq!(b, Vec4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 });
}

#[test]
fn test_scale_new_negative() {
  let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  let b = a * -2.0;
  assert_eq!(
    b,
    Vec4 {
      x: -2.0,
      y: -4.0,
      z: -6.0,
      w: -8.0
    }
  );
}

// --- Mul operator (Vec * Vec) ---

#[test]
fn test_mul_operator() {
  let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  let b = Vec4::new(5.0, 6.0, 7.0, 8.0);
  let c = a * b;
  assert_eq!(
    c,
    Vec4 {
      x: 5.0,
      y: 12.0,
      z: 21.0,
      w: 32.0
    }
  );
}

#[test]
fn test_mul_operator_by_zero() {
  let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  let c = a * Vec4::zero();
  assert_eq!(c, Vec4::zero());
}

#[test]
fn test_mul_operator_by_one_components() {
  let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  let c = a * Vec4::new(1.0, 1.0, 1.0, 1.0);
  assert_eq!(c, a);
}

#[test]
fn test_mul_operator_with_negatives() {
  let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  let b = Vec4::new(-1.0, -2.0, -3.0, -4.0);
  let c = a * b;
  assert_eq!(
    c,
    Vec4 {
      x: -1.0,
      y: -4.0,
      z: -9.0,
      w: -16.0
    }
  );
}

#[test]
fn test_mul_operator_both_negative() {
  let a = Vec4::new(-1.0, -2.0, -3.0, -4.0);
  let b = Vec4::new(-1.0, -2.0, -3.0, -4.0);
  let c = a * b;
  assert_eq!(c, Vec4 { x: 1.0, y: 4.0, z: 9.0, w: 16.0 });
}

// --- MulAssign operator (Vec *= Vec) ---

#[test]
fn test_mul_assign_operator() {
  let mut a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  a *= Vec4::new(5.0, 6.0, 7.0, 8.0);
  assert_eq!(
    a,
    Vec4 {
      x: 5.0,
      y: 12.0,
      z: 21.0,
      w: 32.0
    }
  );
}

#[test]
fn test_mul_assign_operator_by_zero() {
  let mut a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  a *= Vec4::zero();
  assert_eq!(a, Vec4::zero());
}

#[test]
fn test_mul_assign_operator_by_one_components() {
  let mut a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  a *= Vec4::new(1.0, 1.0, 1.0, 1.0);
  assert_eq!(a, Vec4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 });
}

#[test]
fn test_mul_assign_operator_with_negatives() {
  let mut a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  a *= Vec4::new(-1.0, -2.0, -3.0, -4.0);
  assert_eq!(
    a,
    Vec4 {
      x: -1.0,
      y: -4.0,
      z: -9.0,
      w: -16.0
    }
  );
}

#[test]
fn test_mul_assign_mutates_original() {
  let mut a = Vec4::new(2.0, 3.0, 4.0, 5.0);
  let b = Vec4::new(2.0, 2.0, 2.0, 2.0);
  a *= b;
  assert_eq!(a, Vec4 { x: 4.0, y: 6.0, z: 8.0, w: 10.0 });
}

#[test]
fn test_mul_operator_chained() {
  let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  let b = Vec4::new(2.0, 2.0, 2.0, 2.0);
  let c = Vec4::new(3.0, 3.0, 3.0, 3.0);
  let result = a * b * c;
  assert_eq!(
    result,
    Vec4 {
      x: 6.0,
      y: 12.0,
      z: 18.0,
      w: 24.0
    }
  );
}

#[test]
fn test_mul_assign_with_chained_mul() {
  let mut a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  a *= Vec4::new(2.0, 2.0, 2.0, 2.0);
  a *= Vec4::new(3.0, 3.0, 3.0, 3.0);
  assert_eq!(
    a,
    Vec4 {
      x: 6.0,
      y: 12.0,
      z: 18.0,
      w: 24.0
    }
  );
}

// --- Add operator (Vec + Vec) ---

#[test]
fn test_add_operator() {
  let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  let b = Vec4::new(4.0, 5.0, 6.0, 7.0);
  let c = a + b;
  assert_eq!(c, Vec4 { x: 5.0, y: 7.0, z: 9.0, w: 11.0 });
}

#[test]
fn test_add_operator_with_zero() {
  let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  let c = a + Vec4::zero();
  assert_eq!(c, a);
}

#[test]
fn test_add_operator_with_negatives() {
  let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  let b = Vec4::new(-1.0, -2.0, -3.0, -4.0);
  let c = a + b;
  assert_eq!(c, Vec4::zero());
}

#[test]
fn test_add_operator_both_negative() {
  let a = Vec4::new(-1.0, -2.0, -3.0, -4.0);
  let b = Vec4::new(-4.0, -5.0, -6.0, -7.0);
  let c = a + b;
  assert_eq!(
    c,
    Vec4 {
      x: -5.0,
      y: -7.0,
      z: -9.0,
      w: -11.0
    }
  );
}

#[test]
fn test_add_operator_chained() {
  let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  let b = Vec4::new(4.0, 5.0, 6.0, 7.0);
  let c = Vec4::new(7.0, 8.0, 9.0, 10.0);
  let result = a + b + c;
  assert_eq!(
    result,
    Vec4 {
      x: 12.0,
      y: 15.0,
      z: 18.0,
      w: 21.0
    }
  );
}

// --- AddAssign operator (Vec += Vec) ---

#[test]
fn test_add_assign_operator() {
  let mut a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  a += Vec4::new(4.0, 5.0, 6.0, 7.0);
  assert_eq!(a, Vec4 { x: 5.0, y: 7.0, z: 9.0, w: 11.0 });
}

#[test]
fn test_add_assign_operator_with_zero() {
  let mut a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  a += Vec4::zero();
  assert_eq!(a, Vec4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 });
}

#[test]
fn test_add_assign_operator_with_negatives() {
  let mut a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  a += Vec4::new(-1.0, -2.0, -3.0, -4.0);
  assert_eq!(a, Vec4::zero());
}

#[test]
fn test_add_assign_mutates_original() {
  let mut a = Vec4::new(2.0, 3.0, 4.0, 5.0);
  let b = Vec4::new(1.0, 1.0, 1.0, 1.0);
  a += b;
  assert_eq!(a, Vec4 { x: 3.0, y: 4.0, z: 5.0, w: 6.0 });
}

#[test]
fn test_add_assign_with_chained_add() {
  let mut a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  a += Vec4::new(1.0, 1.0, 1.0, 1.0);
  a += Vec4::new(2.0, 2.0, 2.0, 2.0);
  assert_eq!(a, Vec4 { x: 4.0, y: 5.0, z: 6.0, w: 7.0 });
}

// --- Sub operator (Vec - Vec) ---

#[test]
fn test_sub_operator() {
  let a = Vec4::new(5.0, 7.0, 9.0, 11.0);
  let b = Vec4::new(1.0, 2.0, 3.0, 4.0);
  let c = a - b;
  assert_eq!(c, Vec4 { x: 4.0, y: 5.0, z: 6.0, w: 7.0 });
}

#[test]
fn test_sub_operator_with_zero() {
  let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  let c = a - Vec4::zero();
  assert_eq!(c, a);
}

#[test]
fn test_sub_operator_with_negatives() {
  let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  let b = Vec4::new(-1.0, -2.0, -3.0, -4.0);
  let c = a - b;
  assert_eq!(c, Vec4 { x: 2.0, y: 4.0, z: 6.0, w: 8.0 });
}

#[test]
fn test_sub_operator_both_negative() {
  let a = Vec4::new(-1.0, -2.0, -3.0, -4.0);
  let b = Vec4::new(-4.0, -5.0, -6.0, -7.0);
  let c = a - b;
  assert_eq!(c, Vec4 { x: 3.0, y: 3.0, z: 3.0, w: 3.0 });
}

#[test]
fn test_sub_operator_chained() {
  let a = Vec4::new(10.0, 20.0, 30.0, 40.0);
  let b = Vec4::new(1.0, 2.0, 3.0, 4.0);
  let c = Vec4::new(2.0, 4.0, 6.0, 8.0);
  let result = a - b - c;
  assert_eq!(
    result,
    Vec4 {
      x: 7.0,
      y: 14.0,
      z: 21.0,
      w: 28.0
    }
  );
}

// --- SubAssign operator (Vec -= Vec) ---

#[test]
fn test_sub_assign_operator() {
  let mut a = Vec4::new(5.0, 7.0, 9.0, 11.0);
  a -= Vec4::new(1.0, 2.0, 3.0, 4.0);
  assert_eq!(a, Vec4 { x: 4.0, y: 5.0, z: 6.0, w: 7.0 });
}

#[test]
fn test_sub_assign_operator_with_zero() {
  let mut a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  a -= Vec4::zero();
  assert_eq!(a, Vec4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 });
}

#[test]
fn test_sub_assign_operator_with_negatives() {
  let mut a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  a -= Vec4::new(-1.0, -2.0, -3.0, -4.0);
  assert_eq!(a, Vec4 { x: 2.0, y: 4.0, z: 6.0, w: 8.0 });
}

#[test]
fn test_sub_assign_mutates_original() {
  let mut a = Vec4::new(5.0, 6.0, 7.0, 8.0);
  let b = Vec4::new(1.0, 1.0, 1.0, 1.0);
  a -= b;
  assert_eq!(a, Vec4 { x: 4.0, y: 5.0, z: 6.0, w: 7.0 });
}

#[test]
fn test_sub_assign_with_chained_sub() {
  let mut a = Vec4::new(10.0, 10.0, 10.0, 10.0);
  a -= Vec4::new(1.0, 2.0, 3.0, 4.0);
  a -= Vec4::new(1.0, 1.0, 1.0, 1.0);
  assert_eq!(a, Vec4 { x: 8.0, y: 7.0, z: 6.0, w: 5.0 });
}

// --- Div operator (Vec / Vec) ---

#[test]
fn test_div_operator() {
  let a = Vec4::new(8.0, 9.0, 16.0, 25.0);
  let b = Vec4::new(2.0, 3.0, 4.0, 5.0);
  let c = a / b;
  assert_eq!(c, Vec4 { x: 4.0, y: 3.0, z: 4.0, w: 5.0 });
}

#[test]
fn test_div_operator_by_one_components() {
  let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  let c = a / Vec4::new(1.0, 1.0, 1.0, 1.0);
  assert_eq!(c, a);
}

#[test]
fn test_div_operator_with_negatives() {
  let a = Vec4::new(8.0, 9.0, 16.0, 25.0);
  let b = Vec4::new(-2.0, -3.0, -4.0, -5.0);
  let c = a / b;
  assert_eq!(
    c,
    Vec4 {
      x: -4.0,
      y: -3.0,
      z: -4.0,
      w: -5.0
    }
  );
}

#[test]
fn test_div_operator_both_negative() {
  let a = Vec4::new(-8.0, -9.0, -16.0, -25.0);
  let b = Vec4::new(-2.0, -3.0, -4.0, -5.0);
  let c = a / b;
  assert_eq!(c, Vec4 { x: 4.0, y: 3.0, z: 4.0, w: 5.0 });
}

#[test]
fn test_div_operator_self_gives_ident_components() {
  let a = Vec4::new(2.0, 3.0, 4.0, 5.0);
  let c = a / a;
  assert_eq!(c, Vec4 { x: 1.0, y: 1.0, z: 1.0, w: 1.0 });
}

// --- DivAssign operator (Vec /= Vec) ---

#[test]
fn test_div_assign_operator() {
  let mut a = Vec4::new(8.0, 9.0, 16.0, 25.0);
  a /= Vec4::new(2.0, 3.0, 4.0, 5.0);
  assert_eq!(a, Vec4 { x: 4.0, y: 3.0, z: 4.0, w: 5.0 });
}

#[test]
fn test_div_assign_operator_by_one_components() {
  let mut a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  a /= Vec4::new(1.0, 1.0, 1.0, 1.0);
  assert_eq!(a, Vec4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 });
}

#[test]
fn test_div_assign_operator_with_negatives() {
  let mut a = Vec4::new(8.0, 9.0, 16.0, 25.0);
  a /= Vec4::new(-2.0, -3.0, -4.0, -5.0);
  assert_eq!(
    a,
    Vec4 {
      x: -4.0,
      y: -3.0,
      z: -4.0,
      w: -5.0
    }
  );
}

#[test]
fn test_div_assign_mutates_original() {
  let mut a = Vec4::new(8.0, 9.0, 16.0, 25.0);
  let b = Vec4::new(2.0, 3.0, 4.0, 5.0);
  a /= b;
  assert_eq!(a, Vec4 { x: 4.0, y: 3.0, z: 4.0, w: 5.0 });
}

// --- Index ---

#[test]
fn test_index_x() {
  let v = Vec4::new(3.0, 4.0, 5.0, 6.0);
  assert_eq!(v[0], 3.0);
}

#[test]
fn test_index_y() {
  let v = Vec4::new(3.0, 4.0, 5.0, 6.0);
  assert_eq!(v[1], 4.0);
}

#[test]
fn test_index_z() {
  let v = Vec4::new(3.0, 4.0, 5.0, 6.0);
  assert_eq!(v[2], 5.0);
}

#[test]
fn test_index_w() {
  let v = Vec4::new(3.0, 4.0, 5.0, 6.0);
  assert_eq!(v[3], 6.0);
}

#[test]
#[should_panic(expected = "Vec4 index must be in range: 0~3")]
fn test_index_out_of_bounds() {
  let v = Vec4::new(3.0, 4.0, 5.0, 6.0);
  let _ = v[4];
}

// --- Display ---

#[test]
fn test_display() {
  let v = Vec4::new(3.5, 4.5, 5.5, 6.5);
  let s = format!("{}", v);
  assert_eq!(s, "[3.5, 4.5, 5.5, 6.5]");
}

#[test]
fn test_display_negative() {
  let v = Vec4::new(-1.0, -2.0, -3.0, -4.0);
  let s = format!("{}", v);
  assert_eq!(s, "[-1, -2, -3, -4]");
}

// --- Copy & Clone ---

#[test]
fn test_copy_semantics() {
  let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  let b = a;
  // Both should be usable since Vec4 is Copy
  assert_eq!(a, b);
}

#[test]
fn test_clone() {
  let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  let b = a.clone();
  assert_eq!(a, b);
}

// --- Debug ---

#[test]
fn test_debug() {
  let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
  let s = format!("{:?}", v);
  assert!(s.contains("1.0"));
  assert!(s.contains("2.0"));
  assert!(s.contains("3.0"));
  assert!(s.contains("4.0"));
}

// --- Dot product ---

#[test]
fn test_dot_basic() {
  let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  let b = Vec4::new(5.0, 6.0, 7.0, 8.0);
  // 1*5 + 2*6 + 3*7 + 4*8 = 5 + 12 + 21 + 32 = 70
  assert_eq!(a.dot(b), 70.0);
}

#[test]
fn test_dot_with_zero() {
  let a = Vec4::new(5.0, 7.0, 9.0, 11.0);
  assert_eq!(a.dot(Vec4::zero()), 0.0);
}

#[test]
fn test_dot_perpendicular_xy() {
  let a = Vec4::new(1.0, 0.0, 0.0, 0.0);
  let b = Vec4::new(0.0, 1.0, 0.0, 0.0);
  assert_eq!(a.dot(b), 0.0);
}

#[test]
fn test_dot_perpendicular_xz() {
  let a = Vec4::new(1.0, 0.0, 0.0, 0.0);
  let b = Vec4::new(0.0, 0.0, 1.0, 0.0);
  assert_eq!(a.dot(b), 0.0);
}

#[test]
fn test_dot_perpendicular_xw() {
  let a = Vec4::new(1.0, 0.0, 0.0, 0.0);
  let b = Vec4::new(0.0, 0.0, 0.0, 1.0);
  assert_eq!(a.dot(b), 0.0);
}

#[test]
fn test_dot_perpendicular_yz() {
  let a = Vec4::new(0.0, 1.0, 0.0, 0.0);
  let b = Vec4::new(0.0, 0.0, 1.0, 0.0);
  assert_eq!(a.dot(b), 0.0);
}

#[test]
fn test_dot_perpendicular_zw() {
  let a = Vec4::new(0.0, 0.0, 1.0, 0.0);
  let b = Vec4::new(0.0, 0.0, 0.0, 1.0);
  assert_eq!(a.dot(b), 0.0);
}

#[test]
fn test_dot_parallel_same_direction() {
  let a = Vec4::new(2.0, 0.0, 0.0, 0.0);
  let b = Vec4::new(5.0, 0.0, 0.0, 0.0);
  assert_eq!(a.dot(b), 10.0);
}

#[test]
fn test_dot_parallel_opposite_direction() {
  let a = Vec4::new(3.0, 0.0, 0.0, 0.0);
  let b = Vec4::new(-3.0, 0.0, 0.0, 0.0);
  assert_eq!(a.dot(b), -9.0);
}

#[test]
fn test_dot_with_negatives() {
  let a = Vec4::new(-2.0, 3.0, -1.0, 4.0);
  let b = Vec4::new(4.0, -5.0, 2.0, -1.0);
  // -8 + -15 + -2 + -4 = -29
  assert_eq!(a.dot(b), -29.0);
}

#[test]
fn test_dot_commutative() {
  let a = Vec4::new(3.0, 7.0, 2.0, 5.0);
  let b = Vec4::new(2.0, 5.0, 4.0, 1.0);
  assert_eq!(a.dot(b), b.dot(a));
}

#[test]
fn test_dot_self_equals_len_squared() {
  let v = Vec4::new(3.0, 4.0, 5.0, 6.0);
  assert!((v.dot(v) - v.len() * v.len()).abs() < 1e-10);
}

// --- Distance ---

#[test]
fn test_dist_to_self_is_zero() {
  let a = Vec4::new(3.0, 4.0, 5.0, 6.0);
  assert_eq!(a.dist(a), 0.0);
}

#[test]
fn test_dist_1_2_2_4() {
  // diff (1, 2, 2, 4) -> sqrt(1+4+4+16) = sqrt(25) = 5
  let a = Vec4::zero();
  let b = Vec4::new(1.0, 2.0, 2.0, 4.0);
  assert_eq!(a.dist(b), 5.0);
}

#[test]
fn test_dist_along_axis_x() {
  let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  let b = Vec4::new(5.0, 2.0, 3.0, 4.0);
  assert_eq!(a.dist(b), 4.0);
}

#[test]
fn test_dist_along_axis_y() {
  let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  let b = Vec4::new(1.0, 9.0, 3.0, 4.0);
  assert_eq!(a.dist(b), 7.0);
}

#[test]
fn test_dist_along_axis_z() {
  let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  let b = Vec4::new(1.0, 2.0, 11.0, 4.0);
  assert_eq!(a.dist(b), 8.0);
}

#[test]
fn test_dist_along_axis_w() {
  let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  let b = Vec4::new(1.0, 2.0, 3.0, 13.0);
  assert_eq!(a.dist(b), 9.0);
}

#[test]
fn test_dist_symmetric() {
  let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  let b = Vec4::new(7.0, 8.0, 9.0, 10.0);
  assert_eq!(a.dist(b), b.dist(a));
}

#[test]
fn test_dist_with_negatives() {
  let a = Vec4::new(-1.0, -2.0, -2.0, -1.0);
  let b = Vec4::new(2.0, 2.0, 4.0, 1.0);
  // diffs: 3, 4, 6, 2 -> sqrt(9 + 16 + 36 + 4) = sqrt(65)
  assert!((a.dist(b) - f64::sqrt(65.0)).abs() < 1e-10);
}

#[test]
fn test_dist_from_zero_equals_len() {
  let v = Vec4::new(2.0, 3.0, 6.0, 0.0);
  assert!((Vec4::zero().dist(v) - v.len()).abs() < 1e-10);
}

// ============================================================================
// Tightening: asymmetric transposition catchers, w-specific behaviour
// ============================================================================

const TIGHT_EPSILON: f64 = 1e-10;

#[test]
fn test_new_field_order_asymmetric() {
  let v = Vec4::new(10.0, 20.0, 30.0, 40.0);
  assert_eq!(v.x, 10.0);
  assert_eq!(v.y, 20.0);
  assert_eq!(v.z, 30.0);
  assert_eq!(v.w, 40.0);
}

#[test]
fn test_index_matches_field_order() {
  let v = Vec4::new(7.0, 11.0, 13.0, 17.0);
  assert_eq!(v[0], v.x);
  assert_eq!(v[1], v.y);
  assert_eq!(v[2], v.z);
  assert_eq!(v[3], v.w);
}

#[test]
fn test_ident_pinned_behaviour() {
  // The implementation returns w=0.0 for ident() (not w=1.0). This test pins
  // that behaviour so any change is intentional.
  let v = Vec4::ident();
  assert_eq!(v, Vec4::new(1.0, 1.0, 1.0, 0.0));
}

#[test]
fn test_dot_w_participates() {
  // Vectors that agree on xyz but differ in w must give different dot results
  let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  let b = Vec4::new(1.0, 2.0, 3.0, 5.0);
  let c = Vec4::new(1.0, 2.0, 3.0, -5.0);
  assert!((a.dot(b) - a.dot(c) - 40.0).abs() < TIGHT_EPSILON); // (1+4+9+20) - (1+4+9-20) = 40
}

#[test]
fn test_dot_asymmetric() {
  // 2*5 + 3*7 + 4*11 + 6*13 = 10+21+44+78 = 153
  let a = Vec4::new(2.0, 3.0, 4.0, 6.0);
  let b = Vec4::new(5.0, 7.0, 11.0, 13.0);
  assert!((a.dot(b) - 153.0).abs() < TIGHT_EPSILON);
}

#[test]
fn test_dist_asymmetric_with_w() {
  // Deltas (1, 2, 2, 0) -> length 3
  let a = Vec4::new(0.0, 0.0, 0.0, 5.0);
  let b = Vec4::new(1.0, 2.0, 2.0, 5.0);
  assert!((a.dist(b) - 3.0).abs() < TIGHT_EPSILON);
}

#[test]
fn test_dist_w_only_difference() {
  // dist must include w
  let a = Vec4::new(1.0, 1.0, 1.0, 0.0);
  let b = Vec4::new(1.0, 1.0, 1.0, 5.0);
  assert!((a.dist(b) - 5.0).abs() < TIGHT_EPSILON);
}

#[test]
fn test_len_includes_w() {
  // (1, 2, 2, 4) -> sqrt(1+4+4+16) = 5
  let v = Vec4::new(1.0, 2.0, 2.0, 4.0);
  assert!((v.len() - 5.0).abs() < TIGHT_EPSILON);
}

#[test]
fn test_returning_ops_do_not_mutate_operands() {
  let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
  let b = Vec4::new(5.0, 6.0, 7.0, 8.0);
  let _ = a + b;
  let _ = a - b;
  let _ = a * b;
  let _ = a / b;
  let _ = a * 2.0;
  assert_eq!(a, Vec4::new(1.0, 2.0, 3.0, 4.0));
  assert_eq!(b, Vec4::new(5.0, 6.0, 7.0, 8.0));
}
