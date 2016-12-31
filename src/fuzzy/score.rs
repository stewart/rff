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

#[cfg(test)]
mod tests {
    use super::*;
}
