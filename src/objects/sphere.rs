use std::rc::Rc;

use super::{HitData, Object};
use crate::{
    materials::Material,
    structs::{Interval, Point3, Ray},
};

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new<T: Into<f64>>(center: Point3, radius: T, material: Rc<dyn Material>) -> Self {
        Sphere {
            center,
            radius: radius.into(),
            material,
        }
    }
}

impl Object for Sphere {
    fn material(&self) -> Rc<dyn Material> {
        self.material.clone()
    }

    fn does_hit(&self, ray: Ray, interval: Interval) -> Option<HitData> {
        let distance = ray.origin() - self.center;

        let a = ray.direction().length_squared();
        let b = distance.dot(ray.direction());
        let c = distance.length_squared() - self.radius.powi(2);
        let discriminant = b.powi(2) - a * c;

        if discriminant >= 0.0 {
            let underroot_d = discriminant.sqrt();
            let mut root = (-b - underroot_d) / a;

            if interval.excludes(root) {
                root = (-b + underroot_d) / a;

                if interval.excludes(root) {
                    return None;
                }
            }

            let normal = (ray.at(root) - self.center) / self.radius;
            let mut hit_data = HitData::new(ray.at(root), root, self.material.clone());

            hit_data.set_face_and_normal(ray, normal);
            return Some(hit_data);
        }

        None
    }
}
