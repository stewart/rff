// A port of selecta's scoring algorithm.
// selecta (c) 2014 John Hawthorn
// Licensed under the MIT license

use std::cmp::Ordering;
use super::eq;
use super::mat::Mat;
use super::bonus::compute_bonus;
use super::consts::*;

/// A score derived from comparing needle and haystack strings, with optional
/// match position information.
#[derive(Debug)]
pub struct Score {
    /// The computed score value
    pub value: f64,

    /// Optional vector of match positions
    pub positions: Option<Vec<usize>>
}

impl Score {
    /// Creates a new Score from the provided needle and haystack
    ///
    /// # Examples
    ///
    /// ```
    /// use rff::fuzzy::Score;
    /// let score = Score::new("abc", "abc");
    /// assert_eq!(score.value, std::f64::INFINITY);
    /// ```
    pub fn new(needle: &str, haystack: &str) -> Score {
        ScoreBuilder::new(needle, haystack).calculate().as_score()
    }

    /// Creates a new Score from the provided needle and haystack, calculating
    /// match positions.
    ///
    /// # Examples
    ///
    /// ```
    /// use rff::fuzzy::Score;
    /// let score = Score::with_positions("abc", "abc");
    /// assert_eq!(score.value, std::f64::INFINITY);
    /// assert_eq!(score.positions, Some(vec![0, 1, 2]));
    /// ```
    pub fn with_positions(needle: &str, haystack: &str) -> Score {
        ScoreBuilder::new(needle, haystack).calculate().with_positions().as_score()
    }
}

impl PartialOrd for Score {
    #[inline]
    fn partial_cmp(&self, other: &Score) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl PartialEq for Score {
    #[inline]
    fn eq(&self, other: &Score) -> bool {
        self.value == other.value
    }
}

struct ScoreBuilder<'a> {
    needle: &'a str,
    haystack: &'a str,
    len_n: usize,
    len_h: usize,
    d: Option<Mat>,
    m: Option<Mat>,
    score: f64,
    positions: Option<Vec<usize>>
}

impl<'a> ScoreBuilder<'a> {
    #[inline]
    fn new(needle: &'a str, haystack: &'a str) -> ScoreBuilder<'a> {
        let len_n = needle.chars().count();
        let len_h = haystack.chars().count();

        ScoreBuilder {
            needle: needle,
            haystack: haystack,
            len_n: len_n,
            len_h: len_h,
            m: None,
            d: None,
            score: 0.0,
            positions: None
        }
    }

    #[inline]
    fn calculate(mut self) -> ScoreBuilder<'a> {
        if self.len_n == 0 {
            self.score = SCORE_MIN;
            return self;
        }

        if self.len_n == self.len_h {
            self.score = SCORE_MAX;
            return self;
        }

        if self.len_h > 1024 {
            // unreasonably large candidate
            self.score = SCORE_MIN;
            return self;
        }

        let bonus = compute_bonus(self.haystack);

        let mut m = Mat::new(self.len_n, self.len_h);
        let mut d = Mat::new(self.len_n, self.len_h);

        for (i, n) in self.needle.chars().enumerate() {
            let mut prev_score = SCORE_MIN;
            let gap_score = if i == self.len_n - 1 { SCORE_GAP_TRAILING } else { SCORE_GAP_INNER };

            for (j, h) in self.haystack.chars().enumerate() {
                if eq(n, h) {
                    let bonus_score = bonus[j];

                    let score = match i {
                        0 => ((j as f64) * SCORE_GAP_LEADING) + bonus_score,
                        _ if j > 0 => {
                            let m = m.get(i - 1, j - 1).unwrap();
                            let d = d.get(i - 1, j - 1).unwrap();

                            let m = m + bonus_score;
                            let d = d + SCORE_MATCH_CONSECUTIVE;

                            (m).max(d)
                        },
                        _ => SCORE_MIN
                    };

                    prev_score = score.max(prev_score + gap_score);

                    d.set(i, j, score);
                    m.set(i, j, prev_score);
                } else {
                    prev_score += gap_score;

                    d.set(i, j, SCORE_MIN);
                    m.set(i, j, prev_score);
                }
            }
        }

        self.score = m.get(self.len_n - 1, self.len_h - 1).unwrap_or(SCORE_MIN);

        self.m = Some(m);
        self.d = Some(d);

        self
    }

    #[inline]
    fn with_positions(mut self) -> ScoreBuilder<'a> {
        if self.len_n == self.len_h {
            self.positions = Some((0..self.len_n).collect());
        }

        if self.len_n == 0 || self.len_n == self.len_h || self.len_h > 1024 {
            return self;
        }

        let mut positions = vec![0 as usize; self.len_n];
        let mut match_required = false;

        {
            let m = self.m.as_ref().unwrap();
            let d = self.d.as_ref().unwrap();

            let mut j = self.len_h - 1;

            for i in (0..self.len_n).rev() {
                while j > (0 as usize) {
                    let last = if i > 0 && j > 0 { d.get(i - 1, j - 1).unwrap() } else { 0.0 };

                    let d = d.get(i, j).unwrap();
                    let m = m.get(i, j).unwrap();

                    if d != SCORE_MIN && (match_required || d == m) {
                        if i > 0 && j > 0 && m == last + SCORE_MATCH_CONSECUTIVE {
                            match_required = true;
                        }

                        positions[i] = j;

                        break;
                    }

                    j -= 1
                }
            }
        }

        self.positions = Some(positions);
        self
    }

    #[inline]
    fn as_score(self) -> Score {
        Score {
            value: self.score,
            positions: self.positions,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn score(needle: &str, haystack: &str) -> Score {
        Score::new(needle, haystack)
    }

    #[test]
    fn test_eq() {
        let a = Score { value: 1.0, positions: None };
        let b = Score { value: 1.0, positions: None };
        assert_eq!(a, b);
    }

    #[test]
    fn test_cmp() {
        let a = Score { value: 2.0, positions: None };
        let b = Score { value: 1.0, positions: None };
        assert!(a > b);
        assert!(b < a);

        let b = Score { value: 2.0, positions: None };
        assert!(a == b);
    }

    #[test]
    fn relative_scores() {
        // App/Models/Order is better than App/MOdels/zRder
        assert!(score("amor", "app/models/order") > score("amor", "app/models/zrder"));

        // App/MOdels/foo is better than App/M/fOo
        assert!(score("amo", "app/m/foo") < score("amo", "app/models/foo"));

        // GEMFIle.Lock < GEMFILe
        assert!(score("gemfil", "Gemfile.lock") < score("gemfil", "Gemfile"));

        // GEMFIle.Lock < GEMFILe
        assert!(score("gemfil", "Gemfile.lock") < score("gemfil", "Gemfile"));

        // Prefer shorter scorees
        assert!(score("abce", "abcdef") > score("abce", "abc de"));

        // Prefer shorter candidates
        assert!(score("test", "tests") > score("test", "testing"));

        // Scores first letter highly
        assert!(score("test", "testing") > score("test", "/testing"));

        // Prefer shorter scorees
        assert!(score("abc", "    a b c ") > score("abc", " a  b  c "));
        assert!(score("abc", " a b c    ") > score("abc", " a  b  c "));
    }

    #[test]
    fn score_utf8() {
        assert_eq!(score("ß", "öäßéè").value, -0.02);
    }

    #[test]
    fn positions() {
        macro_rules! test_positions {
            ($needle:expr, $haystack:expr, $result:expr) => {
                let score = Score::with_positions($needle, $haystack);
                assert_eq!(score.positions, Some($result));
            }
        }

        test_positions!("amo", "app/models/foo", vec![0, 4, 5]);
        test_positions!("amor", "app/models/order", vec![0, 4, 11, 12]);
        test_positions!("as", "tags", vec![1, 3]);
        test_positions!("abc", "a/a/b/c/c", vec![2, 4, 6]);
        test_positions!("foo", "foo", vec![0, 1, 2]);
        test_positions!("drivers", "/path/to/drivers/file.txt", vec![9, 10, 11, 12, 13, 14, 15]);
    }
}
