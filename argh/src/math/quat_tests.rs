// ==============================================================================================
// Module & file:   math / quat_tests.rs
// Purpose:         Tests for Quat quaternion operations
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:           AI generated
// ==============================================================================================

use super::*;
use std::f64::consts::{FRAC_1_SQRT_2, PI};

const EPSILON: f64 = 1e-10;

const AXIS_X: Vec3 = Vec3 { x: 1.0, y: 0.0, z: 0.0 };
const AXIS_Y: Vec3 = Vec3 { x: 0.0, y: 1.0, z: 0.0 };
const AXIS_Z: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 1.0 };

fn approx_eq(a: f64, b: f64) -> bool {
  (a - b).abs() < EPSILON
}

fn quat_approx_eq(a: &Quat, b: &Quat) -> bool {
  approx_eq(a.w, b.w) && approx_eq(a.x, b.x) && approx_eq(a.y, b.y) && approx_eq(a.z, b.z)
}

// ============================================================================
// Constructors / Default
// ============================================================================

#[test]
fn test_default_is_all_zero() {
  let q: Quat = Default::default();
  assert_eq!(q, Quat { w: 0.0, x: 0.0, y: 0.0, z: 0.0 });
}

#[test]
fn test_new_zero_angle_is_identity() {
  // Zero rotation around any axis should yield the identity quaternion (1, 0, 0, 0)
  let q = Quat::new(AXIS_X, 0.0);
  assert!(quat_approx_eq(&q, &Quat { w: 1.0, x: 0.0, y: 0.0, z: 0.0 }));
}

#[test]
fn test_new_180_around_x() {
  // 180 degrees around X axis: w = cos(90) = 0, x = sin(90) = 1
  let q = Quat::new(AXIS_X, PI);
  assert!(quat_approx_eq(&q, &Quat { w: 0.0, x: 1.0, y: 0.0, z: 0.0 }));
}

#[test]
fn test_new_180_around_y() {
  let q = Quat::new(AXIS_Y, PI);
  assert!(quat_approx_eq(&q, &Quat { w: 0.0, x: 0.0, y: 1.0, z: 0.0 }));
}

#[test]
fn test_new_180_around_z() {
  let q = Quat::new(AXIS_Z, PI);
  assert!(quat_approx_eq(&q, &Quat { w: 0.0, x: 0.0, y: 0.0, z: 1.0 }));
}

#[test]
fn test_new_90_around_x() {
  // 90 degrees around X: w = cos(45) = sqrt(2)/2, x = sin(45) = sqrt(2)/2
  let q = Quat::new(AXIS_X, PI / 2.0);
  let expected = Quat { w: FRAC_1_SQRT_2, x: FRAC_1_SQRT_2, y: 0.0, z: 0.0 };
  assert!(quat_approx_eq(&q, &expected));
}

#[test]
fn test_new_90_around_y() {
  let q = Quat::new(AXIS_Y, PI / 2.0);
  let expected = Quat { w: FRAC_1_SQRT_2, x: 0.0, y: FRAC_1_SQRT_2, z: 0.0 };
  assert!(quat_approx_eq(&q, &expected));
}

#[test]
fn test_new_90_around_z() {
  let q = Quat::new(AXIS_Z, PI / 2.0);
  let expected = Quat { w: FRAC_1_SQRT_2, x: 0.0, y: 0.0, z: FRAC_1_SQRT_2 };
  assert!(quat_approx_eq(&q, &expected));
}

#[test]
fn test_new_arbitrary_axis() {
  // 60 degrees around (1, 1, 0). Note the axis is not unit length, the
  // constructor multiplies xyz by sin(half) without normalising the axis,
  // so the resulting quat is not unit length either.
  let axis = Vec3::new(1.0, 1.0, 0.0);
  let q = Quat::new(axis, PI / 3.0);
  let half = PI / 6.0;
  let s = half.sin();
  let expected = Quat { w: half.cos(), x: s, y: s, z: 0.0 };
  assert!(quat_approx_eq(&q, &expected));
}

#[test]
fn test_new_negative_angle() {
  // A negative angle should flip the sign of the imaginary parts only,
  // since cos is even and sin is odd.
  let q_pos = Quat::new(AXIS_Z, PI / 4.0);
  let q_neg = Quat::new(AXIS_Z, -PI / 4.0);
  assert!(approx_eq(q_pos.w, q_neg.w));
  assert!(approx_eq(q_pos.x, -q_neg.x));
  assert!(approx_eq(q_pos.y, -q_neg.y));
  assert!(approx_eq(q_pos.z, -q_neg.z));
}

#[test]
fn test_new_from_unit_axis_is_unit_quat() {
  // When the axis is unit length, the resulting quaternion is unit length.
  let q = Quat::new(AXIS_X, 1.234);
  let len_sq = q.w * q.w + q.x * q.x + q.y * q.y + q.z * q.z;
  assert!(approx_eq(len_sq, 1.0));
}

// ============================================================================
// normalise
// ============================================================================

#[test]
fn test_normalise_unit_quat_is_unchanged() {
  let q = Quat { w: 1.0, x: 0.0, y: 0.0, z: 0.0 };
  let n = q.normalise();
  assert!(quat_approx_eq(&n, &q));
}

#[test]
fn test_normalise_produces_unit_length() {
  let q = Quat { w: 2.0, x: 3.0, y: 4.0, z: 5.0 };
  let n = q.normalise();
  let len_sq = n.w * n.w + n.x * n.x + n.y * n.y + n.z * n.z;
  assert!(approx_eq(len_sq, 1.0));
}

#[test]
fn test_normalise_preserves_direction() {
  let q = Quat { w: 2.0, x: 4.0, y: 6.0, z: 8.0 };
  let n = q.normalise();
  // All components scaled by the same factor, so ratios are preserved.
  let len = (2.0_f64 * 2.0 + 4.0 * 4.0 + 6.0 * 6.0 + 8.0 * 8.0).sqrt();
  let expected = Quat {
    w: 2.0 / len,
    x: 4.0 / len,
    y: 6.0 / len,
    z: 8.0 / len,
  };
  assert!(quat_approx_eq(&n, &expected));
}

#[test]
fn test_normalise_returns_new_value() {
  // normalise takes &self and returns a new Quat, leaving the original alone.
  let q = Quat { w: 2.0, x: 0.0, y: 0.0, z: 0.0 };
  let _ = q.normalise();
  assert_eq!(q, Quat { w: 2.0, x: 0.0, y: 0.0, z: 0.0 });
}

#[test]
fn test_normalise_arbitrary_rotation_quat_is_idempotent() {
  let q = Quat::new(AXIS_Y, 1.7);
  let n1 = q.normalise();
  let n2 = n1.normalise();
  assert!(quat_approx_eq(&n1, &n2));
}

// ============================================================================
// Multiplication (Hamilton product)
// ============================================================================

const IDENTITY: Quat = Quat { w: 1.0, x: 0.0, y: 0.0, z: 0.0 };

#[test]
fn test_mul_identity_left() {
  let q = Quat::new(AXIS_Z, 0.7);
  let r = IDENTITY * q;
  assert!(quat_approx_eq(&r, &q));
}

#[test]
fn test_mul_identity_right() {
  let q = Quat::new(AXIS_Z, 0.7);
  let r = q * IDENTITY;
  assert!(quat_approx_eq(&r, &q));
}

#[test]
fn test_mul_basis_ij_equals_k() {
  // Hamilton's relations: i*j = k
  let i = Quat { w: 0.0, x: 1.0, y: 0.0, z: 0.0 };
  let j = Quat { w: 0.0, x: 0.0, y: 1.0, z: 0.0 };
  let k = Quat { w: 0.0, x: 0.0, y: 0.0, z: 1.0 };
  assert!(quat_approx_eq(&(i * j), &k));
}

#[test]
fn test_mul_basis_jk_equals_i() {
  let i = Quat { w: 0.0, x: 1.0, y: 0.0, z: 0.0 };
  let j = Quat { w: 0.0, x: 0.0, y: 1.0, z: 0.0 };
  let k = Quat { w: 0.0, x: 0.0, y: 0.0, z: 1.0 };
  assert!(quat_approx_eq(&(j * k), &i));
}

#[test]
fn test_mul_basis_ki_equals_j() {
  let i = Quat { w: 0.0, x: 1.0, y: 0.0, z: 0.0 };
  let j = Quat { w: 0.0, x: 0.0, y: 1.0, z: 0.0 };
  let k = Quat { w: 0.0, x: 0.0, y: 0.0, z: 1.0 };
  assert!(quat_approx_eq(&(k * i), &j));
}

#[test]
fn test_mul_basis_is_anti_commutative() {
  // j*i = -k (opposite of i*j = k)
  let i = Quat { w: 0.0, x: 1.0, y: 0.0, z: 0.0 };
  let j = Quat { w: 0.0, x: 0.0, y: 1.0, z: 0.0 };
  let neg_k = Quat { w: 0.0, x: 0.0, y: 0.0, z: -1.0 };
  assert!(quat_approx_eq(&(j * i), &neg_k));
}

#[test]
fn test_mul_basis_squared_is_minus_one() {
  // i*i = j*j = k*k = -1 (i.e. quaternion (-1, 0, 0, 0))
  let i = Quat { w: 0.0, x: 1.0, y: 0.0, z: 0.0 };
  let j = Quat { w: 0.0, x: 0.0, y: 1.0, z: 0.0 };
  let k = Quat { w: 0.0, x: 0.0, y: 0.0, z: 1.0 };
  let minus_one = Quat { w: -1.0, x: 0.0, y: 0.0, z: 0.0 };
  assert!(quat_approx_eq(&(i * i), &minus_one));
  assert!(quat_approx_eq(&(j * j), &minus_one));
  assert!(quat_approx_eq(&(k * k), &minus_one));
}

#[test]
fn test_mul_compose_two_90_degree_x_rotations_is_180() {
  let q90 = Quat::new(AXIS_X, PI / 2.0);
  let composed = q90 * q90;
  let q180 = Quat::new(AXIS_X, PI);
  assert!(quat_approx_eq(&composed, &q180));
}

#[test]
fn test_mul_compose_two_90_degree_y_rotations_is_180() {
  let q90 = Quat::new(AXIS_Y, PI / 2.0);
  let composed = q90 * q90;
  let q180 = Quat::new(AXIS_Y, PI);
  assert!(quat_approx_eq(&composed, &q180));
}

#[test]
fn test_mul_is_not_commutative_for_different_axes() {
  // Rotations about different axes should not commute.
  let qx = Quat::new(AXIS_X, PI / 2.0);
  let qy = Quat::new(AXIS_Y, PI / 2.0);
  let a = qx * qy;
  let b = qy * qx;
  assert!(!quat_approx_eq(&a, &b));
}

#[test]
fn test_mul_preserves_unit_length() {
  // Product of two unit quaternions is itself unit length.
  let q1 = Quat::new(AXIS_X, 0.7);
  let q2 = Quat::new(AXIS_Y, 1.3);
  let p = q1 * q2;
  let len_sq = p.w * p.w + p.x * p.x + p.y * p.y + p.z * p.z;
  assert!(approx_eq(len_sq, 1.0));
}

// ============================================================================
// Derived traits (Copy / Clone / PartialEq)
// ============================================================================

#[test]
fn test_partial_eq() {
  let a = Quat { w: 1.0, x: 2.0, y: 3.0, z: 4.0 };
  let b = Quat { w: 1.0, x: 2.0, y: 3.0, z: 4.0 };
  let c = Quat { w: 1.0, x: 2.0, y: 3.0, z: 4.5 };
  assert_eq!(a, b);
  assert_ne!(a, c);
}

#[test]
fn test_copy_semantics() {
  // Quat is Copy, so the original is still usable after being passed by value.
  let a = Quat { w: 1.0, x: 2.0, y: 3.0, z: 4.0 };
  let _ = a * IDENTITY;
  assert_eq!(a, Quat { w: 1.0, x: 2.0, y: 3.0, z: 4.0 });
}

#[test]
fn test_clone() {
  let a = Quat { w: 1.0, x: 2.0, y: 3.0, z: 4.0 };
  #[allow(clippy::clone_on_copy)]
  let b = a.clone();
  assert_eq!(a, b);
}
