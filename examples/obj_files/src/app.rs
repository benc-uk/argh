use argh::{engine::Key, math::V3_AXIS_Y, prelude::*};

pub struct MyApp {
  camera: Camera,
  scn: Scene,
}

use std::f64::consts::PI;

impl App for MyApp {
  fn update(&mut self, engine: &mut Engine, dt: f64, t: f64) {
    engine.clear(BLACK);

    let rot = Quat::new(V3_AXIS_Y, 0.5 * dt);
    let mut p = rot.rotate_vec3(self.camera.get_pos());
    p.y = (f64::sin(t * 0.75) * 10.0) + 40.0;
    self.camera.set_pos(p);

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

  scn.add_light(Light::new(v3(89.0, 28.0, 89.0), 1.0, WHITE));
  scn.add_light(Light::new(v3(-88.0, 42.0, -85.0), 1.0, col(0.3, 0.8, 0.3)));

  let table = e.load_obj("assets/models/table/table.obj").expect("obj loading failed");
  scn.add_instance_trans(table, V3_ZERO, v3(-PI / 2.0, 0.0, 0.0), v3(0.3, 0.3, 0.6));

  let plant = e.load_obj("assets/models/house_plant/house_plant.obj").expect("obj loading failed");
  scn.add_instance_trans(plant, v3(12.0, 21.5, -3.0), v3(0.0, 0.9, 0.0), v3(0.6, 0.6, 0.6));

  let hamburger = e.load_obj("assets/models/hamburger/hamburger.obj").expect("obj loading failed");
  scn.add_instance_trans(hamburger, v3(-12.0, 21.5, 7.0), v3(0.0, 0.0, 0.0), v3(0.4, 0.4, 0.4));

  let can = e.load_obj("assets/models/cola_can/cola_can.obj").expect("obj loading failed");
  scn.add_instance_trans(can, v3(-4.0, 21.5, 1.0), v3(0.0, 0.9, 0.0), v3(0.8, 0.8, 0.8));
  scn.add_instance_trans(can, v3(2.0, 21.5, 4.0), v3(0.0, 2.3, 0.0), v3(0.8, 0.8, 0.8));

  let camera = Camera::new_perspective(e.get_aspect(), v3(0.0, 40.0, 30.0), v3(0.0, 25.0, 0.0), 60.0, 0.01, 1000.0).unwrap();

  MyApp { camera, scn }
}
