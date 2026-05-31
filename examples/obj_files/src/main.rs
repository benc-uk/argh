mod app;

use argh::engine::Engine;

fn main() {
  let mut eng = Engine::new(480, 360);
  let mut app = app::new(&mut eng);

  eng.debug = true;
  eng.target_fps = 60;

  eng.start_window(&mut app, "Argh: obj_files", 2);
}
