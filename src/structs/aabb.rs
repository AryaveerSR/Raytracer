use std::ops::{Add, AddAssign};

use crate::{
    interval,
    structs::{Interval, Point3, Ray},
};

#[derive(Debug, Default)]
pub struct AABB {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn from_points(a: Point3, b: Point3) -> Self {
        Self {
            x: interval!(a.x().min(b.x()), a.x().max(b.x())),
            y: interval!(a.y().min(b.y()), a.y().max(b.y())),
            z: interval!(a.z().min(b.z()), a.z().max(b.z())),
        }
    }

    pub fn from_aabb(a: Self, b: Self) -> Self {
        Self {
            x: Interval::from_intervals(a.x, b.x),
            y: Interval::from_intervals(a.y, b.y),
            z: Interval::from_intervals(a.z, b.z),
        }
    }

    pub fn axis<T: Into<u8>>(&self, axis: T) -> Interval {
        match axis.into() {
            0 => self.x,
            1 => self.y,
            _ => self.z,
        }
    }

    pub fn x(&self) -> Interval {
        self.x
    }

    pub fn y(&self) -> Interval {
        self.y
    }

    pub fn z(&self) -> Interval {
        self.z
    }

    pub fn hit(&self, ray: Ray, time: Interval) -> bool {
        for i in 0..3 {
            /* let t0 = ((self[i].min - ray.origin()[i]) / ray.direction()[i])
                .min((self[i].max - ray.origin()[i]) / ray.direction()[i]);

            let t0 = ((self[i].min - ray.origin()[i]) / ray.direction()[i])
                .max((self[i].max - ray.origin()[i]) / ray.direction()[i]); */

            let inv_d = 1.0 / ray.direction().axis(i);
            let origin = ray.origin().axis(i);

            let mut t0 = (self.axis(i).min - origin) * inv_d;
            let mut t1 = (self.axis(i).max - origin) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            if time.max.min(t1) <= time.min.max(t0) {
                return false;
            }
        }

        true
    }
}

impl Add for AABB {
    type Output = AABB;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_aabb(self, rhs)
    }
}

impl AddAssign for AABB {
    fn add_assign(&mut self, rhs: Self) {
        self.x = Interval::from_intervals(self.x, rhs.x);
        self.y = Interval::from_intervals(self.y, rhs.y);
        self.z = Interval::from_intervals(self.z, rhs.z);
    }
}

impl AddAssign<&AABB> for AABB {
    fn add_assign(&mut self, rhs: &AABB) {
        self.x = Interval::from_intervals(self.x, rhs.x);
        self.y = Interval::from_intervals(self.y, rhs.y);
        self.z = Interval::from_intervals(self.z, rhs.z);
    }
}
