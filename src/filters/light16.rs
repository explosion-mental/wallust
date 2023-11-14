//! # light16
use crate::filters::*;

/// Variation of the [`light`] scheme, but with a 16 variation, similar to how [`dark16`] does it.
/// Sorted by [`DarkFirst`]
pub fn light16(c: &[Myrgb]) -> Colors {
    let mut c = super::light::light(c);

    /* First row */
    c.color1 = c.color1.darken(0.25);
    c.color2 = c.color2.darken(0.25);
    c.color3 = c.color3.darken(0.25);
    c.color4 = c.color4.darken(0.25);
    c.color5 = c.color5.darken(0.25);
    c.color6 = c.color6.darken(0.25);

    c
}
