// A port of selecta's scoring algorithm parsing.
// selecta (c) 2014 John Hawthorn
// Licensed under the MIT license

use std::cmp::Ordering;
use super::eq;
use super::mat::Mat;
use super::bonus::compute_bonus;
use super::consts::*;

#[derive(Debug)]
pub struct Score {
    /// The computed score value
    pub value: f64,

    /// Optional vector of match positions
    pub positions: Option<Vec<usize>>
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
    pub fn new(value: f64) -> Score {
        Score { value: value, positions: None }
    }

    /// Creates a new Score with the provided value and match positions
    ///
    /// # Examples
    ///
    /// ```
    /// let score = rff::fuzzy::Score::with_positions(1.0, vec![1, 2, 3]);
    /// assert_eq!(score.value, 1.0);
    /// assert_eq!(score.positions, Some(vec![1, 2, 3]));
    /// ```
    pub fn with_positions(value: f64, positions: Vec<usize>) -> Score {
        Score { value: value, positions: Some(positions) }
    }

    /// Creates a new Score, with value derived from provided needle / haystack
    pub fn calculate(needle: &str, haystack: &str) -> Score {
        let len_n = needle.len();
        let len_h = haystack.len();

        if len_n == 0 { return Score::new(SCORE_MIN); }
        if len_n == len_h { return Score::new(SCORE_MAX); }

        let (m, _) = generate_score_matrices(needle, haystack);

        let score = m.get(len_n - 1, len_h - 1).unwrap_or(SCORE_MIN);
        Score::new(score)
    }

    /// Calculates match positions of needle's chars, in addition to the score
    pub fn calculate_with_positions(needle: &str, haystack: &str) -> Score {
        let len_n = needle.len();
        let len_h = haystack.len();

        if len_n == 0 { return Score::new(SCORE_MIN); }

        if len_n == len_h {
            let positions = (0..len_n).collect();
            return Score::with_positions(SCORE_MAX, positions);
        }

        let (m, d) = generate_score_matrices(needle, haystack);

        let score = m.get(len_n - 1, len_h - 1).unwrap_or(SCORE_MIN);
        let positions = derive_match_positions(len_n, len_h, m, d);

        Score::with_positions(score, positions)
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

#[inline]
fn generate_score_matrices(needle: &str, haystack: &str) -> (Mat, Mat) {
    let len_n = needle.len();
    let len_h = haystack.len();

    let bonus = compute_bonus(haystack);

    let mut d = Mat::new(len_n, len_h);
    let mut m = Mat::new(len_n, len_h);

    for (i, n) in needle.chars().enumerate() {
        let mut prev_score = SCORE_MIN;
        let gap_score = if i == len_n - 1 { SCORE_GAP_TRAILING } else { SCORE_GAP_INNER };

        for (j, h) in haystack.chars().enumerate() {
            if eq(n, h) {
                let mut score = SCORE_MIN;

                let bonus_score = bonus[j];

                if i == 0 {
                    score = ((j as f64) * SCORE_GAP_LEADING) + bonus_score;
                } else if j > 0 {
                    let m = m.get(i - 1, j - 1).unwrap();
                    let d = d.get(i - 1, j - 1).unwrap();

                    score = (m + bonus_score).max(d + SCORE_MATCH_CONSECUTIVE);
                }

                prev_score = score.max(prev_score + gap_score);

                d.set(i, j, score).unwrap();
                m.set(i, j, prev_score).unwrap();
            } else {
                d.set(i, j, SCORE_MIN).unwrap();
                m.set(i, j, prev_score + gap_score).unwrap();
                prev_score += gap_score;
            }
        }
    }

    (m, d)
}

/// Given the length of the input strings, and generated scoring matrices,
/// generates a len_n vector of optimal match positions for each haystack char.
#[inline]
fn derive_match_positions(len_n: usize, len_h: usize, m: Mat, d: Mat) -> Vec<usize> {
    let mut positions = vec![0 as usize; len_n];
    let mut match_required = false;

    let mut j = len_h - 1;

    for i in (0..len_n).rev() {
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

    positions
}

#[cfg(test)]
mod tests {
    use super::*;

    fn score(needle: &str, haystack: &str) -> Score {
        Score::calculate(needle, haystack)
    }

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
    fn positions() {
        macro_rules! test_positions {
            ($needle:expr, $haystack:expr, $result:expr) => {
                let score = Score::calculate_with_positions($needle, $haystack);
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
