use super::*;

//TODO use constants in each module
//const COLOR_ORDER = ColorOrder::LightFirst
//const MIN_COLORS = 6;
// TODO also, just create functions that sort the colors accordingly, no need to do it inside `colorspace` module.

/// # dark
/// Default method to generate colors.
/// This parser only needs 6 _ [0..=5]. Sorted by [`LightFirst`]
pub fn dark(cols: Vec<Srgb>, _orig: Vec<Srgb>) -> Colors {
    let c = cols;
    let ee = Myrgb(Srgb::<u8>::new(238, 238, 238).into_format()); //This is `#EEEEEE`

    // this corresponds to [`LightFirst`] [`ColorOrder`]
    let lightest = c.first().expect("not empty");
    let darkest = c.last().expect("not empty");

    //let bg = darkest.darken(0.8);
    let fg = lightest.lighten(0.65);

    // get the first char of the darkest color
    // let f = format!("{:02x}", darkest.0).chars().last().expect("garanted to have 2 elements by the fmt");

    // Darken the background color slightly, just like pywal
    // TODO maybe just check `chroma` or the like value
    let (col0, bg) = getbg(*darkest);


    let col7  = ee.blend(lightest.into());

    //color 8 needs to be a bit brighter to contrast color0 and background
    let col8  = col7.darken(0.30);

    let col15 = ee.blend(lightest.into());

    Colors {
        background : bg.into(), // background
        foreground : fg.into(),
        cursor : Myrgb(fg).blend(c[4].into()),

        /* First row */
        color0 : col0.into(), // background
        color1 : c[5].into(),
        color2 : c[4].into(),
        color3 : c[3].into(),
        color4 : c[2].into(),
        color5 : c[1].into(),
        color6 : c[0].into(),
        color7 : col7, // fg

        /* Second row */
        color8 : col8, // brighter than col0
        color9 : c[5].into(),
        color10: c[4].into(),
        color11: c[3].into(),
        color12: c[2].into(),
        color13: c[1].into(),
        color14: c[0].into(),
        color15: col15, //a little darken than col7
    }
}

/// Generates bg from a color0 (c)
fn getbg(c: Srgb) -> (Srgb, Srgb) {
    use palette::IntoColor;
    use palette::Desaturate;

    let new: palette::Lch = c.into_format().into_color();
    // XXX mostly to keep the 'desaturated' look of the background, classic feel of good old `dark`
    // palette (and behaviour from pywal)
    let new = new.desaturate(0.8);

    let mut color0 = new;
    let mut bg = new;

    if new.l < 20.0 {
        //color0 it's lighter, needs darkening
        color0 = color0.lighten(0.2);
    } else if new.l < 60.0 {
        color0 = color0.darken_fixed(0.3);
        bg = bg.darken_fixed(0.4);
    } else if new.l < 80.0 {
        color0 = bg.darken_fixed(0.5);
        bg = bg.darken_fixed(0.7);
    } else { //more than 80% lighning
        color0 = color0.darken_fixed(0.6);
        bg = bg.darken_fixed(0.8);
    }

    (color0.into_color(), bg.into_color())
}
