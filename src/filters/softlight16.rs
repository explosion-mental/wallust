
use crate::filters::*;

/// softlight 16 variation scheme
pub fn softlight16(c: Cols) -> Colors {
    super::softlight::softlight(c).to_16col()
}
