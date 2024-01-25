use crate::filters::*;

/// complementary colors variation of harddark with the 16 color variation
pub fn harddarkcomp16(c: Cols) -> Colors {
    harddarkcomp::harddarkcomp(c).to_16col()
}
