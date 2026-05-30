# Engine owns the Scene: framework-style refactor

Design notes for letting `Engine` hold its own `Scene` so the user no longer
passes it into every entry point, and so scenes can be swapped at runtime.

> Scope: this is the alternative shape to `scene-refactor.md`. That doc moves
> world state out of `Engine`. This doc keeps the current ownership of world
> state and changes who holds the `Scene`. Pick one; the two are not meant to
> be composed.

## Motivation

Today `Scene` is a trait the user implements, and the engine never holds it.
The user passes their scene into both entry points:

```rust
engine.tick(&mut scene, dt);          // WASM
engine.start_window(scene, title, scale);  // desktop
```

This works, but it has two friction points:

1. The scene is a function parameter on every frame entry, even though it is
   conceptually a long-lived part of the running engine. The API leaks
   ownership noise that ebiten / Phaser / similar frameworks hide.
2. There is no way to swap scenes at runtime without the user writing their
   own "current scene" wrapper. Common use case (menu, game, pause) needs
   bespoke plumbing.

The proposed shape: `Engine` holds an optional boxed `dyn Scene` and drives
its `update()` itself. User calls `engine.set_scene(...)` once (or any time
later) and `engine.start_window("title", scale)` with no scene parameter.
Mental model is ebiten's `RunGame(&Game{})`.

## Two compile-time problems and their fixes

### 1. `dyn Scene` is unsized, can't be a struct field directly

`scene: dyn Scene` does not compile because the compiler doesn't know the
size of the concrete type. We need indirection: `Box<dyn Scene>` puts the
scene on the heap and gives us a fat pointer (data + vtable), which is
sized.

Use `Option<Box<dyn Scene>>` rather than just `Box<dyn Scene>` for two
reasons:

- `Engine::new` can return an engine before any scene is set.
- `Option::take` solves the borrow-checker problem below cleanly.

Go analogy: a Go interface value is already a fat pointer with implicit heap
allocation. `Box<dyn Scene>` is just that, made explicit. JS analogy: storing
an object reference on a field; the box is what every JS object effectively
has anyway.

### 2. `fn new(e: &mut Engine) -> Self` makes `Scene` not dyn-compatible

A trait method that returns `Self` requires knowing the concrete size at the
call site, which a vtable can't provide. The whole trait becomes unusable
as `dyn Scene` unless we exclude that method from the vtable:

```rust
pub trait Scene {
    fn update(&mut self, engine: &mut Engine, dt: f64, t: f64);
    fn new(e: &mut Engine) -> Self where Self: Sized;
}
```

The `where Self: Sized` clause says "this method only exists for concrete
types; don't try to virtualise it". Existing user code does not need to
change; the constraint is on the trait, not the impl.

## The borrow-checker problem during `update`

Once `scene` is a field on `Engine`, the naive call

```rust
self.scene.update(self, dt, t);
```

is a double mutable borrow of `self` (once to reach `.scene`, once as the
argument). Rust rejects it. Solution: move the scene out for the call, put
it back after.

```rust
pub fn tick(&mut self, dt: f64) {
    self.t += dt;
    self.fps = 1.0 / dt;
    self.last_time = Instant::now();
    let t = self.t;

    if let Some(mut scene) = self.scene.take() {
        scene.update(self, dt, t);
        // If update() called set_scene(), keep the new one.
        if self.scene.is_none() {
            self.scene = Some(scene);
        }
    }

    if self.debug {
        self.draw_string(&format!("FPS: {:.2}", self.fps), 2, 2, BLACK);
        self.draw_string(&format!("FPS: {:.2}", self.fps), 1, 1, WHITE);
    }
}
```

The `if self.scene.is_none()` check makes runtime scene-swapping work from
inside `update`: if a scene swapped itself out for a new one, we keep the
new one and drop the old one at end of block. If nothing swapped, the old
one goes back into place.

## API shape

### `Scene` trait

```rust
pub trait Scene {
    fn update(&mut self, engine: &mut Engine, dt: f64, t: f64);
    fn new(e: &mut Engine) -> Self where Self: Sized;
}
```

Only change: `where Self: Sized` on `new`.

### `Engine` struct

```rust
pub struct Engine {
    // ... all existing fields ...
    scene: Option<Box<dyn Scene>>,
}
```

### `Engine::new`

Initialise `scene: None`. No other changes.

### New method: `set_scene`

```rust
impl Engine {
    pub fn set_scene<S: Scene + 'static>(&mut self, scene: S) {
        self.scene = Some(Box::new(scene));
    }
}
```

The `'static` bound is required because `Box<dyn Scene>` is implicitly
`Box<dyn Scene + 'static>`. Scenes cannot borrow shorter-lived data, which
is the right default.

### `Engine::tick`

Signature changes from `tick<S: Scene>(&mut self, scene: &mut S, dt: f64)` to:

```rust
pub fn tick(&mut self, dt: f64);
```

Body uses the take / call / put-back pattern shown above.

### `Engine::start_window`

Signature changes from
`start_window<S: Scene>(mut self, mut scene: S, title: &str, scale: u8)` to:

```rust
#[cfg(feature = "desktop")]
pub fn start_window(mut self, title: &str, scale: u8)
```

Inside, replace `self.tick(&mut scene, dt)` with `self.tick(dt)`. Everything
else (window setup, input pumping, buffer presentation, exit handling)
stays.

## User code change

Before:

```rust
let mut e = Engine::new(320, 240);
let s = MyScene::new(&mut e);
e.start_window(s, "Argh: Hello World", 2);
```

After:

```rust
let mut e = Engine::new(320, 240);
let s = MyScene::new(&mut e);
e.set_scene(s);
e.start_window("Argh: Hello World", 2);
```

One extra line. Runtime swapping then becomes:

```rust
// inside some scene's update():
engine.set_scene(GameScene::new(engine));
// old scene is dropped automatically after update() returns
```

## File-by-file changes

### `argh/src/engine/scene.rs`

Add `where Self: Sized` to `new`. Two-line change.

### `argh/src/engine/mod.rs`

- Add `scene: Option<Box<dyn Scene>>` to `Engine`.
- Initialise `scene: None` in `Engine::new`.
- Add `set_scene<S: Scene + 'static>` method.
- Change `tick` signature: drop the `S: Scene` generic and `scene` parameter;
  rewrite body to use `self.scene.take()` / put-back-if-empty pattern.
- Change `start_window` signature: drop the `S: Scene` generic and `scene`
  parameter; replace `self.tick(&mut scene, dt)` with `self.tick(dt)`.

### `argh/src/lib.rs`

Update the top-of-crate doc example to call `set_scene` before
`start_window`, and remove the `scene` argument from `start_window`.

### `examples/*/src/main.rs`

For each example: replace

```rust
e.start_window(s, "title", scale);
```

with

```rust
e.set_scene(s);
e.start_window("title", scale);
```

Mechanical, one extra line per example.

### `examples/web_wasm/`

Check the WASM glue. Anywhere it calls `engine.tick(&mut scene, dt)`,
replace with `engine.tick(dt)` and ensure `engine.set_scene(scene)` has been
called once during init.

## Borrow-checker sanity

Inside `update`, the user still has `engine: &mut Engine` as a parameter and
`&mut self` for their scene. The engine's `scene` field is `None` for the
duration of the call (we took it out), so there is no aliasing risk. All
existing patterns inside scenes work unchanged:

```rust
fn update(&mut self, e: &mut Engine, dt: f64, _t: f64) {
    e.clear(BLACK);
    e.get_instance_mut(self.player).rot_y(0.5 * dt);
    e.render_all(&self.camera);
}
```

## Trade-offs

| Cost | Why it's acceptable |
| ---- | ------------------- |
| One heap allocation per scene swap (`Box::new`) | Negligible, happens on transitions, not per frame |
| One virtual call per frame (`scene.update`) | Same as Go interface dispatch; invisible at game-loop scale |
| `Scene` becomes `'static`-bound through `Box<dyn Scene + 'static>` | Scenes can't borrow short-lived data, which is the right default anyway |
| Every example needs a one-line addition | Mechanical |

## What this does *not* do

- Does not change where world state lives. `meshes`, `materials`, `instances`,
  `lights` stay on `Engine` exactly as today. If you also want the split from
  `scene-refactor.md`, that is a separate change.
- Does not introduce scene lifecycle hooks (`on_enter`, `on_exit`). Add only
  if a concrete need appears.
- Does not change the input API or any rendering method signatures.
- Does not remove minifb or change the `desktop` feature gating.
