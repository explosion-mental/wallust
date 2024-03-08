/// # light
/// light background, dark foreground. So the [`ColorOrder`] of [`DarkFirst`] makes sense here.
pub fn light(c: Vec<Srgb>, _orig: Vec<Srgb>) -> Colors {
    // This parser only needs 6 colors [0..=5]
    let lightest = c.last().expect("not empty");
    let darkest = c.first().expect("not empty");

    let bg = lightest.lighten(0.85);
    let fg = darkest.darken(0.55);

    let col7  = darkest.darken(0.55);
    let col15 = darkest.darken(0.85);

    let col8  = lightest.darken(0.3); //color 8 needs to be a bit brighter to contrast color0 and background

    Colors {
        background : bg.into(), // background
        foreground : fg.into(),

        /* First row */
        color0 : Myrgb(*lightest), // background
        color1 : c[5].into(),
        color2 : c[4].into(),
        color3 : c[3].into(),
        color4 : c[2].into(),
        color5 : c[1].into(),
        color6 : c[0].into(),
        color7 : col7.into(), // fg

        /* Second row */
        color8 : col8.into(), // darker than col0
        color9 : c[5].into(),
        color10: c[4].into(),
        color11: c[3].into(),
        color12: c[2].into(),
        color13: c[1].into(),
        color14: c[0].into(),
        color15: col15.into(), //a little darken than col7
    }
}
