// ==============================================================================================
// Purpose:         Minimal working argh app
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// ==============================================================================================

use argh::colour;
use argh::prelude::*;

struct MyApp {}

// Minimal implementation of an argh App
impl App for MyApp {
  // Clear the screen with blue colour then draw some text
  fn update(&mut self, eng: &mut Engine, _: f64, _: f64) {
    eng.clear(colour::BLUE);
    eng.draw_string("Hello World!", 20, 20, colour::WHITE);
  }
}

fn main() {
  let mut app = MyApp {};

  let mut e = Engine::new(320, 240);

  e.start_window(&mut app, "Argh: Hello World", 2, 0);
}
