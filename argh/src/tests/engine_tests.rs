// ==============================================================================================
// Module & file:   engine_tests.rs
// Purpose:         Tests for the Engine type (model registry, time, basic state).
// Author & Date:   AI generated, 2026
// License:         MIT
// Notes:           Wired in as a sibling module of engine/mod.rs via #[path] include.
//                  See misc/testing-spec.md for the house style and conventions.
// ==============================================================================================

use super::*;
use crate::math::{Vec2, Vec3};
use crate::mesh::Mesh;
use crate::model::Model;

// --- Helpers ---

fn triangle_model(name: &str) -> Model {
  let mut m = Mesh::new();
  m.positions = vec![Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)];
  m.normals = vec![Vec3::new(0.0, 0.0, 1.0); 3];
  m.tex_coords = vec![Vec2::new(0.0, 0.0); 3];
  m.indices = vec![0, 1, 2];
  m.tri_count = 1;
  Model::from_mesh(m, name)
}

// --- Construction ---

#[test]
fn test_new_engine_size() {
  let e = Engine::new(800, 600);
  assert_eq!(e.size(), (800, 600));
}

#[test]
fn test_new_engine_aspect_ratio() {
  let e = Engine::new(800, 600);
  assert!((e.aspect() - (800.0 / 600.0)).abs() < 1e-5);
}

#[test]
fn test_new_engine_square_aspect_is_one() {
  let e = Engine::new(512, 512);
  assert!((e.aspect() - 1.0).abs() < 1e-5);
}

#[test]
fn test_new_engine_time_starts_zero() {
  let e = Engine::new(64, 64);
  assert_eq!(e.time(), 0.0);
}

#[test]
fn test_new_engine_stats_zero() {
  let e = Engine::new(64, 64);
  assert_eq!(e.stats(), 0);
}

#[test]
fn test_new_engine_debug_false_by_default() {
  let e = Engine::new(64, 64);
  assert!(!e.debug);
}

#[test]
fn test_new_engine_no_models_yet() {
  let e = Engine::new(64, 64);
  assert_eq!(e.models.len(), 0);
}

// --- tick ---

#[test]
fn test_tick_accumulates_time() {
  let mut e = Engine::new(64, 64);
  let t1 = e.tick(0.5);
  let t2 = e.tick(0.25);
  assert!((t1 - 0.5).abs() < 1e-9);
  assert!((t2 - 0.75).abs() < 1e-9);
}

#[test]
fn test_tick_returns_current_time() {
  let mut e = Engine::new(64, 64);
  for _ in 0..10 {
    e.tick(0.1);
  }
  assert!((e.time() - 1.0).abs() < 1e-6);
}

#[test]
fn test_tick_with_zero_dt_does_not_advance_time() {
  let mut e = Engine::new(64, 64);
  e.tick(0.0);
  assert_eq!(e.time(), 0.0);
}

#[test]
fn test_tick_repeated_monotonically_increasing() {
  let mut e = Engine::new(64, 64);
  let mut prev = 0.0;
  for _ in 0..5 {
    let t = e.tick(0.1);
    assert!(t > prev);
    prev = t;
  }
}

#[test]
fn test_tick_resets_per_frame_triangle_count() {
  // stat_rend_tri_frame is reset to 0 every tick.
  let mut e = Engine::new(64, 64);
  e.stat_rend_tri_frame = 42;
  e.tick(0.01);
  assert_eq!(e.stats(), 0);
}

// --- Model registry ---

#[test]
fn test_add_model_returns_resolvable_handle() {
  let mut e = Engine::new(64, 64);
  let h = e.add_model(triangle_model("tri"));
  assert!(e.models.contains_key(h));
}

#[test]
fn test_model_returns_stored_model() {
  let mut e = Engine::new(64, 64);
  let h = e.add_model(triangle_model("tri"));
  let m = e.model(h);
  assert_eq!(m.name(), "tri");
  assert_eq!(m.tri_count, 1);
}

#[test]
fn test_model_mut_allows_mutation() {
  let mut e = Engine::new(64, 64);
  let h = e.add_model(triangle_model("tri"));
  e.model_mut(h).tri_count = 999;
  assert_eq!(e.model(h).tri_count, 999);
}

#[test]
fn test_add_multiple_models_independent_handles() {
  let mut e = Engine::new(64, 64);
  let h1 = e.add_model(triangle_model("a"));
  let h2 = e.add_model(triangle_model("b"));
  assert_ne!(h1, h2);
  assert_eq!(e.models.len(), 2);
}

// --- draw_debug smoke ---

#[test]
fn test_draw_debug_does_not_panic() {
  let mut e = Engine::new(64, 64);
  e.draw_debug();
}
