/// # dark
/// Default method to generate colors.
/// This parser only needs 6 _ [0..=5]. Sorted by [`LightFirst`]
pub fn dark(cols: Vec<Srgb>, _orig: Vec<Srgb>) -> Colors {
    let c = cols;
    let ee = Myrgb(Srgb::<u8>::new(238, 238, 238).into_format()); //This is `#EEEEEE`

    // this corresponds to [`LightFirst`] [`ColorOrder`]
    let lightest = c.first().expect("not empty");
    let darkest = c.last().expect("not empty");

    let bg = darkest.darken(0.8);
    let fg = lightest.lighten(0.65);

    // get the first char of the darkest color
    let d = darkest.strsrgb();
    // let f = format!("{:02x}", darkest.0).chars().last().expect("garanted to have 2 elements by the fmt");

    // Darken the background color slightly, just like pywal
    let col0  = if &d[0..1] != "0" { bg } else { darkest.darken(0.4) };

    let col7  = ee.blend(lightest.into());

    //color 8 needs to be a bit brighter to contrast color0 and background
    let col8  = col7.darken(0.30);

    let col15 = ee.blend(lightest.into());

    Colors {
        background : bg.into(), // background
        foreground : fg.into(),

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
