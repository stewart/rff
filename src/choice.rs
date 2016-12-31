use std::cmp::Ordering;
use std::fmt;
use fuzzy::{matches, Score};

/// A Choice wraps a String and it's calculated Score
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

impl fmt::Display for Choice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
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

    #[test]
    fn display() {
        let choice = Choice::new("amor", String::from("app/models/order")).unwrap();
        assert_eq!(format!("{}", choice), "app/models/order");
    }
}
