//! Default method to generate colors
use crate::filters::*;

pub fn dark(coo: Vec<Myrgb>) -> Colors {
    // Make sure the vector has 16 colors; if it's lower, derive new generated colors from the
    // ones that already exist (until it's 16)
    let mut c = coo.clone();
    let len = c.len();
    if len < 8 {
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
        color0 : c[0],
        color1 : c[1],
        color2 : c[2],
        color3 : c[3],
        color4 : c[4],
        color5 : c[5],
        color6 : c[6],
        color7 : c[7],
        color8 : c[0],
        color9 : c[1],
        color10: c[2],
        color11: c[3],
        color12: c[4],
        color13: c[5],
        color14: c[6],
        color15: c[7],
    }
}
