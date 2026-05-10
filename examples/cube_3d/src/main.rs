use argh::camera::Camera;
use argh::colour::*;
use argh::engine::{Engine, Scene};
use argh::math::{VEC3_ZERO, Vec3};
use argh::models::{Material, Mesh, SimpleColourTexture};

struct MyScene {
  cube: Mesh,
  cube2: Mesh,
  cube3: Mesh,
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
    self.cube.rot_y(0.01);
    self.cube.rot_x(0.03);
    self.cube2.rot_y(0.03);
    self.cube3.rot_y(0.01);
    self.cube.set_pos(Vec3::new(-px, py, pz));
    self.cube2.set_pos(Vec3::new(px, -py, pz2));
    self.cube3.set_pos(Vec3::new(0., 0., -0.5));

    engine.render_mesh(&self.camera, &self.cube3);
    engine.render_mesh(&self.camera, &self.cube);
    engine.render_mesh(&self.camera, &self.cube2);
  }
}

fn main() {
  let mut e = Engine::new(640, 360, String::from("Argh: cube_3d"), 2);
  e.debug = true;
  e.target_fps = 60;

  let tex = SimpleColourTexture::new(Colour::rand());
  let tex2 = SimpleColourTexture::new(Colour::rand());
  let tex3 = SimpleColourTexture::new(Colour::rand());
  let mat = Material::new(Box::new(tex));
  let mat2 = Material::new(Box::new(tex2));
  let mat3 = Material::new(Box::new(tex3));
  let mut cube = Mesh::new_cube();
  let mut cube2 = Mesh::new_cube();
  let mut cube3 = Mesh::new_cube();
  cube.set_material(mat);
  cube2.set_material(mat2);
  cube3.set_material(mat3);

  let camera = Camera::new_perspective(e.get_aspect(), Vec3::new(0.0, 0.0, 2.8), VEC3_ZERO, 60.0, 0.01, 100.0);
  let s = MyScene { cube, cube2, cube3, camera };

  e.start(s);
}
