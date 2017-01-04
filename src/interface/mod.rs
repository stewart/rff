mod terminal;
mod ansi;

use std::io::{self, Write};
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
    matching: Vec<Choice>,
    selected: usize,
    search: String,
    terminal: Terminal
}

impl Interface {
    /// Creates a new Interface from the provided options.
    pub fn from_opts(opts: Options) -> Interface {
        let mut term = Terminal::from("/dev/tty").unwrap();
        term.set_raw_mode().unwrap();

        Interface {
            choices: opts.choices,
            matching: Vec::new(),
            selected: 0,
            search: opts.initial,
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
                        self.clamp_selected();
                        self.render()?;
                    },

                    Key::Ctrl('p') => {
                        self.selected = self.selected.
                            checked_sub(1).
                            unwrap_or(0);

                        self.clamp_selected();
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

        self.matching = matches;
    }

    fn filter_existing_matches(&mut self) {
        let mut matches = self.matching.
            par_iter().
            map(|c| c.text().to_string()).
            filter_map(|choice| Choice::with_positions(&self.search, choice)).
            collect::<Vec<_>>();

        matches.sort_by(|a, b| b.partial_cmp(a).unwrap());

        self.matching = matches;
    }

    fn render(&mut self) -> io::Result<()> {
        write!(self.terminal, "{}{}", cursor::Column(1), clear::Screen)?;
        write!(self.terminal, "> {}", self.search)?;

        let n = self.matching.iter().take(10).len() as u16;

        self.render_matches()?;

        if n > 0 {
            let column = format!("> {}", self.search).len() as u16;
            write!(self.terminal, "{}{}", cursor::Up(n), cursor::Column(column + 1))?;
        }

        Ok(())
    }

    fn render_matches(&mut self) -> io::Result<()> {
        let max_width = self.terminal.max_width;
        let matches = self.matching.iter().take(10);

        for (i, choice) in matches.enumerate() {
            let selected = i == self.selected;

            write!(self.terminal, "\r\n")?;

            if selected {
                write!(self.terminal, "{}", style::Invert)?;
            }

            if let Some(positions) = choice.positions() {
                let chars = choice.text().chars().take(max_width);

                for (i, ch) in chars.enumerate() {
                    let match_position = positions.iter().any(|p| *p == i);

                    if match_position {
                        let color = color::Fg(color::Colors::Magenta);
                        let reset = color::Fg(color::Reset);
                        write!(self.terminal, "{}{}{}", color, ch, reset)?;
                    } else {
                        write!(self.terminal, "{}", ch)?;
                    }
                }

            } else {
                let text = choice.text()
                    .chars()
                    .take(max_width)
                    .collect::<String>();

                write!(self.terminal, "{}", text)?;
            }

            if selected {
                write!(self.terminal, "{}", style::NoInvert)?;
            }
        }

        Ok(())
    }

    #[inline(always)]
    fn clamp_selected(&mut self) {
        let mut max = self.matching.len();
        if max > 10 { max = 10; }

        if self.selected >= max {
            self.selected = max - 1;
        }
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
