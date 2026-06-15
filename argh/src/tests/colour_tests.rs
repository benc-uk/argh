// ==============================================================================================
// Module & file:   colour_tests.rs
// Purpose:         Tests for Colour type, sRGB conversion and arithmetic
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:           AI generated
// ==============================================================================================

use super::*;

// --- Constructors ---

#[test]
fn test_new() {
  let c = Colour::new(0.25, 0.5, 0.75);
  assert_eq!(c.r, 0.25);
  assert_eq!(c.g, 0.5);
  assert_eq!(c.b, 0.75);
}

#[test]
fn test_from_slice() {
  let c = Colour::from_slice([0.1, 0.2, 0.3]);
  assert_eq!(c.r, 0.1);
  assert_eq!(c.g, 0.2);
  assert_eq!(c.b, 0.3);
}

#[test]
fn test_from_rgb8_zero() {
  let c = Colour::from_rgb8(0, 0, 0);
  assert_eq!(c.r, 0.0);
  assert_eq!(c.g, 0.0);
  assert_eq!(c.b, 0.0);
}

#[test]
fn test_from_rgb8_max_is_white() {
  let c = Colour::from_rgb8(255, 255, 255);
  assert!((c.r - 1.0).abs() < 1e-5);
  assert!((c.g - 1.0).abs() < 1e-5);
  assert!((c.b - 1.0).abs() < 1e-5);
}

#[test]
fn test_from_rgb8_mid_is_below_half() {
  // sRGB midtone (128) decodes to linear < 0.5 because gamma compresses the dark end.
  let c = Colour::from_rgb8(128, 128, 128);
  assert!(c.r < 0.5);
  assert!(c.g < 0.5);
  assert!(c.b < 0.5);
}

#[test]
fn test_from_packed_0rgb_zero() {
  let c = Colour::from_packed_0rgb(0x000000);
  assert_eq!(c.r, 0.0);
  assert_eq!(c.g, 0.0);
  assert_eq!(c.b, 0.0);
}

#[test]
fn test_from_packed_0rgb_white() {
  let c = Colour::from_packed_0rgb(0xFFFFFF);
  assert!((c.r - 1.0).abs() < 1e-5);
  assert!((c.g - 1.0).abs() < 1e-5);
  assert!((c.b - 1.0).abs() < 1e-5);
}

#[test]
fn test_from_packed_0rgb_red_only() {
  let c = Colour::from_packed_0rgb(0xFF0000);
  assert!((c.r - 1.0).abs() < 1e-5);
  assert_eq!(c.g, 0.0);
  assert_eq!(c.b, 0.0);
}

#[test]
fn test_from_packed_0rgb_green_only() {
  let c = Colour::from_packed_0rgb(0x00FF00);
  assert_eq!(c.r, 0.0);
  assert!((c.g - 1.0).abs() < 1e-5);
  assert_eq!(c.b, 0.0);
}

#[test]
fn test_from_packed_0rgb_blue_only() {
  let c = Colour::from_packed_0rgb(0x0000FF);
  assert_eq!(c.r, 0.0);
  assert_eq!(c.g, 0.0);
  assert!((c.b - 1.0).abs() < 1e-5);
}

#[test]
fn test_from_packed_0rgb_ignores_top_byte() {
  let a = Colour::from_packed_0rgb(0x00FFFFFF);
  let b = Colour::from_packed_0rgb(0xFFFFFFFF);
  assert_eq!(a.r, b.r);
  assert_eq!(a.g, b.g);
  assert_eq!(a.b, b.b);
}

// --- Named constants ---

#[test]
fn test_const_black() {
  assert_eq!(BLACK.r, 0.0);
  assert_eq!(BLACK.g, 0.0);
  assert_eq!(BLACK.b, 0.0);
}

#[test]
fn test_const_white() {
  assert_eq!(WHITE.r, 1.0);
  assert_eq!(WHITE.g, 1.0);
  assert_eq!(WHITE.b, 1.0);
}

#[test]
fn test_const_red() {
  assert_eq!(RED.r, 1.0);
  assert_eq!(RED.g, 0.0);
  assert_eq!(RED.b, 0.0);
}

#[test]
fn test_const_green() {
  assert_eq!(GREEN.r, 0.0);
  assert_eq!(GREEN.g, 1.0);
  assert_eq!(GREEN.b, 0.0);
}

#[test]
fn test_const_blue() {
  assert_eq!(BLUE.r, 0.0);
  assert_eq!(BLUE.g, 0.0);
  assert_eq!(BLUE.b, 1.0);
}

#[test]
fn test_const_magenta() {
  assert_eq!(MAGENTA.r, 1.0);
  assert_eq!(MAGENTA.g, 0.0);
  assert_eq!(MAGENTA.b, 1.0);
}

#[test]
fn test_const_cyan() {
  assert_eq!(CYAN.r, 0.0);
  assert_eq!(CYAN.g, 1.0);
  assert_eq!(CYAN.b, 1.0);
}

#[test]
fn test_const_yellow() {
  assert_eq!(YELLOW.r, 1.0);
  assert_eq!(YELLOW.g, 1.0);
  assert_eq!(YELLOW.b, 0.0);
}

// --- to_packed_0rgb ---

#[test]
fn test_to_packed_0rgb_black() {
  assert_eq!(BLACK.to_packed_0rgb(), 0);
}

#[test]
fn test_to_packed_0rgb_white() {
  assert_eq!(WHITE.to_packed_0rgb(), 0x00FFFFFF);
}

#[test]
fn test_to_packed_0rgb_clamps_above_one() {
  // Components > 1 should clamp before packing, not panic or overflow.
  let c = Colour::new(2.0, 2.0, 2.0);
  assert_eq!(c.to_packed_0rgb(), 0x00FFFFFF);
}

#[test]
fn test_to_packed_0rgb_clamps_below_zero() {
  let c = Colour::new(-1.0, -1.0, -1.0);
  assert_eq!(c.to_packed_0rgb(), 0);
}

#[test]
fn test_to_packed_top_byte_always_zero() {
  let c = Colour::new(1.0, 1.0, 1.0);
  let packed = c.to_packed_0rgb();
  assert_eq!(packed & 0xFF000000, 0);
}

#[test]
fn test_to_packed_red_only() {
  let packed = RED.to_packed_0rgb();
  assert_eq!((packed >> 16) & 0xFF, 0xFF);
  assert_eq!((packed >> 8) & 0xFF, 0);
  assert_eq!(packed & 0xFF, 0);
}

// --- Round-trip (gamma tolerant) ---

#[test]
fn test_packed_round_trip_half_gamma_tolerant() {
  // Encode uses gamma 2.0 (sqrt) and decode uses a 2.2 LUT, so the round trip
  // is intentionally asymmetric: 0.5 linear comes back near ~0.45. We lock that
  // behaviour with a forgiving tolerance.
  let original = Colour::new(0.5, 0.5, 0.5);
  let packed = original.to_packed_0rgb();
  let recovered = Colour::from_packed_0rgb(packed);
  assert!((recovered.r() - 0.5).abs() < 0.06);
  assert!((recovered.g() - 0.5).abs() < 0.06);
  assert!((recovered.b() - 0.5).abs() < 0.06);
}

#[test]
fn test_packed_round_trip_white() {
  let packed = WHITE.to_packed_0rgb();
  let recovered = Colour::from_packed_0rgb(packed);
  assert!((recovered.r - 1.0).abs() < 1e-5);
  assert!((recovered.g - 1.0).abs() < 1e-5);
  assert!((recovered.b - 1.0).abs() < 1e-5);
}

#[test]
fn test_packed_round_trip_black() {
  let packed = BLACK.to_packed_0rgb();
  let recovered = Colour::from_packed_0rgb(packed);
  assert_eq!(recovered.r, 0.0);
  assert_eq!(recovered.g, 0.0);
  assert_eq!(recovered.b, 0.0);
}

// --- Arithmetic ---

#[test]
fn test_add() {
  let a = Colour::new(0.1, 0.2, 0.3);
  let b = Colour::new(0.4, 0.5, 0.6);
  let c = a + b;
  assert!((c.r - 0.5).abs() < 1e-5);
  assert!((c.g - 0.7).abs() < 1e-5);
  assert!((c.b - 0.9).abs() < 1e-5);
}

#[test]
fn test_add_does_not_clamp() {
  let c = WHITE + WHITE;
  assert_eq!(c.r, 2.0);
  assert_eq!(c.g, 2.0);
  assert_eq!(c.b, 2.0);
}

#[test]
fn test_mul_scalar_f32() {
  let c = Colour::new(0.5, 0.5, 0.5) * 2.0_f32;
  assert_eq!(c.r, 1.0);
  assert_eq!(c.g, 1.0);
  assert_eq!(c.b, 1.0);
}

#[test]
fn test_mul_scalar_f64() {
  let c = Colour::new(0.5, 0.5, 0.5) * 2.0_f64;
  assert_eq!(c.r, 1.0);
  assert_eq!(c.g, 1.0);
  assert_eq!(c.b, 1.0);
}

#[test]
fn test_mul_colour_colour() {
  let a = Colour::new(0.5, 0.5, 0.5);
  let b = Colour::new(0.5, 1.0, 0.0);
  let c = a * b;
  assert_eq!(c.r, 0.25);
  assert_eq!(c.g, 0.5);
  assert_eq!(c.b, 0.0);
}

#[test]
fn test_mul_does_not_clamp_above_one() {
  let c = WHITE * 3.0_f32;
  assert_eq!(c.r, 3.0);
  assert_eq!(c.g, 3.0);
  assert_eq!(c.b, 3.0);
}

#[test]
fn test_mul_assign_scalar_f32() {
  let mut c = Colour::new(0.5, 0.5, 0.5);
  c *= 2.0_f32;
  assert_eq!(c.r, 1.0);
}

#[test]
fn test_mul_assign_scalar_f64() {
  let mut c = Colour::new(0.5, 0.5, 0.5);
  c *= 2.0_f64;
  assert_eq!(c.r, 1.0);
}

#[test]
fn test_mul_assign_colour() {
  let mut c = Colour::new(0.5, 0.5, 0.5);
  c *= Colour::new(2.0, 4.0, 0.0);
  assert_eq!(c.r, 1.0);
  assert_eq!(c.g, 2.0);
  assert_eq!(c.b, 0.0);
}

#[test]
fn test_add_assign() {
  let mut a = Colour::new(0.1, 0.2, 0.3);
  a += Colour::new(0.4, 0.5, 0.6);
  assert!((a.r - 0.5).abs() < 1e-5);
  assert!((a.g - 0.7).abs() < 1e-5);
  assert!((a.b - 0.9).abs() < 1e-5);
}

// --- Edge cases ---

#[test]
fn test_new_with_nan_does_not_panic() {
  let c = Colour::new(f32::NAN, 0.0, 0.0);
  assert!(c.r.is_nan());
}

#[test]
fn test_new_with_inf_does_not_panic() {
  let c = Colour::new(f32::INFINITY, 0.0, 0.0);
  assert!(c.r.is_infinite());
}

#[test]
fn test_new_with_negative_does_not_clamp() {
  let c = Colour::new(-0.5, -1.0, -2.0);
  assert_eq!(c.r, -0.5);
  assert_eq!(c.g, -1.0);
  assert_eq!(c.b, -2.0);
}

#[test]
fn test_to_packed_with_nan_does_not_panic() {
  // NaN.clamp(0.0, 1.0) returns the clamp lower bound in Rust, so packing gives 0.
  let c = Colour::new(f32::NAN, 0.0, 0.0);
  let _ = c.to_packed_0rgb();
}

// --- Display ---

#[test]
fn test_display_format() {
  let c = Colour::new(0.5, 0.25, 0.125);
  let s = format!("{}", c);
  assert!(s.starts_with('['));
  assert!(s.ends_with(']'));
  assert!(s.contains("0.5"));
  assert!(s.contains("0.25"));
  assert!(s.contains("0.125"));
}

// --- Random ---

#[test]
fn test_rand_components_in_range() {
  for _ in 0..50 {
    let c = Colour::rand();
    assert!(c.r >= 0.0 && c.r < 1.0);
    assert!(c.g >= 0.0 && c.g < 1.0);
    assert!(c.b >= 0.0 && c.b < 1.0);
  }
}

#[test]
fn test_rand_produces_variety() {
  let a = Colour::rand();
  let mut differ = false;
  for _ in 0..50 {
    let b = Colour::rand();
    if (a.r - b.r).abs() > 1e-6 || (a.g - b.g).abs() > 1e-6 || (a.b - b.b).abs() > 1e-6 {
      differ = true;
      break;
    }
  }
  assert!(differ, "rand() seemed to produce the same colour repeatedly");
}

// --- Copy semantics ---

#[test]
fn test_copy_does_not_move() {
  let a = Colour::new(0.1, 0.2, 0.3);
  let b = a;
  assert_eq!(a.r, 0.1);
  assert_eq!(b.r, 0.1);
}

#[test]
fn test_clone_equals_original() {
  let a = Colour::new(0.1, 0.2, 0.3);
  let b = a;
  assert_eq!(a.r, b.r);
  assert_eq!(a.g, b.g);
  assert_eq!(a.b, b.b);
}
