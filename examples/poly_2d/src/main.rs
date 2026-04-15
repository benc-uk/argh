use argh::colour::{BLACK, Colour};
use argh::engine::{Engine, Scene};
use argh::math::{Mat3, Vec2};

struct MyScene {
  points: Vec<Vec2>,
  colours: Vec<Colour>,
  mat: Mat3,
}

impl Scene for MyScene {
  // You must always implement the update method it will be called once per frame
  fn update(&mut self, engine: &mut Engine, _: f64) {
    engine.clear(BLACK);
    self.mat.rot(0.9 * engine.t());
    self.mat.trans(540.0 + f64::sin(engine.t()) * 250.0, 300.0 + f64::sin(engine.t() * 0.7) * 110.0);

    for (i, p) in self.points.iter().enumerate() {
      let np = self.mat * *p;
      engine.set_pixel(np.x as usize, np.y as usize, self.colours[i]);
    }
  }
}

fn main() {
  let mut e = Engine::new(1080, 600, String::from("Argh: 2dpoly"), 2);
  e.debug = true;

  let mut s = MyScene {
    points: Vec::new(),
    colours: Vec::new(),
    mat: Mat3::new(),
  };

  for _ in 1..11000 {
    let x = rand::random::<f64>() * 260 as f64 - 130.0;
    let y = rand::random::<f64>() * 260 as f64 - 130.0;
    let p = Vec2 { x, y };
    s.points.push(p);
    s.colours.push(Colour::rand());
  }

  e.start(s);
}
