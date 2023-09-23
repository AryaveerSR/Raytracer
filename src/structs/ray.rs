//! A structure representing a ray in 3D space.

use crate::structs::vec3::{Point3, Vec3};

/// A ray with an origin point and a direction.
#[derive(Clone, Debug, Copy)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    /// Find the position the ray's head is at at a certain point in time.
    pub fn at<T: Into<f64>>(&self, t: T) -> Point3 {
        self.origin + self.direction * t.into()
    }

    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }
}
