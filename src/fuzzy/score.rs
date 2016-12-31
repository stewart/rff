use std::cmp::Ordering;

#[derive(Copy, Clone, Debug)]
pub struct Score {
    /// The computed score value
    pub value: f32
}

impl Score {
    /// Creates a new Score with the provided value
    ///
    /// # Examples
    ///
    /// ```
    /// let score = rff::fuzzy::Score::new(1.0);
    /// assert_eq!(score.value, 1.0);
    /// ```
    pub fn new(value: f32) -> Score {
        Score {
            value: value
        }
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Score) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl PartialEq for Score {
    fn eq(&self, other: &Score) -> bool {
        self.value == other.value
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq() {
        let a = Score::new(1.0);
        let b = Score::new(1.0);
        assert_eq!(a, b);
    }

    #[test]
    fn test_cmp() {
        let a = Score::new(2.0);
        let b = Score::new(1.0);
        assert!(a > b);
        assert!(b < a);

        let b = Score::new(2.0);
        assert!(a == b);
    }
}
