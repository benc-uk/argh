use crate::buffer::Buffer;
use crate::colour::Colour;
use minifb::{Key, Window, WindowOptions};
use std::time::Instant;

/// All users of argh are expected to provide their own Scene implementation
pub trait Scene {
    /// This method will be called every frame by the main loop, use it to draw and render your scene
    fn update(&mut self, engine: &mut Engine, dt: f64);
}

/// This is the heart of argh, create an instance of the Engine to use the library
pub struct Engine {
    win_size: (usize, usize),
    win_title: String,
    buffer: Buffer,
    t: f64,
}

impl Engine {
    /// Constructor for a new Engine
    /// # Arguments
    /// * `w` - Width of the window in pixels
    /// * `h` - Height of the window in pixels
    /// * `title` - Title of the window
    pub fn new(w: usize, h: usize, title: String) -> Self {
        Self {
            win_size: (w, h),
            win_title: title,
            buffer: Buffer::new(w, h),
            t: 0.0,
        }
    }

    pub fn clear(&mut self, colour: Colour) {
        self.buffer.clear(colour);
    }

    pub fn get_size(self) -> (usize, usize) {
        return (self.win_size.0, self.win_size.1);
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, c: Colour) {
        self.buffer.set_pixel(x, y, c);
    }

    pub fn start<S: Scene>(mut self, mut scene: S) {
        let mut window = Window::new(&self.win_title, self.win_size.0, self.win_size.1, WindowOptions::default()).unwrap_or_else(|e| {
            panic!("{}", e);
        });

        window.set_target_fps(60);
        let mut last_time = Instant::now();

        while window.is_open() && !window.is_key_down(Key::Escape) {
            let now = Instant::now();
            let dt = now.duration_since(last_time).as_secs_f64();
            self.t = self.t + dt;
            last_time = now;

            scene.update(&mut self, dt);

            let res = window.update_with_buffer(&self.buffer.pixels, self.win_size.0, self.win_size.1);
            if res.is_err() {
                println!("Error updating buffer: {}", res.err().unwrap());
            }
        }
    }
}
