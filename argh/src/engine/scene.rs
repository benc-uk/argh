use super::Engine;

/// All users of argh are expected to provide their own Scene implementation
pub trait Scene {
  /// This method will be called every frame by the main loop, use it to draw and render your scene
  fn update(&mut self, engine: &mut Engine, dt: f64, t: f64);

  /// This is mostly convention and not used by the argh engine
  fn new(e: &mut Engine) -> Self;
}
