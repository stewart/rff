mod terminal;
mod ansi;

use std::io::{self, Write, BufWriter};
use self::terminal::{Terminal, Event, Key};
use self::ansi::{clear, color, cursor, style};
use rayon::prelude::*;
use rff::choice::Choice;

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

pub struct Options {
    pub choices: Vec<String>,
    pub initial: String,
}

pub struct Interface {
    choices: Vec<String>,
    matches: Vec<Choice>,
    choices_width: usize,
    selected: usize,
    search: String,
    width: usize,
    terminal: Terminal,
}

impl Interface {
    /// Creates a new Interface from the provided options.
    pub fn from_opts(opts: Options) -> Interface {
        let mut term = Terminal::from("/dev/tty").unwrap();
        term.set_raw_mode().unwrap();

        let choices_width = format!("{}", opts.choices.len()).len();

        Interface {
            choices: opts.choices,
            matches: Vec::new(),
            choices_width: choices_width,
            selected: 0,
            search: opts.initial,
            width: term.max_width,
            terminal: term
        }
    }

    // Starts the interface
    pub fn run(&mut self) -> Result<&str, Error> {
        self.filter_choices();
        self.render()?;

        for event in self.terminal.events()? {
            if let Event::Key(key) = event? {
                match key {
                    Key::Ctrl('c') | Key::Ctrl('d') => {
                        self.clear()?;
                        return Err(Error::Exit);
                    }

                    Key::Char('\n') => {
                        break;
                    },

                    Key::Ctrl('n') => {
                        self.selected += 1;
                        self.render()?;
                    },

                    Key::Ctrl('p') => {
                        self.selected = self.selected.saturating_sub(1);
                        self.render()?;
                    },

                    Key::Char(ch) => {
                        self.search.push(ch);
                        self.filter_existing_matches();
                        self.render()?;
                    },

                    Key::Backspace | Key::Ctrl('h') => {
                        self.search.pop();
                        self.filter_choices();
                        self.render()?;
                    }

                    Key::Ctrl('u') => {
                        self.search.clear();
                        self.filter_choices();
                        self.render()?;
                    }

                    _ => {}
                }
            };
        }

        self.clear()?;
        Ok(self.result())
    }

    fn filter_choices(&mut self) {
        let mut matches = self.choices.
            par_iter().
            cloned().
            filter_map(|choice| Choice::with_positions(&self.search, choice)).
            collect::<Vec<_>>();

        matches.sort_by(|a, b| b.partial_cmp(a).unwrap());

        self.matches = matches;
    }

    fn filter_existing_matches(&mut self) {
        let mut matches = self.matches.
            par_iter().
            map(|c| c.text().to_string()).
            filter_map(|choice| Choice::with_positions(&self.search, choice)).
            collect::<Vec<_>>();

        matches.sort_by(|a, b| b.partial_cmp(a).unwrap());

        self.matches = matches;
    }

    fn prompt(&self) -> String {
        let count = self.matches.len();
        format!("{:width$} > {}", count, self.search, width = self.choices_width)
    }

    fn render(&mut self) -> io::Result<()> {
        // ensure selected index is within bounds
        self.clamp_selected();

        let prompt = self.prompt();
        let matches = self.matches.iter().take(10);
        let n = matches.len() as u16;

        let mut term = BufWriter::new(&mut self.terminal);

        write!(term, "{}{}{}", cursor::Column(1), clear::Screen, prompt)?;

        for (i, choice) in matches.enumerate() {
            let selected = i == self.selected;
            let chars = choice.text().chars().take(self.width);

            write!(term, "\r\n")?;

            if selected {
                write!(term, "{}", style::Invert)?;
            }

            if let Some(positions) = choice.positions() {
                for (i, ch) in chars.enumerate() {
                    if positions.contains(&i) {
                        let color = color::Fg(color::Colors::Magenta);
                        let reset = color::Fg(color::Reset);
                        write!(term, "{}{}{}", color, ch, reset)?;
                    } else {
                        write!(term, "{}", ch)?;
                    }
                }
            } else {
                write!(term, "{}", chars.collect::<String>())?;
            }

            if selected {
                write!(term, "{}", style::NoInvert)?;
            }
        }

        if n > 0 {
            let col = (prompt.len() + 1) as u16;
            write!(term, "{}{}", cursor::Up(n), cursor::Column(col))?;
        }

        Ok(())
    }

    fn clamp_selected(&mut self) {
        let mut max = self.matches.len();
        if max > 10 { max = 10; }

        if self.selected >= max {
            self.selected = if max > 0 { max - 1 } else { 0 };
        }
    }

    fn clear(&mut self) -> Result<(), Error> {
        write!(self.terminal, "{}{}", cursor::Column(1), clear::Screen)?;
        self.terminal.reset()?;
        Ok(())
    }

    fn result(&mut self) -> &str {
        self.matches.iter().
            map(|c| c.text()).
            nth(self.selected).
            unwrap_or(&self.search)
    }
}
