use argh::{engine::Key, math::V3_AXIS_Y, prelude::*};

pub struct MyApp {
  camera: Camera,
  scn: Scene,
}

impl App for MyApp {
  fn update(&mut self, eng: &mut Engine, _dt: f64, _t: f64) {
    eng.clear(BLACK);

    if let Some(mpos) = eng.get_mouse_pos() {
      let mut p = self.camera.get_pos();
      let mut mx = mpos.0 / eng.get_size().0 as f32;
      let mut my = mpos.1 / eng.get_size().1 as f32;
      mx = (mx * 2.0) - 1.0;
      my = -((my * 2.0) - 1.0);
      let rot_y = Quat::new(V3_AXIS_Y, mx * 0.1);
      p.y = (p.y + my * 0.5).clamp(0.8, 7.0);
      p = rot_y.rotate_vec3(p);
      self.camera.set_pos(p);
    }

    eng.render(&self.camera, &self.scn);

    if !eng.get_keys_pressed().is_empty() {
      // Quit on escape
      if eng.is_pressed(Key::Escape) {
        eng.stop();
      }
    }
  }
}

pub fn new(eng: &mut Engine) -> MyApp {
  let mut scn = Scene::new();

  let l1 = Light::new(v3(4.0, 5.0, 0.0), 2.0, col8(255, 91, 0));
  let l2 = Light::new(v3(-4.0, 8.0, 6.0), 2.0, col(0.1, 1.0, 0.2));
  scn.add_light(l1);
  scn.add_light(l2);

  let table = eng.load_obj("assets/models/dungeon/table_long.obj").expect("obj loading failed");
  let btl = eng.load_obj("assets/models/dungeon/bottle_A_labeled_green.obj").expect("obj loading failed");
  let candle = eng.load_obj("assets/models/dungeon/candle_triple.obj").expect("obj loading failed");
  let plate = eng.load_obj("assets/models/dungeon/plate_food_A.obj").expect("obj loading failed");
  let floor = eng.load_obj("assets/models/dungeon/floor_tile_large.obj").expect("obj loading failed");
  let grate = eng.load_obj("assets/models/dungeon/floor_tile_big_grate.obj").expect("obj loading failed");
  let floor_rocks = eng.load_obj("assets/models/dungeon/floor_tile_large_rocks.obj").expect("obj loading failed");
  let chest = eng.load_obj("assets/models/dungeon/chest.obj").expect("obj loading failed");
  let chest_lid = eng.load_obj("assets/models/dungeon/chest_lid.obj").expect("obj loading failed");
  let pillar = eng.load_obj("assets/models/dungeon/pillar.obj").expect("obj loading failed");
  let barrel_large = eng.load_obj("assets/models/dungeon/barrel_large.obj").expect("obj loading failed");

  scn.add_instance_trans(table, V3_ZERO, v3(0.0, 0.9, 0.0), v3(1.0, 1.0, 1.0));
  scn.add_instance_trans(btl, v3(0.0, 1.0, 0.0), v3(0.0, 0.0, 0.0), v3(1.0, 1.0, 1.0));
  scn.add_instance_trans(candle, v3(0.7, 1.0, 0.0), v3(0.0, 0.0, 0.0), v3(1.0, 1.0, 1.0));
  scn.add_instance_trans(plate, v3(-0.9, 1.0, -0.6), v3(0.0, 0.0, 0.0), v3(1.0, 1.0, 1.0));

  scn.add_instance_trans(chest, v3(-4.0, 0.0, 4.0), v3(0.0, -2.9, 0.0), v3(1.0, 1.0, 1.0));
  scn.add_instance_trans(chest_lid, v3(-4.0, 0.0, 4.0), v3(0.0, -2.9, 0.0), v3(1.0, 1.0, 1.0));

  scn.add_instance_trans(pillar, v3(4.0, 0.0, -4.0), v3(0.0, 0.0, 0.0), v3(1.0, 1.0, 1.0));
  scn.add_instance_trans(barrel_large, v3(4.0, 0.0, 4.0), v3(0.0, 0.0, 0.0), v3(1.0, 1.0, 1.0));

  scn.add_instance_trans(floor, v3(0.0, 0.0, 0.0), v3(0.0, 0.0, 0.0), v3(1.0, 1.0, 1.0));
  scn.add_instance_trans(grate, v3(-4.0, 0.0, 0.0), v3(0.0, 0.0, 0.0), v3(1.0, 1.0, 1.0));
  scn.add_instance_trans(floor_rocks, v3(4.0, 0.0, 0.0), v3(0.0, 0.0, 0.0), v3(1.0, 1.0, 1.0));
  scn.add_instance_trans(floor, v3(-4.0, 0.0, 4.0), v3(0.0, 0.0, 0.0), v3(1.0, 1.0, 1.0));
  scn.add_instance_trans(floor, v3(0.0, 0.0, 4.0), v3(0.0, 0.0, 0.0), v3(1.0, 1.0, 1.0));
  scn.add_instance_trans(floor, v3(4.0, 0.0, 4.0), v3(0.0, 0.0, 0.0), v3(1.0, 1.0, 1.0));
  scn.add_instance_trans(floor_rocks, v3(-4.0, 0.0, -4.0), v3(0.0, 0.0, 0.0), v3(1.0, 1.0, 1.0));
  scn.add_instance_trans(floor, v3(0.0, 0.0, -4.0), v3(0.0, 0.0, 0.0), v3(1.0, 1.0, 1.0));
  scn.add_instance_trans(floor, v3(4.0, 0.0, -4.0), v3(0.0, 0.0, 0.0), v3(1.0, 1.0, 1.0));

  let camera = Camera::new_perspective(eng.get_aspect(), v3(0.0, 4.0, 7.0), v3(0.0, 0.0, 0.0), 60.0, 0.1, 100.0).unwrap();

  MyApp { camera, scn }
}
