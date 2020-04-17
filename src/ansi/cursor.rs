use std::fmt::{Display, Formatter, Result};

/// Move the cursor to (row, col) position [(1, 1)-based]
#[derive(Copy, Clone)]
pub struct GoTo(pub u16, pub u16);

impl Display for GoTo {
    fn fmt(&self, f: &mut Formatter) -> Result {
        debug_assert!(self.0 > 0 && self.1 > 0, "ANSI coordinates are 1-based");
        write!(f, csi!("{};{}H"), self.0, self.1)
    }
}

// Move the cursor up N rows
generate_csi_struct!(Up, "A", u16);

// Move the cursor down N rows
generate_csi_struct!(Down, "B", u16);

// Move the cursor left N columns
generate_csi_struct!(Left, "D", u16);

// Move the cursor right N columns
generate_csi_struct!(Right, "C", u16);

// Move the cursor up N lines, and to the beginning of the line
generate_csi_struct!(UpLine, "F", u16);

// Move the cursor down N lines, and to the beginning of the line
generate_csi_struct!(DownLine, "E", u16);

// Set cursor column
#[derive(Copy, Clone)]
pub struct Column(pub u16);

impl Display for Column {
    fn fmt(&self, f: &mut Formatter) -> Result {
        debug_assert!(self.0 > 0, "ANSI coordinates are 1-based");
        write!(f, csi!("{}G"), self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn goto() {
        let s = format!("{}", GoTo(1, 2));
        assert_eq!(s, "\x1b[1;2H");
    }

    #[test]
    fn up() {
        let s = format!("{}", Up(2));
        assert_eq!(s, "\x1b[2A");
    }

    #[test]
    fn down() {
        let s = format!("{}", Down(2));
        assert_eq!(s, "\x1b[2B");
    }

    #[test]
    fn left() {
        let s = format!("{}", Left(2));
        assert_eq!(s, "\x1b[2D");
    }

    #[test]
    fn right() {
        let s = format!("{}", Right(2));
        assert_eq!(s, "\x1b[2C");
    }

    #[test]
    fn up_line() {
        let s = format!("{}", UpLine(2));
        assert_eq!(s, "\x1b[2F");
    }

    #[test]
    fn down_line() {
        let s = format!("{}", DownLine(2));
        assert_eq!(s, "\x1b[2E");
    }

    #[test]
    fn column() {
        let s = format!("{}", Column(1));
        assert_eq!(s, "\x1b[1G");
    }
}
