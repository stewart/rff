extern crate rff;
extern crate clap;
extern crate rayon;

use std::io::{self, Write, BufWriter};
use rff::{stdin, match_and_score};
use rff::interface::Interface;
use clap::{App, Arg};
use rayon::prelude::*;

fn main() {
    let status_code = run();
    std::process::exit(status_code);
}

fn run() -> i32 {
    let matches = App::new("rff").
        version(env!("CARGO_PKG_VERSION")).
        author("Andrew S. <andrew@stwrt.ca>").
        about("A fuzzy finder.").
        arg(
            Arg::with_name("query").
                short("s").
                long("search").
                value_name("QUERY").
                help("Term to search for")
        ).
        arg(
            Arg::with_name("benchmark").
                long("benchmark").
                help("Run rff in benchmark mode")
        ).
        get_matches();

    let has_query = matches.is_present("query");
    let has_benchmark = matches.is_present("benchmark");

    if has_benchmark && !has_query {
        println!("Must specifiy -s/--search with --benchmark");
        return 1
    }

    if has_query {
        let query = matches.value_of("query").unwrap();

        if has_benchmark {
            benchmark(query);
        } else {
            search(query);
        }
    } else {
        interactive();
    }

    return 0
}

fn benchmark(needle: &str) {
    let lines = stdin::slurp();

    // in benchmark mode, we run the match/score/sort loop 100 times
    for _ in 0..100 {
        lines
            .par_iter()
            .filter_map(|line| match_and_score(needle, line))
            .collect::<Vec<_>>()
            .sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap().reverse());
    }
}

fn search(needle: &str) {
    let lines = stdin::slurp();
    let mut lines: Vec<_> = lines
        .par_iter()
        .filter_map(|line| match_and_score(needle, line))
        .collect();

    lines.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap().reverse());

    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());

    for line in &lines {
        writeln!(stdout, "{}", line.0).unwrap();
    }
}

fn interactive() {
    let lines = stdin::slurp();

    match Interface::new(&lines).run() {
        Ok(result) => println!("{}", result),
        Err(error) => println!("{:?}", error),
    }
}
