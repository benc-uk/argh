// ==============================================================================================
// Module & file:   web_wasm / app.rs
// Purpose:         WasmApp is an implementation of the teapots demo
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:           This file is a copy of the teapots example, but with the assets baked as
//                  include_bytes rather than using the filesystem (doesn't exist in WASM)
// ==============================================================================================

use argh::{math::V3_AXIS_Y, prelude::*};

pub struct WasmApp {
  camera: Camera,
  scene: Scene,
}

const CRATE_IMG_BYTES: &[u8] = include_bytes!("../../../assets/textures/crate.png");
const TEAPOT_MDL_LOW_BYTES: &[u8] = include_bytes!("../../../assets/gltf/utah_teapot_low.glb");
const TEAPOT_MDL_MED_BYTES: &[u8] = include_bytes!("../../../assets/gltf/utah_teapot_med.glb");
const TEAPOT_MDL_HIGH_BYTES: &[u8] = include_bytes!("../../../assets/gltf/utah_teapot_high.glb");

impl App for WasmApp {
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
  }
}

// Initialize & create the app
pub fn new(eng: &mut Engine) -> WasmApp {
  let mut scene = Scene::new();

  scene.add_light(Light::new(v3(5.0, 2.0, 5.0), 6.6, BLUE, 0.09, 0.03, false, true));
  scene.add_light(Light::new(v3(-9.0, 1.0, 9.0), 2.7, RED, 0.09, 0.03, false, true));
  scene.add_light(Light::new(v3(0.0, 6.0, -4.0), 2.9, WHITE, 0.09, 0.03, false, true));

  let tp1 = eng.load_gltf_bytes(TEAPOT_MDL_LOW_BYTES).expect("gltf loading failed");
  let tp2 = eng.load_gltf_bytes(TEAPOT_MDL_MED_BYTES).expect("gltf loading failed");
  let tp3 = eng.load_gltf_bytes(TEAPOT_MDL_HIGH_BYTES).expect("gltf loading failed");

  // Crate is a cube primitive with a custom image texture
  let mut crate_mat = Material::new_textured(Texture::from_bytes(CRATE_IMG_BYTES).unwrap());
  crate_mat.specular = BLACK;
  let cube = eng.add_model(primitives::new_cube(crate_mat));

  // Build the scene
  scene.add_instance_mut(tp1).pos_xyz(2.8, 0.0, 0.0).scale(1.0).smooth(false);
  scene.add_instance_mut(tp2).pos_xyz(-2.8, 0.0, -2.8).scale(1.0).rot_y(0.8);
  scene.add_instance_mut(tp3).pos_xyz(-2.8, 0.0, 2.8).scale(1.0).rot_y(-1.1);
  scene.add_instance_mut(cube).pos_xyz(0.0, -6.0, 0.0).scale(12.0);

  let camera = Camera::new_perspective(eng.aspect(), v3(0.0, 5.0, 14.0), v3(0.0, 0.5, 0.0), 50.0, 0.01, 100.0).unwrap();

  WasmApp { camera, scene }
}
