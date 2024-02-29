/// # dark
/// Default method to generate colors.
/// This parser only needs 6 _ [0..=5]. Sorted by [`LightFirst`]
pub fn dark(cols: Vec<Myrgb>, _orig: Vec<Myrgb>) -> Colors {
    let c = cols;
    let ee = Myrgb(238, 238, 238); //This is `#EEEEEE`

    // this corresponds to [`LightFirst`] [`ColorOrder`]
    let lightest = c.first().expect("not empty");
    let darkest = c.last().expect("not empty");

    let bg = darkest.darken(0.8);
    let fg = lightest.lighten(0.65);

    // get the first char of the darkest color
    let f = format!("{:02x}", darkest.0).chars().last().expect("garanted to have 2 elements by the fmt");

    // Darken the background color slightly, just like pywal
    let col0  = if f != '0' { bg } else { darkest.darken(0.4) };

    let col7  = lightest.blend(ee);

    //color 8 needs to be a bit brighter to contrast color0 and background
    let col8  = col7.darken(0.30);

    let col15 = lightest.blend(ee);

    Colors {
        background : bg, // background
        foreground : fg,

        /* First row */
        color0 : col0, // background
        color1 : c[5],
        color2 : c[4],
        color3 : c[3],
        color4 : c[2],
        color5 : c[1],
        color6 : c[0],
        color7 : col7, // fg

        /* Second row */
        color8 : col8, // brighter than col0
        color9 : c[5],
        color10: c[4],
        color11: c[3],
        color12: c[2],
        color13: c[1],
        color14: c[0],
        color15: col15, //a little darken than col7
    }
}
