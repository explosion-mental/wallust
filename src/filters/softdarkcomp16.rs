use crate::filters::*;

/// complementary colors variation of softdark with 16 colors
pub fn softdarkcomp16(c: Cols) -> Colors {
    softdarkcomp::softdarkcomp(c).to_16col()
}
