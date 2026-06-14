# Examples Index

Runnable demos and reference apps for the [`argh`](../argh) software-rendering engine. Each example is a standalone crate in the workspace.

## Quick start

```bash
# Desktop examples (any of the names in the table below)
make run-example EXAMPLE=teapots
# or equivalently:
cargo run --bin teapots --release

# WASM example (served at http://localhost:8000)
make wasm-serve
```

The default for `make run-example` is `EXAMPLE=teapots`. Press `Esc` to quit any desktop demo.

## Index

| Example | What it shows | Key APIs |
|---|---|---|
| [`hello_world`](./hello_world) | Smallest possible `App`: clears the screen and draws a text string. Great starting point. | `Engine::new`, `App::update`, `clear`, `draw_string`, `start_window` |
| [`poly_2d`](./poly_2d) | 200 randomly coloured regular polygons spinning around the screen. Pure 2D, no camera or scene. | `Affine2`, `draw_poly_line`, `Colour::rand`, `Vec2` |
| [`simple_3d`](./simple_3d) | Five animated 3D primitives (cube, two spheres, cylinder, cone) with mixed textured and flat materials, lit by three coloured point lights. | `primitives::new_*`, `Material::new_textured` / `new_flat`, `Instance` builder chain (`.rot_y().pos().scale()`), `InstanceHandle` |
| [`teapots`](./teapots) | Three Utah teapots loaded from `.glb` at low/med/high LOD, sitting on a textured cube floor, with an orbiting camera. | `Engine::load_gltf`, `Quat::new` for camera orbit, multiple lights, smooth/flat shading |
| [`scenes`](./scenes) | Two independent `Scene`s (a bobbing sphere and a spinning cube) switched at runtime by pressing Space. | Multiple `Scene` instances, `add_instance_world`, runtime scene switching |
| [`dungeon`](./dungeon) | Mini first-person dungeon walker built from an ASCII map. Walls, floors, barrels, trunks and boxes are static lit geometry; the player carries a flickering torch. | `Scene::add_static`, `bake_static_lighting`, custom `FpsCamera`, dynamic `Light` mutation per frame, keyboard input |
| [`benchmark`](./benchmark) | The dungeon assets rendered from scripted camera positions (teleports at frames 200/400/600/800). Use for performance measurement and regression checks. | Same APIs as `dungeon`, plus scripted camera transitions via `Camera::set_pos` / `set_look_at` |
| [`web_wasm`](./web_wasm) | The teapots demo recompiled for `wasm32-unknown-unknown` and rendered into an HTML canvas. Assets are embedded with `include_bytes!`. | `Engine::load_gltf_bytes`, `Texture::from_bytes`, `Engine::buffer_copy_bytes`, `wasm-bindgen` glue in `lib.rs` |

## Notes on the WASM example

The `web_wasm` crate builds as a `cdylib` and is driven by a small JS host in `index.html`. The Rust side exposes `start`, `update(dt)`, `pixel_ptr`, `width` and `height`; JS allocates an `ImageData` over the shared WASM memory and blits it to a `<canvas>` each frame.

Build and serve:

```bash
make wasm-serve   # builds with wasm-pack then serves at :8000
```

The `argh` crate is pulled in with `default-features = false, features = ["web"]` so the `minifb` desktop backend is excluded.

## Adding a new example

1. Create a new crate under `examples/your_example/` with its own `Cargo.toml`.
2. Add the path to `members = [...]` in the workspace [`Cargo.toml`](../Cargo.toml).
3. Set `[[bin]] name = "your_example"` so `cargo run --bin your_example` works.
4. Add a row to the table above.
