extern crate libc;

mod consts;
mod ansi;
mod terminal;

pub mod stdin;
pub mod matcher;
pub mod matrix;
pub mod scorer;

pub type Match<'a> = (&'a str, f64);

pub fn match_and_score<'a>(needle: &str, haystack: &'a str) -> Option<Match<'a>> {
    if matcher::matches(needle, haystack) {
        Some((haystack, scorer::score(needle, haystack)))
    } else {
        None
    }
}
