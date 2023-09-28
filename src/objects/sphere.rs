//! A sphere geometry for an object.

use super::{HitData, Object, AABB};
use crate::{
    materials::Material,
    structs::{Interval, Point3, Ray, Vec3},
    vec3, SHUTTER_OPEN_DURATION,
};
use std::sync::Arc;

/// The main structure defining a sphere, with a center, radius, and the material.
#[derive(Debug)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    velocity: Vec3,
    bounding_box: AABB,
    material: Arc<dyn Material + Sync + Send>,
}

impl Sphere {
    pub fn new<T: Into<f64>>(
        center: Point3,
        radius: T,
        material: Arc<dyn Material + Sync + Send>,
        velocity: Vec3,
    ) -> Self {
        //todo! comments

        let radius = radius.into();
        let r_vec = vec3!(radius, radius, radius);

        let initial_box = AABB::from_points(center - r_vec, center + r_vec);

        let final_center = center + velocity * *SHUTTER_OPEN_DURATION.get().unwrap();
        let final_box = AABB::from_points(final_center - r_vec, final_center + r_vec);

        Sphere {
            center,
            radius,
            material,
            velocity,
            bounding_box: initial_box + final_box,
        }
    }
}

impl Object for Sphere {
    fn material(&self) -> Arc<dyn Material + Sync + Send> {
        Arc::clone(&self.material)
    }

    //todo! comments
    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }

    /// Calculating whether a ray hits the sphere.
    fn does_hit(&self, ray: Ray, interval: Interval, time: f64) -> Option<HitData> {
        let center = self.center + self.velocity * time;
        let distance = ray.origin() - center;

        let a = ray.direction().length_squared();
        let b = distance.dot(ray.direction());
        let c = distance.length_squared() - self.radius.powi(2);
        let discriminant = b.powi(2) - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let underroot_d = discriminant.sqrt();
        let mut root = (-b - underroot_d) / a;

        if interval.excludes(root) {
            root = (-b + underroot_d) / a;

            if interval.excludes(root) {
                return None;
            }
        }

        let outward_normal = (ray.at(root) - center) / self.radius;

        let is_front_face = ray.direction().dot(outward_normal) < 0.0;
        let normal = match is_front_face {
            true => outward_normal,
            false => -outward_normal,
        };

        let hit_data = HitData::new(
            ray.at(root),
            root,
            self.material.clone(),
            is_front_face,
            normal,
        );

        Some(hit_data)
    }
}
