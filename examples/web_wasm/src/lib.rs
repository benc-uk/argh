use std::cell::RefCell;

use argh::camera::Camera;
use argh::colour::*;
use argh::engine::{Engine, InstanceHandle, Scene};
use argh::light::Light;
use argh::math::Vec3;
use argh::models::{ImageTexture, Material, SimpleColourTexture};
use argh::primitives;
use wasm_bindgen::prelude::*;

const W: i32 = 800;
const H: i32 = 600;

struct MyScene {
  instances: Vec<InstanceHandle>,
  camera: Camera,
}

const CHECKER_IMG_BYTES: &[u8] = include_bytes!("../../../assets/checker_256.png");
const EARTH_IMG_BYTES: &[u8] = include_bytes!("../../../assets/earth.png");

impl Scene for MyScene {
  fn update(&mut self, e: &mut Engine, dt: f64, t: f64) {
    e.clear(BLACK);

    let mut axis = Vec3::new(0.6, 0.3, 0.9);
    axis.normalize();
    let py = f64::cos(t * 1.0);
    let px = f64::sin(t * 0.7);
    let pz = -0.5 - (f64::sin(t * 1.4) * 0.9);
    let pz2 = f64::sin(t * 0.9) * 0.6;
    e.instance_mut(self.instances[0]).rot_y(0.5 * dt).rot_x(0.8 * dt).set_pos(Vec3::new(-px, py, pz));
    e.instance_mut(self.instances[1]).rot_y(0.9 * dt).rot_x(1.2 * dt).set_pos(Vec3::new(px, -py, pz2));
    e.instance_mut(self.instances[2]).rot_y(0.1 * dt).set_pos(Vec3::new(px * 0.7, py * 1.0, 0.0));

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

  let mut e = Engine::new(W, H, String::from("Argh: simple_3d"), 2);
  e.debug = true;
  e.target_fps = 60;

  e.add_light(Light::new(Vec3::new(3.0, 7.0, 5.0), 1.0, WHITE));
  e.add_light(Light::new(Vec3::new(-6.0, 7.0, 5.0), 0.8, BLUE));
  e.add_light(Light::new(Vec3::new(8.0, -2.0, 9.0), 0.5, RED));

  let cube_tex = ImageTexture::from_bytes(CHECKER_IMG_BYTES).unwrap();
  let earth_tex = ImageTexture::from_bytes(EARTH_IMG_BYTES).unwrap();
  let col_tex1 = SimpleColourTexture::new(Colour::rand());
  let col_tex2 = SimpleColourTexture::new(Colour::rand());

  let crate_mat = e.add_material(Material::new(cube_tex));
  let earth_mat = e.add_material(Material::new(earth_tex));
  let col_mat1 = e.add_material(Material::new(col_tex1));
  let col_mat2 = e.add_material(Material::new(col_tex2));

  let cube = e.add_mesh(primitives::new_cube());
  let sphere1 = e.add_mesh(primitives::new_sphere(8, 12));
  let sphere2 = e.add_mesh(primitives::new_sphere(24, 48));
  let teapot = e.add_mesh(primitives::new_teapot());

  let inst1 = e.add_instance(cube, crate_mat);
  let inst2 = e.add_instance(sphere1, col_mat1);
  let inst3 = e.add_instance(sphere2, earth_mat);
  let inst4 = e.add_instance(teapot, col_mat2);
  e.instance_mut(inst2).smooth = false;
  e.instance_mut(inst4).scale(0.5).set_pos_xyz(0.5, -1.55, -2.0);

  let camera = Camera::new_perspective(e.get_aspect(), Vec3::new(0.0, 1.0, 2.8), Vec3::new(0.0, 0.0, 0.0), 60.0, 0.01, 10.0).unwrap();

  ENGINE.with(|c| *c.borrow_mut() = Some(e));
  SCENE.with(|c| {
    *c.borrow_mut() = Some(MyScene {
      instances: vec![inst1, inst2, inst3],
      camera,
    })
  });
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
