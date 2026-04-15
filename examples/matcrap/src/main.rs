use argh::math::*;

fn main() {
  let v = Vec2::new(2.0, 3.0);
  let mut m = Mat3::new();
  m.trans(10.0, 20.0);

  let v1 = m * v;

  println!("{}", v);
  println!("{}", m);
  println!("{}", v1);
}
