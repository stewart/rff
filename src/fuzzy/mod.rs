mod score;
pub mod consts;

use std::ascii::AsciiExt;
pub use self::score::*;

/// Compares two characters case-insensitively
///
/// # Examples
///
/// ```
/// assert!(rff::fuzzy::eq('a', 'A'));
/// ```
pub fn eq(a: char, b: char) -> bool {
    match a {
        a if a == b => true,
        a if a.is_ascii() && !b.is_ascii() => false,
        a if !a.is_ascii() && b.is_ascii() => false,
        a if a.is_ascii() && b.is_ascii() => {
            a.to_ascii_lowercase().eq(&b.to_ascii_lowercase())
        },
        a => a.to_lowercase().eq(b.to_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_eq() {
        assert!(eq('a', 'A'));
        assert!(eq('山', '山'));
        assert!(!eq('a', 'b'));
    }

    #[bench]
    fn bench_eq_same(b: &mut Bencher) {
        b.iter(|| eq('a', 'a'));
    }

    #[bench]
    fn bench_eq_ascii(b: &mut Bencher) {
        b.iter(|| eq('a', 'A'));
    }

    #[bench]
    fn bench_eq_utf8(b: &mut Bencher) {
        b.iter(|| eq('ø', 'Ø'));
    }

    #[bench]
    fn bench_eq_mixed(b: &mut Bencher) {
        b.iter(|| eq('a', 'Ø'));
    }
}
