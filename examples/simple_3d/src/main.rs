use argh::camera::Camera;
use argh::engine::{Engine, InstanceHandle, MaterialHandle, Scene};
use argh::light::Light;
use argh::math::Vec3;
use argh::models::{ImageTexture, Material, SimpleColourTexture};
use argh::{colour::*, primitives};

struct MyScene {
  instances: Vec<InstanceHandle>,
  materials: Vec<MaterialHandle>,
  camera: Camera,
}

impl Scene for MyScene {
  fn update(&mut self, engine: &mut Engine, dt: f64, t: f64) {
    engine.clear(BLACK);

    let mut axis = Vec3::new(0.6, 0.3, 0.9);
    axis.normalize();
    let py = f64::cos(t * 1.0);
    let px = f64::sin(t * 0.7);
    let pz = -0.5 - (f64::sin(t * 1.4) * 0.9);
    let pz2 = f64::sin(t * 0.9) * 0.6;
    engine.instance_mut(self.instances[0]).rot_y(0.5 * dt).rot_x(0.8 * dt).set_pos(Vec3::new(-px, py, pz));
    engine.instance_mut(self.instances[1]).rot_y(0.9 * dt).rot_x(1.2 * dt).set_pos(Vec3::new(px, -py, pz2));
    engine.instance_mut(self.instances[2]).rot_y(0.1 * dt).set_pos(Vec3::new(px * 0.7, py * 1.0, 0.0));
    engine.instance_mut(self.instances[3]).rot_y(0.3 * dt);

    engine.render_all(&self.camera);

    if !engine.get_keys_pressed().is_empty() {
      if engine.get_keys_pressed()[0].eq(&argh::engine::Key::Space) {
        let mat = engine.material_mut(self.materials[2]);
        mat.set_texture(SimpleColourTexture::new(Colour::rand()));
      }

      if engine.get_keys_pressed()[0].eq(&argh::engine::Key::Escape) {
        engine.stop();
      }
    }
  }
}

fn main() {
  let mut e = Engine::new(800, 600, String::from("Argh: simple_3d"), 2);
  e.debug = true;
  e.target_fps = 60;

  e.add_light(Light::new(Vec3::new(3.0, 7.0, 5.0), 1.0, WHITE));
  e.add_light(Light::new(Vec3::new(-6.0, 7.0, 5.0), 0.8, BLUE));
  e.add_light(Light::new(Vec3::new(8.0, -2.0, 9.0), 0.5, RED));

  let crate_tex = ImageTexture::new("assets/checker_256.png").unwrap();
  let earth_tex = ImageTexture::new("assets/earth.png").unwrap();
  let col_tex1 = SimpleColourTexture::new(Colour::rand());
  let col_tex2 = SimpleColourTexture::new(Colour::rand());

  let crate_mat = e.add_material(Material::new(crate_tex));
  let earth_mat = e.add_material(Material::new(earth_tex));
  let col_mat1 = e.add_material(Material::new(col_tex1));
  let col_mat2 = e.add_material(Material::new(col_tex2));

  let cube = e.add_mesh(primitives::new_cube());
  let sphere1 = e.add_mesh(primitives::new_sphere(8, 12));
  let sphere2 = e.add_mesh(primitives::new_sphere(24, 48));
  let teapot = e.add_mesh(primitives::new_teapot());

  let inst1 = e.add_instance(cube, crate_mat);
  let inst2 = e.add_instance(sphere1, col_mat1);
  let inst3 = e.add_instance(sphere2, earth_mat);
  let inst4 = e.add_instance(teapot, col_mat2);
  e.instance_mut(inst2).smooth = false;
  e.instance_mut(inst4).scale(0.5).set_pos_xyz(0.5, -1.55, -2.0);

  let camera = Camera::new_perspective(e.get_aspect(), Vec3::new(0.0, 1.0, 2.8), Vec3::new(0.0, 0.0, 0.0), 60.0, 0.01, 10.0).unwrap();

  let s = MyScene {
    instances: vec![inst1, inst2, inst3, inst4],
    materials: vec![crate_mat, col_mat1, col_mat2],
    camera,
  };

  e.start(s);
}
