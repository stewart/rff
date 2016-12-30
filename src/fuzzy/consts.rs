//! Useful constants for score calculations.

use std::f32::{INFINITY, NEG_INFINITY};

pub const SCORE_MAX:               f32 = INFINITY;
pub const SCORE_MIN:               f32 = NEG_INFINITY;
pub const SCORE_GAP_LEADING:       f32 = -0.005;
pub const SCORE_GAP_TRAILING:      f32 = -0.005;
pub const SCORE_GAP_INNER:         f32 = -0.01;
pub const SCORE_MATCH_CONSECUTIVE: f32 = 1.0;
pub const SCORE_MATCH_SLASH:       f32 = 0.9;
pub const SCORE_MATCH_WORD:        f32 = 0.8;
pub const SCORE_MATCH_CAPITAL:     f32 = 0.7;
pub const SCORE_MATCH_DOT:         f32 = 0.6;
