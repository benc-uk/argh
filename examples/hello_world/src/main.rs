use argh::colour;
use argh::engine::{App, Engine};

struct MyApp {}

// Minimal implementation of an argh App
impl App for MyApp {
  // Clear the screen with blue colour then draw some text
  fn update(&mut self, e: &mut Engine, _: f64, _: f64) {
    e.clear(colour::BLUE);
    e.draw_string("Hello World!", 20, 20, colour::WHITE);
  }
}

fn main() {
  let mut app = MyApp {};

  let mut e = Engine::new(320, 240);

  e.start_window(&mut app, "Argh: Hello World", 2);
}
