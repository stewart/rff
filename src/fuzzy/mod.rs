mod score;
pub mod consts;

pub use self::score::*;

/// Compares two characters case-insensitively
///
/// # Examples
///
/// ```
/// assert!(rff::fuzzy::eq('a', 'A'));
/// ```
pub fn eq(a: char, b: char) -> bool {
    a.to_uppercase().eq(b.to_uppercase())
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
}
