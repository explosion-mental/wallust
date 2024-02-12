use super::*;

/// complementary colors variation of softlight
pub fn softlightcomp16(c: Cols) -> Colors {
    softlightcomp::softlightcomp(c).to_16col()
}
