//! # harddark
use crate::filters::*;

/// dark background, light foreground, with dark hard hued colors.
/// Sorted by [`DarkFirst`]
pub fn harddark(c: &[Myrgb]) -> Colors {
    //scheme
    let mut s = super::dark::dark(c);

    // invert the order of the colors
    /* first row */
    s.color1 = c[0];
    s.color2 = c[1];
    s.color3 = c[2];
    s.color4 = c[3];
    s.color5 = c[4];
    s.color6 = c[5];

    /* second row */
    s.color9 = c[0];
    s.color10= c[1];
    s.color11= c[2];
    s.color12= c[3];
    s.color13= c[4];
    s.color14= c[5];

    s
}
