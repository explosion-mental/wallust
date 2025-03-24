use super::*;

/// # light
/// light background, dark foreground. So the [`ColorOrder`] of [`DarkFirst`] makes sense here.
pub fn light(c: Vec<Srgb>, _orig: Vec<Srgb>) -> Colors {
    // This parser only needs 6 colors [0..=5]
    let lightest = c.last().expect("not empty");
    let darkest = c.first().expect("not empty");

    let (color0, bg) = getbg(*lightest);
    let fg = darkest.darken(0.55);

    let col7  = darkest.darken(0.55);
    let col15 = darkest.darken(0.85);

    let col8  = lightest.darken(0.3); //color 8 needs to be a bit brighter to contrast color0 and background

    //a little bit darken, to contrast well with the light bg.
    let color1 = c[5].darken_fixed(0.1);

    Colors {
        background : bg.into(), // background
        foreground : fg.into(),
        cursor : Myrgb(fg).blend(c[4].into()),

        /* First row */
        color0 : color0.into(),
        color1 : color1.into(),
        color2 : c[4].into(),
        color3 : c[3].into(),
        color4 : c[2].into(),
        color5 : c[1].into(),
        color6 : c[0].into(),
        color7 : col7.into(), // fg

        /* Second row */
        color8 : col8.into(), // darker than col0
        color9 : color1.into(),
        color10: c[4].into(),
        color11: c[3].into(),
        color12: c[2].into(),
        color13: c[1].into(),
        color14: c[0].into(),
        color15: col15.into(), //a little darken than col7
    }
}


/// Generates bg from a color0 (c) for light bgs
fn getbg(c: Srgb) -> (Srgb, Srgb) {
    use palette::IntoColor;
    use palette::Desaturate;

    let new: palette::Lch = c.into_format().into_color();

    // 'neutral' look of bg and color0
    let new = new.desaturate(0.8);

    let mut color0 = new;
    let mut bg = new;

    if new.l < 20.0 {
        bg = color0.lighten_fixed(0.7);
        color0 = color0.lighten_fixed(0.6);
    } else if new.l < 60.0 {
        color0 = color0.lighten_fixed(0.7);
        bg = bg.lighten_fixed(0.6);
    } else if new.l < 80.0 {
        color0 = bg.lighten_fixed(0.5);
        bg = bg.lighten_fixed(0.3);
    } else { //more than 80% lighning
        color0 = color0.lighten_fixed(0.4);
        bg = bg.lighten_fixed(0.2);
    }

    (color0.into_color(), bg.into_color())
}
