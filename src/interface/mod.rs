mod terminal;
mod ansi;

use std::io::{self, Write};
use self::terminal::{Terminal, Event, Key};
use self::ansi::{clear, cursor, style};
use rff::choice::Choice;

#[derive(Debug)]
pub enum Error {
    CtrlC,
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
    choices: Vec<String>,
    matching: Vec<Choice>,
    selected: usize,
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
            selected: 0,
            search: String::new(),
            terminal: term
        }
    }

    // Starts the interface
    pub fn run(&mut self) -> Result<&str, Error> {
        self.filter_choices();
        self.render()?;

        for event in self.terminal.events()? {
            match event? {
                Event::Key(Key::Ctrl('c')) => {
                    self.clear()?;
                    return Err(Error::CtrlC);
                },

                Event::Key(Key::Char('\n')) => {
                    break;
                },

                Event::Key(Key::Char(ch)) => {
                    self.search.push(ch);
                    self.filter_choices();
                    self.render()?;
                },

                Event::Key(Key::Backspace) => {
                    self.search.pop();
                    self.filter_choices();
                    self.render()?;
                },

                _ => {}
            };
        }

        self.clear()?;
        Ok(self.result())
    }

    fn filter_choices(&mut self) {
        let mut matches = self.choices.
            iter().
            cloned().
            filter_map(|choice| Choice::new(&self.search, choice)).
            collect::<Vec<_>>();

        matches.sort_by(|a, b| b.partial_cmp(a).unwrap());

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

        for (i, choice) in choices.enumerate() {
            write!(self.terminal, "\r\n")?;

            if i == self.selected {
                let invert = style::Invert;
                let reset = style::NoInvert;
                write!(self.terminal, "{}{}{}", invert, choice, reset)?;
            } else {
                write!(self.terminal, "{}", choice)?;
            }
        }

        Ok(number_of_choices)
    }

    fn clear(&mut self) -> Result<(), Error> {
        write!(self.terminal, "{}{}", cursor::Column(1), clear::Screen)?;
        self.terminal.reset()?;
        Ok(())
    }

    fn result(&mut self) -> &str {
        self.matching.iter().
            map(|c| c.text()).
            nth(self.selected).
            unwrap_or(&self.search)
    }
}
