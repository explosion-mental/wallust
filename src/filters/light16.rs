//! Variation of the [`light`] scheme, but with a 16 variation, similar to how [`dark16`] does it.
use crate::filters::*;

pub fn light16(c: &[Myrgb]) -> Colors {
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
        color1 : c[5].darken(0.25),
        color2 : c[4].darken(0.25),
        color3 : c[3].darken(0.25),
        color4 : c[2].darken(0.25),
        color5 : c[1].darken(0.25),
        color6 : c[0].darken(0.25),
        color7 : col7, // fg

        /* Second row */
        color8 : col8, // darker than col0
        color9 : c[5],
        color10: c[4],
        color11: c[3],
        color12: c[2],
        color13: c[1],
        color14: c[0],
        color15: col15, //a little darken than col7
    }
}
