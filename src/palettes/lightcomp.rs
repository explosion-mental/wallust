//! # dark16
use super::*;

/// light complementary variation
pub fn lightcomp(c: Cols) -> Colors {
    light::light(c).to_comp()
}
