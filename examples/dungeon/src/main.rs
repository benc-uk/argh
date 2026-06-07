mod app;

use argh::engine::Engine;

fn main() {
  let mut eng = Engine::new(854, 480);
  let mut app = app::new(&mut eng);

  eng.debug = true;

  eng.start_window(&mut app, "Argh: dungeon", 2);
}
