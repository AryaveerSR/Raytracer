use crate::vec3::{Point3, Vec3};

/// A struct representing a ray
#[derive(Clone, Debug)]
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

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }

    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }
}
