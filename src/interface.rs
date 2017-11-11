use std::io::{self, Write, BufWriter};

use terminal::Terminal;

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

    pub fn run(&mut self) -> Result<String, io::Error> {
        let mut term = BufWriter::new(&mut self.terminal);
        let mut search = String::new();

        for line in &self.lines {
            write!(term, "{}\n", line)?;
        }

        return Ok(search)
    }
}
