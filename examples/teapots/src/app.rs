// ==============================================================================================
// Purpose:         Classic Utah teapots loaded from GLtf files and shown in a little scene
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

  scene.add_light(Light::new_dynamic(v3(5.0, 2.0, 5.0), 6.6, BLUE, 0.09, 0.03));
  scene.add_light(Light::new_dynamic(v3(-9.0, 1.0, 9.0), 2.7, RED, 0.09, 0.03));
  scene.add_light(Light::new_dynamic(v3(0.0, 6.0, -4.0), 2.9, WHITE, 0.09, 0.03));

  let tp1 = eng.load_gltf("assets/gltf/utah_teapot_low.glb").expect("gltf loading failed");
  // Low poly looks cool with flat shading
  eng.model_mut(tp1).flatten();
  let tp2 = eng.load_gltf("assets/gltf/utah_teapot_med.glb").expect("gltf loading failed");
  let tp3 = eng.load_gltf("assets/gltf/utah_teapot_high.glb").expect("gltf loading failed");

  // Crate is a cube primitive with a custom image texture
  let mut crate_mat = Material::new_textured(Texture::new("assets/textures/crate.png").unwrap());
  crate_mat.specular = BLACK;
  let cube = eng.add_model(primitives::new_cube(crate_mat));

  // Build the scene
  scene.add_instance_mut(tp1).pos_xyz(2.8, 0.0, 0.0).scale(1.1);
  scene.add_instance_mut(tp2).pos_xyz(-2.8, 0.0, -2.8).scale(1.1).rot_y(0.8);
  scene.add_instance_mut(tp3).pos_xyz(-2.8, 0.0, 2.8).scale(1.1).rot_y(-1.1);
  scene.add_instance_mut(cube).pos_xyz(0.0, -6.0, 0.0).scale(12.0);

  let camera = Camera::new_perspective(eng.aspect(), v3(0.0, 5.0, 12.0), v3(0.0, 1.2, 0.0), 50.0, 0.01, 100.0).unwrap();

  MyApp { camera, scene }
}
