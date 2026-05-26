mod scene;

use argh::engine::{Engine, Scene};
use scene::MyScene;

fn main() {
  let mut e = Engine::new(640, 480);

  e.debug = true;
  e.target_fps = 0;

  let s = MyScene::new(&mut e);
  e.start_window(s, "Argh: simple_3d", 2);
}
