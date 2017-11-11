use std::io::{self, Write, BufWriter};

use super::{MatchWithPositions, match_and_score_with_positions};
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

pub struct Interface<'a> {
    lines: &'a [String],
    matches: Vec<MatchWithPositions<'a>>,
    search: String,
    terminal: Terminal,
}

impl<'a> Interface<'a> {
    pub fn new(lines: &'a [String]) -> Interface<'a> {
        Interface {
            lines: lines,
            matches: vec![],
            search: String::new(),
            terminal: Terminal::from("/dev/tty").unwrap(),
        }
    }

    pub fn run(&mut self) -> Result<&str, Error> {
        Ok(&self.search)
    }
}
