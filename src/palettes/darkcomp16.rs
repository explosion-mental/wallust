use super::*;

pub fn darkcomp16(c: Cols) -> Colors {
    darkcomp::darkcomp(c).to_16col()
}
