use super::{HitData, Interval, Ray};
use crate::objects::Object;

/// A struct defining the scene.
#[derive(Debug)]
pub struct Scene {
    objects: Vec<Box<dyn Object + Sync + Send>>,
}

impl Scene {
    pub fn add(&mut self, obj: Box<dyn Object + Sync + Send>) {
        self.objects.push(obj);
    }

    /// Check if a ray hits any object in the scene.
    pub fn does_hit(&self, ray: Ray, interval: Interval, time: f64) -> Option<HitData> {
        let mut hit_data: Option<HitData> = None;
        let mut closest = f64::INFINITY; // The first collision should always be the closest.

        for obj in &self.objects {
            match obj.does_hit(ray, interval, time) {
                Some(hit) => {
                    if *hit.time() < closest {
                        closest = *hit.time();
                        hit_data = Some(hit);
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

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}
