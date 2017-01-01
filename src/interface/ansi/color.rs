use std::fmt::{Display, Formatter, Result};

pub trait Color {
    fn write_fg(&self, f: &mut Formatter) -> Result;
    fn write_bg(&self, f: &mut Formatter) -> Result;
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum Colors {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    LightBlack,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    LightWhite
}

impl Color for Colors {
    #[inline]
    fn write_fg(&self, f: &mut Formatter) -> Result {
        write!(f, csi!("38;5;{}m"), *self as u32)
    }

    #[inline]
    fn write_bg(&self, f: &mut Formatter) -> Result {
        write!(f, csi!("48;5;{}m"), *self as u32)
    }
}

impl<'a> Color for &'a Colors {
    #[inline]
    fn write_fg(&self, f: &mut Formatter) -> Result {
        (*self).write_fg(f)
    }

    #[inline]
    fn write_bg(&self, f: &mut Formatter) -> Result {
        (*self).write_bg(f)
    }
}

#[derive(Copy, Clone)]
pub struct Reset;

impl Color for Reset {
    #[inline]
    fn write_fg(&self, f: &mut Formatter) -> Result {
        write!(f, csi!("39m"))
    }

    #[inline]
    fn write_bg(&self, f: &mut Formatter) -> Result {
        write!(f, csi!("49m"))
    }
}

#[derive(Copy, Clone)]
pub struct Fg<C: Color>(pub C);

impl<C: Color> Display for Fg<C> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.0.write_fg(f)
    }
}

#[derive(Copy, Clone)]
pub struct Bg<C: Color>(pub C);

impl<C: Color> Display for Bg<C> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.0.write_bg(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_color {
        ($test: ident, $name: ident, $value: expr) => {
            #[test]
            fn $test() {
                let fg = format!("{}", Fg(Colors::$name));
                let bg = format!("{}", Bg(Colors::$name));

                assert_eq!(fg, concat!("\x1b[38;5;", $value, "m"));
                assert_eq!(bg, concat!("\x1b[48;5;", $value, "m"));
            }
        }
    }

    test_color!(black, Black, 0);
    test_color!(red, Red, 1);
    test_color!(green, Green, 2);
    test_color!(yellow, Yellow, 3);
    test_color!(blue, Blue, 4);
    test_color!(magenta, Magenta, 5);
    test_color!(cyan, Cyan, 6);
    test_color!(white, White, 7);
    test_color!(light_black, LightBlack, 8);
    test_color!(light_red, LightRed, 9);
    test_color!(light_green, LightGreen, 10);
    test_color!(light_yellow, LightYellow, 11);
    test_color!(light_blue, LightBlue, 12);
    test_color!(light_magenta, LightMagenta, 13);
    test_color!(light_cyan, LightCyan, 14);
    test_color!(light_white, LightWhite, 15);

    #[test]
    fn test_reset() {
        let fg = format!("{}", Fg(Reset));
        let bg = format!("{}", Bg(Reset));

        assert_eq!(fg, "\x1b[39m");
        assert_eq!(bg, "\x1b[49m")
    }
}
