//! `structs.rs`
//! Some generally helpful structures

/// A Color struct with some helpful functions.
#[derive(Clone, Debug)]
pub struct Color(pub u32);

impl Color {
    pub const WHITE: Color = Color { 0: 0x00FFFFFF };
    pub const BLACK: Color = Color { 0: 0 };

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Color(((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
    }

    pub fn r(&self) -> u8 {
        ((self.0 & 0x00FF0000) >> 16) as u8
    }

    pub fn g(&self) -> u8 {
        ((self.0 & 0x0000FF00) >> 8) as u8
    }

    pub fn b(&self) -> u8 {
        (self.0 & 0x000000FF) as u8
    }
}

impl TryFrom<u32> for Color {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(Color { 0: value })
    }
}
