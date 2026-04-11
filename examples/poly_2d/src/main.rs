use argh::colour::{BLACK, Colour};
use argh::engine::{Engine, Scene};
use argh::vector::Vec2;

struct MyScene {
  points: Vec<Vec2>,
  colors: Vec<Colour>,
}

impl Scene for MyScene {
  // You must always implement the update method it will be called once per frame
  fn update(&mut self, engine: &mut Engine, _: f64) {
    engine.clear(BLACK);

    for (i, p) in self.points.iter().enumerate() {
      // p.draw(engine, self.colors[i]);
      engine.set_pixel(p.x as usize, p.y as usize, self.colors[i]);
    }
  }
}

fn main() {
  let mut e = Engine::new(800, 600, String::from("Argh: modules/2dpoly"), 2);
  e.debug = true;

  let mut s = MyScene {
    points: Vec::new(),
    colors: Vec::new(),
  };

  for _ in 1..189000 {
    let x = rand::random::<f64>() * e.get_size().0 as f64;
    let y = rand::random::<f64>() * e.get_size().1 as f64;
    let p = Vec2 { x, y };
    s.points.push(p);
    s.colors.push(Colour::rand());
  }
  e.start(s);
}
