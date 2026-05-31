use argh::{engine::Key, prelude::*};

pub struct MyApp {
  camera: Camera,
  scn: Scene,
}

impl App for MyApp {
  fn update(&mut self, engine: &mut Engine, dt: f64, _t: f64) {
    engine.clear(BLACK);

    let cube = self.scn.instances_mut().next().expect("exists");
    // cube.rot_x(-1.5);
    cube.rot_y(dt * 0.8);
    engine.render(&self.camera, &self.scn);

    if !engine.get_keys_pressed().is_empty() {
      // Quit on escape
      if engine.is_pressed(Key::Escape) {
        engine.stop();
      }
    }
  }
}

pub fn new(e: &mut Engine) -> MyApp {
  let mut scn = Scene::new();

  scn.add_light(Light::new(v3(8.0, 92.0, 35.0), 1.0, WHITE));

  let (cube, mat) = e.load_obj("assets/models/chest/chest.obj").expect("obj loading failed");
  scn.add_instance_trans(cube, mat, V3_ZERO, v3(0.0, 0.0, 0.0), v3(1.0, 1.0, 1.0));

  let camera = Camera::new_perspective(e.get_aspect(), v3(5.0, 4.0, 6.0), v3(0.0, 1.0, 0.0), 60.0, 0.01, 110.0).unwrap();

  MyApp { camera, scn }
}
