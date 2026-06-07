mod app;

use argh::engine::Engine;

fn main() {
  let mut eng = Engine::new(640, 360);
  let mut app = app::new();

  eng.debug = true;

  eng.start_window(&mut app, "Argh: simple_3d", 2);
}
