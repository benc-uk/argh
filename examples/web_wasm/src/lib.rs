// ==============================================================================================
// Module & file:   web_wasm / lib.rs
// Purpose:         Entry point for running argh with a web WASM host
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:           This is essentially all boilerplate code which can be reused as-is
//                  Provided with corresponding scene.rs file this code would not need changing
// ==============================================================================================

use std::cell::RefCell;

use argh::engine::{Engine, Scene};
use wasm_bindgen::prelude::*;
mod scene;

use scene::MyScene;

const W: i32 = 1024;
const H: i32 = 576;

// Static globals in Rust require a lot of ugly gymnastics
thread_local! {
  static ENGINE: RefCell<Option<Engine>> = const { RefCell::new(None) };
  static SCENE: RefCell<Option<MyScene>> = const { RefCell::new(None) };
  static PIXELS: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
}

// Allocate everything we need including the shared globals
#[wasm_bindgen(start)]
pub fn start() {
  console_error_panic_hook::set_once();

  let mut e = Engine::new(W, H);
  let scene = MyScene::new(&mut e);

  ENGINE.with(|c| *c.borrow_mut() = Some(e));
  SCENE.with(|c| *c.borrow_mut() = Some(scene));
  PIXELS.with(|c| *c.borrow_mut() = vec![0u8; (W * H * 4) as usize]);
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
// Get a raw pointer to where the PIXELS Vec<u8> sits in memory
pub fn pixel_ptr() -> *const u8 {
  PIXELS.with(|rgba_cell| rgba_cell.borrow().as_ptr())
}

#[wasm_bindgen]
// Called every frame from the JS side to trigger engine tick (and render) then grab the frame as bytes
pub fn update(dt: f64) {
  ENGINE.with_borrow_mut(|e| {
    SCENE.with_borrow_mut(|s| {
      PIXELS.with_borrow_mut(|p| {
        let e = e.as_mut().expect("engine not initialized");

        // Advance the engine one tick or frame
        e.tick(s.as_mut().expect("scene not initialized"), dt);

        // This copies frame the chunk of memory of PIXELS
        e.buffer_copy_bytes(p);
      })
    })
  });
}
