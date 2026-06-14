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

  scene.add_light(Light::new(v3(15.0, 2.0, 5.0), 2.6, BLUE, 0.09, 0.03, false, true));
  scene.add_light(Light::new(v3(-9.0, 1.0, 9.0), 2.7, RED, 0.09, 0.03, false, true));
  scene.add_light(Light::new(v3(4.0, 9.0, 10.0), 3.9, WHITE, 0.09, 0.03, false, true));

  // See https://graphics.cs.utah.edu/teapot/ for source generator of Utah Teapot obj files
  // let teapot_low = eng.load_obj("assets/obj/teapot/utah_teapot_low.obj").expect("obj loading failed");
  // let teapot_high = eng.load_obj("assets/obj/teapot/utah_teapot_high.obj").expect("obj loading failed");
  // eng.model_mut(teapot_low).set_all_material(Material::new_flat(Colour::new(0.7, 0.6, 0.85)));
  // eng.model_mut(teapot_high).set_all_material(Material::new_flat(Colour::new(0.85, 0.7, 0.5)));

  let thing = eng.load_gltf("assets/glft/scene.gltf").expect("gltf loading failed");
  let thing2 = eng.load_gltf("assets/glft/bottle/bottle_A_labeled_green.gltf").expect("gltf loading failed");

  // Crate is a cube primitive with a custom image texture
  let mut crate_mat = Material::new_textured(Texture::new("assets/textures/crate.png").unwrap());
  crate_mat.specular = BLACK;
  let cube = eng.add_model(primitives::new_cube(crate_mat));

  // Build the scene
  scene.add_instance_mut(thing).pos_xyz(2.8, 0.0, 2.8);
  scene.add_instance_mut(thing2).pos_xyz(-2.8, 0.0, 2.8).scale(4.0);
  // scene.add_instance_mut(teapot_high).pos_xyz(-2.8, 0.0, -3.3).rot_y(2.0).scale(1.5);
  scene.add_instance_mut(cube).pos_xyz(0.0, -6.0, 0.0).scale(12.0);

  let camera = Camera::new_perspective(eng.aspect(), v3(0.0, 5.0, 14.0), v3(0.0, 0.5, 0.0), 50.0, 0.01, 100.0).unwrap();

  MyApp { camera, scene }
}
