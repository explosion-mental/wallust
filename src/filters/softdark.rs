//! # softdark

use crate::filters::*;

use super::softlight::softlight;

/// Variant of softlight.
/// Uses the lightest colors and a dark background, as opposed to usual [`dark()`].
/// Similar to [`dark()`] but colors in *inversed* order.
/// Sorted by [`LightFirst`],
pub fn softdark(c: &[Myrgb]) -> Colors {

    let mut ret = softlight(c);

    //lighten fg to maintain a good contrast and darken a bit the bg (super safe)
    let fg = ret.background.lighten(0.35);
    let bg = ret.foreground.darken(0.2);

    ret.background = bg;
    ret.foreground = fg;

    ret
}
