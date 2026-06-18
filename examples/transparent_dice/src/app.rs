// ==============================================================================================
// Purpose:         Demo of loading models & updating their material, also transparency wow
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// ==============================================================================================

use argh::engine::Key;
use argh::math::V3_AXIS_Y;
use argh::prelude::*;

pub struct MyApp {
  camera: Camera,
  scene: Scene,
}

impl App for MyApp {
  // You must always implement the update method it will be called once per frame
  fn update(&mut self, eng: &mut Engine, dt: f64, t: f64) {
    eng.clear(BLACK);

    // This makes the animation independent of framerate
    let rot = Quat::new(V3_AXIS_Y, 0.5 * dt as f32);
    // Rotate & move the camera
    let mut p = rot.rotate_vec3(self.camera.pos());
    p.y = ((f64::sin(t * 0.75) * 2.6) + 36.0) as f32;
    self.camera.set_pos(p);

    // Draw the scene
    eng.render(&self.camera, &self.scene);

    // Quit on escape
    if eng.is_pressed(Key::Escape) {
      eng.stop();
    }
  }
}

pub fn new(eng: &mut Engine) -> MyApp {
  let mut scene = Scene::new();

  scene.add_light(Light::new_dynamic(v3(5.0, 22.0, 7.0), 6.6, CYAN, 0.09, 0.03));
  scene.add_light(Light::new_dynamic(v3(-7.0, 1.0, 4.0), 2.7, YELLOW, 0.09, 0.03));
  scene.add_light(Light::new_dynamic(v3(0.0, 6.0, -7.0), 2.9, WHITE, 0.09, 0.03));

  // Load dice which will be opaque
  let die = eng.load_obj("assets/obj/cola_can/cola_can.obj", false).expect("loaded");

  // This is a example of loading and overriding the material on a object
  let model = eng.model_mut(die);

  // We can interrogate the mesh names & indexes here, but we'll hardcode it as 0 anyhow
  println!("mesh info {:#?}", model.mesh_info());

  // Get the material and update by making semi-opaque, gosh!
  let mut mat = model.mesh_material(2);
  mat.set_opacity(0.3);
  model.set_material(2, mat);

  // Crate is a cube primitive with a custom image texture
  let mut crate_mat = Material::new_textured(Texture::new("assets/textures/crate.png").unwrap());
  crate_mat.specular = BLACK;
  let cube = eng.add_model(primitives::new_cube(crate_mat));

  // Build the scene
  scene.add_instance_mut(die).pos_xyz(3.4, 2.0, 3.4).scale(2.0);
  scene.add_instance_mut(die).pos_xyz(-3.4, 2.0, 3.4).scale(2.0);
  scene.add_instance_mut(die).pos_xyz(-3.4, 2.0, -3.4).scale(2.0);
  scene.add_instance_mut(die).pos_xyz(3.4, 2.0, -3.4).scale(2.0);
  scene.add_instance_mut(cube).pos_xyz(0.0, -6.0, 0.0).scale(12.0);

  let camera = Camera::new_perspective(eng.aspect(), v3(0.0, 5.0, 32.0), v3(0.0, 1.2, 0.0), 50.0, 0.01, 100.0).unwrap();

  MyApp { camera, scene }
}
