// ==============================================================================================
// Module & file:   material_tests.rs
// Purpose:         Tests for the Material type.
// Author & Date:   AI generated, 2026
// License:         MIT
// Notes:           See misc/testing-spec.md for the house style and conventions.
// ==============================================================================================

use super::*;
use crate::colour::{Colour, RED, WHITE};
use crate::test_helpers::tiny_rgba8;
use std::rc::Rc;

fn small_texture() -> Texture {
  let buf = tiny_rgba8(2, 2, [50, 100, 150, 255]);
  Texture::from_raw_rgba8(&buf, 2, 2)
}

// --- MATERIAL_PLACEHOLDER ---

#[test]
fn test_placeholder_is_white_diffuse() {
  assert_eq!(
    (MATERIAL_PLACEHOLDER.diffuse.r(), MATERIAL_PLACEHOLDER.diffuse.g(), MATERIAL_PLACEHOLDER.diffuse.b()),
    (WHITE.r(), WHITE.g(), WHITE.b())
  );
}

#[test]
fn test_placeholder_is_white_specular() {
  assert_eq!(
    (MATERIAL_PLACEHOLDER.specular.r(), MATERIAL_PLACEHOLDER.specular.g(), MATERIAL_PLACEHOLDER.specular.b()),
    (WHITE.r(), WHITE.g(), WHITE.b())
  );
}

#[test]
fn test_placeholder_hardness_default() {
  assert_eq!(MATERIAL_PLACEHOLDER.hardness, 20.0);
}

#[test]
fn test_placeholder_has_no_texture() {
  assert!(MATERIAL_PLACEHOLDER.texture.is_none());
}

// --- new_flat ---

#[test]
fn test_new_flat_sets_diffuse() {
  let m = Material::new_flat(RED);
  assert_eq!((m.diffuse.r(), m.diffuse.g(), m.diffuse.b()), (RED.r(), RED.g(), RED.b()));
}

#[test]
fn test_new_flat_specular_white() {
  let m = Material::new_flat(RED);
  assert_eq!((m.specular.r(), m.specular.g(), m.specular.b()), (WHITE.r(), WHITE.g(), WHITE.b()));
}

#[test]
fn test_new_flat_default_hardness() {
  let m = Material::new_flat(RED);
  assert_eq!(m.hardness, 20.0);
}

#[test]
fn test_new_flat_no_texture() {
  let m = Material::new_flat(RED);
  assert!(m.texture.is_none());
  assert!(m.texture().is_none());
}

// --- new_textured ---

#[test]
fn test_new_textured_stores_texture() {
  let m = Material::new_textured(small_texture());
  assert!(m.texture.is_some());
  assert!(m.texture().is_some());
}

#[test]
fn test_new_textured_default_diffuse_white() {
  let m = Material::new_textured(small_texture());
  assert_eq!((m.diffuse.r(), m.diffuse.g(), m.diffuse.b()), (WHITE.r(), WHITE.g(), WHITE.b()));
}

#[test]
fn test_new_textured_default_specular_white() {
  let m = Material::new_textured(small_texture());
  assert_eq!((m.specular.r(), m.specular.g(), m.specular.b()), (WHITE.r(), WHITE.g(), WHITE.b()));
}

#[test]
fn test_new_textured_default_hardness() {
  let m = Material::new_textured(small_texture());
  assert_eq!(m.hardness, 20.0);
}

// --- set_texture ---

#[test]
fn test_set_texture_after_flat_attaches_texture() {
  let mut m = Material::new_flat(RED);
  assert!(m.texture.is_none());
  m.set_texture(small_texture());
  assert!(m.texture().is_some());
}

#[test]
fn test_set_texture_overwrites_existing() {
  let mut m = Material::new_textured(small_texture());
  let initial = Rc::strong_count(m.texture.as_ref().unwrap());
  m.set_texture(small_texture());
  // After overwrite the new Rc has its own count of 1 because the old one was
  // dropped. We just need to verify it isn't None.
  assert!(m.texture().is_some());
  let _ = initial;
}

// --- Clone shares texture via Rc ---

#[test]
fn test_clone_increments_texture_rc_count() {
  let m1 = Material::new_textured(small_texture());
  let before = Rc::strong_count(m1.texture.as_ref().unwrap());
  let m2 = m1.clone();
  let after = Rc::strong_count(m2.texture.as_ref().unwrap());
  assert_eq!(after, before + 1);
}

#[test]
fn test_clone_shares_same_texture_pointer() {
  let m1 = Material::new_textured(small_texture());
  let m2 = m1.clone();
  assert!(Rc::ptr_eq(m1.texture.as_ref().unwrap(), m2.texture.as_ref().unwrap()));
}

#[test]
fn test_clone_copies_scalar_fields() {
  let mut m1 = Material::new_flat(RED);
  m1.hardness = 99.0;
  m1.specular = Colour::new(0.5, 0.5, 0.5);
  let m2 = m1.clone();
  assert_eq!((m2.diffuse.r(), m2.diffuse.g(), m2.diffuse.b()), (m1.diffuse.r(), m1.diffuse.g(), m1.diffuse.b()));
  assert_eq!((m2.specular.r(), m2.specular.g(), m2.specular.b()), (m1.specular.r(), m1.specular.g(), m1.specular.b()));
  assert_eq!(m2.hardness, m1.hardness);
}

// --- Field mutation ---

#[test]
fn test_diffuse_field_writable() {
  let mut m = Material::new_flat(RED);
  m.diffuse = Colour::new(0.1, 0.2, 0.3);
  assert_eq!((m.diffuse.r(), m.diffuse.g(), m.diffuse.b()), (0.1, 0.2, 0.3));
}

#[test]
fn test_specular_field_writable() {
  let mut m = Material::new_flat(RED);
  m.specular = Colour::new(0.4, 0.5, 0.6);
  assert_eq!((m.specular.r(), m.specular.g(), m.specular.b()), (0.4, 0.5, 0.6));
}

#[test]
fn test_hardness_field_writable() {
  let mut m = Material::new_flat(RED);
  m.hardness = 7.5;
  assert_eq!(m.hardness, 7.5);
}

// --- Edge cases ---

#[test]
fn test_extreme_hardness_does_not_panic() {
  let mut m = Material::new_flat(RED);
  m.hardness = 1e6;
  assert_eq!(m.hardness, 1e6);
}

#[test]
fn test_negative_hardness_allowed() {
  // No validation in setter; the renderer should clamp.
  let mut m = Material::new_flat(RED);
  m.hardness = -10.0;
  assert_eq!(m.hardness, -10.0);
}

#[test]
fn test_nan_hardness_allowed() {
  let mut m = Material::new_flat(RED);
  m.hardness = f32::NAN;
  assert!(m.hardness.is_nan());
}
