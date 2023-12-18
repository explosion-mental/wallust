//! # light16
use crate::filters::*;

/// Variation of the [`light`] scheme, but with a 16 variation, similar to how [`dark16`] does it.
/// Sorted by [`DarkFirst`]
pub fn light16(c: Cols) -> Colors {
    super::light::light(c).to_16col()
}
