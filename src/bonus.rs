const SCORE_MATCH_SLASH: f64 = 0.9;
const SCORE_MATCH_WORD: f64 = 0.8;
const SCORE_MATCH_CAPITAL: f64 = 0.7;
const SCORE_MATCH_DOT: f64 = 0.6;

/// Calculates per-character score bonuses for the provided `haystack`.
pub fn compute(haystack: &str) -> Vec<f64> {
    let mut prev = '/';
    let mut bonus = vec![];

    for current in haystack.chars() {
        bonus.push(for_char(prev, current));
        prev = current;
    }

    bonus
}

/// Derives a score bonus for `current`.
fn for_char(prev: char, current: char) -> f64 {
    match prev {
        '-' | '_' | ' ' => SCORE_MATCH_WORD,
        '/' => SCORE_MATCH_SLASH,
        '.' => SCORE_MATCH_DOT,
        'a'...'z' => match current {
            'A'...'Z' => SCORE_MATCH_CAPITAL,
            _ => 0.0,
        },
        _ => 0.0,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_compute() {
        assert_eq!(
            compute("abcd"),
            vec![SCORE_MATCH_SLASH, 0.0, 0.0, 0.0]
        );

        assert_eq!(
            compute("ab-c_d eF"),
            vec![
                SCORE_MATCH_SLASH,
                0.0,
                0.0,
                SCORE_MATCH_WORD,
                0.0,
                SCORE_MATCH_WORD,
                0.0,
                SCORE_MATCH_WORD,
                SCORE_MATCH_CAPITAL
            ]
        );

        assert_eq!(
            compute("app/models/product.rb"),
            vec![
                SCORE_MATCH_SLASH,
                0.0,
                0.0,
                0.0,
                SCORE_MATCH_SLASH,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                SCORE_MATCH_SLASH,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                SCORE_MATCH_DOT,
                0.0
            ]
        );
    }

    #[test]
    fn test_for_char() {
        macro_rules! assert_bonus {
            ($prev: expr, $current: expr, $expected: expr) => {
                assert_eq!(for_char($prev, $current), $expected);
            };
        }

        // current is alphanumeric, lowercase
        assert_bonus!(' ', 'a', SCORE_MATCH_WORD);
        assert_bonus!('-', 'a', SCORE_MATCH_WORD);
        assert_bonus!('_', 'a', SCORE_MATCH_WORD);
        assert_bonus!('.', 'a', SCORE_MATCH_DOT);
        assert_bonus!('/', 'a', SCORE_MATCH_SLASH);
        assert_bonus!('a', 'a', 0.0);
        assert_bonus!('A', 'a', 0.0);

        // current is alphanumeric, uppercase
        assert_bonus!(' ', 'A', SCORE_MATCH_WORD);
        assert_bonus!('-', 'A', SCORE_MATCH_WORD);
        assert_bonus!('_', 'A', SCORE_MATCH_WORD);
        assert_bonus!('.', 'A', SCORE_MATCH_DOT);
        assert_bonus!('/', 'A', SCORE_MATCH_SLASH);
        assert_bonus!('a', 'A', SCORE_MATCH_CAPITAL);
        assert_bonus!('A', 'A', 0.0);

        // current is digit
        assert_bonus!(' ', '0', SCORE_MATCH_WORD);
        assert_bonus!('-', '0', SCORE_MATCH_WORD);
        assert_bonus!('_', '0', SCORE_MATCH_WORD);
        assert_bonus!('.', '0', SCORE_MATCH_DOT);
        assert_bonus!('/', '0', SCORE_MATCH_SLASH);
        assert_bonus!('a', '0', 0.0);
        assert_bonus!('A', '0', 0.0);
    }
}
