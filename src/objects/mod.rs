use crate::{
    materials::Material,
    structs::{Interval, Point3, Ray, Vec3},
};
use std::sync::Arc;

/// A struct describing a "hit", when a ray hits an object in the scene.
#[derive(Clone)]
pub struct HitData {
    point: Point3,
    normal: Vec3,
    time: f64,
    /// Is this the side facing the camera ?
    /// This is needed for things like refraction in dielectric materials.
    is_front_face: bool,
    pub material: Arc<dyn Material + Sync + Send>,
}

impl HitData {
    pub fn new(
        point: Point3,
        time: f64,
        material: Arc<dyn Material + Sync + Send>,
        is_front_face: bool,
        normal: Vec3,
    ) -> Self {
        HitData {
            point,
            time,
            normal,
            is_front_face,
            material,
        }
    }

    pub fn is_front_face(&self) -> bool {
        self.is_front_face
    }

    pub fn point(&self) -> Point3 {
        self.point
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }
}

/// A trait defining an object, having a material and a method to check if a
/// certain ray hits it or not.
pub trait Object: std::fmt::Debug {
    fn does_hit(&self, ray: Ray, interval: Interval) -> Option<HitData>;
    fn material(&self) -> Arc<dyn Material + Sync + Send>;
}

/// A struct defining the scene.
#[derive(Debug)]
pub struct Scene {
    objects: Vec<Box<dyn Object + Sync + Send>>,
}

impl Scene {
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, obj: Box<dyn Object + Sync + Send>) {
        self.objects.push(obj);
    }

    /// Check if a ray hits any object in the scene.
    pub fn does_hit(&self, ray: Ray, interval: Interval) -> Option<HitData> {
        let mut hit_data: Option<HitData> = None;
        let mut closest = f64::INFINITY; // The first collision is always the closest.

        for obj in &self.objects {
            match obj.does_hit(ray, interval) {
                Some(hit) => {
                    if hit.time < closest {
                        let hit_ = hit.clone();
                        hit_data = Some(hit);
                        closest = hit_.time;
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

mod sphere;

pub use sphere::Sphere;
