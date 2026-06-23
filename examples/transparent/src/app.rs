// ==============================================================================================
// Purpose:         Demo of loading models & updating their material, also transparency wow
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// ==============================================================================================

use argh::engine::Key;
use argh::math::V3_AXIS_Y;
use argh::prelude::*;
use argh::primitives::PlaneOrientation;

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
    p.y = ((f64::sin(t * 0.75) * 2.6) + 6.0) as f32;
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

  scene.add_light(Light::new_dynamic(v3(5.0, 7.0, 8.0), 5.0, WHITE, 0.09, 0.03));
  scene.add_light(Light::new_dynamic(v3(-8.0, 5.0, -5.0), 4.0, MAGENTA, 0.12, 0.06));

  // Load test model which is a mix of opaque and semi-transparent
  let box_mdl = eng.load_obj("assets/obj/glass-box/glass_box.obj", false).expect("loaded");

  // Floor is a plane primitive with a custom image texture
  let mut tex = Texture::new("assets/textures/stone_blocks.jpg").unwrap();
  tex.scale = 8.0;
  let mut floor_mat = Material::new_textured(tex);
  floor_mat.specular = NONE;
  let floor = eng.add_model(primitives::new_plane(floor_mat, 3, PlaneOrientation::XZ));

  // Build the scene
  scene.add_instance_mut(box_mdl).pos_xyz(3.4, 2.0, 3.4).scale(2.0);
  scene.add_instance_mut(box_mdl).pos_xyz(-3.4, 2.0, 3.4).scale(2.0);
  scene.add_instance_mut(box_mdl).pos_xyz(-3.4, 2.0, -3.4).scale(2.0);
  scene.add_instance_mut(box_mdl).pos_xyz(3.4, 2.0, -3.4).scale(2.0);
  scene.add_instance_mut(floor).pos_xyz(0.0, 0.0, 0.0).scale(98.0);

  let camera = Camera::new_perspective(eng.aspect(), v3(0.0, 12.0, 16.0), v3(0.0, 0.8, -1.0), 50.0, 0.01, 100.0).unwrap();

  MyApp { camera, scene }
}
