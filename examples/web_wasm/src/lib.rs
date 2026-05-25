use std::cell::RefCell;

use argh::camera::Camera;
use argh::colour::*;
use argh::engine::{Engine, Scene};
use argh::light::Light;
use argh::math::{AXIS_Y, Quat, Vec3};
use argh::models::{ImageTexture, Material, SimpleColourTexture};
use argh::primitives;
use wasm_bindgen::prelude::*;

const W: i32 = 854;
const H: i32 = 480;

struct MyScene {
  camera: Camera,
}

const CRATE_IMG_BYTES: &[u8] = include_bytes!("../../../assets/crate.png");

impl Scene for MyScene {
  fn update(&mut self, e: &mut Engine, dt: f64, t: f64) {
    e.clear(BLACK);

    // This makes the animation independent of framerate
    let rot = Quat::new(AXIS_Y, 0.5 * dt);
    // Rotate & move the camera
    let mut p = rot.rotate_vec3(self.camera.get_pos());
    p.y = (f64::sin(t * 0.75) * 2.6) + 6.0;
    self.camera.set_pos(p);

    // Draw the scene
    e.render_all(&self.camera);
  }
}

thread_local! {
  static ENGINE: RefCell<Option<Engine>> = const { RefCell::new(None) };
  static SCENE: RefCell<Option<MyScene>> = const { RefCell::new(None) };
  static RGBA: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
}

#[wasm_bindgen(start)]
pub fn start() {
  console_error_panic_hook::set_once();

  let mut e = Engine::new(W, H, "", 1);
  e.debug = true;
  e.target_fps = 0;

  e.add_light(Light::new(Vec3::new(15.0, 2.0, 5.0), 0.6, BLUE));
  e.add_light(Light::new(Vec3::new(-9.0, 1.0, 9.0), 0.7, RED));
  e.add_light(Light::new(Vec3::new(4.0, 9.0, 10.0), 0.9, WHITE));

  let crate_tex = ImageTexture::from_bytes(CRATE_IMG_BYTES).unwrap();
  let mut crate_mat = Material::new(crate_tex);
  crate_mat.specular = 0.0;

  let teapot_mat1 = e.add_material(Material::new(SimpleColourTexture::new(Colour::new(0.7, 0.7, 0.8))));
  let teapot_mat2 = e.add_material(Material::new(SimpleColourTexture::new(Colour::new(0.6, 0.2, 0.7))));
  let crate_mat_hdl = e.add_material(crate_mat);

  let teapot = e.add_mesh(primitives::new_teapot());
  let cube = e.add_mesh(primitives::new_cube());

  e.add_instance_trans(teapot, teapot_mat1, Vec3::new(2.0, 0.0, 2.3), Vec3::new(0.0, 3.0, 0.0), Vec3::new(1.2, 1.5, 1.2));
  e.add_instance_trans(teapot, teapot_mat2, Vec3::new(-2.0, 0.0, -2.9), Vec3::new(0.0, 2.0, 0.0), Vec3::new(1.2, 1.2, 1.2));
  e.add_instance_trans(cube, crate_mat_hdl, Vec3::new(0.0, -6.0, 0.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(12.0, 12.0, 12.0));

  let camera = Camera::new_perspective(e.get_aspect(), Vec3::new(0.0, 5.0, 14.0), Vec3::new(0.0, 0.5, 0.0), 50.0, 0.01, 100.0).unwrap();

  ENGINE.with(|c| *c.borrow_mut() = Some(e));
  SCENE.with(|c| *c.borrow_mut() = Some(MyScene { camera }));
  RGBA.with(|c| *c.borrow_mut() = vec![0u8; (W * H * 4) as usize]);
}

#[wasm_bindgen]
pub fn width() -> u32 {
  W as u32
}

#[wasm_bindgen]
pub fn height() -> u32 {
  H as u32
}

#[wasm_bindgen]
pub fn update(dt: f64) -> Vec<u8> {
  ENGINE.with(|e_cell| {
    SCENE.with(|s_cell| {
      RGBA.with(|r_cell| {
        let mut e_opt = e_cell.borrow_mut();
        let mut s_opt = s_cell.borrow_mut();
        let mut r = r_cell.borrow_mut();
        let e = e_opt.as_mut().expect("engine not initialised");
        let s = s_opt.as_mut().expect("scene not initialised");

        e.tick(s, dt);

        // Get the raw pixels in the buffer and unpack into Vec<u8> (will be Uint8ClampedArray on JS side)
        for (i, &p) in e.buffer_content().iter().enumerate() {
          let o = i * 4;
          r[o] = ((p >> 16) & 0xff) as u8;
          r[o + 1] = ((p >> 8) & 0xff) as u8;
          r[o + 2] = (p & 0xff) as u8;
          r[o + 3] = 0xff;
        }

        r.clone()
      })
    })
  })
}
