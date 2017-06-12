use getopts::Options;

#[derive(Debug, PartialEq)]
pub enum Action {
    InteractiveMode(String),
    Search(String),
    Benchmark(String),
    Exit(i32)
}

/// Parses a provided set of CLI arguments
pub fn parse(args: Vec<String>) -> Action {
    let opts = {
        let mut opts = Options::new();

        opts.optopt("s", "search", "Output sorted matches of QUERY", "QUERY");
        opts.optopt("q", "query", "Use QUERY as the initial search string", "QUERY");
        opts.optflag("", "benchmark", "Run search in benchmark mode");
        opts.optflag("h", "help", "Display this help and exit");
        opts.optflag("v", "version", "Display version information and exit");

        opts
    };

    let matches = match opts.parse(args) {
        Ok(matches) => matches,
        Err(err) => {
            println!("{}", err.to_string());
            print_usage(opts);
            return Action::Exit(1);
        }
    };

    if matches.opt_present("h") {
        print_usage(opts);
        return Action::Exit(0)
    }

    if matches.opt_present("v") {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return Action::Exit(0)
    }

    if matches.opt_present("benchmark") {
        if !matches.opt_present("s") {
            println!("Must specify -s/--search with --benchmark");
            return Action::Exit(1);
        }

        let search = matches.opt_str("s").unwrap();
        return Action::Benchmark(search);
    } else if matches.opt_present("s") {
        let search = matches.opt_str("s").unwrap();
        return Action::Search(search);
    } else {
        let initial = matches.opt_str("q").unwrap_or_default();
        return Action::InteractiveMode(initial)
    }
}

fn print_usage(opts: Options) {
    let prelude = format!("Usage: {} [options]", env!("CARGO_PKG_NAME"));
    print!("{}", opts.usage(&prelude));
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
            parse(vec![]),
            Action::InteractiveMode("".to_string())
        );

        assert_eq!(
            parse(args("-q asdf")),
            Action::InteractiveMode("asdf".to_string())
        );

        assert_eq!(
            parse(args("-s search_term")),
            Action::Search("search_term".to_string())
        );

        assert_eq!(
            parse(args("--benchmark -s search_term")),
            Action::Benchmark("search_term".to_string())
        );
    }

    #[test]
    fn parse_errors() {
        assert_eq!(parse(args("-asdf")), Action::Exit(1));
        assert_eq!(parse(args("--benchmark")), Action::Exit(1));
    }

    #[test]
    fn parse_support() {
        assert_eq!(parse(args("-h")), Action::Exit(0));
        assert_eq!(parse(args("-v")), Action::Exit(0));
        assert_eq!(parse(args("--version")), Action::Exit(0));
    }
}
