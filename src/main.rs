extern crate getopts;

use std::env;
use std::process;
use getopts::{Options, Fail};

fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>();
    let mut opts = Options::new();

    opts.optflag("h", "help", "Display this help and exit");
    opts.optflag("v", "version", "Display version information and exit");

    let matches = match opts.parse(args).map_err(translate_parse_error) {
        Ok(matches) => matches,
        Err(err) => {
            println!("{}", err);
            print_usage(opts);
            process::exit(1);
        }
    };

    if matches.opt_present("h") {
        print_usage(opts);
        return;
    }

    if matches.opt_present("v") {
        print_version();
        return;
    }
}

fn translate_parse_error(err: Fail) -> String {
    match err {
        Fail::ArgumentMissing(opt) => format!("Argument missing: {}", opt),
        Fail::UnrecognizedOption(opt) => format!("Invalid option: {}", opt),
        Fail::OptionMissing(opt) => format!("Missing option: {}", opt),
        Fail::OptionDuplicated(opt) => format!("Duplicated option: {}", opt),
        Fail::UnexpectedArgument(opt) => format!("Unexpected argument: {}", opt)
    }
}

fn print_usage(opts: Options) {
    let prelude = format!("Usage: {} [options]", env!("CARGO_PKG_NAME"));
    print!("{}", opts.usage(&prelude));
}

fn print_version() {
    println!("{}", env!("CARGO_PKG_VERSION"));
}
