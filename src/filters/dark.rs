//! Default method to generate colors
use crate::filters::*;

pub fn dark(coo: Vec<Myrgb>) -> Colors {
    // Make sure the vector has 16 colors; if it's lower, derive new generated colors from the
    // ones that already exist (until it's 16)
    let mut c = coo.clone();
    let len = c.len();
    if len < 6 {
        println!("Not enought colors! Generating new colors from fetches ones...");
        for (i, value) in coo.iter().enumerate() {
            // generate new colors, but switch between the how (method to use)
            let val = if i % 2 == 0 {
                value.darken(0.5)
            } else {
                value.lighten(0.5)
            };
            c.push(val);
        }
    }

    Colors {
        background : c[0].darken(0.7),
        foreground : c[0].lighten(0.7),

        // * First row *
        color0 : c[0].darken(0.8), // background

        color1 : c[0],
        color2 : c[1],
        color3 : c[2],
        color4 : c[3],
        color5 : c[4],
        color6 : c[5],

        color7 : c[5].lighten(0.8),

        // * Second row *
        color8 : c[0].darken(0.65), //bold background

        color9 : c[0],
        color10: c[1],
        color11: c[2],
        color12: c[3],
        color13: c[4],
        color14: c[5],

        color15: c[5].lighten(0.65),
    }
}
