use std::time::Instant;

use argh::prelude::*;

mod app;

fn main() {
  let frames = 1000;

  let mut eng = Engine::new(640, 480);
  let mut app = app::new(&mut eng);
  let mut total_tris = 0;

  let mut last = Instant::now();

  // println!("Triangles: {}", eng.stat_tri_total);
  eprint!("Benchmark running");
  for _f in 0..frames {
    if _f % 20 == 0 {
      eprint!(".")
    }

    let now = Instant::now();
    let dt = now.duration_since(last).as_secs_f64();
    last = now;
    let t = eng.tick(dt);

    app.update(&mut eng, dt, t);
    total_tris += eng.stats();
  }

  println!();
  println!("Elapsed:        {}s", eng.time());
  println!("Time per frame: {}ms", eng.time() * 1000.0 / frames as f64);
  println!("FPS:            {}", frames as f64 / eng.time());
  println!("Tris rendered:  {}", total_tris);
  println!("Tris/sec (1k):  {}", (total_tris as f64 / eng.time()) / 1000.0);
}
