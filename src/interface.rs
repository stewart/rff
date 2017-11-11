use std::io::{self, Write, BufWriter};

use terminal::{self, Terminal};

#[derive(Debug)]
pub enum Error {
    Exit,
    Write(io::Error),
    Reset(terminal::Error)
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Write(err)
    }
}

impl From<terminal::Error> for Error {
    fn from(err: terminal::Error) -> Error {
        Error::Reset(err)
    }
}

pub struct Interface {
    lines: Vec<String>,
    terminal: Terminal,
}

impl Interface {
    pub fn new(lines: Vec<String>) -> Interface {
        Interface {
            lines: lines,
            terminal: Terminal::from("/dev/tty").unwrap(),
        }
    }

    pub fn run(&mut self) -> Result<String, Error> {
        let mut term = BufWriter::new(&mut self.terminal);
        let mut search = String::new();

        for line in &self.lines {
            write!(term, "{}\n", line)?;
        }

        return Ok(search)
    }
}
