//! Functions and other code used commonly across the materials.

use crate::structs::{Interval, Vec3};

/// Generating unit vectors that lie on a hemisphere surface centred at normal.
pub fn random_unit_vector(normal: Vec3) -> Vec3 {
    // Generate a vector inside a unit sphere and normalize it, thus extending it to the surface.
    let vector = loop {
        let p = Vec3::random(Interval::new(-1, 1));

        if p.length_squared() < 1.0 {
            break p.unit_vec();
        }
    };

    // This "flips" generated vectors that are on the opposite hemisphere, so the output is on the preferred hemisphere.
    match vector.dot(normal) > 0.0 {
        true => vector,
        false => -vector,
    }
}
