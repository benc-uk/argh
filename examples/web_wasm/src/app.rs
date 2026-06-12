use argh::{math::V3_AXIS_Y, prelude::*};

pub struct WasmApp {
  camera: Camera,
  scene: Scene,
}

const CRATE_IMG_BYTES: &[u8] = include_bytes!("../../../assets/crate.png");

impl App for WasmApp {
  fn update(&mut self, eng: &mut Engine, dt: f64, t: f64) {
    eng.clear(BLACK);

    // This makes the animation independent of framerate
    let rot = Quat::new(V3_AXIS_Y, 0.5 * dt as f32);

    // Rotate & move the camera
    let mut p = rot.rotate_vec3(self.camera.get_pos());
    p.y = ((f64::sin(t * 0.75) * 2.6) + 6.0) as f32;
    self.camera.set_pos(p);

    // Draw the scene
    eng.render(&self.camera, &self.scene);
  }
}

// Initialize & create the app
pub fn new(eng: &mut Engine) -> WasmApp {
  let mut scene = Scene::new();

  scene.add_light(Light::new(v3(15.0, 2.0, 5.0), 2.6, BLUE, 0.09, 0.03, false, true));
  scene.add_light(Light::new(v3(-9.0, 1.0, 9.0), 2.7, RED, 0.09, 0.03, false, true));
  scene.add_light(Light::new(v3(4.0, 9.0, 10.0), 3.9, WHITE, 0.09, 0.03, false, true));

  let mut crate_mat = Material::new_textured(Texture::from_bytes(CRATE_IMG_BYTES).unwrap());
  crate_mat.specular = BLACK;

  let flat_1 = Material::new_flat(Colour::new(0.7, 0.7, 0.8));
  let flat_2 = Material::new_flat(Colour::new(0.8, 0.7, 0.5));
  let teapot1 = eng.add_model(primitives::new_teapot(flat_1));
  let teapot2 = eng.add_model(primitives::new_teapot(flat_2));
  let cube = eng.add_model(primitives::new_cube(crate_mat));

  scene.add_instance_trans(teapot1, v3(2.0, 0.0, 2.3), v3(0.0, 3.0, 0.0), v3(1.2, 1.5, 1.2));
  scene.add_instance_trans(teapot2, v3(-2.0, 0.0, -2.9), v3(0.0, 2.0, 0.0), v3(1.2, 1.2, 1.2));
  scene.add_instance_trans(cube, v3(0.0, -6.0, 0.0), v3(0.0, 0.0, 0.0), v3(12.0, 12.0, 12.0));
  let camera = Camera::new_perspective(eng.get_aspect(), v3(0.0, 5.0, 14.0), v3(0.0, 0.5, 0.0), 50.0, 0.01, 100.0).unwrap();

  WasmApp { camera, scene }
}
