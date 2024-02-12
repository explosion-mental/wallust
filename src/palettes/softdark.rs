//! # softdark

use super::*;

use super::softlight::softlight;

/// Variant of softlight.
/// Uses the lightest colors and a dark background, as opposed to usual [`dark()`].
/// Similar to [`dark()`] but colors in *inversed* order.
/// Modifies the background to match the most prominent color.
/// Sorted by [`LightFirst`],
pub fn softdark(mut c: Cols) -> Colors {
    let orig = c.to_rgb_orig();
    let ee = Myrgb(238, 238, 238); //This is `#EEEEEE`
    c.orig_histo[0].set_luminance(0.3); //top color

    let mut ret = softlight(c);



    //lighten fg to maintain a good contrast and darken a bit the bg (super safe)
    let fg = ret.background.lighten(0.35);

    //let bg = ret.foreground.darken(0.2);
    let bg = orig[0];//.blend(ret.foreground);

    //on `softlight` the lightest color is `.color1`
    //Make sure these colors contrast properly
    ret.color8 = ret.color1.darken(0.3);
    ret.color15 = ret.color1.blend(ee);

    ret.background = bg;
    ret.foreground = fg;

    ret
}
