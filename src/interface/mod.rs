mod terminal;
mod ansi;

use std::io::{self, Write};
use self::terminal::{Terminal, Event, Key};
use self::ansi::{clear, cursor};
use rff::choice::Choice;

pub struct Interface {
    choices: Vec<String>,
    matching: Vec<Choice>,
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
            matching: Vec::new(),
            search: String::new(),
            terminal: term
        }
    }

    // Starts the interface
    pub fn run(&mut self) {
        self.filter_choices();
        self.render().expect("Unable to render");

        for event in self.terminal.events().unwrap() {
            match event {
                Ok(Event::Key(Key::Ctrl('c'))) => {
                    return;
                },

                Ok(Event::Key(Key::Char('\n'))) => {
                    self.emit();
                    return;
                },

                Ok(Event::Key(Key::Char(ch))) => {
                    self.search.push(ch);
                    self.filter_choices();
                    self.render().expect("Unable to render");
                },

                Ok(Event::Key(Key::Backspace)) => {
                    self.search.pop();
                    self.filter_choices();
                    self.render().expect("Unable to render");
                },

                Ok(_) => {}
                Err(err) => write!(self.terminal, "{:?}", err).unwrap(),
            };
        }
    }

    fn filter_choices(&mut self) {
        let mut matches = self.choices.
            iter().
            cloned().
            filter_map(|choice| Choice::new(&self.search, choice)).
            collect::<Vec<_>>();

        matches.sort_by(|a, b| b.partial_cmp(&a).unwrap());

        self.matching = matches;
    }

    fn render(&mut self) -> io::Result<()> {
        write!(self.terminal, "{}{}", clear::Screen, cursor::Column(1))?;
        write!(self.terminal, "> {}", self.search)?;

        let n = self.render_choices()?;

        if n > 0 {
            let column = format!("> {}", self.search).len() as u16;
            write!(self.terminal, "{}{}", cursor::Up(n), cursor::Column(column + 1))?;
        }

        Ok(())
    }

    fn render_choices(&mut self) -> io::Result<u16> {
        let max_width = self.terminal.max_width as usize;

        let choices = self.matching.
            iter().
            map(|c| c.text()).
            map(|c| {
                c.chars().take(max_width).collect::<String>()
            }).take(10);

        let number_of_choices = choices.len() as u16;

        for choice in choices {
            write!(self.terminal, "\r\n{}", choice)?;
        }

        Ok(number_of_choices)
    }

    fn emit(&mut self) {
        write!(self.terminal, "{}{}", cursor::Column(1), clear::Screen).unwrap();
        self.terminal.reset().unwrap();

        let choice = self.matching.
            iter().
            map(|c| c.text()).
            nth(0).
            unwrap_or("");

        println!("{}", choice);
    }
}
