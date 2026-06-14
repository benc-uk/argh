// ==============================================================================================
// Purpose:         2D polygons spinning about
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// ==============================================================================================

use std::f32::consts::PI;

use argh::app::App;
use argh::prelude::*;

use rand::random_range;

pub struct MyApp {
  polys: Vec<Poly>,
}

struct Poly {
  colour: Colour,
  trans: Vec2,
  scale: f32,
  speed: f32,
  points: Vec<Vec2>,
}

impl App for MyApp {
  // You must always implement the update method it will be called once per frame
  fn update(&mut self, eng: &mut Engine, _: f64, t: f64) {
    eng.clear(BLACK);

    for p in &self.polys {
      let trans = Affine2::new_scale_rot_trans(p.scale, p.scale, p.speed * t as f32, p.trans.x, p.trans.y);
      eng.draw_poly_line(&(trans * &p.points), p.colour);
    }
  }
}

pub fn new() -> MyApp {
  let mut app = MyApp { polys: vec![] };

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

    app.polys.push(p);
  }

  app
}

/// Construct a basic regular polygon, triangle, square, pentagon, hexagon etc
fn simple_poly(count: u32, size: f32) -> Vec<Vec2> {
  let mut out = vec![];
  if count < 3 {
    return out;
  }

  for i in 0..count {
    let mut p = Vec2::new(size, 0.0);
    p.rotate(((2.0 * PI) / count as f32) * i as f32);
    out.push(p);
  }
  out.push(Vec2::new(size, 0.0));

  out
}
