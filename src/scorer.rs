// A port of fzy's scoring algorithm.
// fzy (c) 2014 John Hawthorn
// Licensed under the MIT license
// https://github.com/jhawthorn/fzy

use consts::*;

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

    0.0
}

pub fn compute_bonus(haystack: &str) -> Vec<f64> {
    let mut last_char = '/';
    let bonus = Vec::with_capacity(haystack.chars().count());

    haystack.chars().fold(bonus, |mut vec, ch| {
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
