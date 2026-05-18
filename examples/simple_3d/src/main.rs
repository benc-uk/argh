use argh::camera::Camera;
use argh::engine::{Engine, InstanceHandle, Scene};
use argh::light::Light;
use argh::math::Vec3;
use argh::models::{Material, SimpleColourTexture};
use argh::{colour::*, primitives};

struct MyScene {
  h1: InstanceHandle,
  h2: InstanceHandle,
  h3: InstanceHandle,
  camera: Camera,
}

impl Scene for MyScene {
  // You must always implement the update method it will be called once per frame
  fn update(&mut self, engine: &mut Engine, _: f64, t: f64) {
    engine.clear(BLACK);

    let mut axis = Vec3::new(0.6, 0.3, 0.9);
    axis.normalize();
    let py = f64::sin(t);
    let px = f64::sin(t * 0.7);
    let pz = -0.5 - (f64::sin(t * 1.4) * 0.9);
    let pz2 = f64::sin(t * 0.9) * 0.6;
    engine.instance_mut(self.h1).rot_y(0.01).rot_x(0.03).set_pos(Vec3::new(-px, py, pz));
    engine.instance_mut(self.h2).rot_y(0.03).rot_x(0.04).set_pos(Vec3::new(px, -py, pz2));
    engine.instance_mut(self.h3).rot_y(0.02).set_pos(Vec3::new(px * 0.3, py * 0.5, 0.0));

    engine.render_all(&self.camera);

    if !engine.get_keys_pressed().is_empty() && engine.get_keys_pressed()[0].eq(&argh::engine::Key::Space) {
      let tex = SimpleColourTexture::new(Colour::rand());
      let tex2 = SimpleColourTexture::new(Colour::rand());
      let tex3 = SimpleColourTexture::new(Colour::rand());
      engine.instance_mut(self.h1).set_material(Material::new(tex));
      engine.instance_mut(self.h2).set_material(Material::new(tex2));
      engine.instance_mut(self.h3).set_material(Material::new(tex3));
    }
  }
}

fn main() {
  let mut e = Engine::new(640, 360, String::from("Argh: cube_3d"), 2);
  e.debug = true;
  e.target_fps = 60;

  e.add_light(Light::new(Vec3::new(3.0, 7.0, 5.0), 1.0, WHITE));
  e.add_light(Light::new(Vec3::new(-6.0, 7.0, 5.0), 0.8, BLUE));
  e.add_light(Light::new(Vec3::new(8.0, -2.0, 9.0), 0.5, RED));

  let tex = SimpleColourTexture::new(Colour::rand());
  let tex2 = SimpleColourTexture::new(Colour::rand());
  let tex3 = SimpleColourTexture::new(Colour::rand());
  let mat = Material::new(tex);
  let mat2 = Material::new(tex2);
  let mat3 = Material::new(tex3);
  e.add_mesh("cube", primitives::new_cube());
  e.add_mesh("sphere1", primitives::new_sphere(8, 12));
  e.add_mesh("sphere2", primitives::new_sphere(24, 48));

  let h1 = e.add_instance("cube");
  let h2 = e.add_instance("sphere1");
  let h3 = e.add_instance("sphere2");

  e.instance_mut(h1).set_material(mat);
  e.instance_mut(h2).set_material(mat2).smooth = false;
  e.instance_mut(h3).set_material(mat3);

  let camera = Camera::new_perspective(e.get_aspect(), Vec3::new(0.0, 0.0, 2.8), Vec3::new(0.0, 0.0, 0.0), 60.0, 0.01, 10.0).unwrap();

  let s = MyScene { h1, h2, h3, camera };

  e.start(s);
}
