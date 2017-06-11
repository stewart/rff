use super::consts::*;

/// Calculates a vector of match bonuses from the provided string
#[inline]
pub fn compute_bonus(haystack: &str) -> Vec<f64> {
    let mut last_char = '/';
    let bonus = Vec::with_capacity(haystack.chars().count());

    haystack.chars().fold(bonus, |mut vec, ch| {
        vec.push(for_char(last_char, ch));
        last_char = ch;
        vec
    })
}

pub fn for_char(prev: char, current: char) -> f64 {
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
    fn test_compute_bonus() {
        assert_eq!(compute_bonus("a/b/c/d"), vec![0.9, 0.0, 0.9, 0.0, 0.9, 0.0, 0.9]);
        assert_eq!(compute_bonus("aTestString"), vec![0.9, 0.7, 0.0, 0.0, 0.0, 0.7, 0.0, 0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_for_char() {
        assert_eq!(for_char('*', '*'), 0.0);
        assert_eq!(for_char('a', 'a'), 0.0);

        assert_eq!(for_char('/', 'a'), SCORE_MATCH_SLASH);
        assert_eq!(for_char('/', 'A'), SCORE_MATCH_SLASH);
        assert_eq!(for_char('/', '0'), SCORE_MATCH_SLASH);

        assert_eq!(for_char('-', 'a'), SCORE_MATCH_WORD);
        assert_eq!(for_char('-', 'A'), SCORE_MATCH_WORD);
        assert_eq!(for_char('-', '0'), SCORE_MATCH_WORD);

        assert_eq!(for_char('_', 'a'), SCORE_MATCH_WORD);
        assert_eq!(for_char('_', 'A'), SCORE_MATCH_WORD);
        assert_eq!(for_char('_', '0'), SCORE_MATCH_WORD);

        assert_eq!(for_char(' ', 'a'), SCORE_MATCH_WORD);
        assert_eq!(for_char(' ', 'A'), SCORE_MATCH_WORD);
        assert_eq!(for_char(' ', '0'), SCORE_MATCH_WORD);

        assert_eq!(for_char('.', 'a'), SCORE_MATCH_DOT);
        assert_eq!(for_char('.', 'A'), SCORE_MATCH_DOT);
        assert_eq!(for_char('.', '0'), SCORE_MATCH_DOT);

        assert_eq!(for_char('a', 'A'), SCORE_MATCH_CAPITAL);
    }
}
