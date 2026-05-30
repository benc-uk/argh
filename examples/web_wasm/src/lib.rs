// ==============================================================================================
// Module & file:   web_wasm / lib.rs
// Purpose:         Entry point for running argh with a web WASM host
// Author & Date:   Ben Coleman, 2026
// License:         MIT
// Notes:           This is essentially all boilerplate code which can be reused as-is
//                  Provided with corresponding scene.rs file this code would not need changing
// ==============================================================================================

use std::cell::RefCell;

use argh::prelude::*;
use wasm_bindgen::prelude::*;
mod app;

use app::WasmApp;

const W: i32 = 1024;
const H: i32 = 576;

// Static globals in Rust require a lot of ugly gymnastics
thread_local! {
  static ENGINE: RefCell<Option<Engine>>  = const { RefCell::new(None) };
  static APP:    RefCell<Option<WasmApp>> = const { RefCell::new(None) };
  static PIXELS: RefCell<Vec<u8>>         = const { RefCell::new(Vec::new()) };
}

// Allocate everything we need including the shared globals
#[wasm_bindgen(start)]
pub fn start() {
  console_error_panic_hook::set_once();

  let mut eng = Engine::new(W, H);
  let app = app::new(&mut eng);

  PIXELS.with(|c| *c.borrow_mut() = vec![0u8; (W * H * 4) as usize]);
  APP.with(|c| *c.borrow_mut() = Some(app));
  ENGINE.with(|c| *c.borrow_mut() = Some(eng));
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
// Called every frame from the JS side to advance the app and copy the framebuffer out as bytes
pub fn update(dt: f64) {
  ENGINE.with_borrow_mut(|e| {
    APP.with_borrow_mut(|a| {
      PIXELS.with_borrow_mut(|p| {
        let e = e.as_mut().expect("engine not initialized");
        let a = a.as_mut().expect("app not initialized");

        // Frame pipeline: time bookkeep, render, overlay, copy out
        let t = e.tick(dt);
        a.update(e, dt, t);
        e.draw_debug();

        // This is unique to WASM, copy the framebuffer out to our own location
        e.buffer_copy_bytes(p);
      })
    })
  });
}
