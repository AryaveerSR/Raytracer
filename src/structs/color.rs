//! A structure representing a color.

//todo! Sort this mess
//todo! decide on a data type for r,g and b.
//todo! camera needs f64 ( see tuple mess )
use std::ops::{Add, AddAssign, Div, Mul};

#[derive(Clone, Copy, Debug)]
pub struct Color {
    r: i32,
    g: i32,
    b: i32,
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
        r: 128,
        g: 180,
        b: 255,
    };

    pub fn r(&self) -> i32 {
        self.r
    }

    pub fn g(&self) -> i32 {
        self.g
    }

    pub fn b(&self) -> i32 {
        self.b
    }

    pub fn as_string(&self) -> String {
        format!("{} {} {}", self.r(), self.g(), self.b())
    }

    pub fn new<X: Into<i32>, Y: Into<i32>, Z: Into<i32>>(r: X, g: Y, b: Z) -> Self {
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

impl<T: Into<i32>> Add<T> for Color {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Color::new(self.r() + rhs, self.g() + rhs, self.b() + rhs)
    }
}

impl<T: Into<i32>> AddAssign<T> for Color {
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
            (self.r() as f64 * scalar) as i32,
            (self.g() as f64 * scalar) as i32,
            (self.b() as f64 * scalar) as i32,
        )
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, color: Color) -> Self::Output {
        Color::new(
            (self.r() as f64 * color.r() as f64) as i32,
            (self.g() as f64 * color.g() as f64) as i32,
            (self.b() as f64 * color.b() as f64) as i32,
        )
    }
}

impl<T: Into<f64>> Div<T> for Color {
    type Output = Self;

    fn div(self, scalar: T) -> Self::Output {
        let scalar = scalar.into();
        Color::new(
            (self.r() as f64 / scalar) as i32,
            (self.g() as f64 / scalar) as i32,
            (self.b() as f64 / scalar) as i32,
        )
    }
}

#[macro_export]
macro_rules! color {
    ($r:expr, $g:expr, $b:expr) => {
        crate::structs::Color::new($r, $g, $b)
    };
}
