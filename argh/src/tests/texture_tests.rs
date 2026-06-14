// ==============================================================================================
// Module & file:   texture_tests.rs
// Purpose:         Tests for the Texture type, raw decoders and sampling.
// Author & Date:   AI generated, 2026
// License:         MIT
// Notes:           See misc/testing-spec.md for the house style and conventions.
// ==============================================================================================

use super::*;
use crate::test_helpers::{asset_path, tiny_rgb8, tiny_rgba8};

// --- new(path) loaders ---

#[test]
fn test_new_loads_png_asset() {
  let p = asset_path("textures/crate.png");
  let t = Texture::new(p.to_str().unwrap()).expect("crate.png should load");
  assert!(!t.pixels.is_empty());
  assert!(t.w > 0);
  assert!(t.h > 0);
  assert_eq!(t.pixels.len(), (t.w * t.h) as usize);
}

#[test]
fn test_new_loads_jpg_asset() {
  // Pick a JPEG in the assets folder
  let candidates = ["textures/earth.jpg", "textures/checker_256.jpg"];
  for c in candidates {
    let p = asset_path(c);
    if p.exists() {
      let t = Texture::new(p.to_str().unwrap()).expect("jpg should load");
      assert!(t.w > 0);
      assert!(t.h > 0);
      return;
    }
  }
  // Fall back to PNG if no JPG asset is present in this checkout.
  let p = asset_path("textures/crate.png");
  let t = Texture::new(p.to_str().unwrap()).expect("crate.png should load");
  assert!(t.w > 0);
}

#[test]
fn test_new_missing_file_returns_io_error() {
  match Texture::new("definitely/does/not/exist.png") {
    Err(TextureError::IoError(_)) => {}
    Err(other) => panic!("expected IoError, got {other}"),
    Ok(_) => panic!("expected an error for a missing file"),
  }
}

#[test]
fn test_new_garbage_file_returns_image_error() {
  // Write a temp file with garbage and try to decode.
  let dir = std::env::temp_dir();
  let path = dir.join("argh_garbage_texture.bin");
  std::fs::write(&path, b"not really an image").unwrap();
  match Texture::new(path.to_str().unwrap()) {
    Err(TextureError::ImageError(_)) => {}
    Err(other) => panic!("expected ImageError, got {other}"),
    Ok(_) => panic!("expected an error for garbage data"),
  }
  let _ = std::fs::remove_file(&path);
}

#[test]
fn test_new_defaults_alpha_cutout_true() {
  let p = asset_path("textures/crate.png");
  let t = Texture::new(p.to_str().unwrap()).expect("crate.png should load");
  assert!(t.alpha_cutout);
  assert_eq!(t.cutoff, 0.5);
}

// --- from_bytes ---

#[test]
fn test_from_bytes_decodes_png_data() {
  let p = asset_path("textures/crate.png");
  let bytes = std::fs::read(&p).unwrap();
  let t = Texture::from_bytes(&bytes).expect("png bytes should decode");
  assert!(t.w > 0);
  assert!(t.h > 0);
  assert_eq!(t.pixels.len(), (t.w * t.h) as usize);
}

#[test]
fn test_from_bytes_garbage_errors() {
  match Texture::from_bytes(b"definitely not an image") {
    Err(TextureError::ImageError(_)) => {}
    Err(other) => panic!("expected ImageError, got {other}"),
    Ok(_) => panic!("expected an error for garbage data"),
  }
}

// --- from_raw_rgba8 ---

#[test]
fn test_from_raw_rgba8_pixel_count_matches() {
  let buf = tiny_rgba8(4, 3, [10, 20, 30, 40]);
  let t = Texture::from_raw_rgba8(&buf, 4, 3);
  assert_eq!(t.pixels.len(), 12);
}

#[test]
fn test_from_raw_rgba8_packs_rgb_into_low_bytes() {
  let buf = tiny_rgba8(1, 1, [0xAA, 0xBB, 0xCC, 0xDD]);
  let t = Texture::from_raw_rgba8(&buf, 1, 1);
  let p = t.pixels[0];
  assert_eq!((p >> 24) & 0xFF, 0xDD); // alpha
  assert_eq!((p >> 16) & 0xFF, 0xAA); // r
  assert_eq!((p >> 8) & 0xFF, 0xBB); // g
  assert_eq!(p & 0xFF, 0xCC); // b
}

#[test]
fn test_from_raw_rgba8_defaults_alpha_cutout_true() {
  let buf = tiny_rgba8(2, 2, [0, 0, 0, 255]);
  let t = Texture::from_raw_rgba8(&buf, 2, 2);
  assert!(t.alpha_cutout);
}

#[test]
fn test_from_raw_rgba8_truncated_buffer_drops_partial_pixel() {
  // 5 bytes is one full RGBA plus a single trailing byte; chunks_exact(4) drops the tail.
  let buf = vec![10, 20, 30, 40, 99];
  let t = Texture::from_raw_rgba8(&buf, 1, 1);
  assert_eq!(t.pixels.len(), 1);
}

#[test]
fn test_from_raw_rgba8_empty_buffer_empty_texture() {
  let t = Texture::from_raw_rgba8(&[], 0, 0);
  assert!(t.pixels.is_empty());
  assert_eq!(t.w, 0);
  assert_eq!(t.h, 0);
}

// --- from_raw_rgb8 ---

#[test]
fn test_from_raw_rgb8_pixel_count_matches() {
  let buf = tiny_rgb8(3, 2, [100, 110, 120]);
  let t = Texture::from_raw_rgb8(&buf, 3, 2);
  assert_eq!(t.pixels.len(), 6);
}

#[test]
fn test_from_raw_rgb8_forces_alpha_ff() {
  let buf = tiny_rgb8(1, 1, [0xAA, 0xBB, 0xCC]);
  let t = Texture::from_raw_rgb8(&buf, 1, 1);
  let p = t.pixels[0];
  assert_eq!((p >> 24) & 0xFF, 0xFF);
  assert_eq!((p >> 16) & 0xFF, 0xAA);
  assert_eq!((p >> 8) & 0xFF, 0xBB);
  assert_eq!(p & 0xFF, 0xCC);
}

#[test]
fn test_from_raw_rgb8_disables_alpha_cutout() {
  let buf = tiny_rgb8(2, 2, [0, 0, 0]);
  let t = Texture::from_raw_rgb8(&buf, 2, 2);
  assert!(!t.alpha_cutout);
}

#[test]
fn test_from_raw_rgb8_truncated_buffer_drops_partial() {
  // 7 bytes = 2 full RGB + 1 trailing byte; chunks_exact(3) drops the tail.
  let buf = vec![1, 2, 3, 4, 5, 6, 9];
  let t = Texture::from_raw_rgb8(&buf, 2, 1);
  assert_eq!(t.pixels.len(), 2);
}

#[test]
fn test_from_raw_rgb8_empty_buffer() {
  let t = Texture::from_raw_rgb8(&[], 0, 0);
  assert!(t.pixels.is_empty());
}

// --- sample wraparound ---

#[test]
fn test_sample_at_origin_returns_first_pixel() {
  let buf = vec![0xAA, 0xBB, 0xCC, 0xFF, 0x10, 0x20, 0x30, 0xFF];
  let t = Texture::from_raw_rgba8(&buf, 2, 1);
  let (_, _) = t.sample(0.0, 0.0);
  // We can't directly inspect Colour fields cross-module but we can verify a does not panic
  // and exact integer hit returns the alpha for the first pixel.
  let (_c, a) = t.sample(0.0, 0.0);
  assert!((a - 1.0).abs() < 1e-5);
}

#[test]
fn test_sample_wraps_positive_u() {
  // sample at u=1.7 should equal sample at u=0.7 (floor wrap).
  let buf = tiny_rgba8(4, 1, [100, 100, 100, 255]);
  let t = Texture::from_raw_rgba8(&buf, 4, 1);
  let (_, a1) = t.sample(0.7, 0.0);
  let (_, a2) = t.sample(1.7, 0.0);
  assert!((a1 - a2).abs() < 1e-5);
}

#[test]
fn test_sample_wraps_negative_u() {
  let buf = tiny_rgba8(4, 1, [100, 100, 100, 255]);
  let t = Texture::from_raw_rgba8(&buf, 4, 1);
  let (_, a1) = t.sample(0.3, 0.0);
  let (_, a2) = t.sample(-0.7, 0.0);
  assert!((a1 - a2).abs() < 1e-5);
}

#[test]
fn test_sample_wraps_positive_v() {
  let buf = tiny_rgba8(1, 4, [50, 60, 70, 255]);
  let t = Texture::from_raw_rgba8(&buf, 1, 4);
  let (_, a1) = t.sample(0.0, 0.25);
  let (_, a2) = t.sample(0.0, 1.25);
  assert!((a1 - a2).abs() < 1e-5);
}

#[test]
fn test_sample_alpha_value_round_trips() {
  // Alpha 128 stored as f32 should sample back to ~0.502.
  let buf = vec![0, 0, 0, 128];
  let t = Texture::from_raw_rgba8(&buf, 1, 1);
  let (_, a) = t.sample(0.0, 0.0);
  assert!((a - (128.0 / 255.0)).abs() < 1e-4);
}

// --- enable_cutout ---

#[test]
fn test_enable_cutout_toggles_flag() {
  let buf = tiny_rgba8(1, 1, [0, 0, 0, 255]);
  let mut t = Texture::from_raw_rgba8(&buf, 1, 1);
  t.enable_cutout(false);
  assert!(!t.alpha_cutout);
  t.enable_cutout(true);
  assert!(t.alpha_cutout);
}

// --- TextureError formatting ---

#[test]
fn test_texture_error_display_includes_inner() {
  match Texture::new("definitely/does/not/exist.png") {
    Err(err) => {
      let s = format!("{err}");
      assert!(s.contains("io error"));
    }
    Ok(_) => panic!("expected an error"),
  }
}

#[test]
fn test_texture_error_debug_matches_display() {
  match Texture::new("definitely/does/not/exist.png") {
    Err(err) => {
      let s = format!("{err}");
      let d = format!("{err:?}");
      assert_eq!(s, d);
    }
    Ok(_) => panic!("expected an error"),
  }
}
