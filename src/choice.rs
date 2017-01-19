use std::cmp::Ordering;
use fuzzy::{matches, Score};

/// A Choice wraps a str and its derived Score.
#[derive(Debug)]
pub struct Choice<'a>(pub &'a str, Score);

impl<'a> Choice<'a> {
    /// Creates a new Choice if the needle matches the haystack.
    ///
    /// # Examples
    ///
    /// ```
    /// assert!(rff::Choice::new("amo", "app/models/order").is_some());
    /// ```
    pub fn new(needle: &str, haystack: &'a str) -> Option<Choice<'a>> {
        if matches(&needle, &haystack) {
            let score = Score::new(&needle, &haystack);
            Some(Choice(haystack, score))
        } else {
            None
        }
    }

    /// Creates a new Choice with derived match positions
    pub fn with_positions(needle: &str, haystack: &'a str) -> Option<Choice<'a>> {
        if matches(&needle, &haystack) {
            let score = Score::with_positions(&needle, &haystack);
            Some(Choice(haystack, score))
        } else {
            None
        }
    }

    /// Gets a ref to the Choice's positions.
    pub fn positions(&self) -> Option<&Vec<usize>> {
        self.1.positions.as_ref()
    }
}

impl<'a> PartialEq for Choice<'a> {
    fn eq(&self, other: &Choice) -> bool {
        self.1.eq(&other.1)
    }
}

impl<'a> PartialOrd for Choice<'a> {
    fn partial_cmp(&self, other: &Choice) -> Option<Ordering> {
        self.1.partial_cmp(&other.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partial_cmp() {
        let a = Choice::new("amor", "app/models/order").unwrap();
        let b = Choice::new("amor", "app/models/zrder").unwrap();

        assert!(a > b);
    }
}
