use std::borrow::Cow;
use std::io::{self, Write, BufWriter};

use super::{MatchWithPositions, match_and_score_with_positions};
use ansi::{clear, color, cursor, style};
use terminal::{self, Terminal, Key, Event};

use rayon::prelude::*;

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
    selected: usize,

    choices_width: usize,
    width: usize,

    terminal: Terminal,
}

impl<'a> Interface<'a> {
    // Creates a new Interface with the provided lines
    pub fn new(lines: &'a [String]) -> Interface<'a> {
        let mut terminal = Terminal::from("/dev/tty").unwrap();
        let choices_width = format!("{}", lines.len()).len();

        terminal.set_raw_mode().unwrap();

        Interface {
            lines: lines,
            matches: vec![],
            search: String::new(),
            selected: 0,
            choices_width: choices_width,
            width: terminal.max_width,
            terminal: terminal,
        }
    }

    // Runs the Interface, returning either the final selection, the final
    // search string if nothing matches, or an error
    pub fn run(&mut self) -> Result<Cow<'a, str>, Error> {
        self.filter_matches();
        self.render()?;

        for event in self.terminal.events()? {
            if let Event::Key(key) = event? {
                match key {
                    Key::Ctrl('c') | Key::Ctrl('d') | Key::Escape => {
                        self.reset()?;
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
                        self.filter_existing();
                        self.render()?;
                    },

                    Key::Backspace | Key::Ctrl('h') => {
                        self.search.pop();
                        self.filter_matches();
                        self.render()?;
                    }

                    Key::Ctrl('u') => {
                        self.search.clear();
                        self.filter_matches();
                        self.render()?;
                    }

                    _ => {}
                }
            };
        }

        self.reset()?;
        Ok(self.take_result())
    }

    // Matches and scores `lines` by `search`, sorting the result
    fn filter_matches(&mut self) {
        let ref search = self.search;

        self.matches = self.lines.
            par_iter().
            filter_map(|line| match_and_score_with_positions(search, line)).
            collect();

        self.matches.par_sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap().reverse());
    }

    // Matches and scores the existing `matches` by `search`, sorting the result
    fn filter_existing(&mut self) {
        let ref search = self.search;

        self.matches = self.matches.
            par_iter().
            filter_map(|&(line, _, _)| match_and_score_with_positions(search, line)).
            collect();

        self.matches.par_sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap().reverse());
    }

    // Renders the current state of the Interface to it's `terminal`
    fn render(&mut self) -> io::Result<()> {
        self.clamp_selected();

        let prompt = self.prompt();
        let matches = self.matches.iter().take(10);
        let n = matches.len() as u16;

        let mut term = BufWriter::new(&mut self.terminal);

        write!(term, "{}{}{}", cursor::Column(1), clear::Screen, prompt)?;

        for (i, choice) in matches.enumerate() {
            let selected = i == self.selected;
            let chars = choice.0.chars().take(self.width);

            write!(term, "\r\n")?;

            if selected {
                write!(term, "{}", style::Invert)?;
            }

            let ref positions = choice.2;

            for (i, ch) in chars.enumerate() {
                if positions.contains(&i) {
                    let color = color::Fg(color::Colors::Magenta);
                    let reset = color::Fg(color::Reset);
                    write!(term, "{}{}{}", color, ch, reset)?;
                } else {
                    write!(term, "{}", ch)?;
                }
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

    // Generates the input prompt
    fn prompt(&self) -> String {
        let count = self.matches.len();
        format!("{:width$} > {}", count, self.search, width = self.choices_width)
    }

    // Clamps `selected`, such that it doesn't overflow the matches length
    fn clamp_selected(&mut self) {
        let mut max = self.matches.len();
        if max > 10 { max = 10; }

        if self.selected >= max {
            self.selected = if max > 0 { max - 1 } else { 0 };
        }
    }

    // Resets the `terminal`
    fn reset(&mut self) -> Result<(), Error> {
        write!(self.terminal, "{}{}", cursor::Column(1), clear::Screen)?;
        self.terminal.reset()?;
        Ok(())
    }

    // Get a reference to the selected item, or take the search (and replace it with empty)
    fn take_result(&mut self) -> Cow<'a, str> {
        self.matches.iter().
            nth(self.selected).
            map(|choice| Cow::Borrowed(choice.0)).
            unwrap_or_else(||
                Cow::Owned(std::mem::take(&mut self.search))
            )
    }
}
