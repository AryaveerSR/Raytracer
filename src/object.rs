//! Structures for "objects" in a scene, and the scene itself.

use crate::{
    structs::{Interval, Point3, Ray, Vec3},
    vec3,
};

#[derive(Copy, Clone)]
pub struct HitData {
    point: Point3,
    normal: Vec3,
    time: f64,
    is_front_face: bool,
}

impl HitData {
    pub fn new(point: Point3, time: f64) -> Self {
        HitData {
            point,
            time,
            normal: vec3!(0, 0, 0),
            is_front_face: true,
        }
    }

    pub fn point(&self) -> Point3 {
        self.point
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    fn set_face_and_normal(&mut self, ray: Ray, outward_normal: Vec3) {
        let is_front_face = ray.direction().dot(outward_normal) < 0.0;
        self.is_front_face = is_front_face;

        self.normal = if is_front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

pub trait Object {
    fn does_hit(&self, ray: Ray, interval: Interval) -> Option<HitData>;
}

pub struct Scene {
    objects: Vec<Box<dyn Object>>,
}

impl Scene {
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, obj: Box<dyn Object>) {
        self.objects.push(obj);
    }

    pub fn does_hit(&self, ray: Ray, interval: Interval) -> Option<HitData> {
        let mut hit_data: Option<HitData> = None;
        let mut closest = interval.max + 1.0;

        for obj in &self.objects {
            match obj.does_hit(ray, interval) {
                Some(hit) => {
                    if hit.time < closest {
                        hit_data = Some(hit);
                        closest = hit.time;
                    }
                }
                None => continue,
            }
        }

        hit_data
    }

    pub fn new() -> Self {
        Scene { objects: vec![] }
    }
}

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new<T: Into<f64>>(center: Point3, radius: T) -> Self {
        Sphere {
            center,
            radius: radius.into(),
        }
    }

    pub fn random_unit_vector(normal: Vec3) -> Vec3 {
        let vector = loop {
            let p = Vec3::random(Interval::new(-1, 1));
            if p.length_squared() < 1.0 {
                break p.unit_vec();
            }
        };

        if vector.dot(normal) > 0.0 {
            return vector;
        } else {
            return -vector;
        }
    }
}

impl Object for Sphere {
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
            let mut hit_data = HitData::new(ray.at(root), root);

            hit_data.set_face_and_normal(ray, normal);
            return Some(hit_data);
        }

        None
    }
}
