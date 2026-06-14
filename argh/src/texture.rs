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

  /// Treat alpha transparent pixels as invisible (cut them out). Defaults to true
  pub(crate) alpha_cutout: bool,

  /// What alpha value will apply alpha_cutout
  pub(crate) cutoff: f32,
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
      alpha_cutout: true,
      cutoff: 0.5,
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
      alpha_cutout: true,
      cutoff: 0.5,
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
      alpha_cutout: true,
      cutoff: 0.5,
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
      alpha_cutout: false,
      cutoff: 0.5, // ignored
    }
  }

  /// Sample the texture with wrap-around addressing.
  /// Uses floor() to fold any UV into [0, 1] before scaling to texel space.
  /// Works for any texture size (pow2 or not)
  #[inline(always)]
  pub(crate) fn sample(&self, u: f32, v: f32) -> (Colour, f32) {
    let uf = u - u.floor();
    let vf = v - v.floor();
    let x = (uf * self.w as f32) as u32;
    let y = (vf * self.h as f32) as u32;
    let p = unsafe { *self.pixels.get_unchecked((y * self.w + x) as usize) };
    let a = ((p >> 24) & 0xFF) as f32 * INV_255;

    (Colour::from_packed_0rgb(p), a)
  }

  /// Enable or disable alpha cutout
  pub fn enable_cutout(&mut self, cutout: bool) {
    self.alpha_cutout = cutout
  }
}
