extern crate clap;

mod stdin;

use clap::App;

fn main() {
    let args = App::new("rff")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Andrew Stewart <andrew@stwrt.ca>")
        .get_matches();

    let choices = stdin::slurp();

    println!("args: {:?}", args);
    println!("num of choices: {}", choices.len());
}
