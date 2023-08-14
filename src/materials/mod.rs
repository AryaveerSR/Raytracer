use crate::{
    objects::HitData,
    structs::{Color, Ray},
};

pub trait Material {
    fn scatter(&self, hit: HitData, ray: Ray) -> (Ray, Color);
}

mod lambertian;
mod metal;

pub use lambertian::Lambertian;
pub use metal::Metal;
