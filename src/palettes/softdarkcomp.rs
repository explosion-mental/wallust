use super::*;

/// complementary colors variation of softdark
pub fn softdarkcomp(c: Cols) -> Colors {
    softdark::softdark(c).to_comp()
}
