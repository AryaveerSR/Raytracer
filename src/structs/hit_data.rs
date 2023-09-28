use super::{Point3, Vec3};
use crate::materials::Material;
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

    pub fn time(&self) -> &f64 {
        &self.time
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
