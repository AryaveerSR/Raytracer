use super::{commons::random_unit_vector, Material};
use crate::{
    objects::HitData,
    structs::{Color, Ray},
};

/// Structure representing a lambertian surface, which is a type of an ideal "matte" surface.
///
/// `albedo` is the effective color of the surface. Everytime a ray bounces off the material,
/// its respective components are multiplied by a factor of `albedo / 255`.
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, hit: HitData, _: Ray) -> (Ray, Color) {
        // A lambertian scattering simply involves adding a random unit vector to the normal.
        let direction = hit.normal() + random_unit_vector(hit.normal());

        (Ray::new(hit.point(), direction), self.albedo)
    }
}
