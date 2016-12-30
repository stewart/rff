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
#[inline(always)]
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

/// Searches for needle's chars in the haystack
///
/// # Examples
///
/// ```
/// assert!(rff::fuzzy::matches("amo", "app/models/order"));
/// ```
pub fn matches(needle: &str, haystack: &str) -> bool {
    let mut hchars = haystack.chars();

    'outer: for n in needle.chars() {
        loop {
            match hchars.next() {
                Some(h) if eq(h, n) => continue 'outer,
                Some(_) => continue,
                None => return false
            }
        }
    }

    true
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

    #[test]
    fn test_matches() {
        assert!(matches("a", "a"));
        assert!(matches("a", "abc"));
        assert!(matches("abc", "abc"));
        assert!(matches("ABC", "abc"));
        assert!(matches("abc", "a1b2c3"));
        assert!(matches("abc", "a1b2c3"));
        assert!(matches("test", "t/e/s/t"));
        assert!(matches("test", "t💣e💣s💣t"));
        assert!(matches("💣💣💣", "t💣e💣s💣t"));

        assert!(!matches("abc", "ab"));
        assert!(!matches("abc", "cab"));
        assert!(!matches("abc", ""));

        assert!(matches("", ""));
        assert!(matches("", "ab"));

        // UTF-8 case testing
        assert!(matches("a", "A"));
        assert!(matches("A", "a"));
        assert!(matches("山", "山"));
        assert!(matches("café", "CAFÉ"))
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

    #[bench]
    fn bench_matches(b: &mut Bencher) {
        b.iter(|| matches("amor", "app/models/order.rb"))
    }

    #[bench]
    fn bench_matches_mixed_case(b: &mut Bencher) {
        b.iter(|| matches("AMOr", "App/Models/Order.rb"))
    }

    #[bench]
    fn bench_matches_multiple(b: &mut Bencher) {
        b.iter(|| {
            matches("amor", "app/models/order.rb");
            matches("amor", "spec/models/order_spec.rb");
            matches("amor", "other_garbage.rb");
            matches("amor", "Gemfile");
            matches("amor", "node_modules/test/a/thing.js");
            matches("amor", "vendor/bundle/ruby/gem.rb")
        })
    }
}
