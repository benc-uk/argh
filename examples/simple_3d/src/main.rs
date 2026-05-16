use argh::camera::Camera;
use argh::engine::{Engine, Scene};
use argh::light::Light;
use argh::math::Vec3;
use argh::models::{Material, Mesh, SimpleColourTexture};
use argh::{colour::*, primitives};

struct MyScene {
  obj1: Mesh,
  obj2: Mesh,
  obj3: Mesh,
  camera: Camera,
}

impl Scene for MyScene {
  // You must always implement the update method it will be called once per frame
  fn update(&mut self, engine: &mut Engine, _: f64) {
    engine.clear(BLACK);

    let mut axis = Vec3::new(0.6, 0.3, 0.9);
    axis.normalize();
    let py = f64::sin(engine.t());
    let px = f64::sin(engine.t() * 0.7);
    let pz = -0.5 - (f64::sin(engine.t() * 1.4) * 0.9);
    let pz2 = f64::sin(engine.t() * 0.9) * 0.6;
    self.obj1.rot_y(0.01);
    self.obj1.rot_x(0.03);
    self.obj2.rot_y(0.03);
    self.obj2.rot_z(0.04);
    self.obj3.rot_y(0.01);
    self.obj1.set_pos(Vec3::new(-px, py, pz));
    self.obj2.set_pos(Vec3::new(px, -py, pz2));
    self.obj3.set_pos(Vec3::new(px * 0.3, py * 0.5, 0.0));

    engine.render_mesh(&self.camera, &self.obj1);
    engine.render_mesh(&self.camera, &self.obj2);
    engine.render_mesh(&self.camera, &self.obj3);

    if !engine.get_keys_pressed().is_empty() {
      if engine.get_keys_pressed()[0].eq(&argh::engine::Key::Space) {
        let tex = SimpleColourTexture::new(Colour::rand());
        let tex2 = SimpleColourTexture::new(Colour::rand());
        let tex3 = SimpleColourTexture::new(Colour::rand());
        let mat = Material::new(tex);
        let mat2 = Material::new(tex2);
        let mat3 = Material::new(tex3);
        self.obj1.set_material(mat);
        self.obj2.set_material(mat2);
        self.obj3.set_material(mat3);
      }
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
  let mut obj1 = primitives::new_cube();
  let mut obj2 = primitives::new_sphere(8, 12);
  let mut obj3 = primitives::new_sphere(12, 24);
  obj1.set_material(mat);
  obj2.set_material(mat2);
  obj3.set_material(mat3);
  obj2.smooth = false;

  let camera = Camera::new_perspective(e.get_aspect(), Vec3::new(0.0, 0.0, 2.8), Vec3::new(0.0, 0.0, 0.0), 60.0, 0.01, 10.0).unwrap();

  let s = MyScene { obj1, obj2, obj3, camera };

  e.start(s);
}
