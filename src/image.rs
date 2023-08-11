//! Abstractions for interacting with image files.

use crate::color::Color;
use std::fs;

//TODO: Maybe a image_format trait for implementing multiple file formats ?

pub struct PPM {
    width: u16,
    height: u16,
    /// Array of rows, which, in turn, are arrays of u32.
    /// u32 represents the color as 0x00RRGGBB
    data: Vec<Vec<Color>>,
}

/// ### A Struct to handle PPM file format
/// https://en.wikipedia.org/wiki/Netpbm#PPM_example
impl PPM {
    fn as_file_data(&self) -> String {
        let mut data = format!("P3\n{} {}\n255", self.width, self.height);

        for row in &self.data {
            for pixel in row {
                data.push_str(&format!("\n{} {} {}", pixel.r(), pixel.g(), pixel.b()));
            }
        }

        data
    }

    pub fn write_to_file(&self, file_path: String) {
        fs::write(file_path, self.as_file_data()).expect("Wrote to file")
    }

    pub fn new(width: u16, height: u16, data: Vec<Vec<Color>>) -> Self {
        PPM {
            width,
            height,
            data,
        }
    }
}
