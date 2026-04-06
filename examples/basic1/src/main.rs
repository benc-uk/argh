use argh::colour::{BLACK, Colour};
use argh::core::{Engine, Scene};

struct MyScene {
    t: f64,
}

const W: usize = 1024;
const H: usize = 768;

impl Scene for MyScene {
    // You must always implement the update method it will be called once per frame
    fn update(&mut self, engine: &mut Engine, dt: f64) {
        self.t += dt;
        let r = (self.t * 255.0 * 2.0 % 255.0) as u8;

        engine.clear(BLACK);

        for y in 0..H {
            for x in 0..W {
                let c2 = Colour::new(r, (x * 3 % 255) as u8, (y * 3 % 255) as u8);
                engine.set_pixel(x, y, c2);
            }
        }
    }
}

fn main() {
    let e = Engine::new(W, H, String::from("Argh: modules/basic1"));
    let s = MyScene { t: 0.0 };
    e.start(s);
}
