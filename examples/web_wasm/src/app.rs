use argh::{math::V3_AXIS_Y, prelude::*};

pub struct WasmApp {
  camera: Camera,
  scene: Scene,
}

const _CRATE_IMG_BYTES: &[u8] = include_bytes!("../../../assets/textures/crate.png");
const _TEAPOT_MDL_LOW_BYTES: &[u8] = include_bytes!("../../../assets/obj/teapot/utah_teapot_low.obj");
const _TEAPOT_MDL_HIGH_BYTES: &[u8] = include_bytes!("../../../assets/obj/teapot/utah_teapot_high.obj");

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

  scene.add_light(Light::new(v3(15.0, 2.0, 5.0), 2.6, BLUE, 0.09, 0.03, false, true));
  scene.add_light(Light::new(v3(-9.0, 1.0, 9.0), 2.7, RED, 0.09, 0.03, false, true));
  scene.add_light(Light::new(v3(4.0, 9.0, 10.0), 3.9, WHITE, 0.09, 0.03, false, true));

  let mut crate_mat = Material::new_textured(Texture::new("assets/crate.png").unwrap());
  crate_mat.specular = BLACK;

  // See https://graphics.cs.utah.edu/teapot/ for source generator of Utah Teapot obj files
  let teapot_low = eng.load_obj("assets/obj/teapot/utah_teapot_low.obj").expect("obj loading failed");
  let teapot_high = eng.load_obj("assets/obj/teapot/utah_teapot_high.obj").expect("obj loading failed");

  eng.model_mut(teapot_low).set_all_material(Material::new_flat(Colour::new(0.7, 0.6, 0.85)));
  eng.model_mut(teapot_high).set_all_material(Material::new_flat(Colour::new(0.85, 0.7, 0.5)));

  let cube = eng.add_model(primitives::new_cube(crate_mat));

  scene.add_instance_mut(teapot_low).pos_xyz(2.8, 0.0, 2.8).rot_y(3.0).scale(1.6).smooth(false);
  scene.add_instance_mut(teapot_high).pos_xyz(-2.8, 0.0, -3.3).rot_y(2.0).scale(1.5);
  scene.add_instance_mut(cube).pos_xyz(0.0, -6.0, 0.0).scale(12.0);

  let camera = Camera::new_perspective(eng.aspect(), v3(0.0, 5.0, 14.0), v3(0.0, 0.5, 0.0), 50.0, 0.01, 100.0).unwrap();

  WasmApp { camera, scene }
}
