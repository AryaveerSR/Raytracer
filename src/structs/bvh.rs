use super::{HitData, Interval, Ray, AABB};
use crate::objects::Object;

enum Hittable {
    Object(Box<dyn Object + Sync + Send>),
    BVH(BVHNode),
}

pub struct BVHNode {
    bounding_box: AABB,
    left: Box<Hittable>,
    right: Box<Hittable>,
}

impl BVHNode {
    pub fn hit(&self, ray: Ray, time: Interval) -> Option<HitData> {
        if !self.bounding_box.hit(ray, time) {
            return None;
        }

        todo!()
    }
}
