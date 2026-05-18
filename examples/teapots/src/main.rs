use argh::camera::Camera;
use argh::engine::{Engine, InstanceHandle, Scene};
use argh::light::Light;
use argh::math::Vec3;
use argh::models::{Material, SimpleColourTexture};
use argh::{colour::*, primitives};

struct MyScene {
  h1: InstanceHandle,
  h2: InstanceHandle,
  camera: Camera,
}

impl Scene for MyScene {
  // You must always implement the update method it will be called once per frame
  fn update(&mut self, engine: &mut Engine, dt: f64, _: f64) {
    engine.clear(BLACK);

    engine.instance_mut(self.h1).rot_y(dt * 1.2);
    engine.instance_mut(self.h2).rot_y(dt * -1.4);
    engine.render_all(&self.camera);

    if !engine.get_keys_pressed().is_empty() && engine.get_keys_pressed()[0].eq(&argh::engine::Key::Escape) {
      engine.stop();
    }
  }
}

fn main() {
  let mut e = Engine::new(800, 600, String::from("Argh: teapots"), 1);
  e.debug = true;
  e.target_fps = 60;

  e.add_light(Light::new(Vec3::new(15.0, 7.0, 5.0), 0.6, BLUE));
  e.add_light(Light::new(Vec3::new(-9.0, 1.0, 9.0), 0.9, RED));
  e.add_light(Light::new(Vec3::new(4.0, 9.0, 10.0), 0.8, WHITE));

  let tex = SimpleColourTexture::new(Colour::rand());
  let mat = Material::new(tex);
  e.add_mesh("teapot", primitives::new_teapot());

  let h1 = e.add_instance("teapot");
  e.instance_mut(h1).set_material(mat).set_pos_xyz(2.0, -1.0, -3.0);

  let h2 = e.add_instance("teapot");
  e.instance_mut(h2)
    .set_material(Material::new(SimpleColourTexture::new(Colour::rand())))
    .set_pos_xyz(-2.0, -1.0, -6.0);

  let camera = Camera::new_perspective(e.get_aspect(), Vec3::new(0.0, 3.0, 2.3), Vec3::new(0.0, 0.5, -2.0), 60.0, 0.01, 10.0).unwrap();
  let s = MyScene { h1, h2, camera };

  e.start(s);
}
