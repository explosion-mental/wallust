//! # softlight

use crate::filters::*;

/// light background, dark foreground. Uses the lightest colors, might not give enough contrast.
/// Sorted by [`LightFirst`]
pub fn softlight(co: Cols) -> Colors {
    let c = co.clone().to_rgb();
    Colors {
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
        ..super::light::light(co)
    }
}
