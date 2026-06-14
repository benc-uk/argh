// ==============================================================================================
// Module & file:   engine / draw2d_tests.rs
// Purpose:         Smoke tests for the Engine's 2D drawing helpers.
// Author & Date:   AI generated, 2026
// License:         MIT
// Notes:           State-only checks; we don't validate exact pixels beyond a couple of
//                  sentinel coordinates.
//                  See misc/testing-spec.md for the house style and conventions.
// ==============================================================================================

use crate::colour::{BLACK, Colour, RED, WHITE};
use crate::engine::Engine;
use crate::math::Vec2;

// --- clear ---

#[test]
fn test_clear_fills_pixel_buffer() {
  let mut e = Engine::new(8, 8);
  e.clear(RED);
  let packed = RED.to_packed_0rgb();
  assert_eq!(e.buffer.pixels[0], packed);
  assert_eq!(e.buffer.pixels[e.buffer.pixels.len() - 1], packed);
}

#[test]
fn test_clear_also_clears_depth() {
  let mut e = Engine::new(8, 8);
  for d in e.buffer.depth.iter_mut() {
    *d = 0.5;
  }
  e.clear(BLACK);
  for d in &e.buffer.depth {
    assert_eq!(*d, 0.0);
  }
}

// --- draw_string ---

#[test]
fn test_draw_string_does_not_panic() {
  let mut e = Engine::new(128, 32);
  e.draw_string("hello", 1, 1, WHITE);
}

#[test]
fn test_draw_string_empty_string_is_noop() {
  let mut e = Engine::new(32, 32);
  e.clear(BLACK);
  let before: Vec<u32> = e.buffer.pixels.clone();
  e.draw_string("", 1, 1, WHITE);
  assert_eq!(before, e.buffer.pixels);
}

// --- draw_rect ---

#[test]
fn test_draw_rect_modifies_target_pixels() {
  let mut e = Engine::new(16, 16);
  e.clear(BLACK);
  e.draw_rect(2, 2, 4, 4, WHITE);
  // The pixel at (3,3) should now be white.
  let idx = 3 * e.buffer.w + 3;
  assert_eq!(e.buffer.pixels[idx], WHITE.to_packed_0rgb());
}

#[test]
fn test_draw_rect_clipped_at_edge_does_not_panic() {
  let mut e = Engine::new(8, 8);
  e.clear(BLACK);
  e.draw_rect(6, 6, 100, 100, WHITE);
  // Bottom-right pixel inside the rect should be white.
  let idx = 7 * e.buffer.w + 7;
  assert_eq!(e.buffer.pixels[idx], WHITE.to_packed_0rgb());
}

// --- draw_line ---

#[test]
fn test_draw_line_does_not_panic_on_diagonal() {
  let mut e = Engine::new(16, 16);
  e.clear(BLACK);
  e.draw_line(0, 0, 15, 15, WHITE);
  // Endpoints should be set.
  assert_eq!(e.buffer.pixels[0], WHITE.to_packed_0rgb());
  let last = 15 * e.buffer.w + 15;
  assert_eq!(e.buffer.pixels[last], WHITE.to_packed_0rgb());
}

#[test]
fn test_draw_line_works_with_swapped_endpoints() {
  let mut e = Engine::new(16, 16);
  e.clear(BLACK);
  e.draw_line(15, 15, 0, 0, WHITE);
  assert_eq!(e.buffer.pixels[0], WHITE.to_packed_0rgb());
}

#[test]
fn test_draw_line_horizontal_segment() {
  let mut e = Engine::new(16, 16);
  e.clear(BLACK);
  e.draw_line(0, 5, 15, 5, WHITE);
  for x in 0..16 {
    let idx = 5 * e.buffer.w + x;
    assert_eq!(e.buffer.pixels[idx], WHITE.to_packed_0rgb());
  }
}

#[test]
fn test_draw_line_vertical_segment() {
  let mut e = Engine::new(16, 16);
  e.clear(BLACK);
  e.draw_line(7, 0, 7, 15, WHITE);
  for y in 0..16 {
    let idx = y * e.buffer.w + 7;
    assert_eq!(e.buffer.pixels[idx], WHITE.to_packed_0rgb());
  }
}

// --- draw_poly_line ---

#[test]
fn test_draw_poly_line_does_not_panic_on_open_shape() {
  let mut e = Engine::new(32, 32);
  e.clear(BLACK);
  let points = vec![Vec2::new(2.0, 2.0), Vec2::new(10.0, 2.0), Vec2::new(10.0, 10.0)];
  e.draw_poly_line(&points, WHITE);
}

#[test]
fn test_draw_poly_line_empty_is_noop() {
  let mut e = Engine::new(32, 32);
  e.clear(BLACK);
  let before = e.buffer.pixels.clone();
  e.draw_poly_line(&[], WHITE);
  assert_eq!(before, e.buffer.pixels);
}

#[test]
fn test_draw_poly_line_single_point_is_noop() {
  let mut e = Engine::new(32, 32);
  e.clear(BLACK);
  let before = e.buffer.pixels.clone();
  e.draw_poly_line(&[Vec2::new(5.0, 5.0)], WHITE);
  assert_eq!(before, e.buffer.pixels);
}

// --- Colour interaction smoke ---

#[test]
fn test_clear_with_arbitrary_colour() {
  let mut e = Engine::new(4, 4);
  let c = Colour::new(0.5, 0.5, 0.5);
  e.clear(c);
  let packed = c.to_packed_0rgb();
  for p in &e.buffer.pixels {
    assert_eq!(*p, packed);
  }
}
