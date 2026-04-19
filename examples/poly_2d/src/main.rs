use argh::colour::{BLACK, Colour};
use argh::engine::{Engine, Scene};
use argh::helpers;
use argh::math::{Mat3, Vec2};
use rand::random_range;

struct MyScene {
  polys: Vec<Poly>,
}

struct Poly {
  colour: Colour,
  trans: Vec2,
  scale: f64,
  speed: f64,
  points: Vec<Vec2>,
}

impl Scene for MyScene {
  // You must always implement the update method it will be called once per frame
  fn update(&mut self, engine: &mut Engine, _: f64) {
    engine.clear(BLACK);

    for p in &self.polys {
      let trans = Mat3::new_scale_rot_trans(p.scale, p.scale, p.speed * engine.t(), p.trans.x, p.trans.y);
      engine.draw_poly(trans * &p.points, p.colour);
    }
  }
}

fn main() {
  let mut e = Engine::new(800, 600, String::from("Argh: 2dpoly"), 2);
  e.debug = true;
  e.target_fps = 60;

  let mut s = MyScene { polys: vec![] };

  for _ in 0..200 {
    let p = Poly {
      points: helpers::simple_poly(random_range(3..7), 20.0),
      scale: random_range(1.0..3.0),
      speed: random_range(1.0..3.0),
      trans: Vec2 {
        x: 400.0 + random_range(-390.0..390.0),
        y: 300.0 + random_range(-290.0..290.0),
      },
      colour: Colour::rand(),
    };

    s.polys.push(p);
  }

  e.start(s);
}
