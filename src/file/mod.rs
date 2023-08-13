//! Abstractions for interacting with image files.

mod ppm;

use std::io;

pub trait FileWriter {
    fn writer(&mut self) -> &mut dyn io::Write;
    fn new<X: Into<u16>, Y: Into<u16>>(file: &str, width: X, height: Y) -> Self;
}

pub use ppm::PPMFile;
