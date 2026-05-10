use std::f64::consts::PI;

use argh::colour::{BLACK, Colour};
use argh::engine::{Engine, Scene};
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
      engine.draw_poly_line(trans * &p.points, p.colour);
    }
  }
}

fn main() {
  let mut e = Engine::new(640, 360, String::from("Argh: poly_2d"), 2);
  e.debug = true;
  e.target_fps = 60;

  let mut s = MyScene { polys: vec![] };

  for _ in 0..200 {
    let p = Poly {
      points: simple_poly(random_range(3..7), 20.0),
      scale: random_range(1.0..3.0),
      speed: random_range(1.0..3.0),
      trans: Vec2 {
        x: 320.0 + random_range(-310.0..310.0),
        y: 180.0 + random_range(-170.0..170.0),
      },
      colour: Colour::rand(),
    };

    s.polys.push(p);
  }

  e.start(s);
}

/// Construct a basic regular polygon, triangle, square, pentagon, hexagon etc
fn simple_poly(count: u32, size: f64) -> Vec<Vec2> {
  let mut out = vec![];
  if count < 3 {
    return out;
  }

  for i in 0..count {
    let mut p = Vec2::new(size, 0.0);
    p.rotate(((2.0 * PI) / count as f64) * i as f64);
    out.push(p);
  }
  out.push(Vec2::new(size, 0.0));

  out
}
