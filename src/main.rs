extern crate rff;
extern crate clap;

use std::io::{self, Write, BufWriter};
use rff::{stdin, matcher, scorer};
use clap::{App, Arg};

fn main() {
    let matches = App::new("rff").
        version(env!("CARGO_PKG_VERSION")).
        author("Andrew S. <andrew@stwrt.ca>").
        about("A fuzzy finder.").
        arg(
            Arg::with_name("QUERY").
                help("Term to search for").
                required(true).
                index(1)
        ).
        get_matches();

    let query = matches.value_of("QUERY").unwrap();
    let lines = stdin::slurp();

    let mut lines: Vec<_> = lines.iter()
        .filter_map(|line| {
            if matcher::matches(&query, line) {
                Some((line, scorer::score(&query, line)))
            } else {
                None
            }
        })
        .collect();

    lines.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap().reverse());

    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());

    for line in &lines {
        writeln!(stdout, "{}", line.0).unwrap();
    }
}
