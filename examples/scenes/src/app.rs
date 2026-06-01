use argh::engine::Key;
use argh::prelude::*;

pub struct MyApp {
  camera: Camera,
  scn1: Scene,
  scn2: Scene,
  active_scn: u8,
}

impl App for MyApp {
  fn update(&mut self, engine: &mut Engine, _dt: f64, t: f64) {
    engine.clear(BLACK);

    let cube = self.scn2.instances_mut().next().expect("exists");
    cube.rot_y(0.02);
    let sphere = self.scn1.instances_mut().next().expect("exists");
    sphere.set_pos(Vec3 { x: 0.0, y: f64::sin(t), z: 0.0 });

    match self.active_scn {
      1 => engine.render(&self.camera, &self.scn1),
      2 => engine.render(&self.camera, &self.scn2),
      _ => engine.render(&self.camera, &self.scn2),
    }

    if !engine.get_keys_pressed().is_empty() {
      // Space changes which scene is rendered
      if engine.is_pressed(Key::Space) {
        self.active_scn += 1;
        if self.active_scn > 2 {
          self.active_scn = 1;
        }
      }

      // Quit on escape
      if engine.is_pressed(Key::Escape) {
        engine.stop();
      }
    }
  }
}

pub fn new(e: &mut Engine) -> MyApp {
  let mut scene1 = Scene::new();
  let mut scene2 = Scene::new();

  scene1.add_light(Light::new(v3(8.0, 7.0, 5.0), 1.0, WHITE));
  scene2.add_light(Light::new(v3(-8.0, 7.0, 5.0), 1.0, WHITE));

  let col_mat1 = e.add_material(Material::new_flat(Colour::rand()));
  let col_mat2 = e.add_material(Material::new_flat(Colour::rand()));

  let sphere1 = e.add_mesh(primitives::new_sphere(24, 48));
  let cube = e.add_mesh(primitives::new_cube());

  scene1.add_instance(sphere1, col_mat1);
  scene2.add_instance_trans(cube, col_mat2, V3_ZERO, v3(0.0, 1.9, 0.0), v3(1.0, 1.0, 1.0));

  let camera = Camera::new_perspective(e.get_aspect(), v3(0.0, 1.0, 3.0), V3_ZERO, 60.0, 0.01, 50.0).unwrap();

  MyApp {
    camera,
    scn1: scene1,
    scn2: scene2,
    active_scn: 1,
  }
}
