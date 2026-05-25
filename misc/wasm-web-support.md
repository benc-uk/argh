# WASM / Web Support for ARGH

Notes and design advice for running `argh` from a browser (WebAssembly), without
rewriting the engine. Captures both a minimal-seams approach and a higher-level
`start_wasm` convenience that would absorb the boilerplate into the library.

No code has been written. This is design and reasoning only.

## Why it won't run in a browser today

- `argh` hard-depends on `minifb`, which talks to native window systems
  (X11 / Win32 / Cocoa) and does not target `wasm32-unknown-unknown`.
- `Engine::start()` owns the loop: it calls `Window::new(...)`, blocks on
  `while window.is_open()`, drives time with `std::time::Instant`, and sources
  input via `window.get_keys()`.
- `std::time::Instant` panics on `wasm32-unknown-unknown` (it works on
  `wasm32-wasi`, but not in browser).

So with zero code edits, `argh` cannot be used from a web app. The rest of this
document is what needs to happen, kept as small as possible, and then a
higher-level convenience layer on top.

## What's already WASM friendly

- `Buffer.pixels` is just a `Vec<u32>` packed `0RGB`. That maps directly onto a
  `<canvas>` `ImageData` after a byte swap to RGBA.
- All maths, models, rasterisation, lighting, text, primitives are pure CPU
  Rust with no native or threaded deps. Nothing in `src/` outside the
  `engine/` files touches `minifb`.
- The `Scene` trait is trivial: one `update(&mut Engine, dt, t)`. Easy to drive
  from a JS `requestAnimationFrame` callback.

## Tier 1: minimal seams (smallest possible change)

Treat `minifb` purely as a desktop "host". For web, provide a different host
(browser canvas + rAF). To make that possible the engine needs three tiny
seams:

1. Put `minifb` behind a cargo feature, e.g. `default = ["desktop"]`, and gate
   `Engine::start()` plus the `minifb` imports in `engine/mod.rs` and
   `engine/input.rs` with `#[cfg(feature = "desktop")]`.
2. Add a host-agnostic frame entry point:
   ```rust
   pub fn tick<S: Scene>(&mut self, scene: &mut S, dt: f64)
   ```
   It does only the body of the current loop: snapshot input, call
   `scene.update`, optionally draw the FPS overlay. No window calls.
3. Add accessors the host can read / write each frame:
   - `pub fn pixels(&self) -> &[u32]` (or `&[u8]` after a swap helper)
   - `pub fn set_keys(&mut self, held: Vec<Key>, pressed: Vec<Key>)` so the
     WASM host can push keyboard state in.

That's the whole "minimal change" surface. Existing desktop examples keep
working unchanged because `start()` is still there under the default feature.

### Optional, but worth doing at the same time

Replace `std::time::Instant` inside `start()` with the
[`web-time`](https://crates.io/crates/web-time) crate. It's a drop-in
`Instant` / `SystemTime` that compiles for both desktop and browser, so the
timing code can be shared if the engine ever drives its own loop on web too.

### What the web side then looks like

- New crate (e.g. `examples/web`) with `crate-type = ["cdylib"]`, depending on
  `argh` with `default-features = false`.
- `wasm-bindgen` + `web-sys` features: `Window`, `Document`,
  `HtmlCanvasElement`, `CanvasRenderingContext2d`, `ImageData`,
  `KeyboardEvent`, `Performance`.
- `#[wasm_bindgen(start)]` builds `Engine::new(w, h, "", 1)` and a `Scene`,
  grabs the canvas 2D context, and registers a `requestAnimationFrame` closure
  that:
  1. Computes `dt` from `performance.now()`.
  2. Calls `engine.set_keys(...)` from a small key-event buffer maintained in
     JS or Rust.
  3. Calls `engine.tick(&mut scene, dt)`.
  4. Reads `engine.pixels()`, byte-swaps `0RGB` to `RGBA` into a
     `Uint8ClampedArray` (4 bytes per pixel), wraps it in
     `ImageData::new_with_u8_clamped_array_and_sh`, and calls
     `ctx.put_image_data(...)`.
- Build with `wasm-pack build --target web` (or `trunk` for a one-shot dev
  server).

## Tier 2: a `start_wasm` convenience inside the library

Tier 1 works but the web example ends up much bigger than the desktop one. A
`start_wasm` method absorbs the boilerplate into the library so the web
example shrinks back to roughly the same size as the desktop one.

### The one thing that's not negotiable

In a browser you cannot run a blocking main loop. `start()` is a
`while window.is_open()` loop; on WASM that would freeze the page and prevent
any input or rendering from ever happening. The browser owns the event loop
and the only way back into your code each frame is `requestAnimationFrame`.

So `start_wasm` cannot mirror `start()`. It has to:

1. Set everything up.
2. Install a `requestAnimationFrame` closure that captures the engine + scene.
3. Return immediately.

### What `start_wasm` would absorb

A single `engine.start_wasm("canvas-id", scene)` call would do all of this for
the user:

- Look up the `<canvas>` by id from `web_sys::window().document()`.
- Set its `width` / `height` to the engine's logical size.
- Grab the 2D context.
- Allocate a persistent `Uint8ClampedArray` / `Vec<u8>` of `w*h*4` bytes for
  the RGBA scratch buffer (reused every frame, not reallocated).
- Wire up `keydown` / `keyup` listeners on `window` that translate JS
  `KeyboardEvent.code` to the re-exported `Key` enum and push into the
  engine's `keys` / `keys_pressed` vecs. (Needs a mapping table; `minifb`
  doesn't give one for free.)
- Register a `requestAnimationFrame` closure that, each frame:
  - Computes `dt` from `performance.now()` (or `web_time::Instant`).
  - Calls `scene.update(&mut engine, dt, t)`.
  - Draws the optional FPS overlay if `engine.debug`.
  - Walks `engine.buffer.pixels` and writes RGBA bytes into the scratch buffer
    (swap from packed `0RGB`).
  - Wraps it as `ImageData` and calls `ctx.put_image_data(...)`.
  - Re-schedules itself unless `engine.exit` is set.

### What it pulls into the `argh` crate

Keep this clean with a `web` cargo feature mirroring `desktop`:

```toml
[features]
default = ["desktop"]
desktop = ["dep:minifb"]
web = [
  "dep:wasm-bindgen",
  "dep:js-sys",
  "dep:web-sys",
  "dep:web-time",
  "dep:console_error_panic_hook",
]

[dependencies]
minifb = { version = "0.28", optional = true }
wasm-bindgen = { version = "0.2", optional = true }
js-sys = { version = "0.3", optional = true }
web-time = { version = "1", optional = true }
console_error_panic_hook = { version = "0.1", optional = true }

[dependencies.web-sys]
version = "0.3"
optional = true
features = [
  "Window",
  "Document",
  "HtmlCanvasElement",
  "CanvasRenderingContext2d",
  "ImageData",
  "KeyboardEvent",
  "Performance",
]
```

A new `engine/start_web.rs` lives behind `#[cfg(feature = "web")]` and
contains all the rAF + canvas + key-mapping logic. `engine/mod.rs` only adds
one new `pub fn start_wasm(...)` gated by the same cfg.

The desktop build is unaffected: no new transitive deps, no codegen changes.

### Suggested API shape

```rust
// new, sits next to `start`
#[cfg(feature = "web")]
pub fn start_wasm<S: Scene + 'static>(
    self,
    canvas_id: &str,
    scene: S,
) -> Result<(), JsValue>;
```

Notes on the signature:

- `S: 'static` is forced: the scene is moved into a closure that outlives the
  call. Same for the engine itself.
- Return `Result<_, JsValue>` so canvas lookup failures bubble up to the JS
  console naturally rather than panicking.
- Take `canvas_id: &str` so the public API doesn't leak `web-sys` types into
  user code. Optional: a second overload that takes the
  `HtmlCanvasElement` directly for advanced users.
- Don't try to mirror `start()`'s signature exactly. The desktop one blocks,
  this one returns immediately. That's a real semantic difference and it's
  fine to surface it in the name.

Internally the standard rAF dance is unavoidable:

```text
let engine = Rc::new(RefCell::new(self));
let scene  = Rc::new(RefCell::new(scene));
let cb     = Rc::new(RefCell::new(None));
let cb2    = cb.clone();
*cb.borrow_mut() = Some(Closure::wrap(Box::new(move || {
    // borrow, tick, blit, re-schedule cb2
}) as Box<dyn FnMut()>));
window.request_animation_frame(
    cb.borrow().as_ref().unwrap().as_ref().unchecked_ref(),
)?;
Ok(())
```

That `Rc<RefCell<Option<Closure>>>` pattern is the main reason this is worth
hiding in the library: every user would otherwise reinvent it badly.

### Things that get easier

- Centralised key-mapping table from JS `KeyboardEvent.code` → `minifb::Key`.
  One place to maintain, every web user benefits.
- The `0RGB → RGBA` swap happens once, correctly, in optimised Rust, not in
  hand-rolled JS.
- `engine.debug` FPS overlay just works on web with no extra effort.
- `engine.stop()` can short-circuit rAF rescheduling, so users get the same
  shutdown semantics on both targets.
- `web-time::Instant` can be used everywhere and unify the timing code with
  `start()`, removing the only other `std::time::Instant` usage.

### New responsibilities for the crate

- It now owns a small `web-sys` surface and a key-mapping table. Both are
  stable but they're maintenance.
- Panics in `scene.update` will tear down the rAF loop silently unless
  `console_error_panic_hook` is installed at the top of `start_wasm`. Do that,
  it's one line and saves users hours.
- Error handling shifts from "panic on `Window::new`" to "return `Result`".
  The two `start*` functions end up with different return types. Live with it.
- If users want to embed the canvas inside a larger app and drive their own
  loop, `start_wasm` is too opinionated. Keep the Tier 1 seams (`tick()`,
  `pixels()`, `set_keys()`) **as well**, so power users can opt out. The cost
  is tiny, the value is large.

## Web-specific gotchas (apply to both tiers)

- **Pixel format conversion:** the current packed `0RGB` is little-endian
  `u32`, so in memory it's `B G R 0`. Easiest is a small loop that writes
  `r, g, b, 255` per pixel into an RGBA scratch buffer. Don't be tempted to
  hand the `u32` buffer straight to `ImageData`, the channel order won't match.
- **`minifb::Scale`** is meaningless in a browser. Size the canvas via CSS or
  the `<canvas width height>` attributes; render at the engine's logical size
  and let the browser scale.
- **High-DPI:** render 1:1 (matches the desktop `minifb` behaviour) and let
  users add CSS scaling. Document it.
- **Canvas resize:** ignore by default. The desktop `start()` disables resize
  too, so the behaviour matches.
- **Focus:** the canvas needs `tabindex="0"` for keyboard events, or listen on
  `window`. Listening on `window` is friendlier; do that and document that the
  canvas doesn't need focus.
- **Page visibility:** when the tab is backgrounded, `requestAnimationFrame`
  is throttled to ~1 Hz. `dt` will spike. Either cap `dt` to e.g. 0.1s inside
  `start_wasm`, or document the behaviour and leave it to scenes.
- **No threads, no `rayon`, no file I/O.** Anything loaded (textures, OBJs
  later) needs to come from `fetch` or `include_bytes!`, not `std::fs`.
- **`println!`** goes via `console.log` only if `console_error_panic_hook` and
  `wasm-bindgen`'s log macros are wired up. The existing "no lights" print in
  `Engine::start()` is fine because it's behind `#[cfg(feature = "desktop")]`
  under this plan.

## Recommendation

Do both tiers, in this order:

1. Land the `desktop` / `web` feature split and the `tick()` / `pixels()` /
   `set_keys()` accessors first. That's the minimal-change tier and unblocks
   any user who wants to embed `argh` in a larger web app.
2. Then add `start_wasm("canvas-id", scene)` in a separate file behind
   `#[cfg(feature = "web")]` that just composes those primitives plus the
   rAF + key-mapping glue.

End result: a one-line user API on both platforms, no corner painted for
users who want a custom host, and the desktop build stays exactly as it is
today.
