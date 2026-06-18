mod app;

use argh::engine::Engine;

fn main() {
  let mut eng = Engine::new(640, 360);
  let mut app = app::new(&mut eng);

  eng.debug = true;

  eng.start_window(&mut app, "Argh: transparent_dice", 2, 0);
}
