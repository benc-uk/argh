use std::cell::RefCell;

use argh::engine::{Engine, Scene};
use wasm_bindgen::prelude::*;
mod scene;

use scene::MyScene;

const W: i32 = 640;
const H: i32 = 360;

thread_local! {
  static ENGINE: RefCell<Option<Engine>> = const { RefCell::new(None) };
  static SCENE: RefCell<Option<MyScene>> = const { RefCell::new(None) };
  static RGBA: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
}

#[wasm_bindgen(start)]
pub fn start() {
  console_error_panic_hook::set_once();

  let mut e = Engine::new(W, H);
  let scene = MyScene::new(&mut e);

  ENGINE.with(|c| *c.borrow_mut() = Some(e));
  SCENE.with(|c| *c.borrow_mut() = Some(scene));
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
