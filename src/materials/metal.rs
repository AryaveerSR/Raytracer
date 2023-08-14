use super::Material;
use crate::{
    objects::HitData,
    structs::{Color, Ray},
};

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, hit: HitData, ray: Ray) -> (Ray, Color) {
        let v = ray.direction().unit_vec();
        let direction = v - (hit.normal() * v.dot(hit.normal()));
        (Ray::new(hit.point(), direction), self.albedo)
    }
}
