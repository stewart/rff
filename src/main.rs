#![deny(unused_must_use)]

extern crate clap;
extern crate rff;

mod stdin;

use clap::{App,Arg};
use rff::*;

fn main() {
    let args = App::new("rff")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Andrew Stewart <andrew@stwrt.ca>")
        .arg(Arg::with_name("NEEDLE").
             help("Term to search for").
             required(true).
             index(1))
        .get_matches();

    let choices = stdin::slurp();

    let needle = args.value_of("NEEDLE").expect("Unable to get needle");

    let query: Vec<String> = choices.
        into_iter().
        filter(|haystack| matches(&needle, &haystack)).
        collect();

    print!("{}", query.join("\n"));
}
