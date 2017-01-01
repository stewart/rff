mod terminal;
mod ansi;

use std::io::{self, Write};
use self::terminal::{Terminal, Event, Key};
use self::ansi::{clear, cursor};

pub struct Interface {
    choices: Vec<String>,
    search: String,
    terminal: Terminal
}

impl Interface {
    /// Creates a new Interface with the provided input choices.
    pub fn with_choices(choices: Vec<String>) -> Interface {
        let mut term = Terminal::from("/dev/tty").unwrap();
        term.set_raw_mode().unwrap();

        Interface {
            choices: choices,
            search: String::new(),
            terminal: term
        }
    }

    // Starts the interface
    pub fn run(&mut self) {
        self.render().expect("Unable to render");

        for event in self.terminal.events().unwrap() {
            match event {
                Ok(Event::Key(Key::Ctrl('c'))) => {
                    return;
                },

                Ok(Event::Key(Key::Char(ch))) => {
                    self.search.push(ch);
                    self.render().expect("Unable to render");
                },

                Ok(Event::Key(Key::Backspace)) => {
                    self.search.pop();
                    self.render().expect("Unable to render");
                },

                Ok(_) => {}
                Err(err) => write!(self.terminal, "{:?}", err).unwrap(),
            };
        }
    }

    fn render(&mut self) -> io::Result<()> {
        let ref mut term = self.terminal;
        write!(term, "{}{}", clear::Line, cursor::Column(1))?;
        write!(term, "> {}", self.search)?;
        Ok(())
    }
}
