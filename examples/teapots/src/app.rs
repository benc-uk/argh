use argh::engine::Key;
use argh::math::V3_AXIS_Y;
use argh::prelude::*;

pub struct MyApp {
  camera: Camera,
  scene: Scene,
}

impl App for MyApp {
  // You must always implement the update method it will be called once per frame
  fn update(&mut self, engine: &mut Engine, dt: f64, t: f64) {
    engine.clear(BLACK);

    // This makes the animation independent of framerate
    let rot = Quat::new(V3_AXIS_Y, 0.5 * dt);
    // Rotate & move the camera
    let mut p = rot.rotate_vec3(self.camera.get_pos());
    p.y = (f64::sin(t * 0.75) * 2.6) + 6.0;
    self.camera.set_pos(p);

    // Draw the scene
    engine.render(&self.camera, &self.scene);

    // Quit on escape
    if engine.is_pressed(Key::Escape) {
      engine.stop();
    }
  }
}

pub fn new(e: &mut Engine) -> MyApp {
  let mut scene = Scene::new();

  scene.add_light(Light::new(v3(15.0, 2.0, 5.0), 0.6, BLUE));
  scene.add_light(Light::new(v3(-9.0, 1.0, 9.0), 0.7, RED));
  scene.add_light(Light::new(v3(4.0, 9.0, 10.0), 0.9, WHITE));

  let crate_tex = Texture::image("assets/crate.png").unwrap();
  let mut crate_mat = Material::new(crate_tex);
  crate_mat.specular = 0.0;

  let teapot_mat1 = e.add_material(Material::new(Texture::solid(Colour::new(0.7, 0.7, 0.8))));
  let teapot_mat2 = e.add_material(Material::new(Texture::solid(Colour::new(0.6, 0.2, 0.7))));
  let crate_mat_hdl = e.add_material(crate_mat);

  let teapot = e.add_mesh(primitives::new_teapot());
  let cube = e.add_mesh(primitives::new_cube());

  scene.add_instance_trans(teapot, teapot_mat1, v3(2.0, 0.0, 2.3), v3(0.0, 3.0, 0.0), v3(1.2, 1.5, 1.2));
  scene.add_instance_trans(teapot, teapot_mat2, v3(-2.0, 0.0, -2.9), v3(0.0, 2.0, 0.0), v3(1.2, 1.2, 1.2));
  scene.add_instance_trans(cube, crate_mat_hdl, v3(0.0, -6.0, 0.0), v3(0.0, 0.0, 0.0), v3(12.0, 12.0, 12.0));
  let camera = Camera::new_perspective(e.get_aspect(), v3(0.0, 5.0, 14.0), v3(0.0, 0.5, 0.0), 50.0, 0.01, 100.0).unwrap();

  MyApp { camera, scene }
}
