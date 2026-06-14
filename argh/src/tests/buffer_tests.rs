// ==============================================================================================
// Module & file:   buffer_tests.rs
// Purpose:         Tests for the internal pixel/depth Buffer
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:           AI generated
// ==============================================================================================

use super::*;
use crate::colour::{BLACK, RED, WHITE};

// --- Constructor ---

#[test]
fn test_new_dimensions() {
  let b = Buffer::new(10, 5);
  assert_eq!(b.w, 10);
  assert_eq!(b.h, 5);
}

#[test]
fn test_new_pixel_buffer_length() {
  let b = Buffer::new(10, 5);
  assert_eq!(b.pixels.len(), 50);
}

#[test]
fn test_new_depth_buffer_length() {
  let b = Buffer::new(10, 5);
  assert_eq!(b.depth.len(), 50);
}

#[test]
fn test_new_pixels_zeroed() {
  let b = Buffer::new(4, 4);
  for p in &b.pixels {
    assert_eq!(*p, 0);
  }
}

#[test]
fn test_new_depth_zeroed() {
  let b = Buffer::new(4, 4);
  for d in &b.depth {
    assert_eq!(*d, 0.0);
  }
}

#[test]
fn test_new_zero_dimensions_allowed() {
  let b = Buffer::new(0, 0);
  assert_eq!(b.pixels.len(), 0);
  assert_eq!(b.depth.len(), 0);
}

// --- clear ---

#[test]
fn test_clear_fills_all_pixels() {
  let mut b = Buffer::new(3, 3);
  b.clear(RED);
  let target = RED.to_packed_0rgb();
  for p in &b.pixels {
    assert_eq!(*p, target);
  }
}

#[test]
fn test_clear_resets_depth() {
  let mut b = Buffer::new(3, 3);
  for d in b.depth.iter_mut() {
    *d = 0.7;
  }
  b.clear(BLACK);
  for d in &b.depth {
    assert_eq!(*d, 0.0);
  }
}

#[test]
fn test_clear_with_black_packs_zero() {
  let mut b = Buffer::new(2, 2);
  // First dirty the buffer.
  for p in b.pixels.iter_mut() {
    *p = 0x123456;
  }
  b.clear(BLACK);
  for p in &b.pixels {
    assert_eq!(*p, 0);
  }
}

// --- clear_depth ---

#[test]
fn test_clear_depth_only_touches_depth() {
  let mut b = Buffer::new(2, 2);
  b.clear(RED);
  let red_packed = RED.to_packed_0rgb();
  for d in b.depth.iter_mut() {
    *d = 0.5;
  }
  b.clear_depth();
  // Pixels unchanged.
  for p in &b.pixels {
    assert_eq!(*p, red_packed);
  }
  // Depth zeroed.
  for d in &b.depth {
    assert_eq!(*d, 0.0);
  }
}

// --- set_pixel ---

#[test]
fn test_set_pixel_in_bounds() {
  let mut b = Buffer::new(10, 5);
  b.set_pixel(2, 3, RED);
  let idx = 3 * 10 + 2;
  assert_eq!(b.pixels[idx], RED.to_packed_0rgb());
}

#[test]
fn test_set_pixel_other_pixels_untouched() {
  let mut b = Buffer::new(10, 5);
  b.set_pixel(2, 3, RED);
  for (i, p) in b.pixels.iter().enumerate() {
    if i != 3 * 10 + 2 {
      assert_eq!(*p, 0);
    }
  }
}

#[test]
fn test_set_pixel_out_of_bounds_x_noop() {
  let mut b = Buffer::new(10, 5);
  b.set_pixel(99, 0, RED);
  for p in &b.pixels {
    assert_eq!(*p, 0);
  }
}

#[test]
fn test_set_pixel_out_of_bounds_y_noop() {
  let mut b = Buffer::new(10, 5);
  b.set_pixel(0, 99, RED);
  for p in &b.pixels {
    assert_eq!(*p, 0);
  }
}

#[test]
fn test_set_pixel_at_edge_in_bounds() {
  let mut b = Buffer::new(10, 5);
  b.set_pixel(9, 4, RED); // last valid pixel
  assert_eq!(b.pixels[4 * 10 + 9], RED.to_packed_0rgb());
}

// --- set_pixel_depth ---

#[test]
fn test_set_pixel_depth_writes_when_z_greater() {
  let mut b = Buffer::new(4, 4);
  b.set_pixel_depth(1, 1, RED, 0.5);
  let idx = 4 + 1;
  assert_eq!(b.depth[idx], 0.5);
  assert_eq!(b.pixels[idx], RED.to_packed_0rgb());
}

#[test]
fn test_set_pixel_depth_ignores_when_z_smaller() {
  let mut b = Buffer::new(4, 4);
  b.set_pixel_depth(1, 1, RED, 0.8);
  b.set_pixel_depth(1, 1, WHITE, 0.3);
  // RED still in place because 0.3 is not greater than 0.8.
  let idx = 4 + 1;
  assert_eq!(b.depth[idx], 0.8);
  assert_eq!(b.pixels[idx], RED.to_packed_0rgb());
}

#[test]
fn test_set_pixel_depth_overwrites_when_z_greater() {
  let mut b = Buffer::new(4, 4);
  b.set_pixel_depth(1, 1, RED, 0.3);
  b.set_pixel_depth(1, 1, WHITE, 0.8);
  let idx = 4 + 1;
  assert_eq!(b.depth[idx], 0.8);
  assert_eq!(b.pixels[idx], WHITE.to_packed_0rgb());
}

// --- fill_rect ---

#[test]
fn test_fill_rect_basic() {
  let mut b = Buffer::new(10, 10);
  b.fill_rect(2, 3, 4, 2, RED);
  let target = RED.to_packed_0rgb();

  // Sample inside the rect.
  assert_eq!(b.pixels[3 * 10 + 2], target);
  assert_eq!(b.pixels[3 * 10 + 5], target);
  assert_eq!(b.pixels[4 * 10 + 2], target);
  assert_eq!(b.pixels[4 * 10 + 5], target);

  // Sample outside.
  assert_eq!(b.pixels[0], 0);
  assert_eq!(b.pixels[2 * 10 + 2], 0);
  assert_eq!(b.pixels[5 * 10 + 2], 0);
}

#[test]
fn test_fill_rect_clipped_at_right_edge() {
  let mut b = Buffer::new(10, 5);
  // Rect extends well past the right edge; should not panic.
  b.fill_rect(8, 0, 100, 1, RED);
  let target = RED.to_packed_0rgb();
  assert_eq!(b.pixels[8], target);
  assert_eq!(b.pixels[9], target);
}

#[test]
fn test_fill_rect_clipped_at_bottom_edge() {
  let mut b = Buffer::new(10, 5);
  b.fill_rect(0, 4, 1, 100, RED);
  let target = RED.to_packed_0rgb();
  assert_eq!(b.pixels[4 * 10], target);
}

#[test]
fn test_fill_rect_zero_size_noop() {
  let mut b = Buffer::new(10, 5);
  b.fill_rect(2, 2, 0, 0, RED);
  for p in &b.pixels {
    assert_eq!(*p, 0);
  }
}

// --- draw_char ---

#[test]
fn test_draw_char_known_glyph_no_panic() {
  let mut b = Buffer::new(40, 40);
  b.draw_char('A', 0, 0, RED);
}

#[test]
fn test_draw_char_unknown_glyph_noop() {
  let mut b = Buffer::new(40, 40);
  // Zero-width space - very unlikely to be in the glyph table.
  b.draw_char('\u{200B}', 0, 0, RED);
  for p in &b.pixels {
    assert_eq!(*p, 0);
  }
}

#[test]
fn test_draw_char_off_screen_x_noop_panic_free() {
  let mut b = Buffer::new(40, 40);
  // Draws clip in x via the inner bounds check.
  b.draw_char('A', 39, 0, RED);
}
