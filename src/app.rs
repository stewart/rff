use std::env;

use getopts::Options;

#[derive(Debug, PartialEq)]
pub enum Action {
    InteractiveMode(String),
    Search(String),
    Benchmark(String),
    Exit(i32)
}

#[derive(Debug)]
pub struct App {
    pub action: Action
}

impl App {
    pub fn new() -> App {
        let args: Vec<String> = env::args().skip(1).collect();
        App::from_cli_args(args)
    }

    pub fn from_cli_args(args: Vec<String>) -> App {
        let opts = opts();

        let matches = match opts.parse(args) {
            Ok(matches) => matches,
            Err(err) => {
                println!("{}", err.to_string());
                print_usage(opts);
                return App { action: Action::Exit(1) };
            }
        };

        if matches.opt_present("h") {
            print_usage(opts);
            return App { action: Action::Exit(0) };
        }

        if matches.opt_present("v") {
            print_version();
            return App { action: Action::Exit(0) };
        }

        let action;

        if matches.opt_present("benchmark") {
            if !matches.opt_present("s") {
                println!("Must specify -s/--search with --benchmark");
                return App { action: Action::Exit(1) };
            }

            let search = matches.opt_str("s").unwrap();
            action = Action::Benchmark(search);
        } else if matches.opt_present("s") {
            let search = matches.opt_str("s").unwrap();
            action = Action::Search(search);
        } else {
            let initial = matches.opt_str("q").unwrap_or_default();
            action = Action::InteractiveMode(initial)
        }

        App { action: action }
    }
}

fn opts() -> Options {
    let mut opts = Options::new();

    opts.optopt("s", "search", "Output sorted matches of QUERY", "QUERY");
    opts.optopt("q", "query", "Use QUERY as the initial search string", "QUERY");
    opts.optflag("", "benchmark", "Run search in benchmark mode");
    opts.optflag("h", "help", "Display this help and exit");
    opts.optflag("v", "version", "Display version information and exit");

    opts
}

fn print_usage(opts: Options) {
    let prelude = format!("Usage: {} [options]", env!("CARGO_PKG_NAME"));
    print!("{}", opts.usage(&prelude));
}

fn print_version() {
    println!("{}", env!("CARGO_PKG_VERSION"));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(st: &str) -> Vec<String> {
        st.split(' ').map(|s| s.to_string()).collect()
    }

    #[test]
    fn parse_valid_modes() {
        assert_eq!(
            App::from_cli_args(vec![]).action,
            Action::InteractiveMode("".to_string())
        );

        assert_eq!(
            App::from_cli_args(args("-q asdf")).action,
            Action::InteractiveMode("asdf".to_string())
        );

        assert_eq!(
            App::from_cli_args(args("-s search_term")).action,
            Action::Search("search_term".to_string())
        );

        assert_eq!(
            App::from_cli_args(args("--benchmark -s search_term")).action,
            Action::Benchmark("search_term".to_string())
        );
    }

    #[test]
    fn parse_errors() {
        assert_eq!(App::from_cli_args(args("-asdf")).action, Action::Exit(1));
        assert_eq!(App::from_cli_args(args("--benchmark")).action, Action::Exit(1));
    }

    #[test]
    fn parse_support() {
        assert_eq!(App::from_cli_args(args("-h")).action, Action::Exit(0));
        assert_eq!(App::from_cli_args(args("-v")).action, Action::Exit(0));
        assert_eq!(App::from_cli_args(args("--version")).action, Action::Exit(0));
    }
}
