use rand::Rng;

use super::Material;
use crate::{
    objects::HitData,
    structs::{Color, Ray, Vec3},
};

pub struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self {
            index_of_refraction: ir,
        }
    }

    fn refract(r_in: Vec3, normal: Vec3, refraction_ratio: f64) -> Vec3 {
        let cos = -r_in.dot(normal).min(1.0);
        let r_out_perp = (r_in + normal * cos) * refraction_ratio;
        let r_out_para = normal * -(1.0 - r_out_perp.length_squared()).abs().sqrt();

        r_out_para + r_out_perp
    }

    fn reflect(r_in: Vec3, normal: Vec3) -> Vec3 {
        r_in.unit_vec() - normal * (r_in.unit_vec().dot(normal) * 2.0)
    }

    // google phrase "Schlick's approximation"
    fn reflectance(cos: f64, refraction_ratio: f64) -> f64 {
        let r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
        let r1 = r0 * r0;

        r1 + (1.0 - r1) * (1.0 - cos).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, hit: HitData, r_in: Ray) -> (Ray, Color) {
        let refraction_ratio = match hit.is_front_face() {
            true => 1.0 / self.index_of_refraction,
            false => self.index_of_refraction,
        };

        let r_in_unit = r_in.direction().unit_vec();

        let cos = -r_in_unit.dot(hit.normal()).min(1.0); //? we do this calc again in refract, would it be petty if i just pass this value.
        let sin = (1.0 - cos * cos).sqrt();

        let cannot_refract = refraction_ratio * sin > 1.0;

        let direction = match cannot_refract
            || (Self::reflectance(cos, refraction_ratio) > rand::thread_rng().gen_range(0.0..1.0))
        {
            true => Self::reflect(r_in.direction(), hit.normal()),
            false => Self::refract(r_in_unit, hit.normal(), refraction_ratio),
        };

        (Ray::new(hit.point(), direction), Color::WHITE)
    }
}
