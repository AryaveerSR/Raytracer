pub mod image;
pub mod structs;
pub mod vec3;

use crate::{image::PPM, structs::Color};

pub fn do_stuff() {
    let width = 255_u16;
    let height = 255_u16;

    let mut data = vec![vec![Color::BLACK; 255]; 255];

    for i in 0..height {
        for j in 0..width {
            data[i as usize][j as usize] = Color::from_rgb(i as u8, j as u8, 0);
        }
    }

    PPM::new(width, height, data).write_to_file("output.ppm".to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    /// `cargo test output_gradient -- --ignored`
    /// A neat function to output a gradient
    fn output_gradient() {
        let width = 255_u16;
        let height = 255_u16;

        let mut data = vec![vec![Color::BLACK; 255]; 255];

        for i in 0..height {
            for j in 0..width {
                data[i as usize][j as usize] = Color::from_rgb(i as u8, j as u8, 0);
            }
        }

        PPM::new(width, height, data).write_to_file("gradient.ppm".to_string());
    }
}
