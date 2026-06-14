// ==============================================================================================
// Module & file:   helpers_tests.rs
// Purpose:         Tests for the internal helper functions used by the rasteriser.
// Author & Date:   AI generated, 2026
// License:         MIT
// Notes:           See misc/testing-spec.md for the house style and conventions.
// ==============================================================================================

use super::*;
use crate::colour::WHITE;
use crate::engine::LightHandle;
use crate::light::Light;
use crate::math::{Vec3, Vec4};
use slotmap::SlotMap;

// --- Helpers ---

fn empty_lights() -> SlotMap<LightHandle, Light> {
  SlotMap::with_key()
}

fn single_light(light: Light) -> SlotMap<LightHandle, Light> {
  let mut sm: SlotMap<LightHandle, Light> = SlotMap::with_key();
  sm.insert(light);
  sm
}

// --- compute_outcode ---

#[test]
fn test_compute_outcode_inside_returns_zero() {
  let v = Vec4::new(0.0, 0.0, 0.5, 1.0);
  assert_eq!(compute_outcode(&v), 0);
}

#[test]
fn test_compute_outcode_left_plane() {
  // x + w < 0 means out-left. Pick x = -2, w = 1.
  let v = Vec4::new(-2.0, 0.0, 0.5, 1.0);
  let code = compute_outcode(&v);
  assert!(code & OUT_LEFT != 0);
}

#[test]
fn test_compute_outcode_right_plane() {
  // w - x < 0 means out-right. Pick x = 2, w = 1.
  let v = Vec4::new(2.0, 0.0, 0.5, 1.0);
  let code = compute_outcode(&v);
  assert!(code & OUT_RIGHT != 0);
}

#[test]
fn test_compute_outcode_bottom_plane() {
  let v = Vec4::new(0.0, -2.0, 0.5, 1.0);
  let code = compute_outcode(&v);
  assert!(code & OUT_BOTTOM != 0);
}

#[test]
fn test_compute_outcode_top_plane() {
  let v = Vec4::new(0.0, 2.0, 0.5, 1.0);
  let code = compute_outcode(&v);
  assert!(code & OUT_TOP != 0);
}

#[test]
fn test_compute_outcode_far_plane() {
  // z < 0 means out-far.
  let v = Vec4::new(0.0, 0.0, -0.5, 1.0);
  let code = compute_outcode(&v);
  assert!(code & OUT_FAR != 0);
}

#[test]
fn test_compute_outcode_near_plane() {
  // w - z < 0 means out-near.
  let v = Vec4::new(0.0, 0.0, 2.0, 1.0);
  let code = compute_outcode(&v);
  assert!(code & OUT_NEAR != 0);
}

#[test]
fn test_compute_outcode_corner_combines_multiple_planes() {
  // Top-right corner outside should trip OUT_RIGHT and OUT_TOP.
  let v = Vec4::new(5.0, 5.0, 0.5, 1.0);
  let code = compute_outcode(&v);
  assert!(code & OUT_RIGHT != 0);
  assert!(code & OUT_TOP != 0);
}

// --- shade_vert ---

#[test]
fn test_shade_vert_no_lights_returns_black() {
  let lights = empty_lights();
  let (diff, spec) = shade_vert(&lights, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 0.0, 5.0), 20.0);
  assert_eq!(diff.channels(), (0.0, 0.0, 0.0));
  assert_eq!(spec.channels(), (0.0, 0.0, 0.0));
}

#[test]
fn test_shade_vert_back_facing_normal_returns_no_diffuse_or_spec() {
  // Normal points away from light; n_dot_l clamps to 0.
  let lights = single_light(Light::new(Vec3::new(0.0, 0.0, 5.0), 1.0, WHITE, 0.0, 0.0, false, false));
  let (diff, spec) = shade_vert(
    &lights,
    Vec3::new(0.0, 0.0, 0.0),
    Vec3::new(0.0, 0.0, -1.0), // facing away from light
    Vec3::new(0.0, 0.0, 10.0),
    20.0,
  );
  assert_eq!(diff.channels(), (0.0, 0.0, 0.0));
  assert_eq!(spec.channels(), (0.0, 0.0, 0.0));
}

#[test]
fn test_shade_vert_facing_light_produces_diffuse() {
  let lights = single_light(Light::new(Vec3::new(0.0, 0.0, 1.0), 1.0, WHITE, 0.0, 0.0, false, false));
  let (diff, _spec) = shade_vert(&lights, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 0.0, 5.0), 20.0);
  let (r, _, _) = diff.channels();
  assert!(r > 0.5, "expected strong diffuse, got r={r}");
}

#[test]
fn test_shade_vert_aligned_view_produces_specular() {
  // Light, eye and normal all aligned along +Z. Reflection vector aligns with view.
  let lights = single_light(Light::new(Vec3::new(0.0, 0.0, 5.0), 1.0, WHITE, 0.0, 0.0, false, false));
  let (_diff, spec) = shade_vert(&lights, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 0.0, 5.0), 20.0);
  let (r, _, _) = spec.channels();
  assert!(r > 0.5, "expected strong specular when view/light/normal aligned, got r={r}");
}

#[test]
fn test_shade_vert_attenuation_falls_off_with_distance() {
  // Same setup but compare two distances.
  let near = single_light(Light::new(Vec3::new(0.0, 0.0, 1.0), 1.0, WHITE, 0.1, 0.1, false, false));
  let far = single_light(Light::new(Vec3::new(0.0, 0.0, 10.0), 1.0, WHITE, 0.1, 0.1, false, false));

  let (diff_near, _) = shade_vert(&near, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 0.0, 5.0), 20.0);
  let (diff_far, _) = shade_vert(&far, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 0.0, 5.0), 20.0);
  assert!(diff_near.channels().0 > diff_far.channels().0, "near light should be brighter than far light");
}

#[test]
fn test_shade_vert_brightness_scales_diffuse() {
  let dim = single_light(Light::new(Vec3::new(0.0, 0.0, 1.0), 0.25, WHITE, 0.0, 0.0, false, false));
  let bright = single_light(Light::new(Vec3::new(0.0, 0.0, 1.0), 1.0, WHITE, 0.0, 0.0, false, false));
  let (dim_d, _) = shade_vert(&dim, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 0.0, 5.0), 20.0);
  let (bright_d, _) = shade_vert(&bright, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 0.0, 5.0), 20.0);
  // dim is 0.25 brightness so should be roughly a quarter.
  assert!(bright_d.channels().0 > dim_d.channels().0);
  assert!((bright_d.channels().0 * 0.25 - dim_d.channels().0).abs() < 1e-4);
}

// --- shade_vert_diffuse ---

#[test]
fn test_shade_vert_diffuse_skips_non_dynamic_lights() {
  // is_dynamic=false: should be skipped, returns black.
  let lights = single_light(Light::new(Vec3::new(0.0, 0.0, 1.0), 1.0, WHITE, 0.0, 0.0, true, false));
  let diff = shade_vert_diffuse(&lights, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
  assert_eq!(diff.channels(), (0.0, 0.0, 0.0));
}

#[test]
fn test_shade_vert_diffuse_uses_dynamic_lights() {
  let lights = single_light(Light::new(Vec3::new(0.0, 0.0, 1.0), 1.0, WHITE, 0.0, 0.0, false, true));
  let diff = shade_vert_diffuse(&lights, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
  assert!(diff.channels().0 > 0.5);
}

#[test]
fn test_shade_vert_diffuse_back_facing_returns_black() {
  let lights = single_light(Light::new(Vec3::new(0.0, 0.0, 1.0), 1.0, WHITE, 0.0, 0.0, false, true));
  let diff = shade_vert_diffuse(&lights, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
  assert_eq!(diff.channels(), (0.0, 0.0, 0.0));
}

#[test]
fn test_shade_vert_diffuse_no_lights_returns_black() {
  let lights = empty_lights();
  let diff = shade_vert_diffuse(&lights, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
  assert_eq!(diff.channels(), (0.0, 0.0, 0.0));
}

// --- FpsAveragerEight ---

#[test]
fn test_fps_averager_empty_returns_zero() {
  let a = FpsAveragerEight::new();
  assert_eq!(a.avg_fps(), 0.0);
}

#[test]
fn test_fps_averager_single_sample_returns_itself() {
  let mut a = FpsAveragerEight::new();
  a.add_fps(60.0);
  assert!((a.avg_fps() - 60.0).abs() < 1e-4);
}

#[test]
fn test_fps_averager_partial_fill_averages_correctly() {
  let mut a = FpsAveragerEight::new();
  for v in [10.0, 20.0, 30.0, 40.0] {
    a.add_fps(v);
  }
  // avg of 4 samples: (10+20+30+40)/4 = 25.0
  assert!((a.avg_fps() - 25.0).abs() < 1e-4);
}

#[test]
fn test_fps_averager_full_buffer_averages_correctly() {
  let mut a = FpsAveragerEight::new();
  for _ in 0..8 {
    a.add_fps(64.0);
  }
  assert!((a.avg_fps() - 64.0).abs() < 1e-4);
}

#[test]
fn test_fps_averager_ninth_sample_evicts_first() {
  let mut a = FpsAveragerEight::new();
  // Fill with eight 100s
  for _ in 0..8 {
    a.add_fps(100.0);
  }
  // Ninth sample 0 evicts the first 100. New sum = 700, avg = 87.5.
  a.add_fps(0.0);
  assert!((a.avg_fps() - 87.5).abs() < 1e-4);
}

#[test]
fn test_fps_averager_ring_wraps_repeatedly() {
  let mut a = FpsAveragerEight::new();
  // Fill with eight 50s
  for _ in 0..8 {
    a.add_fps(50.0);
  }
  // Now overwrite all 8 slots with 200.0
  for _ in 0..8 {
    a.add_fps(200.0);
  }
  assert!((a.avg_fps() - 200.0).abs() < 1e-4);
}
