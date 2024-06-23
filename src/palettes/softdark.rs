use super::*;

/// # softdark
/// Variant of softlight.
/// Uses the lightest colors and a dark background, as opposed to usual [`dark()`].
/// Similar to [`dark()`] but colors in *inversed* order.
/// Modifies the background to match the most prominent color.
/// Sorted by [`LightFirst`],
pub fn softdark(c: Vec<Srgb>, orig: Vec<Srgb>) -> Colors {
    use palette::IntoColor;
    use palette::Saturate;

    //let orig = c.to_rgb_orig();
    let ee = Srgb::<u8>::new(238, 238, 238); //This is `#EEEEEE`

    //let bg = ret.foreground.darken(0.2);
    //let bg = orig[0].lighten(0.3);//.blend(ret.foreground);

    // let bg = Myrgb(orig[0]).saturate(0.2);
    // let bg: Srgb = bg.0;

    let mut bg: palette::Hsl = orig[0].into_color();
    //ensure the background is darken ENOUGH
    if bg.lightness  > 0.5 { bg = bg.darken(0.5); }
    if bg.saturation < 0.5 { bg = bg.saturate(0.15); }

    let bg: Srgb = bg.into_color();

    let mut ret = softlight(c, orig);
    //lighten fg to maintain a good contrast and darken a bit the bg (super safe)
    let fg = ret.background.lighten(0.35);


    //on `softlight` the lightest color is `.color1`
    //Make sure these colors contrast properly
    ret.color8 = ret.color1.darken(0.3);
    ret.color15 = ret.color1.blend(ee.into());

    ret.background = bg.into();
    ret.foreground = fg.into();

    ret
}
