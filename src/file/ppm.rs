use super::FileWriter;
use crate::{HEIGHT, WIDTH};
use std::{fs::File, io::Write};

pub struct PPMFile {
    file: File,
}

impl FileWriter for PPMFile {
    fn writer(&mut self) -> &mut dyn Write {
        &mut self.file
    }

    fn new(file: &str) -> Self {
        let mut ppm = PPMFile {
            file: File::create(file).unwrap(),
        };

        let writer = ppm.writer();
        writeln!(writer, "P3").unwrap();
        writeln!(
            writer,
            "{} {}",
            WIDTH.get().expect("OnceCell not initialized."),
            HEIGHT.get().expect("OnceCell not initialized."),
        )
        .unwrap();
        writeln!(writer, "255").unwrap();

        ppm
    }
}
