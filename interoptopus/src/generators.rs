use crate::writer::IndentWriter;
use crate::Error;
use std::fs::File;
use std::path::Path;

pub trait Interop {
    fn write_to(&self, w: &mut IndentWriter) -> Result<(), Error>;

    fn write_file<P: AsRef<Path>>(&self, file_name: P) -> Result<(), Error> {
        let mut file = File::create(file_name)?;
        let mut writer = IndentWriter::new(&mut file);

        self.write_to(&mut writer)
    }
}
