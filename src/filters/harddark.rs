//! # harddark
//! dark background, light foreground, with dark hard hued colors.
//! Sorted by [`DarkFirst`]
use crate::filters::*;

pub fn harddark(c: &[Myrgb]) -> Colors {
    let ee = Myrgb(238, 238, 238); //This is `#EEEEEE`

    // This parser only needs 6 colors [0..=5]
    let lightest = c.last().expect("not empty");
    let darkest = c.first().expect("not empty");

    let bg = darkest.darken(0.55);
    let fg = lightest.lighten(0.85);

    let col7  = lightest.blend(ee);
    let col15 = lightest.blend(ee).darken(0.2);

    let col8  = col7.darken(0.30); //color 8 needs to be a bit brighter to contrast color0 and background

    // darken the background color slightly, just like pywal
    let f = format!("{:02x}", darkest.0).chars().last().expect("garanted to have 2 elements by the fmt");
    let col0  = if f != '0' { bg } else { darkest.darken(0.4) };

    Colors {
        background : bg, // background
        foreground : fg,

        /* First row */
        color0 : col0, // background
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
