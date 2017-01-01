use std::fmt::{Display, Formatter, Result};

generate_csi_struct!(AfterCursor, "K");
generate_csi_struct!(BeforeCursor, "1K");
generate_csi_struct!(Line, "2K");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn after_cursor() {
        assert_eq!(format!("{}", AfterCursor), "\x1b[K");
    }

    #[test]
    fn before_cursor() {
        assert_eq!(format!("{}", BeforeCursor), "\x1b[1K");
    }

    #[test]
    fn line() {
        assert_eq!(format!("{}", Line), "\x1b[2K");
    }
}
