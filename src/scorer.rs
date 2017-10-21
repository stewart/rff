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
}
