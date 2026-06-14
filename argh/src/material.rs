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
  texture::Texture,
};

#[cfg(test)]
#[path = "tests/material_tests.rs"]
mod material_tests;

/// Material holds parameters for rendering the surface of a mesh, can be textured or flat
#[derive(Clone)]
pub struct Material {
  /// Diffuse or base colour of the object
  pub diffuse: Colour,

  /// How this object reflects specular lighting, nearly always WHITE
  pub specular: Colour,

  /// Size of specular highlights, higher > smaller
  pub hardness: f32,

  // Internal texture, might be None
  pub(crate) texture: Option<Rc<Texture>>,
}

/// Most basic Material possible
pub const MATERIAL_PLACEHOLDER: Material = Material {
  diffuse: WHITE,
  hardness: 20.0,
  specular: WHITE,
  texture: None,
};

impl Material {
  /// Create a textured material from given texture
  pub fn new_textured(tex: Texture) -> Self {
    Self {
      diffuse: WHITE,
      specular: WHITE,
      hardness: 20.0,
      texture: Some(Rc::new(tex)),
    }
  }

  /// Create a flat or solid coloured material
  pub fn new_flat(colour: Colour) -> Self {
    Self {
      diffuse: colour,
      specular: WHITE,
      hardness: 20.0,
      texture: None,
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
}
