// ==============================================================================================
// Module & file:   camera_tests.rs
// Purpose:         Tests for the Camera type.
// Author & Date:   AI generated, 2026
// License:         MIT
// Notes:           See misc/testing-spec.md for the house style and conventions.
// ==============================================================================================

use super::*;
use crate::math::Vec3;
use crate::test_helpers::{EPS, EPS_TRIG, assert_mat4_near};

// --- Construction ---

#[test]
fn test_new_perspective_succeeds_for_normal_values() {
  let c = Camera::new_perspective(16.0 / 9.0, Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, 0.0), 60.0, 0.1, 1000.0);
  assert!(c.is_ok());
}

#[test]
fn test_new_perspective_zero_near_is_error() {
  let c = Camera::new_perspective(1.0, Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, 0.0), 60.0, 0.0, 100.0);
  assert!(c.is_err());
}

#[test]
fn test_new_perspective_far_equal_near_is_error() {
  let c = Camera::new_perspective(1.0, Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, 0.0), 60.0, 10.0, 10.0);
  assert!(c.is_err());
}

#[test]
fn test_new_perspective_far_less_than_near_is_error() {
  let c = Camera::new_perspective(1.0, Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, 0.0), 60.0, 100.0, 1.0);
  assert!(c.is_err());
}

#[test]
fn test_new_perspective_tiny_near_huge_far_ok() {
  let c = Camera::new_perspective(1.0, Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, 0.0), 60.0, 0.0001, 1.0e6);
  assert!(c.is_ok());
}

#[test]
fn test_new_perspective_tiny_aspect_ok() {
  let c = Camera::new_perspective(0.01, Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, 0.0), 60.0, 0.1, 100.0);
  assert!(c.is_ok());
}

#[test]
fn test_new_perspective_large_aspect_ok() {
  let c = Camera::new_perspective(100.0, Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, 0.0), 60.0, 0.1, 100.0);
  assert!(c.is_ok());
}

// --- Getters ---

#[test]
fn test_pos_returns_constructor_value() {
  let pos = Vec3::new(1.5, 2.5, -3.5);
  let c = Camera::new_perspective(1.0, pos, Vec3::new(0.0, 0.0, 0.0), 60.0, 0.1, 100.0).unwrap();
  assert_eq!(c.pos(), pos);
}

#[test]
fn test_look_at_returns_constructor_value() {
  let look_at = Vec3::new(4.0, -1.0, 7.0);
  let c = Camera::new_perspective(1.0, Vec3::new(0.0, 0.0, 5.0), look_at, 60.0, 0.1, 100.0).unwrap();
  assert_eq!(c.look_at(), look_at);
}

// --- Mutators ---

#[test]
fn test_set_pos_updates_position() {
  let mut c = Camera::new_perspective(1.0, Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, 0.0), 60.0, 0.1, 100.0).unwrap();
  let new_pos = Vec3::new(10.0, 5.0, 2.0);
  c.set_pos(new_pos);
  assert_eq!(c.pos(), new_pos);
}

#[test]
fn test_set_pos_updates_view_matrix() {
  let mut c = Camera::new_perspective(1.0, Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, 0.0), 60.0, 0.1, 100.0).unwrap();
  let before = c.view_mat;
  c.set_pos(Vec3::new(10.0, 5.0, 2.0));
  let after = c.view_mat;

  let differs = before
    .raw_for_test()
    .iter()
    .flatten()
    .zip(after.raw_for_test().iter().flatten())
    .any(|(a, b)| (a - b).abs() > EPS);
  assert!(differs, "view matrix should change after set_pos");
}

#[test]
fn test_set_pos_preserves_projection_matrix() {
  let mut c = Camera::new_perspective(1.0, Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, 0.0), 60.0, 0.1, 100.0).unwrap();
  let before = c.pers_mat;
  c.set_pos(Vec3::new(10.0, 5.0, 2.0));
  let after = c.pers_mat;
  assert_mat4_near(&before, &after, EPS);
}

#[test]
fn test_set_look_at_updates_look_at() {
  let mut c = Camera::new_perspective(1.0, Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, 0.0), 60.0, 0.1, 100.0).unwrap();
  let new_look = Vec3::new(1.0, 2.0, -3.0);
  c.set_look_at(new_look);
  assert_eq!(c.look_at(), new_look);
}

#[test]
fn test_set_look_at_updates_view_matrix() {
  let mut c = Camera::new_perspective(1.0, Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, 0.0), 60.0, 0.1, 100.0).unwrap();
  let before = c.view_mat;
  c.set_look_at(Vec3::new(5.0, 5.0, 0.0));
  let after = c.view_mat;
  let differs = before
    .raw_for_test()
    .iter()
    .flatten()
    .zip(after.raw_for_test().iter().flatten())
    .any(|(a, b)| (a - b).abs() > EPS);
  assert!(differs, "view matrix should change after set_look_at");
}

#[test]
fn test_set_look_at_preserves_projection() {
  let mut c = Camera::new_perspective(1.0, Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, 0.0), 60.0, 0.1, 100.0).unwrap();
  let before = c.pers_mat;
  c.set_look_at(Vec3::new(5.0, 5.0, 0.0));
  let after = c.pers_mat;
  assert_mat4_near(&before, &after, EPS);
}

#[test]
fn test_set_pos_repeated_settles_on_final() {
  let mut c = Camera::new_perspective(1.0, Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, 0.0), 60.0, 0.1, 100.0).unwrap();
  c.set_pos(Vec3::new(1.0, 0.0, 0.0));
  c.set_pos(Vec3::new(2.0, 0.0, 0.0));
  c.set_pos(Vec3::new(3.0, 0.0, 0.0));
  assert_eq!(c.pos(), Vec3::new(3.0, 0.0, 0.0));
}

// --- View-matrix orthonormality ---

#[test]
fn test_view_matrix_rotation_block_preserves_length() {
  // The 3x3 rotation block of an orthonormal view matrix has unit-length rows
  // (and columns). We don't care about storage convention; we just iterate the
  // four "outer" slots and treat the first three components as a basis vector.
  let c = Camera::new_perspective(1.0, Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, 0.0), 60.0, 0.1, 100.0).unwrap();
  let raw = c.view_mat.raw_for_test();

  for i in 0..3 {
    let basis = raw[i];
    let len = (basis[0] * basis[0] + basis[1] * basis[1] + basis[2] * basis[2]).sqrt();
    assert!((len - 1.0).abs() < EPS_TRIG, "basis {i} should be unit length, got {len}");
  }
}

// --- Locked behaviour (current quirk) ---

#[test]
fn test_pos_equal_look_at_produces_finite_or_nan_matrix() {
  // Degenerate case: forward vector is zero. Math::new_look_at returns a matrix
  // (possibly with NaN), but it must not panic. We lock that "does not panic"
  // behaviour here. If at some point this is changed to return an error,
  // update this test along with the change.
  let p = Vec3::new(1.0, 2.0, 3.0);
  let c = Camera::new_perspective(1.0, p, p, 60.0, 0.1, 100.0);
  assert!(c.is_ok(), "current behaviour: degenerate look-at does not return an error");
}
