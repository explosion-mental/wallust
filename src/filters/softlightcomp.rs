use crate::filters::*;

/// complementary colors variation of softlight
pub fn softlightcomp(c: Cols) -> Colors {
    softlight::softlight(c).to_comp()
}
