use argh::colour::{BLACK, Colour};
use argh::core::{Engine, Scene};

struct MyScene {}

const W: usize = 1024;
const H: usize = 768;

impl Scene for MyScene {
    // You must always implement the update method it will be called once per frame
    fn update(&mut self, engine: &mut Engine, _: f64) {
        let r = (engine.t() * 255.0 * 2.0 % 255.0) as u8;

        engine.clear(BLACK);

        for y in 0..H {
            for x in 0..W {
                let mut c2 = Colour::new(r, (x * 3 % 255) as u8, (y * 3 % 255) as u8);
                c2.scale(engine.t() * 2.0 % 2.0);
                engine.set_pixel(x, y, c2);
            }
        }
    }
}

fn main() {
    let e = Engine::new(W, H, String::from("Argh: modules/basic1"));
    let s = MyScene {};
    e.start(s);
}
