extern crate clap;

mod stdin;

use clap::App;
use std::process;

// A Result shorthand for the CLI integration
type Result<T> = std::result::Result<T, &'static str>;

fn main() {
    match run() {
        Ok(_) => process::exit(0),
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    }
}

fn run() -> Result<()> {
    let args = App::new("rff")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Andrew Stewart <andrew@stwrt.ca>")
        .get_matches();

    let choices = stdin::slurp();

    println!("args: {:?}", args);
    println!("num of choices: {}", choices.len());

    Ok(())
}
