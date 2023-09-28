use crate::structs::{Color, HitData, Ray};

pub trait Material: std::fmt::Debug {
    fn scatter(&self, hit: HitData, ray: Ray) -> (Ray, Color);
}

mod commons;
mod dielectric;
mod lambertian;
mod metal;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;
