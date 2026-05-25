use argh::colour;
use argh::engine::{Engine, Scene};

// You must always implement the update method it will be called once per frame
struct MyScene {}
impl Scene for MyScene {
  fn update(&mut self, e: &mut Engine, _: f64, _: f64) {
    e.clear(colour::BLUE);
    e.draw_string("Hello World!", 20, 20, colour::WHITE);
  }
}

fn main() {
  let eng = Engine::new(320, 240, "Argh: Hello World", 2);
  eng.start(MyScene {});
}
