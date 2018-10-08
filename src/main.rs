#![deny(unused_must_use)]

extern crate clap;
extern crate rff;

use std::io::{self, BufRead, Result};

use clap::{App, Arg};

fn main() {
    let args = App::new("rff")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Andrew Stewart <andrew@stwrt.ca>")
        .arg(
            Arg::with_name("NEEDLE")
                .help("Term to search for")
                .required(true)
                .index(1),
        ).get_matches();

    let choices = read_choices().expect("unable to read choices from stdin");

    let needle = args.value_of("NEEDLE").expect("unable to get needle");

    let results: Vec<(f64, String)> = choices
        .into_iter()
        .filter(|haystack| rff::matches(&needle, &haystack))
        .map(|haystack| (rff::score(&needle, &haystack), haystack))
        .collect();

    for (score, candidate) in results {
        println!("{}: {}", score, candidate);
    }
}

/// Reads all available lines of input from stdin.
fn read_choices() -> Result<Vec<String>> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().collect();
    lines
}
