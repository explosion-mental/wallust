//! # harddark
use crate::filters::*;

/// dark background, light foreground, with dark hard hued colors.
/// Sorted by [`DarkFirst`]
pub fn harddark16(c: &[Myrgb]) -> Colors {
    super::harddark::harddark(c).to_16col()
}
