use std::cmp::Ordering;
use fuzzy::{matches, Score};

/// A Choice wraps a String and it's calculated Score
#[derive(Debug)]
pub struct Choice(String, Score);

impl Choice {
    /// Creates a new Choice if the needle matches the haystack.
    ///
    /// # Examples
    ///
    /// ```
    /// use rff::choice::Choice;
    /// assert!(Choice::new("amo", String::from("app/models/order")).is_some());
    /// ```
    pub fn new(needle: &str, haystack: String) -> Option<Choice> {
        if matches(&needle, &haystack) {
            let score = Score::calculate(&needle, &haystack);
            Some(Choice(haystack, score))
        } else {
            None
        }
    }

    /// Creates a new Choice with derived match positions
    pub fn with_positions(needle: &str, haystack: String) -> Option<Choice> {
        if matches(&needle, &haystack) {
            let score = Score::calculate_with_positions(&needle, &haystack);
            Some(Choice(haystack, score))
        } else {
            None
        }
    }

    /// Gets the Choice's text as a &str.
    pub fn text(&self) -> &str {
        &self.0
    }

    /// Gets a ref to the Choice's positions.
    pub fn positions(&self) -> Option<&Vec<usize>> {
        self.1.positions.as_ref()
    }
}

impl PartialEq for Choice {
    fn eq(&self, other: &Choice) -> bool {
        self.1.eq(&other.1)
    }
}

impl PartialOrd for Choice {
    fn partial_cmp(&self, other: &Choice) -> Option<Ordering> {
        self.1.partial_cmp(&other.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn create_choice(b: &mut Bencher) {
        b.iter(|| Choice::new("app/models", String::from("app/models/order")))
    }

    #[test]
    fn partial_cmp() {
        let a = Choice::new("amor", String::from("app/models/order")).unwrap();
        let b = Choice::new("amor", String::from("app/models/zrder")).unwrap();

        assert!(a > b);
    }
}
