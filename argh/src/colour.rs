#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Colour(u32);

impl Colour {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self((r as u32) << 16 | (g as u32) << 8 | b as u32)
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self((a as u32) << 24 | (r as u32) << 16 | (g as u32) << 8 | b as u32)
    }

    pub const fn r(self) -> u8 {
        (self.0 >> 16) as u8
    }
    pub const fn g(self) -> u8 {
        (self.0 >> 8) as u8
    }
    pub const fn b(self) -> u8 {
        self.0 as u8
    }
    pub const fn a(self) -> u8 {
        (self.0 >> 24) as u8
    }

    pub const fn as_u32(self) -> u32 {
        self.0
    }
}

pub const BLACK: Colour = Colour::new(0, 0, 0);
pub const WHITE: Colour = Colour::new(255, 255, 255);
pub const RED: Colour = Colour::new(255, 0, 0);
pub const GREEN: Colour = Colour::new(0, 255, 0);
pub const BLUE: Colour = Colour::new(0, 0, 255);
