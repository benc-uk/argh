// ==============================================================================================
// Module & file:   material.rs
// Purpose:         Material parameters used to shade the surface of a mesh
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:
// ==============================================================================================

use std::rc::Rc;

use crate::{
  colour::{Colour, WHITE},
  material::BlendMode::Opaque,
  texture::Texture,
};

#[cfg(test)]
#[path = "tests/material_tests.rs"]
mod material_tests;

/// Blend mode used for transparency
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BlendMode {
  /// Opaque surface, no alpha, the default
  Opaque,

  /// Alpha blending used for transparent and semi-opaque surfaces
  AlphaBlend,

  /// Transparent but additive, to be used for fire and fx etc. Not implemented
  Additive,

  /// Mask mode implements an alpha cutout (discard) on texels based on alpha values
  Mask,
}

/// Material holds parameters for rendering the surface of a mesh, can be textured or flat
#[derive(Clone)]
pub struct Material {
  /// Diffuse or base colour of the object, and combined with the texture colour
  pub diffuse: Colour,

  /// How this object reflects specular lighting, nearly always WHITE
  pub specular: Colour,

  /// Size of specular highlights, higher > smaller
  /// A good range of values is between 10~100
  pub hardness: f32,

  /// For semi-transparent objects only and blend_mode != Opaque
  /// 1.0 = fully opaque, 0.0 = invisible
  pub opacity: f32,

  /// Threshold of alpha for masking/discarding to happen, default 0.5
  /// Only used when blend_mode == Mask
  pub mask_cutoff: f32,

  // Internal texture, might be None
  pub(crate) texture: Option<Rc<Texture>>,

  // Defaults to Opaque
  pub(crate) blend_mode: BlendMode,
}

/// Most basic Material possible
pub const MATERIAL_PLACEHOLDER: Material = Material {
  diffuse: WHITE,
  hardness: 20.0,
  specular: WHITE,
  texture: None,
  blend_mode: Opaque,
  mask_cutoff: 0.5,
  opacity: 1.0,
};

impl Material {
  /// Create a textured material from given texture
  pub fn new_textured(tex: Texture) -> Self {
    Self {
      diffuse: WHITE,
      specular: WHITE,
      hardness: 20.0,
      texture: Some(Rc::new(tex)),
      blend_mode: Opaque,
      mask_cutoff: 0.5,
      opacity: 1.0,
    }
  }

  /// Create a flat or solid coloured material
  pub fn new_flat(colour: Colour) -> Self {
    Self {
      diffuse: colour,
      specular: WHITE,
      hardness: 20.0,
      texture: None,
      blend_mode: Opaque,
      mask_cutoff: 0.5,
      opacity: 1.0,
    }
  }

  /// Simple setter for the texture
  pub fn set_texture(&mut self, tex: Texture) {
    self.texture = Some(Rc::new(tex))
  }

  /// Get the internal texture
  pub fn texture(&self) -> Option<&Rc<Texture>> {
    self.texture.as_ref()
  }

  /// Is this material opaque?
  #[inline]
  pub fn is_opaque(&self) -> bool {
    self.blend_mode == BlendMode::Opaque
  }
}
