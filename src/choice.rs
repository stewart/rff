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

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn create_choice(b: &mut Bencher) {
        b.iter(|| Choice::new("app/models", String::from("app/models/order")))
    }
}
