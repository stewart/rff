// Shortcut for generating an esc-prefixed CSI sequence
macro_rules! csi {
    ($( $l:expr ),*) => {
        concat!("\x1b[", $( $l ),*)
    };
}

// Shortcut for generating a struct that prints as a CSI sequence
macro_rules! generate_csi_struct {
    ($name:ident, $value:expr) => {
        #[derive(Copy, Clone)]
        pub struct $name;

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter) -> Result {
                write!(f, csi!($value))
            }
        }
    };

    ($name:ident, $value:expr, u16) => {
        #[derive(Copy, Clone)]
        pub struct $name(pub u16);

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter) -> Result {
                write!(f, csi!("{}", $value), self.0)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn csi() {
        assert_eq!(csi!("123"), "\x1b[123");
    }
}
