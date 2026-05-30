mod app;

use argh::engine::Engine;

fn main() {
  let mut eng = Engine::new(640, 480);
  let mut app = app::new(&mut eng);

  eng.debug = true;
  eng.target_fps = 0;

  eng.start_window(&mut app, "Argh: scenes", 2);
}
