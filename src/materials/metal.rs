use super::{commons::random_unit_vector, Material};
use crate::{
    objects::HitData,
    structs::{Color, Ray},
};

/// Structure representing a metal surface.
///
/// `albedo` is the effective color of the surface. Everytime a ray bounces off the material,
/// its respective components are multiplied by a factor of `albedo / 255`.
///
/// `fuzz` is the factor for the randomness induced in the reflected ray's direction.
/// It should be between 0 and 1. Values above 1 just result in noise, and negatives are
/// the same as the randomness covers negative and positive deviations equally.
#[derive(Debug)]
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
}

impl Material for Metal {
    fn scatter(&self, hit: HitData, ray: Ray) -> (Ray, Color) {
        let v = ray.direction().unit_vec();

        // Resulting vector = v - 2b
        // Where `b` has direction along normal and magnitude of v's "height" along normal.
        //
        // The `random_unit_vector()` introduces some randomness into the scattered ray's direction, creating "fuzz".
        let direction = v - (hit.normal() * (v.dot(hit.normal()) * 2.0))
            + random_unit_vector(hit.normal()) * self.fuzz;

        (Ray::new(hit.point(), direction), self.albedo)
    }
}
