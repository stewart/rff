//! Useful constants for score calculations.

use std::f64::{INFINITY, NEG_INFINITY};

pub const SCORE_MAX:               f64 = INFINITY;
pub const SCORE_MIN:               f64 = NEG_INFINITY;
pub const SCORE_GAP_LEADING:       f64 = -0.005;
pub const SCORE_GAP_TRAILING:      f64 = -0.005;
pub const SCORE_GAP_INNER:         f64 = -0.01;
pub const SCORE_MATCH_CONSECUTIVE: f64 = 1.0;
pub const SCORE_MATCH_SLASH:       f64 = 0.9;
pub const SCORE_MATCH_WORD:        f64 = 0.8;
pub const SCORE_MATCH_CAPITAL:     f64 = 0.7;
pub const SCORE_MATCH_DOT:         f64 = 0.6;
