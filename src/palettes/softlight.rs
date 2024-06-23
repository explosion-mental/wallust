use super::*;

/// # softlight
/// light background, dark foreground. Uses the lightest colors, might not give enough contrast.
/// Sorted by [`LightFirst`]
pub fn softlight(c: Vec<Srgb>, _orig: Vec<Srgb>) -> Colors {
    Colors {
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
        ..light(c, _orig)
    }
}
