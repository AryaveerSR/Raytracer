use super::Material;
use crate::{
    objects::HitData,
    structs::{Color, Interval, Ray, Vec3},
};

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new<T: Into<f64>>(albedo: Color, fuzz: T) -> Self {
        Metal {
            albedo,
            fuzz: fuzz.into(),
        }
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

impl Material for Metal {
    fn scatter(&self, hit: HitData, ray: Ray) -> (Ray, Color) {
        let v = ray.direction().unit_vec();
        let direction = v - (hit.normal() * v.dot(hit.normal()))
            + Self::random_unit_vector(hit.normal()) * self.fuzz;
        (Ray::new(hit.point(), direction), self.albedo)
    }
}
