// ==============================================================================================
// Module & file:   colour.rs
// Purpose:         Standard RGB Colour type
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use core::fmt;
use std::{fmt::Formatter, ops::*};

#[cfg(test)]
#[path = "tests/colour_tests.rs"]
mod colour_tests;

/// A RGB colour tuple. Linear RGB colour, components in [0.0, 1.0] for normal use but range is not enforced
#[derive(Debug, Clone, Copy)]
pub struct Colour {
  r: f32,
  g: f32,
  b: f32,
}

impl Colour {
  /// Get red component
  #[inline]
  pub fn r(&self) -> f32 {
    self.r
  }

  /// Get green component
  #[inline]
  pub fn g(&self) -> f32 {
    self.g
  }

  /// Get blue component
  #[inline]
  pub fn b(&self) -> f32 {
    self.b
  }
}

pub(crate) const INV_255: f32 = 1.0 / 255.0;

/// Lookup table mapping an sRGB-encoded byte (0..=255) to its linear-light value, using a 2.2 gamma curve.
/// Used by the `from_*` byte-input constructors so per-pixel decode is a single load instead of `f32::powf`.
#[rustfmt::skip]
static SRGB_TO_LINEAR: [f32; 256] = [
  0e0, 5.0770514e-6, 2.3328002e-5, 5.6921755e-5, 1.07187356e-4, 1.7512396e-4, 2.6154373e-4, 3.671362e-4,
  4.9250375e-4, 6.381828e-4, 8.0465846e-4, 9.923743e-4, 1.2017394e-3, 1.4331344e-3, 1.686915e-3, 1.9634159e-3,
  2.2629532e-3, 2.5858255e-3, 2.9323183e-3, 3.302703e-3, 3.6972393e-3, 4.116177e-3, 4.559755e-3, 5.028203e-3,
  5.5217445e-3, 6.040593e-3, 6.584957e-3, 7.1550366e-3, 7.7510267e-3, 8.373117e-3, 9.021491e-3, 9.696328e-3,
  1.0397803e-2, 1.1126082e-2, 1.1881335e-2, 1.266372e-2, 1.3473397e-2, 1.4310519e-2, 1.5175238e-2, 1.60677e-2,
  1.6988052e-2, 1.7936433e-2, 1.8912982e-2, 1.9917838e-2, 2.0951131e-2, 2.2012994e-2, 2.3103556e-2, 2.4222942e-2,
  2.5371276e-2, 2.6548682e-2, 2.775528e-2, 2.8991185e-2, 3.0256517e-2, 3.155139e-2, 3.2875914e-2, 3.4230206e-2,
  3.5614368e-2, 3.7028514e-2, 3.8472746e-2, 3.9947167e-2, 4.145189e-2, 4.2987008e-2, 4.4552624e-2, 4.614884e-2,
  4.7775757e-2, 4.9433462e-2, 5.1122054e-2, 5.284163e-2, 5.459228e-2, 5.63741e-2, 5.818718e-2, 6.003161e-2,
  6.1907478e-2, 6.381487e-2, 6.5753885e-2, 6.772459e-2, 6.9727086e-2, 7.176145e-2, 7.3827766e-2, 7.5926125e-2,
  7.805659e-2, 8.021926e-2, 8.241421e-2, 8.464151e-2, 8.6901255e-2, 8.919351e-2, 9.151836e-2, 9.387587e-2,
  9.626612e-2, 9.86892e-2, 1.0114516e-1, 1.036341e-1, 1.06156066e-1, 1.0871115e-1, 1.1129942e-1, 1.13920934e-1,
  1.1657578e-1, 1.19264014e-1, 1.2198571e-1, 1.2474094e-1, 1.2752977e-1, 1.3035227e-1, 1.3320851e-1, 1.3609855e-1,
  1.3902245e-1, 1.4198029e-1, 1.4497213e-1, 1.4799802e-1, 1.5105805e-1, 1.5415226e-1, 1.5728073e-1, 1.6044351e-1,
  1.6364066e-1, 1.6687226e-1, 1.7013837e-1, 1.7343903e-1, 1.7677432e-1, 1.8014428e-1, 1.8354899e-1, 1.869885e-1,
  1.9046287e-1, 1.9397216e-1, 1.9751643e-1, 2.0109573e-1, 2.0471011e-1, 2.0835964e-1, 2.1204439e-1, 2.1576439e-1,
  2.1951973e-1, 2.2331043e-1, 2.2713655e-1, 2.3099814e-1, 2.3489527e-1, 2.38828e-1, 2.4279638e-1, 2.4680044e-1,
  2.5084025e-1, 2.5491586e-1, 2.5902736e-1, 2.6317474e-1, 2.673581e-1, 2.7157745e-1, 2.7583286e-1, 2.801244e-1,
  2.8445208e-1, 2.88816e-1, 2.9321617e-1, 2.9765266e-1, 3.021255e-1, 3.0663478e-1, 3.1118053e-1, 3.1576276e-1,
  3.2038158e-1, 3.2503697e-1, 3.2972905e-1, 3.344578e-1, 3.3922336e-1, 3.4402567e-1, 3.4886485e-1, 3.5374093e-1,
  3.5865393e-1, 3.6360392e-1, 3.6859095e-1, 3.7361506e-1, 3.7867627e-1, 3.8377467e-1, 3.8891026e-1, 3.9408314e-1,
  3.9929333e-1, 4.0454084e-1, 4.0982577e-1, 4.151481e-1, 4.2050794e-1, 4.2590532e-1, 4.3134022e-1, 4.3681276e-1,
  4.4232297e-1, 4.4787085e-1, 4.534565e-1, 4.5907992e-1, 4.6474114e-1, 4.7044027e-1, 4.7617728e-1, 4.8195225e-1,
  4.8776522e-1, 4.9361622e-1, 4.9950528e-1, 5.054325e-1, 5.1139784e-1, 5.174014e-1, 5.2344316e-1, 5.2952325e-1,
  5.356416e-1, 5.4179835e-1, 5.4799354e-1, 5.542271e-1, 5.604992e-1, 5.668098e-1, 5.7315886e-1, 5.7954663e-1,
  5.85973e-1, 5.9243804e-1, 5.989418e-1, 6.054843e-1, 6.120656e-1, 6.186857e-1, 6.2534475e-1, 6.3204265e-1,
  6.3877946e-1, 6.4555526e-1, 6.523701e-1, 6.5922403e-1, 6.6611695e-1, 6.730491e-1, 6.8002033e-1, 6.8703085e-1,
  6.9408053e-1, 7.011695e-1, 7.082978e-1, 7.154654e-1, 7.2267246e-1, 7.299189e-1, 7.372048e-1, 7.445302e-1,
  7.5189507e-1, 7.592996e-1, 7.667436e-1, 7.742273e-1, 7.817507e-1, 7.8931373e-1, 7.9691654e-1, 8.045591e-1,
  8.122415e-1, 8.199637e-1, 8.277258e-1, 8.355278e-1, 8.433697e-1, 8.5125166e-1, 8.591736e-1, 8.671355e-1,
  8.7513757e-1, 8.831797e-1, 8.9126205e-1, 8.993845e-1, 9.075472e-1, 9.1575015e-1, 9.2399335e-1, 9.3227684e-1,
  9.406007e-1, 9.4896495e-1, 9.5736957e-1, 9.6581465e-1, 9.743002e-1, 9.8282623e-1, 9.9139285e-1, 1e0,
];

// Helper static colours

/// Solid black (0, 0, 0)
pub const BLACK: Colour = Colour::new(0.0, 0.0, 0.0);
/// Solid black (0, 0, 0)
pub const NONE: Colour = Colour::new(0.0, 0.0, 0.0);
/// Solid white (1, 1, 1)
pub const WHITE: Colour = Colour::new(1.0, 1.0, 1.0);
/// Pure red (1, 0, 0)
pub const RED: Colour = Colour::new(1.0, 0.0, 0.0);
/// Pure green (0, 1, 0)
pub const GREEN: Colour = Colour::new(0.0, 1.0, 0.0);
/// Pure blue (0, 0, 1)
pub const BLUE: Colour = Colour::new(0.0, 0.0, 1.0);
/// Magenta (1, 0, 1)
pub const MAGENTA: Colour = Colour::new(1.0, 0.0, 1.0);
/// Cyan (0, 1, 1)
pub const CYAN: Colour = Colour::new(0.0, 1.0, 1.0);
/// Yellow (1, 1, 0)
pub const YELLOW: Colour = Colour::new(1.0, 1.0, 0.0);

impl Colour {
  /// Create a new Colour from given RGB values
  pub const fn new(r: f32, g: f32, b: f32) -> Self {
    Self { r, g, b }
  }

  /// Create a new Colour from given u8 RGB values (0 - 255). Inputs are treated as sRGB-encoded and
  /// decoded to linear via the `SRGB_TO_LINEAR` lookup table, so a colour-picker hex code lands in
  /// linear space ready for lighting maths.
  #[inline(always)]
  pub const fn from_rgb8(r: u8, g: u8, b: u8) -> Self {
    Self {
      r: SRGB_TO_LINEAR[r as usize],
      g: SRGB_TO_LINEAR[g as usize],
      b: SRGB_TO_LINEAR[b as usize],
    }
  }

  /// Create a new Colour from a three tuple
  pub const fn from_slice(v: [f32; 3]) -> Self {
    Self { r: v[0], g: v[1], b: v[2] }
  }

  /// Return as packed 0rgb u32 representation for use with the internal `Buffer`. Alpha is always 0
  /// This encodes back to sRGB with cheap sqrt() gamma correction
  #[inline(always)]
  pub fn to_packed_0rgb(self) -> u32 {
    let r = (f32::sqrt(self.r.clamp(0.0, 1.0)) * 255.0 + 0.5) as u32;
    let g = (f32::sqrt(self.g.clamp(0.0, 1.0)) * 255.0 + 0.5) as u32;
    let b = (f32::sqrt(self.b.clamp(0.0, 1.0)) * 255.0 + 0.5) as u32;
    (r << 16) | (g << 8) | b
  }

  /// Take a packed 0rgb u32 and output as a Colour. The byte channels are treated as sRGB-encoded
  /// and decoded to linear via the `SRGB_TO_LINEAR` lookup table.
  #[inline(always)]
  pub const fn from_packed_0rgb(p: u32) -> Self {
    Self {
      r: SRGB_TO_LINEAR[((p >> 16) & 0xFF) as usize],
      g: SRGB_TO_LINEAR[((p >> 8) & 0xFF) as usize],
      b: SRGB_TO_LINEAR[(p & 0xFF) as usize],
    }
  }

  /// Create a random RGB colour
  pub fn rand() -> Self {
    let r = rand::random_range(0.0..1.0);
    let g = rand::random_range(0.0..1.0);
    let b = rand::random_range(0.0..1.0);
    Self::new(r, g, b)
  }
}

impl Mul<f32> for Colour {
  type Output = Self;
  fn mul(self, s: f32) -> Self {
    Self::new(self.r * s, self.g * s, self.b * s)
  }
}

impl Mul<f64> for Colour {
  type Output = Self;
  fn mul(self, s: f64) -> Self {
    let sf32 = s as f32;
    Self::new(self.r * sf32, self.g * sf32, self.b * sf32)
  }
}

impl Mul<Self> for Colour {
  type Output = Self;
  fn mul(self, c: Self) -> Self {
    Self::new(self.r * c.r, self.g * c.g, self.b * c.b)
  }
}

impl Add for Colour {
  type Output = Self;
  fn add(self, c: Self) -> Self {
    Self::new(self.r + c.r, self.g + c.g, self.b + c.b)
  }
}

impl MulAssign<f32> for Colour {
  fn mul_assign(&mut self, s: f32) {
    *self = *self * s;
  }
}

impl MulAssign<f64> for Colour {
  fn mul_assign(&mut self, s: f64) {
    *self = *self * s;
  }
}

impl MulAssign<Self> for Colour {
  fn mul_assign(&mut self, c: Self) {
    *self = *self * c;
  }
}

impl AddAssign<Self> for Colour {
  fn add_assign(&mut self, c: Self) {
    *self = *self + c;
  }
}

impl fmt::Display for Colour {
  /// Human readable form [x, y, z]
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "[{}, {}, {}]", self.r, self.g, self.b)
  }
}
