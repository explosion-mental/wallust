//! # softdark

use crate::filters::*;

use super::softlight::softlight;

/// Variant of softlight.
/// Uses the lightest colors and a dark background, as opposed to usual [`dark()`].
/// Similar to [`dark()`] but colors in *inversed* order.
/// Modifies the background to match the most prominent color.
/// Sorted by [`LightFirst`],
pub fn softdark(c: Cols) -> Colors {
    let orig = c.to_rgb_orig();
    let mut ret = softlight(c);

    //lighten fg to maintain a good contrast and darken a bit the bg (super safe)
    let fg = ret.background.lighten(0.35);
    //let bg = ret.foreground.darken(0.2);
    let bg = orig[0].blend(ret.foreground).darken(0.4);

    ret.background = bg;
    ret.foreground = fg;

    ret
}
