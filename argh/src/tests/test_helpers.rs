// ==============================================================================================
// Module & file:   test_helpers.rs
// Purpose:         Shared utilities for the test suite
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:           AI generated. Only compiled under #[cfg(test)].
// ==============================================================================================

#![allow(dead_code)]

use std::path::PathBuf;

use crate::math::{Mat4, Vec3};

/// Resolves a path relative to the `assets/` directory in the workspace root.
/// `CARGO_MANIFEST_DIR` for this crate points at `<root>/argh`, so we walk up one
/// to reach the workspace root, then descend into `assets/` and append `rel`.
pub(crate) fn asset_path(rel: &str) -> PathBuf {
  let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  let workspace_root = manifest.parent().expect("workspace root not found");
  workspace_root.join("assets").join(rel)
}

/// Build a flat raw RGBA8 buffer, all pixels set to `fill`.
pub(crate) fn tiny_rgba8(w: u32, h: u32, fill: [u8; 4]) -> Vec<u8> {
  let mut out = Vec::with_capacity((w * h * 4) as usize);
  for _ in 0..(w * h) {
    out.extend_from_slice(&fill);
  }
  out
}

/// Build a flat raw RGB8 buffer, all pixels set to `fill`.
pub(crate) fn tiny_rgb8(w: u32, h: u32, fill: [u8; 3]) -> Vec<u8> {
  let mut out = Vec::with_capacity((w * h * 3) as usize);
  for _ in 0..(w * h) {
    out.extend_from_slice(&fill);
  }
  out
}

/// Assert two Vec3s are equal within `eps` per component.
pub(crate) fn assert_vec3_near(a: Vec3, b: Vec3, eps: f32) {
  assert!(
    (a.x - b.x).abs() < eps && (a.y - b.y).abs() < eps && (a.z - b.z).abs() < eps,
    "vec3 mismatch: got {}, expected ~{} (eps {})",
    a,
    b,
    eps
  );
}

/// Assert two Mat4s are equal within `eps` per element.
pub(crate) fn assert_mat4_near(a: &Mat4, b: &Mat4, eps: f32) {
  let ar = a.raw_for_test();
  let br = b.raw_for_test();
  for r in 0..4 {
    for c in 0..4 {
      assert!(
        (ar[r][c] - br[r][c]).abs() < eps,
        "mat4 mismatch at [{},{}]: got {}, expected {} (eps {})",
        r,
        c,
        ar[r][c],
        br[r][c],
        eps
      );
    }
  }
}

/// Default epsilon for arithmetic comparisons.
pub(crate) const EPS: f32 = 1e-5;

/// Looser epsilon for trig-heavy comparisons (rotations, perspectives).
pub(crate) const EPS_TRIG: f32 = 1e-4;
