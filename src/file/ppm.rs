use super::FileWriter;
use std::{fs::File, io::Write};

pub struct PPMFile {
    file: File,
}

impl FileWriter for PPMFile {
    fn writer(&mut self) -> &mut dyn Write {
        &mut self.file
    }

    fn new(file: &str, width: u16, height: u16) -> Self {
        let mut ppm = PPMFile {
            file: File::create(file).unwrap(),
        };

        let writer = ppm.writer();
        writeln!(writer, "P3").unwrap();
        writeln!(writer, "{} {}", width, height).unwrap();
        writeln!(writer, "255").unwrap();

        ppm
    }
}
