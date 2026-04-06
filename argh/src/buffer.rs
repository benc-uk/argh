use crate::colour::Colour;

pub struct Buffer {
    pub(crate) pixels: Vec<u32>,
    w: usize,
    h: usize,
}

impl Buffer {
    pub(crate) fn new(w: usize, h: usize) -> Self {
        Self { pixels: vec![0; w * h], w, h }
    }

    pub(crate) fn clear(&mut self, colour: Colour) {
        self.pixels.fill(colour.as_u32());
    }

    pub(crate) fn set_pixel(&mut self, x: usize, y: usize, c: Colour) {
        if x < self.w && y < self.h {
            self.pixels[y * self.w + x] = c.as_u32();
        }
    }
}
