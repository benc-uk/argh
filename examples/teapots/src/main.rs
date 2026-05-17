use argh::camera::Camera;
use argh::engine::{Engine, Scene};
use argh::light::Light;
use argh::math::Vec3;
use argh::models::{Material, Mesh, SimpleColourTexture};
use argh::{colour::*, primitives};

struct MyScene {
  obj1: Mesh,
  camera: Camera,
}

impl Scene for MyScene {
  // You must always implement the update method it will be called once per frame
  fn update(&mut self, engine: &mut Engine, _: f64, _: f64) {
    engine.clear(BLACK);

    self.obj1.rot_y(0.01);
    engine.render_mesh(&self.camera, &self.obj1);

    if !engine.get_keys_pressed().is_empty() && engine.get_keys_pressed()[0].eq(&argh::engine::Key::Escape) {
      engine.stop();
    }
  }
}

fn main() {
  let mut e = Engine::new(800, 600, String::from("Argh: teapots"), 1);
  e.debug = true;
  e.target_fps = 50;

  // e.add_light(Light::new(Vec3::new(15.0, 17.0, 5.0), 0.6, BLUE));
  e.add_light(Light::new(Vec3::new(-9.0, 1.0, 9.0), 0.9, RED));
  e.add_light(Light::new(Vec3::new(4.0, 9.0, 10.0), 0.8, WHITE));

  let tex = SimpleColourTexture::new(Colour::rand());
  let mat = Material::new(tex);
  let mut obj1 = primitives::new_teapot();

  obj1.set_material(mat);
  obj1.set_pos_xyz(0.0, -1.0, -3.0);
  let camera = Camera::new_perspective(e.get_aspect(), Vec3::new(0.0, 3.0, 2.3), Vec3::new(0.0, 0.5, -2.0), 60.0, 0.01, 10.0).unwrap();

  let s = MyScene { obj1, camera };

  e.start(s);
}
