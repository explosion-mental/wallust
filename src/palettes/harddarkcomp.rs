//! # dark16
use super::*;

/// complementary colors variation of harddark
pub fn harddarkcomp(c: Cols) -> Colors {
    harddark::harddark(c).to_comp()
}
