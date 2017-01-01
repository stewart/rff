use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read, Result};

pub struct Terminal {
    file: File
}

impl Terminal {
    /// Creates a new Terminal from the provided filename
    pub fn from(filename: &str) -> Result<Terminal> {
        let file = OpenOptions::new().write(true).read(true).open(filename)?;

        let terminal = Terminal {
            file: file
        };

        Ok(terminal)
    }
}

impl Read for Terminal {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.file.read(buf)
    }
}

impl Write for Terminal {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.file.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.file.flush()
    }
}
