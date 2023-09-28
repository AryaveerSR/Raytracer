//! Abstractions for interacting with image files.

use crate::structs::Color;

pub trait FileWriter {
    fn write(&mut self, color: Color);
}

mod ppm;

pub use ppm::PPMFile;
