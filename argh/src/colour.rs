// ==============================================================================================
// Module & file:   colour.rs
// Purpose:         Standard RGB Colour type
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use core::fmt;
use std::{fmt::Formatter, ops::*};

/// A RGB colour tuple. Linear RGB colour, components in [0.0, 1.0] for normal use but range is not enforced
#[derive(Debug, Clone, Copy)]
pub struct Colour {
  r: f32,
  g: f32,
  b: f32,
}

pub(crate) const INV_255: f32 = 1.0 / 255.0;

const ENCODE_GAMMA: f32 = 1.0 / 2.2;

/// Lookup table mapping an sRGB-encoded byte (0..=255) to its linear-light value, using a 2.2 gamma curve.
/// Used by the `from_*` byte-input constructors so per-pixel decode is a single load instead of `f32::powf`.
#[rustfmt::skip]
static SRGB_TO_LINEAR: [f32; 256] = [
  0.0000000000e+00, 5.0770519007e-06, 2.3328004666e-05, 5.6921765712e-05, 1.0718736234e-04, 1.7512397750e-04, 2.6154375455e-04, 3.6713626982e-04,
  4.9250378719e-04, 6.3818284217e-04, 8.0465849951e-04, 9.9237430407e-04, 1.2017395224e-03, 1.4331345897e-03, 1.6869153168e-03, 1.9634162134e-03,
  2.2629531607e-03, 2.5858255962e-03, 2.9323183239e-03, 3.3027030320e-03, 3.6972395789e-03, 4.1161770933e-03, 4.5597549225e-03, 5.0282034569e-03,
  5.5217448502e-03, 6.0405936548e-03, 6.5849573826e-03, 7.1550370046e-03, 7.7510273977e-03, 8.3731177451e-03, 9.0214918980e-03, 9.6963287017e-03,
  1.0397802293e-02, 1.1126082368e-02, 1.1881334435e-02, 1.2663720032e-02, 1.3473396940e-02, 1.4310519375e-02, 1.5175238160e-02, 1.6067700891e-02,
  1.6988052089e-02, 1.7936433340e-02, 1.8912983424e-02, 1.9917838439e-02, 2.0951131915e-02, 2.2012994919e-02, 2.3103556158e-02, 2.4222942068e-02,
  2.5371276905e-02, 2.6548682828e-02, 2.7755279978e-02, 2.8991186547e-02, 3.0256518852e-02, 3.1551391400e-02, 3.2875916948e-02, 3.4230206565e-02,
  3.5614369685e-02, 3.7028514162e-02, 3.8472746320e-02, 3.9947171002e-02, 4.1451891611e-02, 4.2987010163e-02, 4.4552627316e-02, 4.6148842422e-02,
  4.7775753556e-02, 4.9433457556e-02, 5.1122050056e-02, 5.2841625523e-02, 5.4592277282e-02, 5.6374097552e-02, 5.8187177474e-02, 6.0031607136e-02,
  6.1907475605e-02, 6.3814870949e-02, 6.5753880260e-02, 6.7724589685e-02, 6.9727084443e-02, 7.1761448846e-02, 7.3827766328e-02, 7.5926119456e-02,
  7.8056589958e-02, 8.0219258736e-02, 8.2414205888e-02, 8.4641510725e-02, 8.6901251788e-02, 8.9193506862e-02, 9.1518352999e-02, 9.3875866526e-02,
  9.6266123063e-02, 9.8689197541e-02, 1.0114516421e-01, 1.0363409666e-01, 1.0615606781e-01, 1.0871114998e-01, 1.1129941482e-01, 1.1392093341e-01,
  1.1657577618e-01, 1.1926401301e-01, 1.2198571317e-01, 1.2474094539e-01, 1.2752977781e-01, 1.3035227806e-01, 1.3320851318e-01, 1.3609854974e-01,
  1.3902245373e-01, 1.4198029069e-01, 1.4497212560e-01, 1.4799802298e-01, 1.5105804687e-01, 1.5415226081e-01, 1.5728072789e-01, 1.6044351073e-01,
  1.6364067149e-01, 1.6687227189e-01, 1.7013837322e-01, 1.7343903633e-01, 1.7677432164e-01, 1.8014428915e-01, 1.8354899846e-01, 1.8698850876e-01,
  1.9046287882e-01, 1.9397216705e-01, 1.9751643144e-01, 2.0109572962e-01, 2.0471011884e-01, 2.0835965596e-01, 2.1204439750e-01, 2.1576439961e-01,
  2.1951971807e-01, 2.2331040834e-01, 2.2713652551e-01, 2.3099812432e-01, 2.3489525922e-01, 2.3882798427e-01, 2.4279635325e-01, 2.4680041960e-01,
  2.5084023644e-01, 2.5491585657e-01, 2.5902733249e-01, 2.6317471640e-01, 2.6735806018e-01, 2.7157741544e-01, 2.7583283346e-01, 2.8012436526e-01,
  2.8445206156e-01, 2.8881597280e-01, 2.9321614913e-01, 2.9765264045e-01, 3.0212549636e-01, 3.0663476620e-01, 3.1118049906e-01, 3.1576274374e-01,
  3.2038154879e-01, 3.2503696252e-01, 3.2972903297e-01, 3.3445780792e-01, 3.3922333494e-01, 3.4402566130e-01, 3.4886483408e-01, 3.5374090010e-01,
  3.5865390593e-01, 3.6360389792e-01, 3.6859092220e-01, 3.7361502465e-01, 3.7867625093e-01, 3.8377464649e-01, 3.8891025654e-01, 3.9408312608e-01,
  3.9929329990e-01, 4.0454082257e-01, 4.0982573844e-01, 4.1514809166e-01, 4.2050792617e-01, 4.2590528571e-01, 4.3134021381e-01, 4.3681275380e-01,
  4.4232294882e-01, 4.4787084180e-01, 4.5345647549e-01, 4.5907989242e-01, 4.6474113497e-01, 4.7044024530e-01, 4.7617726540e-01, 4.8195223705e-01,
  4.8776520188e-01, 4.9361620131e-01, 4.9950527660e-01, 5.0543246883e-01, 5.1139781888e-01, 5.1740136750e-01, 5.2344315521e-01, 5.2952322242e-01,
  5.3564160932e-01, 5.4179835595e-01, 5.4799350220e-01, 5.5422708777e-01, 5.6049915220e-01, 5.6680973490e-01, 5.7315887507e-01, 5.7954661178e-01,
  5.8597298395e-01, 5.9243803032e-01, 5.9894178949e-01, 6.0548429991e-01, 6.1206559987e-01, 6.1868572750e-01, 6.2534472080e-01, 6.3204261762e-01,
  6.3877945565e-01, 6.4555527244e-01, 6.5237010541e-01, 6.5922399181e-01, 6.6611696878e-01, 6.7304907328e-01, 6.8002034217e-01, 6.8703081215e-01,
  6.9408051980e-01, 7.0116950153e-01, 7.0829779366e-01, 7.1546543234e-01, 7.2267245360e-01, 7.2991889335e-01, 7.3720478736e-01, 7.4453017127e-01,
  7.5189508058e-01, 7.5929955070e-01, 7.6674361686e-01, 7.7422731422e-01, 7.8175067777e-01, 7.8931374242e-01, 7.9691654291e-01, 8.0455911389e-01,
  8.1224148990e-01, 8.1996370532e-01, 8.2772579446e-01, 8.3552779146e-01, 8.4336973039e-01, 8.5125164518e-01, 8.5917356966e-01, 8.6713553752e-01,
  8.7513758237e-01, 8.8317973767e-01, 8.9126203681e-01, 8.9938451305e-01, 9.0754719952e-01, 9.1575012928e-01, 9.2399333525e-01, 9.3227685026e-01,
  9.4060070704e-01, 9.4896493818e-01, 9.5736957620e-01, 9.6581465350e-01, 9.7430020239e-01, 9.8282625505e-01, 9.9139284359e-01, 1.0000000000e+00,
];

// Helper static colours

pub const BLACK: Colour = Colour::new(0.0, 0.0, 0.0);
pub const WHITE: Colour = Colour::new(1.0, 1.0, 1.0);
pub const RED: Colour = Colour::new(1.0, 0.0, 0.0);
pub const GREEN: Colour = Colour::new(0.0, 1.0, 0.0);
pub const BLUE: Colour = Colour::new(0.0, 0.0, 1.0);
pub const MAGENTA: Colour = Colour::new(1.0, 0.0, 1.0);
pub const CYAN: Colour = Colour::new(0.0, 1.0, 1.0);
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

  /// Return as packed 0rgb u32 representation of the colour for use with the internal `Buffer`. Alpha is always 0 as minifb ignores it
  /// This encodes back to sRGB with gamma correction
  #[inline(always)]
  pub fn to_packed_0rgb(self) -> u32 {
    let r = (f32::powf(self.r.clamp(0.0, 1.0), ENCODE_GAMMA) * 255.0 + 0.5) as u32;
    let g = (f32::powf(self.g.clamp(0.0, 1.0), ENCODE_GAMMA) * 255.0 + 0.5) as u32;
    let b = (f32::powf(self.b.clamp(0.0, 1.0), ENCODE_GAMMA) * 255.0 + 0.5) as u32;
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
