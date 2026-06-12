use argh::engine::Key;
use argh::prelude::*;

pub struct MyApp {
  camera: Camera,
  scn1: Scene,
  scn2: Scene,
  active_scn: u8,
}

impl App for MyApp {
  fn update(&mut self, eng: &mut Engine, _dt: f64, t: f64) {
    eng.clear(BLACK);

    let cube = self.scn2.instances_mut().next().expect("exists");
    cube.rot_y(0.02);
    let sphere = self.scn1.instances_mut().next().expect("exists");
    sphere.set_pos(Vec3 {
      x: 0.0,
      y: f32::sin(t as f32),
      z: 0.0,
    });

    match self.active_scn {
      1 => eng.render(&self.camera, &self.scn1),
      2 => eng.render(&self.camera, &self.scn2),
      _ => eng.render(&self.camera, &self.scn2),
    }

    if !eng.get_keys_pressed().is_empty() {
      // Space changes which scene is rendered
      if eng.is_pressed(Key::Space) {
        self.active_scn += 1;
        if self.active_scn > 2 {
          self.active_scn = 1;
        }
      }

      // Quit on escape
      if eng.is_pressed(Key::Escape) {
        eng.stop();
      }
    }
  }
}

pub fn new(eng: &mut Engine) -> MyApp {
  let mut scene1 = Scene::new();
  let mut scene2 = Scene::new();

  scene1.add_light(Light::new(v3(8.0, 7.0, 5.0), 1.0, WHITE, 0.09, 0.03, false, true));
  scene2.add_light(Light::new(v3(-8.0, 7.0, 5.0), 1.0, WHITE, 0.09, 0.03, false, true));

  let check_tex = Texture::new("assets/checker_256.png").unwrap();
  let sphere1 = eng.add_model(primitives::new_sphere(Material::new_textured(check_tex), 24, 48));
  let cube = eng.add_model(primitives::new_cube(Material::new_flat(Colour::rand())));

  scene1.add_instance(sphere1);
  scene2.add_instance_trans(cube, V3_ZERO, v3(0.0, 1.9, 0.0), v3(1.0, 1.0, 1.0));

  let camera = Camera::new_perspective(eng.get_aspect(), v3(0.0, 1.0, 3.0), V3_ZERO, 60.0, 0.01, 50.0).unwrap();

  MyApp {
    camera,
    scn1: scene1,
    scn2: scene2,
    active_scn: 1,
  }
}
