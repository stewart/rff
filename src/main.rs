extern crate rff;
extern crate clap;

use std::io::{self, Write, BufWriter};
use rff::{stdin, match_and_score};
use rff::interface::Interface;
use clap::{App, Arg};

fn main() {
    let matches = App::new("rff").
        version(env!("CARGO_PKG_VERSION")).
        author("Andrew S. <andrew@stwrt.ca>").
        about("A fuzzy finder.").
        arg(
            Arg::with_name("QUERY").
                short("s").
                long("search").
                help("Term to search for")
        ).
        arg(
            Arg::with_name("benchmark").
                long("benchmark").
                help("Run rff in benchmark mode")
        ).
        get_matches();

    let lines = stdin::slurp();

    if matches.is_present("QUERY") {
        let query = matches.value_of("QUERY").unwrap();

        if matches.is_present("benchmark") {
            benchmark(query, lines);
        } else {
            search(query, lines);
        }
    } else {
        Interface::new(lines).run();
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
