use std::time::Instant;

use argh::prelude::*;

mod app;

fn main() {
  let frames = 200;
  let fake_dt = 3.0;

  let mut eng = Engine::new(640, 480);
  let mut app = app::new(&mut eng);

  let start = Instant::now();

  for _f in 0..frames {
    let t = eng.tick(fake_dt);
    app.update(&mut eng, fake_dt, t);
  }

  let time = Instant::now().duration_since(start);

  println!("Time taken {}s", time.as_secs_f64());
  println!("Time per frame {}ms", time.as_millis() as f64 / frames as f64);
}
