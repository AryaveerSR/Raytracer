//! Structures representing a point and vector in 3D space.

use super::Interval;
use rand::Rng;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Debug, Copy)]
pub struct Vec3(f64, f64, f64);

// A point and a vector are basically the same, and are often used interchangeably,
// but its still good to have different types for code clarity.
pub type Point3 = Vec3;

const NEAR_ZERO_OFFSET: f64 = 1e-8; // 10^-8

impl Vec3 {
    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.0.powi(2) + self.1.powi(2) + self.2.powi(2)
    }

    pub fn dot(&self, rhs: Vec3) -> f64 {
        (self.0 * rhs.0) + (self.1 * rhs.1) + (self.2 * rhs.2)
    }

    pub fn cross(&self, rhs: Vec3) -> Vec3 {
        Vec3(
            (self.1 * rhs.2) - (self.2 * rhs.1),
            (self.0 * rhs.2) - (self.2 * rhs.0),
            (self.0 * rhs.1) - (self.1 * rhs.0),
        )
    }

    /// Check if the vector is almost zero.
    ///
    /// This is used when calculating the scattered rays.
    pub fn near_zero(&self) -> bool {
        (self.0.abs() < NEAR_ZERO_OFFSET)
            & (self.1.abs() < NEAR_ZERO_OFFSET)
            & (self.2.abs() < NEAR_ZERO_OFFSET)
    }

    /// Both min and max are **inclusive**
    pub fn random(interval: Interval) -> Vec3 {
        let mut rng = rand::thread_rng();

        let range = interval.to_range();

        // Cloning a range is zero-cost.
        Vec3(
            rng.gen_range(range.clone()),
            rng.gen_range(range.clone()),
            rng.gen_range(range),
        )
    }

    pub fn unit_vec(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn new<X: Into<f64>, Y: Into<f64>, Z: Into<f64>>(x: X, y: Y, z: Z) -> Self {
        Vec3(x.into(), y.into(), z.into())
    }
}

// Macros:
// Usage: `vec3!(0,0,0)`.

#[macro_export]
macro_rules! vec3 {
    ($x:expr, $y:expr, $z:expr) => {
        $crate::structs::Vec3::new($x, $y, $z)
    };
}

#[macro_export]
macro_rules! point3 {
    ($x:expr, $y:expr, $z:expr) => {
        $crate::structs::Point3::new($x, $y, $z)
    };
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl<T: Into<f64>> Add<T> for Vec3 {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self(self.0 + rhs, self.1 + rhs, self.2 + rhs)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl<T: Into<f64>> Sub<T> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self(self.0 - rhs, self.1 - rhs, self.2 - rhs)
    }
}

impl Mul<Self> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl<T: Into<f64>> Mul<T> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: T) -> Self::Output {
        let scalar = scalar.into();
        Self(self.0 * scalar, self.1 * scalar, self.2 * scalar)
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}

impl<T: Into<f64>> Div<T> for Vec3 {
    type Output = Self;

    fn div(self, scalar: T) -> Self::Output {
        let scalar = scalar.into();
        Self(self.0 / scalar, self.1 / scalar, self.2 / scalar)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
    }
}

impl<T: Into<f64>> MulAssign<T> for Vec3 {
    fn mul_assign(&mut self, scalar: T) {
        let scalar = scalar.into();
        self.0 *= scalar;
        self.1 *= scalar;
        self.2 *= scalar;
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
        self.1 /= rhs.1;
        self.2 /= rhs.2;
    }
}

impl<T: Into<f64>> DivAssign<T> for Vec3 {
    fn div_assign(&mut self, scalar: T) {
        let scalar = scalar.into();
        self.0 /= scalar;
        self.1 /= scalar;
        self.2 /= scalar;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

//? The index way of accessing x,y and z values results
//? in un-idiomatic code. Use `.x()`, `.y()` or `.z()` instead.
/*
impl Index<u8> for Vec3 {
    type Output = f64;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Out of bounds"),
        }
    }
}
*/
