use std::fmt::{Display, Formatter, Result};

// Clears from the cursor to the end of the line
generate_csi_struct!(AfterCursor, "K");

// Clears from the cursor to the beginning of the line
generate_csi_struct!(BeforeCursor, "1K");

// Clears the current line
generate_csi_struct!(Line, "2K");

// Clears from the cursor until the end of the screen
generate_csi_struct!(Screen, "J");

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

    #[test]
    fn screen() {
        assert_eq!(format!("{}", Screen), "\x1b[J");
    }
}
