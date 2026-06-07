use argh::prelude::*;

pub struct MyApp {
  cube_hdl: InstanceHandle,
  sphere1_hdl: InstanceHandle,
  sphere2_hdl: InstanceHandle,
  teapot_hdl: InstanceHandle,
  camera: Camera,
  scene: Scene,
}

impl App for MyApp {
  fn update(&mut self, eng: &mut Engine, dt: f64, t: f64) {
    let dt = dt as f32;
    let t = t as f32;
    eng.clear(BLACK);
    let scn = &mut self.scene; // Just for convenience 

    let mut axis = v3(0.6, 0.3, 0.9);
    axis.normalize();
    let py = f32::cos(t * 1.0);
    let px = f32::sin(t * 0.7);
    let pz = -0.5 - (f32::sin(t * 1.4) * 0.9);
    let pz2 = f32::sin(t * 0.9) * 0.6;
    scn.instance_mut(self.cube_hdl).rot_y(0.5 * dt).rot_x(0.8 * dt).set_pos(v3(-px, py, pz));
    scn.instance_mut(self.sphere1_hdl).rot_y(0.9 * dt).rot_x(1.2 * dt).set_pos(v3(px, -py, pz2));
    scn.instance_mut(self.sphere2_hdl).rot_y(0.1 * dt).set_pos(v3(px * 0.7, py * 1.0, 0.0));
    scn.instance_mut(self.teapot_hdl).rot_y(0.3 * dt);

    eng.render(&self.camera, &self.scene);

    if !eng.get_keys_pressed().is_empty() {
      // Quit on escape
      if eng.get_keys_pressed()[0].eq(&argh::engine::Key::Escape) {
        eng.stop();
      }
    }
  }
}

pub fn new(eng: &mut Engine) -> MyApp {
  let mut scene = Scene::new();

  scene.add_light(Light::new(v3(3.0, 3.0, 4.0), 1.0, WHITE));
  scene.add_light(Light::new(v3(-5.0, 3.0, 4.0), 1.8, BLUE));
  scene.add_light(Light::new(v3(2.0, -1.0, 2.0), 1.0, RED));

  let crate_tex = Texture::new("assets/checker_256.png").unwrap();
  let earth_tex = Texture::new("assets/earth.png").unwrap();

  let cube = eng.add_model(primitives::new_cube(Material::new_textured(crate_tex)));
  let sphere1 = eng.add_model(primitives::new_sphere(Material::new_flat(Colour::rand()), 8, 12));
  let sphere2 = eng.add_model(primitives::new_sphere(Material::new_textured(earth_tex), 24, 48));
  let teapot = eng.add_model(primitives::new_teapot(Material::new_flat(Colour::rand())));

  let cube_hdl = scene.add_instance(cube);
  let sphere1_hdl = scene.add_instance(sphere1);
  let sphere2_hdl = scene.add_instance(sphere2);
  let teapot_hdl = scene.add_instance(teapot);
  scene.instance_mut(sphere1_hdl).smooth = false;
  scene.instance_mut(teapot_hdl).scale(0.5).set_pos_xyz(0.5, -1.55, -2.0);

  let camera = Camera::new_perspective(eng.get_aspect(), v3(0.0, 1.0, 2.8), v3(0.0, 0.0, 0.0), 60.0, 0.01, 10.0).unwrap();

  MyApp {
    cube_hdl,
    sphere1_hdl,
    sphere2_hdl,
    teapot_hdl,
    camera,
    scene,
  }
}
