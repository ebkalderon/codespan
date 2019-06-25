use codespan::Location;
use std::{fmt, io};
use termcolor::WriteColor;

use crate::term::Config;

/// The 'location focus' of a source code snippet.
///
/// This is displayed in a way that other tools can understand, for
/// example when command+clicking in iTerm.
///
/// ```text
/// test:2:9
/// ```
pub struct Locus<FileName> {
    file_name: FileName,
    location: Location,
}

impl<FileName: fmt::Display> Locus<FileName> {
    pub fn new(file_name: FileName, location: Location) -> Locus<FileName> {
        Locus {
            file_name,
            location,
        }
    }

    pub fn emit(&self, writer: &mut impl WriteColor, _config: &Config) -> io::Result<()> {
        write!(
            writer,
            "{file}:{line}:{column}",
            file = self.file_name,
            line = self.location.line.number(),
            column = self.location.column.number(),
        )
    }
}
