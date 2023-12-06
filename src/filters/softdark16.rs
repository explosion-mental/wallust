use crate::filters::*;

/// Variant of softlight with 16 colors
pub fn softdark16(c: &[Myrgb]) -> Colors {
    super::softdark::softdark(c).to_16col()
}
