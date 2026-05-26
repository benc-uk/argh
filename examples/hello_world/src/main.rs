use argh::colour;
use argh::engine::{Engine, Scene};

struct MyScene {}

// Minimal implementation of Scene
impl Scene for MyScene {
  // Clear the screen with blue colour then draw some text
  fn update(&mut self, e: &mut Engine, _: f64, _: f64) {
    e.clear(colour::BLUE);
    e.draw_string("Hello World!", 20, 20, colour::WHITE);
  }

  // Minimal new function as our scene has no fields
  fn new(_: &mut Engine) -> Self {
    MyScene {}
  }
}

fn main() {
  let mut e = Engine::new(320, 240);

  let s = MyScene::new(&mut e);

  e.start_window(s, "Argh: Hello World", 2);
}
