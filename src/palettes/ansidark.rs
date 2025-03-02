use super::*;

/// # ansidark
/// Usually should take 8 colors from lchansi, but there are workaround for MIN_COLS (6).
/// Sorted by [`LightFirst`]
pub fn ansidark(c: Vec<Srgb>, _orig: Vec<Srgb>) -> Colors {

    let col7 = {
        let ee = Myrgb(Srgb::<u8>::new(238, 238, 238).into_format()); //This is `#EEEEEE`
        let lightest = c[5];
        match c.get(7) {
            Some(s) => s.into(),
            None => ee.blend(lightest.into()),
        }
    };

    let col5;

    let col6 = {
        match c.get(6) {
            Some(s) => {
                col5 = Myrgb(c[5]);
                s.into()
            },
            None => {
                col5 = Myrgb(c[2]).blend(c[4].into());
                Myrgb(c[1]).blend(c[3].into())
            },
        }
    };

    Colors {
        background : c[0].darken(0.2).into(), // background
        foreground : col7,
        cursor : col7,


        /* First row */
        color0 : c[0].lighten(0.1).into(),
        color1 : c[1].into(),
        color2 : c[2].into(),
        color3 : c[3].into(),
        color4 : c[4].into(),
        color5 : col5,
        color6 : col6,
        color7 : col7,

        /* Second row */
        color8 : c[0].lighten(0.25).into(),
        color9 : c[1].into(),
        color10: c[2].into(),
        color11: c[3].into(),
        color12: c[4].into(),
        color13: col5,
        color14: col6,
        color15: col7,
    }
}
