use argh::colour::{BLACK, Colour};
use argh::engine::{Engine, Scene};

struct MyScene {}

const W: i32 = 1024;
const H: i32 = 768;

impl Scene for MyScene {
  // You must always implement the update method it will be called once per frame
  fn update(&mut self, engine: &mut Engine, _: f64) {
    let r = (engine.t() * 255.0 * 2.0 % 255.0) as f32;

    engine.clear(BLACK);

    for y in 0..H {
      for x in 0..W {
        let mut c2 = Colour::new(r, x as f32 * 3.0, y as f32 * 3.0);
        c2 *= engine.t() * 2.0 % 2.0;
        engine.set_pixel(x as usize, y as usize, c2);
      }
    }
  }
}

fn main() {
  let e = Engine::new(W, H, String::from("Argh: basic1"), 1);
  let s = MyScene {};
  e.start(s);
}
