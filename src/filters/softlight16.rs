
use crate::filters::*;

/// softlight 16 variation scheme
pub fn softlight16(c: &[Myrgb]) -> Colors {
    super::softlight::softlight(c).to_16col()
}
