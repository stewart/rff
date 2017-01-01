extern crate getopts;
extern crate rayon;
extern crate libc;
extern crate rff;

mod interface;

use std::env;
use std::io::{self, BufRead, Write, BufWriter};
use std::process;
use rayon::prelude::*;
use getopts::{Options, Fail};
use rff::choice::Choice;
use interface::Interface;

fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>();
    let mut opts = Options::new();

    opts.optopt("s", "search", "Output sorted matches of QUERY", "QUERY");
    opts.optflag("", "benchmark", "Run search in benchmark mode");
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

    if matches.opt_present("benchmark") {
        if !matches.opt_present("s") {
            println!("Must specify -s/--search with --benchmark");
            process::exit(1);
        }

        let choices = get_choices();
        let search = matches.opt_str("s").unwrap();

        for _ in 0..100 {
            let mut choices = choices.
                par_iter().
                cloned().
                filter_map(|choice| Choice::new(&search, choice)).
                collect::<Vec<Choice>>();

            choices.sort_by(|a, b| b.partial_cmp(a).unwrap());
        }
    } else if matches.opt_present("s") {
        let search = matches.opt_str("s").unwrap();
        let mut choices = get_choices().
            into_par_iter().
            filter_map(|choice| Choice::new(&search, choice)).
            collect::<Vec<Choice>>();

        choices.sort_by(|a, b| b.partial_cmp(a).unwrap());

        let stdout = io::stdout();
        let mut stdout = BufWriter::new(stdout.lock());

        for choice in choices {
            writeln!(stdout, "{}", choice.text()).unwrap();
        }
    } else {
        let mut interface = Interface::with_choices(get_choices());
        interface.run()
    }
}

fn get_choices() -> Vec<String> {
    let mut lines = vec![];

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        lines.push(line.unwrap());
    }

    lines
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
