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
    pub value: f32,
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
        Score { value: value }
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
                    score = ((j as f32) * SCORE_GAP_LEADING) + bonus_score;
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

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

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

    #[bench]
    fn bench_score(b: &mut Bencher) {
        b.iter(|| Score::calculate("amor", "app/models/order.rb"))
    }

    #[bench]
    fn bench_score_multiple(b: &mut Bencher) {
        b.iter(|| {
            Score::calculate("amor", "app/models/order.rb");
            Score::calculate("amor", "spec/models/order_spec.rb");
            Score::calculate("amor", "other_garbage.rb");
            Score::calculate("amor", "Gemfile");
            Score::calculate("amor", "node_modules/test/a/thing.js");
            Score::calculate("amor", "vendor/bundle/ruby/gem.rb")
        })
    }

}
