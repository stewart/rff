extern crate getopts;
extern crate rayon;
extern crate libc;
extern crate rff;

mod interface;
mod app;

use std::io::{self, BufRead, Write, BufWriter};
use std::process;
use rayon::prelude::*;
use rff::Choice;
use app::{App, Action};
use interface::{Interface, Error};

fn main() {
    let app = App::new();

    match app.action {
        Action::Exit(code) => {
            process::exit(code)
        }

        Action::InteractiveMode(initial) => {
            let choices = get_choices();
            let mut interface = Interface::new(&choices, initial);

            match interface.run() {
                Ok(result) => println!("{}", result),
                Err(Error::Exit) => process::exit(1),
                Err(e) => {
                    println!("error: {:?}", e);
                    process::exit(1);
                }
            }
        }

        Action::Search(search) => {
            let choices = get_choices();

            let mut choices = choices.
                par_iter().
                filter_map(|choice| Choice::new(&search, choice)).
                collect::<Vec<Choice>>();

            choices.sort_by(|a, b| a.partial_cmp(b).unwrap().reverse());

            let stdout = io::stdout();
            let mut stdout = BufWriter::new(stdout.lock());

            for choice in choices {
                writeln!(stdout, "{}", choice.0).unwrap();
            }
        }

        Action::Benchmark(search) => {
            let choices = get_choices();

            for _ in 0..100 {
                choices.
                    par_iter().
                    filter_map(|choice| Choice::new(&search, choice)).
                    collect::<Vec<Choice>>().
                    sort_by(|a, b| a.partial_cmp(b).unwrap().reverse());
            }
        }
    }
}

fn get_choices() -> Vec<String> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(Result::unwrap).collect();
    lines
}
