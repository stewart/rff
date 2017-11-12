extern crate libc;
extern crate rayon;

mod consts;
mod ansi;
mod terminal;

pub mod stdin;
pub mod matcher;
pub mod matrix;
pub mod scorer;
pub mod interface;

pub type Match<'a> = (&'a str, f64);
pub type MatchWithPositions<'a> = (&'a str, f64, Vec<usize>);

pub fn match_and_score<'a>(needle: &str, haystack: &'a str) -> Option<Match<'a>> {
    if matcher::matches(needle, haystack) {
        Some((haystack, scorer::score(needle, haystack)))
    } else {
        None
    }
}

pub fn match_and_score_with_positions<'a>(needle: &str, haystack: &'a str) -> Option<MatchWithPositions<'a>> {
    if matcher::matches(needle, haystack) {
        let (score, positions) = scorer::score_with_positions(needle, haystack);
        Some((haystack, score, positions))
    } else {
        None
    }
}
