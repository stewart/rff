// A port of fzy's scoring algorithm.
// fzy (c) 2014 John Hawthorn
// Licensed under the MIT license
// https://github.com/jhawthorn/fzy

use consts::*;
use matcher::eq;
use matrix::Matrix;

pub fn score(needle: &str, haystack: &str) -> f64 {
    let needle_length = needle.chars().count();

    // empty needle
    if needle_length == 0 {
        return SCORE_MIN;
    }

    let haystack_length = haystack.chars().count();

    // perfect match
    if needle_length == haystack_length {
        return SCORE_MAX;
    }

    // unreasonably large haystack
    if haystack_length > 1024 {
        return SCORE_MIN;
    }

    let bonus = compute_bonus(haystack);

    let mut m = Matrix::new(needle_length, haystack_length);
    let mut d = Matrix::new(needle_length, haystack_length);

    for (i, n) in needle.chars().enumerate() {
        let mut prev_score = SCORE_MIN;
        let gap_score = if i == needle_length - 1 { SCORE_GAP_TRAILING } else { SCORE_GAP_INNER };

        for (j, h) in haystack.chars().enumerate() {
            if eq(n, h) {
                let bonus_score = bonus[j];

                let score = match i {
                    0 => ((j as f64) * SCORE_GAP_LEADING) + bonus_score,
                    _ if j > 0 => {
                        let m = m.get(i - 1, j - 1);
                        let d = d.get(i - 1, j - 1);

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

    *m.get(needle_length - 1, haystack_length - 1)
}

pub fn compute_bonus(haystack: &str) -> Vec<f64> {
    let mut last_char = '/';

    let (_, len) = haystack.chars().size_hint();
    let len = len.unwrap_or_else(|| haystack.chars().count());

    haystack.chars().fold(Vec::with_capacity(len), |mut vec, ch| {
        vec.push(bonus_for_char(last_char, ch));
        last_char = ch;
        vec
    })
}

fn bonus_for_char(prev: char, current: char) -> f64 {
    match current {
        'a' ... 'z' | '0' ... '9' => bonus_for_prev(prev),
        'A' ... 'Z' => {
            match prev {
                'a' ... 'z' => SCORE_MATCH_CAPITAL,
                _ => bonus_for_prev(prev)
            }
        }
        _ => 0.0
    }
}

fn bonus_for_prev(ch: char) -> f64 {
    match ch {
        '/' => SCORE_MATCH_SLASH,
        '-' | '_' | ' ' => SCORE_MATCH_WORD,
        '.' => SCORE_MATCH_DOT,
        _ => 0.0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_basic() {
        assert_eq!(score("", "asdf"), SCORE_MIN);
        assert_eq!(score("asdf", "asdf"), SCORE_MAX);

        let huge_string = "X".repeat(1025);
        assert_eq!(score("asdf", &huge_string), SCORE_MIN);
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
        assert_eq!(score("ß", "öäßéè"), -0.02);
    }

    #[test]
    fn test_compute_bonus() {
        assert_eq!(compute_bonus("a/b/c/d"), vec![0.9, 0.0, 0.9, 0.0, 0.9, 0.0, 0.9]);
        assert_eq!(compute_bonus("aTestString"), vec![0.9, 0.7, 0.0, 0.0, 0.0, 0.7, 0.0, 0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_for_char() {
        assert_eq!(bonus_for_char('*', '*'), 0.0);
        assert_eq!(bonus_for_char('a', 'a'), 0.0);

        assert_eq!(bonus_for_char('/', 'a'), SCORE_MATCH_SLASH);
        assert_eq!(bonus_for_char('/', 'A'), SCORE_MATCH_SLASH);
        assert_eq!(bonus_for_char('/', '0'), SCORE_MATCH_SLASH);

        assert_eq!(bonus_for_char('-', 'a'), SCORE_MATCH_WORD);
        assert_eq!(bonus_for_char('-', 'A'), SCORE_MATCH_WORD);
        assert_eq!(bonus_for_char('-', '0'), SCORE_MATCH_WORD);

        assert_eq!(bonus_for_char('_', 'a'), SCORE_MATCH_WORD);
        assert_eq!(bonus_for_char('_', 'A'), SCORE_MATCH_WORD);
        assert_eq!(bonus_for_char('_', '0'), SCORE_MATCH_WORD);

        assert_eq!(bonus_for_char(' ', 'a'), SCORE_MATCH_WORD);
        assert_eq!(bonus_for_char(' ', 'A'), SCORE_MATCH_WORD);
        assert_eq!(bonus_for_char(' ', '0'), SCORE_MATCH_WORD);

        assert_eq!(bonus_for_char('.', 'a'), SCORE_MATCH_DOT);
        assert_eq!(bonus_for_char('.', 'A'), SCORE_MATCH_DOT);
        assert_eq!(bonus_for_char('.', '0'), SCORE_MATCH_DOT);

        assert_eq!(bonus_for_char('a', 'A'), SCORE_MATCH_CAPITAL);
    }
}
