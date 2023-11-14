//! # softlight

use crate::filters::*;

/// light background, dark foreground. Uses the lightest colors, might not give enough contrast.
/// Sorted by [`LightFirst`]
pub fn softlight(c: &[Myrgb]) -> Colors {
    // This parser only needs 6 colors [0..=5]
    let lightest = c.last().expect("not empty");
    let darkest = c.first().expect("not empty");

    let bg = lightest.lighten(0.85);
    let fg = darkest.darken(0.55);

    let col7  = darkest.darken(0.55);
    let col15 = darkest.darken(0.85);

    let col8  = lightest.darken(0.3); //color 8 needs to be a bit brighter to contrast color0 and background

    Colors {
        background : bg, // background
        foreground : fg,

        /* First row */
        color0 : *lightest, // background
        color1 : c[0],
        color2 : c[1],
        color3 : c[2],
        color4 : c[3],
        color5 : c[4],
        color6 : c[5],
        color7 : col7, // fg

        /* Second row */
        color8 : col8, // darker than col0
        color9 : c[0],
        color10: c[1],
        color11: c[2],
        color12: c[3],
        color13: c[4],
        color14: c[5],
        color15: col15, //a little darken than col7
    }
}
