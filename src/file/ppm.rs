use super::FileWriter;
use crate::structs::Color;
use std::{fs::File, io::Write};

pub struct PPMFile {
    file: Box<File>,
}

impl PPMFile {
    pub fn new(file: &str, width: u16, height: u16) -> Self {
        let mut file = File::create(file).expect("File creation failed.");

        writeln!(file, "P3").unwrap();
        writeln!(file, "{} {}", width, height).unwrap();
        writeln!(file, "255").unwrap();

        PPMFile {
            file: Box::new(file),
        }
    }
}

impl FileWriter for PPMFile {
    fn write(&mut self, color: Color) {
        writeln!(self.file, "{} {} {}", color.r(), color.g(), color.b())
            .expect("Writing to file failed.");
    }
}
