use super::*;

/// # harddark
/// dark background, light foreground, with dark hard hued colors.
/// Sorted by [`DarkFirst`]
pub fn harddark(c: Vec<Srgb>, orig: Vec<Srgb>) -> Colors {
    let bg = orig[0].darken(0.65);

    Colors {
        background : bg.into(),
        /* First row */
        color1 : c[0].into(),
        color2 : c[1].into(),
        color3 : c[2].into(),
        color4 : c[3].into(),
        color5 : c[4].into(),
        color6 : c[5].into(),

        /* Second row */
        color9 : c[0].into(),
        color10: c[1].into(),
        color11: c[2].into(),
        color12: c[3].into(),
        color13: c[4].into(),
        color14: c[5].into(),
        ..dark(c, orig)
    }
}
