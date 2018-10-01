#![deny(unused_must_use)]

mod bonus;

const SCORE_MAX: f64 = std::f64::INFINITY;
const SCORE_MIN: f64 = std::f64::NEG_INFINITY;

/// Returns true if `needle` fuzzily matches `haystack` - i.e., all the
/// characters in `needle` are present in `haystack` in the same order.
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

    // compares two characters case-insensitively
    // prefers direct and ascii-only comparison when possible for performance
    let eq = |a: char, b: char| -> bool {
        match a {
            _ if a == b => true,
            _ if a.is_ascii() || b.is_ascii() => a.eq_ignore_ascii_case(&b),
            _ => a.to_lowercase().eq(b.to_lowercase()),
        }
    };

    needle.chars().all(|n| hchars.any(|h| eq(n, h)))
}

/// Scores the provided strings based on insert-only edit distance.
/// This operates under the assumption that `needle` fuzzily matches `haystack`.
pub fn score(needle: &str, haystack: &str) -> f64 {
    // an empty needle doesn't match anything.
    if needle.is_empty() {
        return SCORE_MIN;
    }

    // if the needle and the haystack are identical, that's perfect.
    if needle == haystack {
        return SCORE_MAX;
    }

    let bonus = bonus::compute(haystack);

    0.0
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

    #[test]
    fn test_score_basic() {
        assert!(SCORE_MAX > SCORE_MIN);

        assert_eq!(score("abc", "abc"), SCORE_MAX);
        assert_eq!(score("", "abc"), SCORE_MIN);
        assert_eq!(score("abc", "def"), 0.0);
    }
}
