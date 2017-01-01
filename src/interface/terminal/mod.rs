use std::fs::{File, OpenOptions};
use std::io::Result;

pub struct Terminal {
    file: File
}

impl Terminal {
    pub fn from(filename: &str) -> Result<Terminal> {
        let file = OpenOptions::new().write(true).read(true).open(filename)?;

        let terminal = Terminal {
            file: file
        };

        Ok(terminal)
    }
}
