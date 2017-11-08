use std::io::{Write, BufWriter};

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

    pub fn run(&mut self) {
        let mut term = BufWriter::new(&mut self.terminal);

        for line in &self.lines {
            write!(term, "{}\n", line);
        }
    }
}
