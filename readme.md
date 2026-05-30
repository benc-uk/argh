# ARGH: Another Rust Graphics Helper

[![CI](https://img.shields.io/github/actions/workflow/status/benc-uk/argh/ci.yml?label=CI)](https://github.com/benc-uk/argh/actions/workflows/ci.yml)
[![Docs](https://img.shields.io/github/actions/workflow/status/benc-uk/argh/docs.yml?label=Docs)](https://github.com/benc-uk/argh/actions/workflows/docs.yml)
[![License: MIT](https://img.shields.io/github/license/benc-uk/argh)](https://github.com/benc-uk/argh/blob/main/LICENSE)
[![Rust](https://img.shields.io/badge/rust-2024_edition-orange?logo=rust)](https://www.rust-lang.org/)
[![GitHub last commit](https://img.shields.io/github/last-commit/benc-uk/argh)](https://github.com/benc-uk/argh/commits/main)

ARGH is a learning project to build a software renderer in Rust. It is not intended to be a full-featured game engine, but rather a simple framework for experimenting with graphics programming concepts.

**It is purposely being developed without the use of AI coding assistants, code is written by hand in the traditional way**

Features:

- Window and framebuffer backed by [minifb](https://docs.rs/minifb/latest/minifb/)
- Entirely software (CPU) based rendering loop and buffer operations
- Core maths libraries for vectors, matrices and quaternions, implemented from scratch
- Simple scene management
- 3D rendering
  - Matrix operations for affine transforms
  - Rendering pipeline for meshes, with z-buffering and basic clipping (no Sutherland-Hodgman polygon clipping yet)
  - Diffuse illumination and Gouraud shading + texture mapping
  - Cameras with perspective projection
  - Simple meshes, materials and textures
    - Textures can be image based texture maps or simple solid colours
  - Generators for cubes and spheres, and teapots (what graphics system would be complete without the classic Newell Teapot!)
- Methods for drawing 2D primitives, pixels, lines and text

## Examples

<video src="https://github.com/user-attachments/assets/d7a717f7-1ad2-4392-9c1d-51f683e005f0" controls></video>

See the [`examples/`](./examples) directory for runnable demos (`basic1`, `hello_world`, `poly_2d`, `rects`, `simple_3d`).

## Usage

ARGH is not currently published to crates.io. Add it as a git dependency in your `Cargo.toml`:

```toml
[dependencies]
argh = { git = "https://github.com/benc-uk/argh" }
```

Then create a simple application by implementing the `App` trait and `update()` method, and creating & starting the engine (with an instance of your app):

```rust
use argh::colour::BLUE;
use argh::engine::{Engine, App};

struct MyApp {}

impl App for MyApp {
  fn update(&mut self, e: &mut Engine, dt: f64, t: f64) {
    e.clear(BLUE);
    // Draw the rest of your frame here
  }
}

fn main() {
  let mut eng = Engine::new(800, 600);
  let mut app = MyApp {};
  eng.start_window(&mut app, "Hello World", 1);
}
```

## Reference

- [Library API reference docs here](https://code.benco.io/argh/argh/index.html)

## Technical Notes

Graphics & 3D conventions followed internally by this engine are mostly the same as OpenGL, except clip space:

- Screen space has [x: 0, y:0] as top-left corner of the viewport. So Y increases downward
- We use a right handed coordinate system
  - Camera will be looking down the negative Z-axis; -Z is further away, +Z closer (or behind)
- CCW for vertices in triangle meshes
- Clip space z range is [0, +w] (NDC z is [0, +1] after perspective divide), unlike OpenGL's [-w, +w] / [-1, +1]

## Building and Running Locally

- Have Rust & Cargo installed
- Don't be on Windows (generally good advice). minifb works best on Linux & macOS; for Windows builds use the `build-win` make target to cross-compile via the `x86_64-pc-windows-gnu` toolchain
- Run `make`

```
  🎮 Argh Engine

  build-win       🔨 Build all crates for Windows x64
  build           🔨 Build all crates
  check           ✅ Type check all crates
  clean           🗑️ Clean build artefacts
  clippy          📎 Run clippy lints
  doc-open        📖 Generate and open documentation
  doc             📚 Generate documentation
  fmt-check       🔍 Check formatting (CI)
  fmt             🎨 Format all code
  help            💡 Show this help message
  lint            🧹 Run all lints (fmt + clippy)
  release-win     🚀 Build all crates for Windows x64 (release)
  release         🚀 Build all crates (release)
  run-example     🚀 Run an example as a desktop app
  site            📚 Build the project site combining docs and WASM example(s)
  test            🧪 Run all tests
  wasm-build      🕸️  Build the web_wasm example with wasm-pack
  wasm-serve      🌐 Build and serve the web_wasm example on http://localhost:8000
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
