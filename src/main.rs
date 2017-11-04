extern crate rff;
extern crate clap;

use std::io::{self, Write, BufWriter};
use rff::{stdin, matcher, scorer};
use clap::{App, Arg};

type Match<'a> = (&'a str, f64);

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
        arg(
            Arg::with_name("benchmark").
                long("benchmark").
                help("Run rff in benchmark mode")
        ).
        get_matches();

    let query = matches.value_of("QUERY").unwrap();
    let lines = stdin::slurp();

    if matches.is_present("benchmark") {
        benchmark(query, lines);
    } else {
        search(query, lines);
    }
}

fn match_and_score<'a>(needle: &str, haystack: &'a str) -> Option<Match<'a>> {
    if matcher::matches(needle, haystack) {
        Some((haystack, scorer::score(needle, haystack)))
    } else {
        None
    }
}

fn benchmark(needle: &str, lines: Vec<String>) {
    // in benchmark mode, we run the match/score/sort loop 100 times
    for _ in 0..100 {
        lines
            .iter()
            .filter_map(|line| match_and_score(needle, line))
            .collect::<Vec<_>>()
            .sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap().reverse());
    }
}

fn search(needle: &str, lines: Vec<String>) {
    let mut lines: Vec<_> = lines
        .iter()
        .filter_map(|line| match_and_score(needle, line))
        .collect();

    lines.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap().reverse());

    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());

    for line in &lines {
        writeln!(stdout, "{}", line.0).unwrap();
    }
}
