use super::Material;
use crate::{
    objects::HitData,
    structs::{Color, Ray, Vec3},
};
use rand::Rng;

/// Structure representing a dielectric surface.
///
/// Dielectric materials are ones that allow light to pass through them (eg. glass, water), suffering some refraction
/// Index of refraction defines the magnitude of refraction of light.
pub struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self {
            index_of_refraction: ir,
        }
    }

    /// Refraction of light through the material.
    ///
    /// If you want to try understanding the derivation :-
    /// https://raytracing.github.io/books/RayTracingInOneWeekend.html#dielectrics/snell'slaw
    /// https://physics.stackexchange.com/questions/435512/snells-law-in-vector-form
    /// https://graphicscompendium.com/raytracing/10-reflection-refraction
    fn refract(r_in: Vec3, normal: Vec3, refraction_ratio: f64, cos: f64) -> Vec3 {
        // The output vector can be decomposed into two vectors, one perpendicular to normal and one parallel to it.
        // Then we can sum these at the end to return the full vector.

        let r_out_perp = (r_in + normal * cos) * refraction_ratio;
        let r_out_para = normal * -(1.0 - r_out_perp.length_squared()).abs().sqrt();

        r_out_para + r_out_perp
    }

    /// Reflection, just like in metals (see `metal.rs`);
    fn reflect(r_in_unit: Vec3, normal: Vec3) -> Vec3 {
        r_in_unit - normal * (r_in_unit.dot(normal) * 2.0)
    }

    /// Schlick's approximation.
    ///
    /// This outputs a reflection coefficient, which tells how much of light should reflect back,
    /// depending on the angle (specifically, cos of that angle) and the refraction ratio.
    ///
    /// Think how glass becomes opaque when viewed from almost a vertical angle.
    fn reflectance(cos: f64, refraction_ratio: f64) -> f64 {
        let r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
        let r1 = r0 * r0;

        r1 + (1.0 - r1) * (1.0 - cos).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, hit: HitData, r_in: Ray) -> (Ray, Color) {
        // If the ray is hitting the front face, it is entering the object.
        // If its hitting the back, its leaving, thus the index of refraction should be reversed.
        let refraction_ratio = match hit.is_front_face() {
            true => 1.0 / self.index_of_refraction,
            false => self.index_of_refraction,
        };

        let r_in_unit = r_in.direction().unit_vec();

        // a ⋅ b = cos θ   (unit vectors)
        // where θ is the angle of incidence.
        let cos = r_in_unit.dot(-hit.normal()).min(1.0);
        let sin = (1.0 - cos * cos).sqrt();

        // From snell's law, refraction ratio * sin of incidence angle = sin of refracted angle.
        // However, max. value of sine function is 1, so if this expression exceeds 1, there is no
        // solution for snell's equation, thus refraction is not possible.
        let cannot_refract = refraction_ratio * sin > 1.0;

        let reflection_coefficient = Self::reflectance(cos, refraction_ratio); // Check method comments.
        let random_double = rand::thread_rng().gen_range(0.0..1.0);

        let direction = match cannot_refract || (reflection_coefficient > random_double) {
            true => Self::reflect(r_in_unit, hit.normal()),
            false => Self::refract(r_in_unit, hit.normal(), refraction_ratio, cos),
        };

        (Ray::new(hit.point(), direction), Color::WHITE)
    }
}
