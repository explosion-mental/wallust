use super::*;

/// light complementary variation
pub fn lightcomp16(c: Cols) -> Colors {
    lightcomp::lightcomp(c).to_16col()
}
