use super::Material;
use crate::{
    objects::HitData,
    structs::{Color, Interval, Ray, Vec3},
};

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }

    fn random_unit_vector(normal: Vec3) -> Vec3 {
        let vector = loop {
            let p = Vec3::random(Interval::new(-1, 1));
            if p.length_squared() < 1.0 {
                break p.unit_vec();
            }
        };

        if vector.dot(normal) > 0.0 {
            vector
        } else {
            -vector
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, hit: HitData, _: Ray) -> (Ray, Color) {
        let direction = hit.normal() + Lambertian::random_unit_vector(hit.normal());
        (Ray::new(hit.point(), direction), self.albedo)
    }
}
