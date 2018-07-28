/// Returns true if characters in `needle` occur in `haystack`, in the same order.
///
/// # Examples
///
/// ```
/// assert!(rff::matches("srclib", "src/lib.rs"));
/// assert!(rff::matches("amo", "app/models/order.rb"));
/// assert!(!rff::matches("oma", "app/models/order.rb"));
/// ```
pub fn matches(needle: &str, haystack: &str) -> bool {
    if needle == "" || needle == haystack {
        return true;
    }

    let mut hchars = haystack.chars();

    // compares characters case-insensitively
    // prefers ascii-only comparison for speed
    let eq = |a: char, b: char| -> bool {
        match a {
            _ if a == b => true,
            _ if a.is_ascii() || b.is_ascii() => a.eq_ignore_ascii_case(&b),
            _ => a.to_lowercase().eq(b.to_lowercase()),
        }
    };

    needle.chars().all(|n| hchars.any(|h| eq(n, h)))
}

#[cfg(test)]
mod test {
    use super::*;

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

        // UTF-8
        assert!(matches("a", "A"));
        assert!(matches("A", "a"));
        assert!(matches("山", "山"));
        assert!(matches("café", "CAFÉ"));
        assert!(matches("weiß", "WEIẞ"));
        assert!(matches("хди́ь", "ХОДИ́ТЬ"));
    }
}
