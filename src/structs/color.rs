//! A structure representing a color.

use std::ops::{Add, Mul};

#[derive(Clone, Copy, Debug)]
pub struct Color(u32);

impl Color {
    pub const WHITE: Color = Color { 0: 0x00FFFFFF };
    pub const RED: Color = Color { 0: 0x00FF0000 };
    pub const BLUE: Color = Color { 0: 0x000B0BFF };
    pub const BLACK: Color = Color { 0: 0 };

    pub fn r(&self) -> u8 {
        ((self.0 & 0x00FF0000) >> 16) as u8
    }

    pub fn g(&self) -> u8 {
        ((self.0 & 0x0000FF00) >> 8) as u8
    }

    pub fn b(&self) -> u8 {
        (self.0 & 0x000000FF) as u8
    }

    pub fn as_string(&self) -> String {
        format!("{} {} {}", self.r(), self.g(), self.b())
    }

    pub fn new<X: Into<u32>, Y: Into<u32>, Z: Into<u32>>(r: X, g: Y, b: Z) -> Self {
        Color((r.into() << 16) | (g.into() << 8) | b.into())
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Color) -> Self::Output {
        Color::new(self.r() + rhs.r(), self.g() + rhs.g(), self.b() + rhs.b())
    }
}

impl<T: Into<u8>> Add<T> for Color {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Color::new(self.r() + rhs, self.g() + rhs, self.b() + rhs)
    }
}

impl<T: Into<f64>> Mul<T> for Color {
    type Output = Self;

    fn mul(self, scalar: T) -> Self::Output {
        let scalar = scalar.into();
        Color::new(
            (self.r() as f64 * scalar) as u8,
            (self.g() as f64 * scalar) as u8,
            (self.b() as f64 * scalar) as u8,
        )
    }
}
