//! # softdark
//! Variant of softlight.
//! Uses the lightest colors and a dark background, as opposed to usual [`dark()`].
//! Similar to [`dark()`] but colors in *inversed* order.
//! Sorted by [`LightFirst`],

use crate::filters::*;

use super::softlight::softlight;

pub fn softdark(c: &[Myrgb]) -> Colors {

    let mut ret = softlight(c);
    let fg = ret.background;
    let bg = ret.foreground;

    ret.background = bg;
    ret.foreground = fg;

    ret
}
