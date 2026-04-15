use argh::colour::{BLACK, Colour};
use argh::engine::{Engine, Scene};

struct MyScene {}

const W: i32 = 800;
const H: i32 = 600;

impl Scene for MyScene {
  // You must always implement the update method it will be called once per frame
  fn update(&mut self, engine: &mut Engine, _: f64) {
    engine.clear(BLACK);

    for _ in 0..600 {
      // p.draw(engine, self.colors[i]);
      let x = (rand::random::<f64>() * W as f64) as usize;
      let y = (rand::random::<f64>() * H as f64) as usize;
      let w = (rand::random::<f64>() * 300.0) as usize;
      let h = (rand::random::<f64>() * 190.0) as usize;
      let colour = Colour::rand();

      engine.draw_rect(x, y, w, h, colour);
    }
  }
}

fn main() {
  let mut e = Engine::new(W, H, String::from("Argh: rects"), 2);
  e.debug = true;

  let s = MyScene {};
  e.start(s);
}
