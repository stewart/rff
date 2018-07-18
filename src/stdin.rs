use std::io::{self, BufRead};

pub type InputLines = Vec<String>;

pub fn slurp() -> InputLines {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    read_lines(stdin)
}

// This fn is extracted from the above to work around a lifetime issue.
// TODO: Investigate if this is necessary after NLL hits stable.
fn read_lines<T: BufRead>(buf: T) -> InputLines {
    buf.lines().map(Result::unwrap).collect()
}
