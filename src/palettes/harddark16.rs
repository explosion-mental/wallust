//! # harddark
use super::*;

/// dark background, light foreground, with dark hard hued colors.
/// Sorted by [`DarkFirst`]
pub fn harddark16(c: Cols) -> Colors {
    super::harddark::harddark(c).to_16col()
}
