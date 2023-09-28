//! Abstractions for interacting with image files.

use std::io;

pub trait FileWriter {
    fn writer(&mut self) -> &mut dyn io::Write;
    fn new(file: &str, width: u16, height: u16) -> Self;
}

mod ppm;

pub use ppm::PPMFile;
