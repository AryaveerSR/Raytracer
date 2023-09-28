use crate::{
    materials::Material,
    structs::{HitData, Interval, Ray},
};
use std::sync::Arc;

/// A trait defining an object, having a material and a method to check if a
/// certain ray hits it or not.
pub trait Object: std::fmt::Debug {
    fn does_hit(&self, ray: Ray, interval: Interval) -> Option<HitData>;
    fn material(&self) -> Arc<dyn Material + Sync + Send>;
}

mod sphere;

pub use sphere::Sphere;
