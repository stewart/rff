use std::fmt::{Display, Formatter, Result};

generate_csi_struct!(Reset, "m");

generate_csi_struct!(Bold, "1m");
generate_csi_struct!(Italic, "3m");
generate_csi_struct!(Underline, "4m");
generate_csi_struct!(Invert, "7m");

generate_csi_struct!(NoBold, "21m");
generate_csi_struct!(NoItalic, "23m");
generate_csi_struct!(NoUnderline, "24m");
generate_csi_struct!(NoInvert, "27m");

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_style {
        ($test: ident, $name: ident, $value: expr) => {
            #[test]
            fn $test() {
                let result = format!("{}", $name);
                let expected = concat!("\x1b[", $value);
                assert_eq!(result, expected);
            }
        };
    }

    test_style!(reset, Reset, "m");
    test_style!(bold, Bold, "1m");
    test_style!(italic, Italic, "3m");
    test_style!(underline, Underline, "4m");
    test_style!(invert, Invert, "7m");
    test_style!(no_bold, NoBold, "21m");
    test_style!(no_italic, NoItalic, "23m");
    test_style!(no_underline, NoUnderline, "24m");
    test_style!(no_invert, NoInvert, "27m");
}
