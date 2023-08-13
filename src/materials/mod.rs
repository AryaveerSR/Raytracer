use crate::{objects::HitData, structs::Ray};

pub trait Material {
    fn scatter(&self, hit: HitData) -> (Ray, f64);
}

mod lambertian;
pub use lambertian::Lambertian;
