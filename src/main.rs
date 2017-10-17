extern crate rff;

use std::env;

fn main() {
    if let Some(search_term) = env::args().nth(1) {
        let lines = rff::stdin::slurp();
        println!("Lines:\n{:?}", lines);
        println!("Search: {}", search_term);
    }
}
