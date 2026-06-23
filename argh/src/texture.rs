// ==============================================================================================
// Module & file:   texture.rs
// Purpose:         Texture loading, decoding and sampling
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use image::{ImageError, ImageReader};
use std::io;

use crate::colour::{Colour, INV_255};

#[cfg(test)]
#[path = "tests/texture_tests.rs"]
mod texture_tests;

/// How should texture wrapping be handled
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TextureWrap {
  /// Repeat and tile the texture in both U and V (like `GL_REPEAT`)
  Repeat,

  /// Clamp coords outside 0-1 to the nearest edge texel (like `GL_CLAMP_TO_EDGE`)
  Clamp,
}

/// Errors that can occur when loading or decoding a [Texture]
#[derive(thiserror::Error)]
pub enum TextureError {
  /// The underlying image crate failed to decode the image data
  #[error("image error: {0}")]
  ImageError(#[from] ImageError),
  /// An I/O error occurred reading the texture file from disk
  #[error("io error: {0}")]
  IoError(#[from] io::Error),
}

impl std::fmt::Debug for TextureError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self, f)
  }
}

/// Holds a pixels of an image and not much else
pub struct Texture {
  pixels: Vec<u32>, // packed 0RGB to match buffer format
  w: u32,
  h: u32,
  /// Addressing mode used when sampling outside the 0-1 UV range
  pub wrap: TextureWrap,
  /// UV multiplier applied before addressing, so >1.0 tiles (Repeat) or stretches (Clamp)
  pub scale: f32,
}

// In Rust enums can have methods and an implementation, which is kinda wild
impl Texture {
  /// Load a texture from an image file on disk. Supports any format the `image` crate can decode (PNG, JPEG, BMP, etc).
  pub fn new(path: &str) -> Result<Self, TextureError> {
    println!("Trying to load texture image file: {}", path);

    let img = ImageReader::open(path)?.decode()?.to_rgba8(); // was to_rgb8
    let (w, h) = img.dimensions();
    let pixels = img
      .pixels()
      .map(|p| {
        ((p[3] as u32) << 24) |   // alpha in top byte
        ((p[0] as u32) << 16) |
        ((p[1] as u32) << 8)  |
        (p[2] as u32)
      })
      .collect();

    Ok(Self {
      pixels,
      w,
      h,
      wrap: TextureWrap::Repeat,
      scale: 1.0,
    })
  }

  /// Load a texture from a byte buffer (e.g. an embedded asset via `include_bytes!`).
  pub fn from_bytes(bytes: &[u8]) -> Result<Self, TextureError> {
    let img = image::load_from_memory(bytes)?.to_rgba8();
    let (w, h) = img.dimensions();
    let pixels = img
      .pixels()
      .map(|p| {
        ((p[3] as u32) << 24) |   // alpha in top byte
        ((p[0] as u32) << 16) |
        ((p[1] as u32) << 8)  |
        (p[2] as u32)
      })
      .collect();

    Ok(Self {
      pixels,
      w,
      h,
      wrap: TextureWrap::Repeat,
      scale: 1.0,
    })
  }

  /// Load a texture from a raw byte RGBA8 slice
  pub fn from_raw_rgba8(bytes: &[u8], w: u32, h: u32) -> Self {
    let pixels: Vec<u32> = bytes
      .chunks_exact(4)
      .map(|p| {
        ((p[3] as u32) << 24) |   // alpha in top byte
        ((p[0] as u32) << 16) |
        ((p[1] as u32) << 8)  |
        (p[2] as u32)
      })
      .collect();

    Self {
      pixels,
      w,
      h,
      wrap: TextureWrap::Repeat,
      scale: 1.0,
    }
  }

  /// Load a texture from a raw byte RGB8 slice (no alpha channel)
  pub fn from_raw_rgb8(bytes: &[u8], w: u32, h: u32) -> Self {
    let pixels: Vec<u32> = bytes
      .chunks_exact(3)
      .map(|p| {
        (0xFF_u32 << 24) |        // opaque alpha in top byte
        ((p[0] as u32) << 16) |
        ((p[1] as u32) << 8)  |
        (p[2] as u32)
      })
      .collect();

    Self {
      pixels,
      w,
      h,
      wrap: TextureWrap::Repeat,
      scale: 1.0,
    }
  }

  /// Sample the texture at `(u, v)`, honouring the [`TextureWrap`] addressing mode.
  ///
  /// `scale` is applied first (so `scale = 2.0` tiles the image twice under
  /// [`TextureWrap::Repeat`]). The coords are then folded into `[0, 1]`:
  /// * [`TextureWrap::Repeat`] keeps the fractional part (`u - u.floor()`) so the
  ///   image tiles. This also folds negative coords, e.g. `-0.25 -> 0.75`.
  /// * [`TextureWrap::Clamp`] snaps anything outside `[0, 1]` onto the nearest edge
  ///   texel, matching `GL_CLAMP_TO_EDGE`.
  ///
  /// Works for any texture size (pow2 or not). Returns the texel Colour plus its alpha.
  #[inline(always)]
  pub(crate) fn sample(&self, u: f32, v: f32) -> (Colour, f32) {
    let us = u * self.scale;
    let vs = v * self.scale;

    let (uf, vf) = match self.wrap {
      TextureWrap::Repeat => (us - us.floor(), vs - vs.floor()),
      TextureWrap::Clamp => (us.clamp(0.0, 1.0), vs.clamp(0.0, 1.0)),
    };

    // Clamp the texel index to the last valid texel. In Clamp mode this catches
    // uf/vf == 1.0; in Repeat it guards against floating-point landing exactly on
    // the texel count. Keeps the get_unchecked below sound for every input.
    let x = ((uf * self.w as f32) as u32).min(self.w - 1);
    let y = ((vf * self.h as f32) as u32).min(self.h - 1);
    let p = unsafe { *self.pixels.get_unchecked((y * self.w + x) as usize) };

    // Alpha not part of Colour (yet)
    let a = ((p >> 24) & 0xFF) as f32 * INV_255;

    (Colour::from_packed_0rgb(p), a)
  }
}
