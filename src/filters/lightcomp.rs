//! # dark16
use crate::filters::*;

/// light complementary variation
pub fn lightcomp(c: Cols) -> Colors {
    light::light(c).to_comp()
}
