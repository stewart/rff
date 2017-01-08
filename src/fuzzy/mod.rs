mod mat;
mod score;
pub mod bonus;
pub mod consts;

use std::ascii::AsciiExt;
pub use self::score::Score;

/// Searches for needle's chars in the haystack
///
/// # Examples
///
/// ```
/// assert!(rff::fuzzy::matches("amo", "app/models/order"));
/// ```
#[inline]
pub fn matches(needle: &str, haystack: &str) -> bool {
    if needle == "" || needle == haystack {
        return true;
    }

    let mut hchars = haystack.chars();

    'outer: for n in needle.chars() {
        while let Some(h) = hchars.next() {
            if eq(n, h) {
                continue 'outer
            }
        }

        return false
    }

    true
}

/// Compares two characters case-insensitively
#[inline(always)]
fn eq(a: char, b: char) -> bool {
    match a {
        _ if a == b => true,
        _ if a.is_ascii() || b.is_ascii() => a.eq_ignore_ascii_case(&b),
        _ => a.to_lowercase().eq(b.to_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert!(matches("café", "CAFÉ"));
        assert!(matches("weiß", "WEIẞ"));
    }
}
