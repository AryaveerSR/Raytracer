//! A structure representing a color.

use std::ops::{Add, AddAssign, Mul};

#[derive(Clone, Copy, Debug)]
pub struct Color {
    r: u16,
    g: u16,
    b: u16,
}

impl Color {
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
    };
    pub const RED: Color = Color { r: 255, g: 0, b: 0 };
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };
    pub const BLUE: Color = Color {
        r: 20,
        g: 20,
        b: 255,
    };

    pub fn r(&self) -> u16 {
        self.r
    }

    pub fn g(&self) -> u16 {
        self.g
    }

    pub fn b(&self) -> u16 {
        self.b
    }

    pub fn as_string(&self) -> String {
        format!("{} {} {}", self.r(), self.g(), self.b())
    }

    pub fn new<X: Into<u16>, Y: Into<u16>, Z: Into<u16>>(r: X, g: Y, b: Z) -> Self {
        Color {
            r: r.into(),
            g: g.into(),
            b: b.into(),
        }
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Color) -> Self::Output {
        Color::new(self.r() + rhs.r(), self.g() + rhs.g(), self.b() + rhs.b())
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Color) {
        *self = Color {
            r: self.r() + rhs.r(),
            g: self.g() + rhs.g(),
            b: self.b() + rhs.b(),
        };
    }
}

impl<T: Into<u16>> Add<T> for Color {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Color::new(self.r() + rhs, self.g() + rhs, self.b() + rhs)
    }
}

impl<T: Into<u16>> AddAssign<T> for Color {
    fn add_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        *self = Color {
            r: self.r() + rhs,
            g: self.g() + rhs,
            b: self.b() + rhs,
        }
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
