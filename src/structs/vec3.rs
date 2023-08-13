//! Structures representing a point and vector in 3D space.

use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Debug, Copy)]
pub struct Vec3(f64, f64, f64);

pub type Point3 = Vec3;

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

    pub fn unit_vec(&self) -> Vec3 {
        self.clone() / self.length()
    }

    pub fn new<X: Into<f64>, Y: Into<f64>, Z: Into<f64>>(x: X, y: Y, z: Z) -> Self {
        Vec3(x.into(), y.into(), z.into())
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            0: self.0 + rhs.0,
            1: self.1 + rhs.1,
            2: self.2 + rhs.2,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            0: self.0 - rhs.0,
            1: self.1 - rhs.1,
            2: self.2 - rhs.2,
        }
    }
}

impl Mul<Self> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            0: self.0 * rhs.0,
            1: self.1 * rhs.1,
            2: self.2 * rhs.2,
        }
    }
}

impl<T: Into<f64>> Mul<T> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: T) -> Self::Output {
        let scalar = scalar.into();
        Self {
            0: self.0 * scalar,
            1: self.1 * scalar,
            2: self.2 * scalar,
        }
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            0: self.0 / rhs.0,
            1: self.1 / rhs.1,
            2: self.2 / rhs.2,
        }
    }
}

impl<T: Into<f64>> Div<T> for Vec3 {
    type Output = Self;

    fn div(self, scalar: T) -> Self::Output {
        let scalar = scalar.into();
        Self {
            0: self.0 / scalar,
            1: self.1 / scalar,
            2: self.2 / scalar,
        }
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
        Self {
            0: -self.0,
            1: -self.1,
            2: -self.2,
        }
    }
}

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

#[macro_export]
macro_rules! vec3 {
    ($x:expr, $y:expr, $z:expr) => {
        Vec3::new($x, $y, $z)
    };
}

#[macro_export]
macro_rules! point3 {
    ($x:expr, $y:expr, $z:expr) => {
        Point3::new($x, $y, $z)
    };
}