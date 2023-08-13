use super::FileWriter;
use std::{fs::File, io::Write};

pub struct PPMFile {
    file: File,
}

impl FileWriter for PPMFile {
    fn writer(&mut self) -> &mut dyn Write {
        &mut self.file
    }

    fn new<X: Into<u16>, Y: Into<u16>>(file: &str, width: X, height: Y) -> Self {
        let mut ppm = PPMFile {
            file: File::create(file).unwrap(),
        };

        let writer = ppm.writer();
        writeln!(writer, "P3").unwrap();
        writeln!(writer, "{} {}", width.into(), height.into()).unwrap();
        writeln!(writer, "255").unwrap();

        ppm
    }
}
