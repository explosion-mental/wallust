//! Default method to generate colors
use crate::filters::*;

pub fn dark(coo: Vec<Myrgb>) -> Colors {
    let mut c = coo.clone();
    let len = c.len();

    // This method requires at least 6 colors, if not, generate new ones
    //TODO generate new colors by mixing (and maybe tweak by some random number)
    if len < 6 {
        eprintln!("Not enought colors! Generating new colors from fetches ones...");
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

    let ee = Myrgb(238, 238, 238); //This is `#EEEEEE`

    // This parser only needs 6 colors [0..=5]
    let lightest = c.first().expect("not empty");
    let darkest = c.last().expect("not empty");

    let bg = darkest.darken(0.8);
    let fg = lightest.lighten(0.65);

    let col7  = lightest.blend(ee);
    let col15 = lightest.blend(ee).darken(0.2);

    let col8  = col7.darken(0.30); //color 8 needs to be a bit brighter to contrast color0 and background

    // darken the background color slightly, just like pywal
    let f = format!("{:02x}", darkest.0).chars().last().expect("garanted to have 2 elements by the fmt");
    let col0  = if f != '0' { bg } else { darkest.darken(0.4) };

    Colors {
        background : bg, // background
        foreground : fg,

        /* First row */
        color0 : col0, // background
        color1 : c[6],
        color2 : c[5],
        color3 : c[4],
        color4 : c[3],
        color5 : c[2],
        color6 : c[1],
        color7 : col7, // fg

        /* Second row */
        color8 : col8, // brighter than col0
        color9 : c[6],
        color10: c[5],
        color11: c[4],
        color12: c[3],
        color13: c[2],
        color14: c[1],
        color15: col15, //a little darken than col7
    }
}
