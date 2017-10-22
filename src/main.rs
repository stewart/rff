extern crate rff;

use std::env;
use std::io::{self, Write, BufWriter};
use rff::{stdin, matcher, scorer};

fn main() {
    if let Some(search_term) = env::args().nth(1) {
        let lines = stdin::slurp();

        let mut lines: Vec<_> = lines.iter()
            .filter(|line| { matcher::matches(&search_term, line) })
            .map(|line| { (line, scorer::score(&search_term, line)) })
            .collect();

        lines.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap().reverse());

        let stdout = io::stdout();
        let mut stdout = BufWriter::new(stdout.lock());

        for line in &lines {
            writeln!(stdout, "{}", line.0).unwrap();
        }
    }
}
