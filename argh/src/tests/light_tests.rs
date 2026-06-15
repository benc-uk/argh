// ==============================================================================================
// Module & file:   light_tests.rs
// Purpose:         Tests for Light struct and its constructors
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:           AI generated
// ==============================================================================================

use super::*;
use crate::colour::{GREEN, RED, WHITE};

// --- Constructors ---

#[test]
fn test_new_sets_all_fields() {
  let l = Light::new_static(Vec3::new(1.0, 2.0, 3.0), 0.5, RED, 0.1, 0.01);
  assert_eq!(l.pos, Vec3::new(1.0, 2.0, 3.0));
  assert_eq!(l.brightness, 0.5);
  assert_eq!((l.colour.r(), l.colour.g(), l.colour.b()), (RED.r(), RED.g(), RED.b()));
  assert_eq!(l.atten_linear, 0.1);
  assert_eq!(l.atten_quad, 0.01);
  assert!(l.is_static);
  assert!(!l.is_dynamic);
}

#[test]
fn test_default_position_zero() {
  let l = Light::default();
  assert_eq!(l.pos, V3_ZERO);
}

#[test]
fn test_default_brightness_one() {
  let l = Light::default();
  assert_eq!(l.brightness, 1.0);
}

#[test]
fn test_default_colour_white() {
  let l = Light::default();
  assert_eq!((l.colour.r(), l.colour.g(), l.colour.b()), (WHITE.r(), WHITE.g(), WHITE.b()));
}

#[test]
fn test_default_attenuation() {
  let l = Light::default();
  assert_eq!(l.atten_linear, 0.09);
  assert_eq!(l.atten_quad, 0.032);
}

#[test]
fn test_default_flags() {
  let l = Light::default();
  assert!(!l.is_static);
  assert!(l.is_dynamic);
  assert!(l.is_enabled);
}

// --- Copy / Clone semantics ---

#[test]
fn test_light_is_copy() {
  let a = Light::default();
  let b = a;
  // Both usable after assignment thanks to Copy.
  assert_eq!(a.brightness, 1.0);
  assert_eq!(b.brightness, 1.0);
}

#[test]
fn test_light_clone_matches() {
  let a = Light::new_dynamic(Vec3::new(1.0, 2.0, 3.0), 0.7, GREEN, 0.5, 0.25);
  let b = a;
  assert_eq!(a.pos, b.pos);
  assert_eq!(a.brightness, b.brightness);
  assert_eq!((a.colour.r(), a.colour.g(), a.colour.b()), (b.colour.r(), b.colour.g(), b.colour.b()));
  assert_eq!(a.atten_linear, b.atten_linear);
  assert_eq!(a.atten_quad, b.atten_quad);
  assert_eq!(a.is_static, b.is_static);
  assert_eq!(a.is_dynamic, b.is_dynamic);
}

// --- Field mutation ---

#[test]
fn test_field_mutation_brightness() {
  let mut l = Light::default();
  l.brightness = 0.25;
  assert_eq!(l.brightness, 0.25);
}

#[test]
fn test_field_mutation_pos() {
  let mut l = Light::default();
  l.pos = Vec3::new(5.0, 6.0, 7.0);
  assert_eq!(l.pos, Vec3::new(5.0, 6.0, 7.0));
}

#[test]
fn test_field_mutation_both_static_and_dynamic() {
  let mut l = Light::default();
  l.is_static = true;
  l.is_dynamic = true;
  assert!(l.is_static);
  assert!(l.is_dynamic);
}

// --- Debug formatting ---

#[test]
fn test_debug_format_contains_struct_name() {
  let l = Light::default();
  let s = format!("{:?}", l);
  assert!(s.contains("Light"));
}

// --- Edge cases ---

#[test]
fn test_new_with_negative_brightness_allowed() {
  let l = Light::new_dynamic(V3_ZERO, -1.0, WHITE, 0.0, 0.0);
  assert_eq!(l.brightness, -1.0);
}

#[test]
fn test_new_with_huge_brightness_allowed() {
  let l = Light::new_dynamic(V3_ZERO, 1e6, WHITE, 0.0, 0.0);
  assert_eq!(l.brightness, 1e6);
}

#[test]
fn test_new_with_zero_attenuation_allowed() {
  let l = Light::new_dynamic(V3_ZERO, 1.0, WHITE, 0.0, 0.0);
  assert_eq!(l.atten_linear, 0.0);
  assert_eq!(l.atten_quad, 0.0);
}

#[test]
fn test_new_with_nan_brightness_allowed() {
  let l = Light::new_dynamic(V3_ZERO, f32::NAN, WHITE, 0.0, 0.0);
  assert!(l.brightness.is_nan());
}
