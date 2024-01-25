//! # dark16
use crate::filters::*;

/// complementary colors variation of harddark
pub fn harddarkcomp(c: Cols) -> Colors {
    harddark::harddark(c).to_comp()
}
