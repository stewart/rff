mod terminal;

use std::io::Write;
use self::terminal::{Terminal, Event, Key};

pub struct Interface {
    choices: Vec<String>,
    terminal: Terminal
}

impl Interface {
    /// Creates a new Interface with the provided input choices.
    pub fn with_choices(choices: Vec<String>) -> Interface {
        let mut term = Terminal::from("/dev/tty").unwrap();
        term.set_raw_mode().unwrap();

        Interface {
            choices: choices,
            terminal: term
        }
    }

    // Starts the interface
    pub fn run(&mut self) {
        let ref mut term = self.terminal;

        write!(term, "{} choices to search through\r\n", self.choices.len());

        for event in term.events().unwrap() {
            match event {
                Ok(Event::Key(Key::Ctrl('c'))) => {
                    return;
                },

                Ok(event) => write!(term, "{:?}\r\n", event),
                Err(err) => write!(term, "{:?}\r\n", err),
            };
        }
    }
}
