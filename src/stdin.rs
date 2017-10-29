use std::io::{self, BufRead};

/// A useful alias for the backing storage we parse STDIN into.
pub type InputLines = Vec<String>;

/// Pulls lines of input from STDIN into an `InputLines`.
pub fn slurp() -> InputLines {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    read_lines(stdin)
}

fn read_lines<T: BufRead>(buf: T) -> InputLines {
    buf.lines().map(Result::unwrap).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_lines() {
        let input = b"a\nb\nc";
        let slice = &input[..];

        let expected = [
            String::from("a"),
            String::from("b"),
            String::from("c")
        ];

        assert_eq!(read_lines(slice), expected);
    }
}
