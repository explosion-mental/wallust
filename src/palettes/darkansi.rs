use super::*;

/// # dark
/// Default method to generate colors.
/// This parser only needs 6 _ [0..=5]. Sorted by [`LightFirst`]
pub fn darkansi(c: Vec<Srgb>, _orig: Vec<Srgb>) -> Colors {
    Colors {
        background : c[0].into(), // background
        foreground : c[7].into(),

        /* First row */
        color0 : c[0].into(), // background
        color1 : c[1].into(),
        color2 : c[2].into(),
        color3 : c[3].into(),
        color4 : c[4].into(),
        color5 : c[5].into(),
        color6 : c[6].into(),
        color7 : c[7].into(), // fg

        /* Second row */
        color8 : c[0].into(), // brighter than col0
        color9 : c[1].into(),
        color10: c[2].into(),
        color11: c[3].into(),
        color12: c[4].into(),
        color13: c[5].into(),
        color14: c[6].into(),
        color15: c[7].into(),
    }
}
