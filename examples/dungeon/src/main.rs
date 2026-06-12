mod app;
mod fps_camera;

use argh::engine::Engine;

fn main() {
  //let mut eng = Engine::new(1024, 768);
  // let mut eng = Engine::new(640, 480);
  let mut eng = Engine::new(320, 240);
  let mut app = app::new(&mut eng);

  eng.debug = true;

  eng.start_window(&mut app, "Argh: dungeon", 4, 0);
}
