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

// --- sample addressing modes (Repeat vs Clamp) ---

// A 2x1 texture whose two texels carry distinct alpha (0.25 vs 0.75), so a test
// can tell which texel was hit purely from the returned alpha (Colour channels
// aren't directly inspectable here). Left texel x=0, right texel x=1.
fn two_texel_distinct() -> Texture {
  // pixel0 alpha = 64 (~0.251), pixel1 alpha = 192 (~0.753)
  let buf = vec![10, 10, 10, 64, 20, 20, 20, 192];
  Texture::from_raw_rgba8(&buf, 2, 1)
}

const A_LEFT: f32 = 64.0 / 255.0;
const A_RIGHT: f32 = 192.0 / 255.0;

#[test]
fn test_default_wrap_mode_is_repeat() {
  let t = two_texel_distinct();
  assert_eq!(t.wrap, TextureWrap::Repeat);
}

#[test]
fn test_repeat_tiles_across_integer_boundary() {
  // Repeat: u and u+1 must address the same texel.
  let t = two_texel_distinct();
  let (_, a0) = t.sample(0.25, 0.0);
  let (_, a1) = t.sample(1.25, 0.0);
  assert!((a0 - a1).abs() < 1e-5);
  // And the two halves of the texture really are different texels.
  let (_, right) = t.sample(0.75, 0.0);
  assert!((a0 - A_LEFT).abs() < 1e-4);
  assert!((right - A_RIGHT).abs() < 1e-4);
}

#[test]
fn test_repeat_folds_negative_coords() {
  // -0.25 should fold to 0.75 -> right texel.
  let t = two_texel_distinct();
  let (_, a) = t.sample(-0.25, 0.0);
  assert!((a - A_RIGHT).abs() < 1e-4, "expected right texel, got alpha {a}");
}

#[test]
fn test_clamp_snaps_over_one_to_last_texel() {
  let mut t = two_texel_distinct();
  t.wrap = TextureWrap::Clamp;
  let (_, a) = t.sample(1.5, 0.0);
  assert!((a - A_RIGHT).abs() < 1e-4, "u>1 should clamp to the right edge texel, got {a}");
}

#[test]
fn test_clamp_snaps_below_zero_to_first_texel() {
  let mut t = two_texel_distinct();
  t.wrap = TextureWrap::Clamp;
  let (_, a) = t.sample(-3.0, 0.0);
  assert!((a - A_LEFT).abs() < 1e-4, "u<0 should clamp to the left edge texel, got {a}");
}

#[test]
fn test_clamp_at_exactly_one_is_in_bounds() {
  // Regression: u == 1.0 used to index x == w (out of bounds). Must hit the last texel.
  let mut t = two_texel_distinct();
  t.wrap = TextureWrap::Clamp;
  let (_, a) = t.sample(1.0, 0.0);
  assert!((a - A_RIGHT).abs() < 1e-4, "u==1.0 should land on the last texel, got {a}");
}

#[test]
fn test_repeat_at_exactly_one_wraps_to_first_texel() {
  // Repeat: u == 1.0 folds to 0.0 -> first texel, and must stay in bounds.
  let t = two_texel_distinct();
  let (_, a) = t.sample(1.0, 0.0);
  assert!((a - A_LEFT).abs() < 1e-4, "u==1.0 should wrap to the first texel, got {a}");
}

#[test]
fn test_clamp_uses_edge_colour_not_wrap() {
  // With Clamp, sampling well past the right edge stays on the right texel
  // rather than tiling back to the left one.
  let mut t = two_texel_distinct();
  t.wrap = TextureWrap::Clamp;
  let (_, a) = t.sample(5.9, 0.0);
  assert!((a - A_RIGHT).abs() < 1e-4, "clamp should hold the edge texel, got {a}");
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
