use argh::colour::BLUE;
use argh::engine::{Engine, Scene};

// You must always implement the update method it will be called once per frame
struct MyScene {}
impl Scene for MyScene {
  fn update(&mut self, e: &mut Engine, _: f64) {
    e.clear(BLUE);
  }
}

fn main() {
  let eng = Engine::new(800, 600, String::from("Argh: Hello World"), 1);
  eng.start(MyScene {});
}
