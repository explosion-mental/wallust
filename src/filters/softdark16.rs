use crate::filters::*;

/// Variant of softlight with 16 colors
pub fn softdark16(c: Cols) -> Colors {
    let mut ret = softdark::softdark(c).to_16col();
    ret.color15 = ret.color15.lighten(0.5);
    ret
}
