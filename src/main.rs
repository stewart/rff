extern crate rff;

use std::env;
use rff::{stdin, matcher};

fn main() {
    if let Some(search_term) = env::args().nth(1) {
        let lines = stdin::slurp();

        let lines: Vec<_> = lines.iter()
            .filter(|line| { matcher::matches(&search_term, line) })
            .collect();

        println!("Lines:\n{:?}", lines);
        println!("Search: {}", search_term);
    }
}
