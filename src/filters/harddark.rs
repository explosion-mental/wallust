//! # harddark
use crate::filters::*;

/// dark background, light foreground, with dark hard hued colors.
/// Sorted by [`DarkFirst`]
pub fn harddark(co: Cols) -> Colors {
    let c = co.to_rgb();
    let orig = co.to_rgb_orig();
    let bg = orig[0].darken(0.65);

    Colors {
        background : bg,
        /* First row */
        color1 : c[0],
        color2 : c[1],
        color3 : c[2],
        color4 : c[3],
        color5 : c[4],
        color6 : c[5],

        /* Second row */
        color9 : c[0],
        color10: c[1],
        color11: c[2],
        color12: c[3],
        color13: c[4],
        color14: c[5],
        ..super::dark::dark(co)
    }
}
