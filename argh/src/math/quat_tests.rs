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
  let expected = Quat {
    w: FRAC_1_SQRT_2,
    x: FRAC_1_SQRT_2,
    y: 0.0,
    z: 0.0,
  };
  assert!(quat_approx_eq(&q, &expected));
}

#[test]
fn test_new_90_around_y() {
  let q = Quat::new(AXIS_Y, PI / 2.0);
  let expected = Quat {
    w: FRAC_1_SQRT_2,
    x: 0.0,
    y: FRAC_1_SQRT_2,
    z: 0.0,
  };
  assert!(quat_approx_eq(&q, &expected));
}

#[test]
fn test_new_90_around_z() {
  let q = Quat::new(AXIS_Z, PI / 2.0);
  let expected = Quat {
    w: FRAC_1_SQRT_2,
    x: 0.0,
    y: 0.0,
    z: FRAC_1_SQRT_2,
  };
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
  let expected = Quat {
    w: half.cos(),
    x: s,
    y: s,
    z: 0.0,
  };
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

// ============================================================================
// Tightening: rotation roundtrips, composition order, normalise edges
// ============================================================================

use crate::math::Mat4;
use std::f64::consts::FRAC_PI_2;

#[test]
fn test_new_full_turn_is_negative_identity() {
  // 360 degrees gives a quaternion that represents the same rotation as
  // identity but with w = -1 (double cover of SO(3))
  let q = Quat::new(AXIS_Z, 2.0 * PI);
  assert!(approx_eq(q.w, -1.0));
  assert!(approx_eq(q.x, 0.0));
  assert!(approx_eq(q.y, 0.0));
  assert!(approx_eq(q.z, 0.0));
}

#[test]
fn test_new_zero_axis_yields_identity() {
  // Pinning current behaviour: zero axis with non-zero angle gives w=cos(a/2),
  // x=y=z=0. This represents the identity rotation regardless of angle.
  let q = Quat::new(Vec3::new(0.0, 0.0, 0.0), 1.234);
  assert_eq!(q.x, 0.0);
  assert_eq!(q.y, 0.0);
  assert_eq!(q.z, 0.0);
  assert!(approx_eq(q.w, (1.234_f64 * 0.5).cos()));
}

#[test]
fn test_normalise_preserves_unit_quat() {
  let q = Quat::new(AXIS_Z, FRAC_PI_2);
  let n = q.normalise();
  assert!(quat_approx_eq(&q, &n));
}

#[test]
fn test_normalise_scales_to_unit() {
  let q = Quat { w: 2.0, x: 4.0, y: 4.0, z: 0.0 }; // length 6
  let n = q.normalise();
  let len_sq = n.w * n.w + n.x * n.x + n.y * n.y + n.z * n.z;
  assert!(approx_eq(len_sq, 1.0));
  // Direction preserved: ratios should match
  assert!(approx_eq(n.x / n.w, 2.0));
  assert!(approx_eq(n.y / n.w, 2.0));
}

#[test]
fn test_normalise_zero_quat_yields_nan() {
  // Pin current behaviour: zero quaternion divides by zero
  let z = Quat { w: 0.0, x: 0.0, y: 0.0, z: 0.0 };
  let n = z.normalise();
  assert!(n.w.is_nan() && n.x.is_nan() && n.y.is_nan() && n.z.is_nan());
}

#[test]
fn test_mul_identity_with_asymmetric_quat() {
  let q = Quat { w: 0.5, x: 0.1, y: -0.3, z: 0.7 };
  let id = Quat { w: 1.0, x: 0.0, y: 0.0, z: 0.0 };
  assert!(quat_approx_eq(&(id * q), &q));
  assert!(quat_approx_eq(&(q * id), &q));
}

#[test]
fn test_mul_inverse_via_negative_angle_yields_identity_rotation() {
  // q * q_inv as rotations should leave any vector unchanged
  let q = Quat::new(Vec3::new(0.6, -0.8, 0.0), 1.2); // unit axis, arbitrary angle
  let q_inv = Quat::new(Vec3::new(0.6, -0.8, 0.0), -1.2);
  let v = Vec3::new(1.0, 2.0, 3.0);
  let m = Mat4::new_rot(q) * Mat4::new_rot(q_inv);
  let r = m * &v;
  assert!((r.x - v.x).abs() < EPSILON);
  assert!((r.y - v.y).abs() < EPSILON);
  assert!((r.z - v.z).abs() < EPSILON);
}

#[test]
fn test_mul_composition_order_via_vector_rotation() {
  // Pin the meaning of q1 * q2: with this codebase's matrix convention,
  // Mat4::new_rot(q1 * q2) applied to v should equal the composition of
  // the two individual rotations (in some order). This test pins WHICH order.
  let qa = Quat::new(AXIS_X, FRAC_PI_2);
  let qb = Quat::new(AXIS_Z, FRAC_PI_2);
  let v = Vec3::new(1.0, 0.0, 0.0);

  let combined = Mat4::new_rot(qa * qb) * &v;
  let stepwise_a_then_b = Mat4::new_rot(qb) * &(Mat4::new_rot(qa) * &v);
  let stepwise_b_then_a = Mat4::new_rot(qa) * &(Mat4::new_rot(qb) * &v);

  // Exactly one of the two stepwise orderings should match qa*qb composition
  let matches_a_then_b =
    (combined.x - stepwise_a_then_b.x).abs() < EPSILON && (combined.y - stepwise_a_then_b.y).abs() < EPSILON && (combined.z - stepwise_a_then_b.z).abs() < EPSILON;
  let matches_b_then_a =
    (combined.x - stepwise_b_then_a.x).abs() < EPSILON && (combined.y - stepwise_b_then_a.y).abs() < EPSILON && (combined.z - stepwise_b_then_a.z).abs() < EPSILON;
  assert!(matches_a_then_b || matches_b_then_a, "qa*qb did not compose to either rotation order");
  // And the two orderings should differ for these axes (proves non-commutativity)
  assert!(!(matches_a_then_b && matches_b_then_a));
}

#[test]
fn test_mul_asymmetric_components_pinned() {
  // Hand-computed Hamilton product to catch sign/component bugs
  // a = (1, 2, 3, 4) with (w, x, y, z) = (1, 2, 3, 4)
  // b = (5, 6, 7, 8)
  // Per current impl (see Mul for Quat in quat.rs):
  //   r.w = b.w*a.w - b.x*a.x - b.y*a.y - b.z*a.z
  //   r.x = b.w*a.x + b.x*a.w - b.y*a.z + b.z*a.y
  //   r.y = b.w*a.y + b.x*a.z + b.y*a.w - b.z*a.x
  //   r.z = b.w*a.z - b.x*a.y + b.y*a.x + b.z*a.w
  // With a = (w=1, x=2, y=3, z=4), b = (w=5, x=6, y=7, z=8):
  //   r.w = 5 - 12 - 21 - 32 = -60
  //   r.x = 10 + 6 - 28 + 24 = 12
  //   r.y = 15 + 24 + 7 - 16 = 30
  //   r.z = 20 - 18 + 14 + 8 = 24
  let a = Quat { w: 1.0, x: 2.0, y: 3.0, z: 4.0 };
  let b = Quat { w: 5.0, x: 6.0, y: 7.0, z: 8.0 };
  let r = a * b;
  assert!(approx_eq(r.w, -60.0));
  assert!(approx_eq(r.x, 12.0));
  assert!(approx_eq(r.y, 30.0));
  assert!(approx_eq(r.z, 24.0));
}

// ============================================================================
// ident()
// ============================================================================

#[test]
fn test_ident_is_identity_quat() {
  let q = Quat::ident();
  assert_eq!(q, Quat { w: 1.0, x: 0.0, y: 0.0, z: 0.0 });
}

#[test]
fn test_ident_is_unit_length() {
  let q = Quat::ident();
  let len_sq = q.w * q.w + q.x * q.x + q.y * q.y + q.z * q.z;
  assert!(approx_eq(len_sq, 1.0));
}

#[test]
fn test_ident_is_neutral_under_multiplication() {
  let q = Quat::new(AXIS_Y, 0.83);
  assert!(quat_approx_eq(&(Quat::ident() * q), &q));
  assert!(quat_approx_eq(&(q * Quat::ident()), &q));
}

#[test]
fn test_ident_as_rotation_matrix_is_identity() {
  // When converted to a rotation matrix it should be the identity transform
  let m = Mat4::new_rot(Quat::ident());
  let v = Vec3::new(1.7, -2.3, 4.1);
  let r = m * &v;
  assert!((r.x - v.x).abs() < EPSILON);
  assert!((r.y - v.y).abs() < EPSILON);
  assert!((r.z - v.z).abs() < EPSILON);
}

// ============================================================================
// rot_x / rot_y / rot_z (in-place axis rotations)
//
// These post-multiply self by a rotation around the given axis, i.e.
// self_new = self * R_axis(a). They mutate in place.
// ============================================================================

#[test]
fn test_rot_x_from_identity_matches_new() {
  // Starting from identity, rotating by angle a around X should equal Quat::new(AXIS_X, a)
  let mut q = Quat::ident();
  q.rot_x(FRAC_PI_2);
  let expected = Quat::new(AXIS_X, FRAC_PI_2);
  assert!(quat_approx_eq(&q, &expected));
}

#[test]
fn test_rot_y_from_identity_matches_new() {
  let mut q = Quat::ident();
  q.rot_y(FRAC_PI_2);
  let expected = Quat::new(AXIS_Y, FRAC_PI_2);
  assert!(quat_approx_eq(&q, &expected));
}

#[test]
fn test_rot_z_from_identity_matches_new() {
  let mut q = Quat::ident();
  q.rot_z(FRAC_PI_2);
  let expected = Quat::new(AXIS_Z, FRAC_PI_2);
  assert!(quat_approx_eq(&q, &expected));
}

#[test]
fn test_rot_x_zero_angle_is_noop() {
  let original = Quat::new(AXIS_Y, 0.7);
  let mut q = original;
  q.rot_x(0.0);
  assert!(quat_approx_eq(&q, &original));
}

#[test]
fn test_rot_y_zero_angle_is_noop() {
  let original = Quat::new(AXIS_Z, 0.7);
  let mut q = original;
  q.rot_y(0.0);
  assert!(quat_approx_eq(&q, &original));
}

#[test]
fn test_rot_z_zero_angle_is_noop() {
  let original = Quat::new(AXIS_X, 0.7);
  let mut q = original;
  q.rot_z(0.0);
  assert!(quat_approx_eq(&q, &original));
}

#[test]
fn test_rot_x_preserves_unit_length() {
  let mut q = Quat::new(AXIS_Z, 1.1);
  q.rot_x(0.4);
  let len_sq = q.w * q.w + q.x * q.x + q.y * q.y + q.z * q.z;
  assert!(approx_eq(len_sq, 1.0));
}

#[test]
fn test_rot_y_preserves_unit_length() {
  let mut q = Quat::new(AXIS_X, 1.1);
  q.rot_y(0.4);
  let len_sq = q.w * q.w + q.x * q.x + q.y * q.y + q.z * q.z;
  assert!(approx_eq(len_sq, 1.0));
}

#[test]
fn test_rot_z_preserves_unit_length() {
  let mut q = Quat::new(AXIS_Y, 1.1);
  q.rot_z(0.4);
  let len_sq = q.w * q.w + q.x * q.x + q.y * q.y + q.z * q.z;
  assert!(approx_eq(len_sq, 1.0));
}

#[test]
fn test_rot_x_two_halves_equals_full() {
  let mut a = Quat::ident();
  a.rot_x(0.6);
  a.rot_x(0.6);
  let b = Quat::new(AXIS_X, 1.2);
  assert!(quat_approx_eq(&a, &b));
}

#[test]
fn test_rot_y_two_halves_equals_full() {
  let mut a = Quat::ident();
  a.rot_y(0.6);
  a.rot_y(0.6);
  let b = Quat::new(AXIS_Y, 1.2);
  assert!(quat_approx_eq(&a, &b));
}

#[test]
fn test_rot_z_two_halves_equals_full() {
  let mut a = Quat::ident();
  a.rot_z(0.6);
  a.rot_z(0.6);
  let b = Quat::new(AXIS_Z, 1.2);
  assert!(quat_approx_eq(&a, &b));
}

#[test]
fn test_rot_x_negative_undoes_positive() {
  let mut q = Quat::new(AXIS_Z, 0.5);
  let original = q;
  q.rot_x(0.7);
  q.rot_x(-0.7);
  assert!(quat_approx_eq(&q, &original));
}

#[test]
fn test_rot_y_negative_undoes_positive() {
  let mut q = Quat::new(AXIS_X, 0.5);
  let original = q;
  q.rot_y(0.7);
  q.rot_y(-0.7);
  assert!(quat_approx_eq(&q, &original));
}

#[test]
fn test_rot_z_negative_undoes_positive() {
  let mut q = Quat::new(AXIS_Y, 0.5);
  let original = q;
  q.rot_z(0.7);
  q.rot_z(-0.7);
  assert!(quat_approx_eq(&q, &original));
}

#[test]
fn test_rot_x_matches_postmultiply_by_axis_quat() {
  // Pin the convention: rot_x(a) is equivalent to self = self * Quat::new(AXIS_X, a)
  let start = Quat::new(Vec3::new(0.6, -0.8, 0.0), 0.9);
  let mut by_method = start;
  by_method.rot_x(0.4);
  let by_mul = start * Quat::new(AXIS_X, 0.4);
  assert!(quat_approx_eq(&by_method, &by_mul));
}

#[test]
fn test_rot_y_matches_postmultiply_by_axis_quat() {
  let start = Quat::new(Vec3::new(0.0, 0.8, 0.6), 0.9);
  let mut by_method = start;
  by_method.rot_y(0.4);
  let by_mul = start * Quat::new(AXIS_Y, 0.4);
  assert!(quat_approx_eq(&by_method, &by_mul));
}

#[test]
fn test_rot_z_matches_postmultiply_by_axis_quat() {
  let start = Quat::new(Vec3::new(0.7, 0.0, 0.7), 0.9);
  let mut by_method = start;
  by_method.rot_z(0.4);
  let by_mul = start * Quat::new(AXIS_Z, 0.4);
  assert!(quat_approx_eq(&by_method, &by_mul));
}

#[test]
fn test_rot_x_then_rot_x_via_vector_rotation() {
  // End-to-end: a vector rotated by an accumulated quat should match
  // the same vector rotated by the equivalent single-angle quat
  let mut q = Quat::ident();
  q.rot_x(0.3);
  q.rot_x(0.4);
  let m_acc = Mat4::new_rot(q);
  let m_eq = Mat4::new_rot(Quat::new(AXIS_X, 0.7));
  let v = Vec3::new(1.0, 2.0, 3.0);
  let ra = m_acc * &v;
  let rb = m_eq * &v;
  assert!((ra.x - rb.x).abs() < EPSILON);
  assert!((ra.y - rb.y).abs() < EPSILON);
  assert!((ra.z - rb.z).abs() < EPSILON);
}

#[test]
fn test_rot_x_then_rot_z_compose_via_vector() {
  // q = I * R_x(90) then * R_z(90) = R_x(90) * R_z(90)
  // When applied to a vector, q * v * q_conj rotates by R_z FIRST, then R_x
  // (this is the standard "rightmost rotation applied first" rule for
  // quaternion composition). Compare against an explicit two-step rotation.
  let mut q = Quat::ident();
  q.rot_x(FRAC_PI_2);
  q.rot_z(FRAC_PI_2);
  let m_via_quat = Mat4::new_rot(q);

  let m_z = Mat4::new_rot(Quat::new(AXIS_Z, FRAC_PI_2));
  let m_x = Mat4::new_rot(Quat::new(AXIS_X, FRAC_PI_2));

  let v = Vec3::new(1.0, 0.0, 0.0);
  let via_quat = m_via_quat * &v;
  let manual = m_x * &(m_z * &v); // apply Z first, then X

  assert!((via_quat.x - manual.x).abs() < EPSILON);
  assert!((via_quat.y - manual.y).abs() < EPSILON);
  assert!((via_quat.z - manual.z).abs() < EPSILON);
}

// ============================================================================
// World-frame vs local-frame rotation (the "wine bottle problem")
//
// rot_x/y/z         (LOCAL frame, post-multiply): self_new = self * R_axis(a)
// rot_x/y/z_world   (WORLD frame, pre-multiply):  self_new = R_axis(a) * self
//
// Classic scenario: a mesh authored along its +Z axis is "pre-tilted" upright
// with rot_x(-PI/2). After that pre-tilt:
//   - Model +Z points along world +Y (the "up" direction).
//   - Model +Y points along world -Z.
// So calling LOCAL rot_y after the pre-tilt rotates around world -Z (tumble),
// while WORLD rot_y_world rotates around world Y (spin like a top).
// These tests pin that distinction in for good.
// ============================================================================

#[test]
fn test_pretilt_puts_model_z_at_world_y() {
  // Identity, then pre-tilt by -90 around X (first rotation, so local == world).
  let mut q = Quat::ident();
  q.rot_x(-FRAC_PI_2);

  // Bottle neck direction (model +Z) should now point along world +Y.
  let neck = q.rotate_vec3(Vec3::new(0.0, 0.0, 1.0));
  assert!(approx_eq(neck.x, 0.0));
  assert!(approx_eq(neck.y, 1.0));
  assert!(approx_eq(neck.z, 0.0));

  // Model +Y now points along world -Z. This is what makes a subsequent
  // LOCAL rot_y "tumble" the bottle rather than spin it.
  let model_y = q.rotate_vec3(Vec3::new(0.0, 1.0, 0.0));
  assert!(approx_eq(model_y.x, 0.0));
  assert!(approx_eq(model_y.y, 0.0));
  assert!(approx_eq(model_y.z, -1.0));
}

#[test]
fn test_rot_y_world_after_pretilt_preserves_world_up() {
  // After the pre-tilt, world-frame rotation around Y must leave the "up"
  // direction unchanged, because rotating around the Y axis leaves Y fixed.
  let mut q = Quat::ident();
  q.rot_x(-FRAC_PI_2);
  q.rot_y_world(FRAC_PI_2);

  // Bottle neck (model +Z, already aligned with world +Y) stays at world +Y.
  let neck = q.rotate_vec3(Vec3::new(0.0, 0.0, 1.0));
  assert!(approx_eq(neck.x, 0.0));
  assert!(approx_eq(neck.y, 1.0));
  assert!(approx_eq(neck.z, 0.0));

  // Sanity: a horizontal feature on the bottle (model +X, which the pre-tilt
  // leaves at world +X) moves to world -Z under a +90 spin around world +Y.
  let side = q.rotate_vec3(Vec3::new(1.0, 0.0, 0.0));
  assert!(approx_eq(side.x, 0.0));
  assert!(approx_eq(side.y, 0.0));
  assert!(approx_eq(side.z, -1.0));
}

#[test]
fn test_rot_y_local_after_pretilt_tumbles_instead_of_spinning() {
  // The "broken-feeling" behaviour: local-Y rotation after the pre-tilt
  // rotates around the MODEL's +Y, which the pre-tilt placed at world -Z.
  // So the bottle's neck does NOT stay at world +Y; it tumbles to world +X.
  let mut q = Quat::ident();
  q.rot_x(-FRAC_PI_2);
  q.rot_y(FRAC_PI_2);

  let neck = q.rotate_vec3(Vec3::new(0.0, 0.0, 1.0));
  assert!(approx_eq(neck.x, 1.0));
  assert!(approx_eq(neck.y, 0.0));
  assert!(approx_eq(neck.z, 0.0));
}

#[test]
fn test_rot_y_world_and_local_only_diverge_when_prior_rotation_exists() {
  // From identity the two frames are equivalent: the local frame IS the world
  // frame when nothing has rotated yet.
  let mut q_local = Quat::ident();
  q_local.rot_y(FRAC_PI_2);
  let mut q_world = Quat::ident();
  q_world.rot_y_world(FRAC_PI_2);
  assert!(quat_approx_eq(&q_local, &q_world), "from identity, local and world Y must agree");

  // With a prior rotation in play, the two frames must produce different quats.
  let mut q_local = Quat::ident();
  q_local.rot_x(-FRAC_PI_2);
  q_local.rot_y(FRAC_PI_2);
  let mut q_world = Quat::ident();
  q_world.rot_x(-FRAC_PI_2);
  q_world.rot_y_world(FRAC_PI_2);
  assert!(!quat_approx_eq(&q_local, &q_world), "after a prior rotation, local and world Y must differ");
}
