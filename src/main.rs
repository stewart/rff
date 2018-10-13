extern crate clap;
extern crate rayon;
extern crate rff;

use clap::{App, Arg};
use rayon::prelude::*;
use rff::interface::{Error, Interface};
use rff::{match_and_score, stdin};
use std::io::{self, BufWriter, Write};

fn main() {
    let status_code = run();
    std::process::exit(status_code);
}

fn run() -> i32 {
    let matches = App::new("rff")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Andrew S. <andrew@stwrt.ca>")
        .about("A fuzzy finder.")
        .arg(
            Arg::with_name("query")
                .short("s")
                .long("search")
                .value_name("QUERY")
                .help("Term to search for"),
        ).arg(Arg::with_name("benchmark").long("benchmark").help("Run rff in benchmark mode"))
        .get_matches();

    let has_query = matches.is_present("query");
    let has_benchmark = matches.is_present("benchmark");

    if has_benchmark && !has_query {
        println!("Must specifiy -s/--search with --benchmark");
        return 1;
    }

    if has_query {
        let query = matches.value_of("query").unwrap();

        if has_benchmark {
            benchmark(query);
        } else {
            search(query);
        }

        return 0;
    } else {
        return interactive();
    }
}

fn benchmark(needle: &str) {
    let lines = stdin::slurp();

    // in benchmark mode, we run the match/score/sort loop 100 times
    for _ in 0..100 {
        lines
            .par_iter()
            .filter_map(|line| match_and_score(needle, line))
            .collect::<Vec<_>>()
            .par_sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap().reverse());
    }
}

fn search(needle: &str) {
    let lines = stdin::slurp();
    let mut lines: Vec<_> =
        lines.par_iter().filter_map(|line| match_and_score(needle, line)).collect();

    lines.par_sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap().reverse());

    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());

    for line in &lines {
        writeln!(stdout, "{}", line.0).unwrap();
    }
}

fn interactive() -> i32 {
    let lines = stdin::slurp();

    match Interface::new(&lines).run() {
        Ok(result) => println!("{}", result),
        Err(Error::Exit) => return 1,
        Err(error) => {
            eprintln!("{:?}", error);
            return 1;
        }
    }

    return 0;
}
