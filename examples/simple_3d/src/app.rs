use argh::camera::Camera;
use argh::engine::{App, Engine, InstanceHandle, MaterialHandle, Scene};
use argh::light::Light;
use argh::math::Vec3;
use argh::models::{Material, Texture};
use argh::{colour::*, primitives};

pub struct MyApp {
  cube_hdl: InstanceHandle,
  sphere1_hdl: InstanceHandle,
  sphere2_hdl: InstanceHandle,
  teapot_hdl: InstanceHandle,
  teapot_mat_hdl: MaterialHandle,
  camera: Camera,
  scene: Scene,
}

impl App for MyApp {
  fn update(&mut self, engine: &mut Engine, dt: f64, t: f64) {
    engine.clear(BLACK);
    let scn = &mut self.scene; // Just for convenience 

    let mut axis = Vec3::new(0.6, 0.3, 0.9);
    axis.normalize();
    let py = f64::cos(t * 1.0);
    let px = f64::sin(t * 0.7);
    let pz = -0.5 - (f64::sin(t * 1.4) * 0.9);
    let pz2 = f64::sin(t * 0.9) * 0.6;
    scn.instance_mut(self.cube_hdl).rot_y(0.5 * dt).rot_x(0.8 * dt).set_pos(Vec3::new(-px, py, pz));
    scn.instance_mut(self.sphere1_hdl).rot_y(0.9 * dt).rot_x(1.2 * dt).set_pos(Vec3::new(px, -py, pz2));
    scn.instance_mut(self.sphere2_hdl).rot_y(0.1 * dt).set_pos(Vec3::new(px * 0.7, py * 1.0, 0.0));
    scn.instance_mut(self.teapot_hdl).rot_y(0.3 * dt);

    engine.render(&self.camera, &self.scene);

    if !engine.get_keys_pressed().is_empty() {
      // When space is pressed change the teapot colour, don't ask why
      if engine.get_keys_pressed()[0].eq(&argh::engine::Key::Space) {
        let mat = engine.material_mut(self.teapot_mat_hdl);
        mat.set_texture(Texture::solid(Colour::rand()));
      }

      // Quit on escape
      if engine.get_keys_pressed()[0].eq(&argh::engine::Key::Escape) {
        engine.stop();
      }
    }
  }
}

pub fn new(e: &mut Engine) -> MyApp {
  let mut scene = Scene::new();

  scene.add_light(Light::new(Vec3::new(3.0, 7.0, 5.0), 1.0, WHITE));
  scene.add_light(Light::new(Vec3::new(-6.0, 7.0, 5.0), 0.8, BLUE));
  scene.add_light(Light::new(Vec3::new(8.0, -2.0, 9.0), 0.5, RED));

  let crate_tex = Texture::image("assets/checker_256.png").unwrap();
  let earth_tex = Texture::image("assets/earth.png").unwrap();
  let col_tex1 = Texture::Solid(Colour::rand());
  let col_tex2 = Texture::Solid(Colour::rand());

  let crate_mat = e.add_material(Material::new(crate_tex));
  let earth_mat = e.add_material(Material::new(earth_tex));
  let col_mat1 = e.add_material(Material::new(col_tex1));
  let col_mat2 = e.add_material(Material::new(col_tex2));

  let cube = e.add_mesh(primitives::new_cube());
  let sphere1 = e.add_mesh(primitives::new_sphere(8, 12));
  let sphere2 = e.add_mesh(primitives::new_sphere(24, 48));
  let teapot = e.add_mesh(primitives::new_teapot());

  let cube_hdl = scene.add_instance(cube, crate_mat);
  let sphere1_hdl = scene.add_instance(sphere1, col_mat1);
  let sphere2_hdl = scene.add_instance(sphere2, earth_mat);
  let teapot_hdl = scene.add_instance(teapot, col_mat2);
  scene.instance_mut(sphere1_hdl).smooth = false;
  scene.instance_mut(teapot_hdl).scale(0.5).set_pos_xyz(0.5, -1.55, -2.0);

  let camera = Camera::new_perspective(e.get_aspect(), Vec3::new(0.0, 1.0, 2.8), Vec3::new(0.0, 0.0, 0.0), 60.0, 0.01, 10.0).unwrap();

  MyApp {
    cube_hdl,
    sphere1_hdl,
    sphere2_hdl,
    teapot_hdl,
    teapot_mat_hdl: col_mat2,
    camera,
    scene,
  }
}
