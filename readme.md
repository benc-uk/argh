# ARGH: Another Rust Graphics Helper

ARGH is a learning project to build a software renderer in Rust. It is not intended to be a full-featured game engine, but rather a simple framework for experimenting with graphics programming concepts.

## Usage

To use ARGH, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
argh = "0.1"
```

Then, you can create a simple application by implementing the `Scene` trait and starting the engine:

```rust
use argh::core::{Engine, Scene};

struct MyScene {}
impl Scene for MyScene {
    fn update(&mut self, e: &mut Engine, _: f64) {
        // Update & draw here
    }
}

fn main() {
    let eng = Engine::new(800, 600, String::from("Hello World"));
    eng.start(MyScene {});
}
```

## Features

- Software rendering to a window
- Simple scene management
- Easy to extend with custom rendering logic

## Reference

- to be added

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
