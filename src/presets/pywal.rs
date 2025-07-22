//! # Pywal
//! This backend differs from the wal backend, because it tries to be as loyal as possible to the
//! original pywal program palettes.

// use std::path::Path;

use crate::colors::Colors;
use crate::colors::Myrgb;

use palette::Srgb;
use palette::cast::ComponentsAs;
use palette::IntoColor;
//use anyhow::Result;

/// pywal uses the first, last and from 6-8 colors from the pallete.
/// TODO We need a way to tell `colorspaces` module to not chop off these colors.
/// Or rather, 'presets', that use a defined palete, colorspace and backend, and can have special
/// tweaks in please of aesthetics.
/// In the meantime, this backend will ignore colorspaces
/// https://github.com/dylanaraps/pywal/blob/236aa48e741ff8d65c4c3826db2813bf2ee6f352/pywal/backends/wal.py#L60
// pub fn pywal(f: &Path) -> Result<Vec<u8>> {
//     let c = crate::backends::wal::wal(f);
//     c
//     //raw_colors = colors[:1] + colors[8:16] + colors[8:-1]
//     // let mut new = vec![];
//     // new.extend_from_slice(&c[..1]);
//     // new.extend_from_slice(&c[8..16]);
//     // new.extend_from_slice(&c[8..c.len()-1]);
//     //
//     // Ok(new)
// }
pub fn cs(c: Vec<u8>) -> Vec<Srgb> {
    let s: &[Srgb<u8>] = c.components_as();
    s
        .iter()
        .map(|x| x.into_linear().into_color())
        .collect::<Vec<Srgb>>()

}

pub fn palette(c: Vec<Srgb>) -> Colors {
    //raw_colors = colors[:1] + colors[8:16] + colors[8:-1]
    let mut new = vec![c[0]];
    new.extend_from_slice(&c[8..16]);
    new.extend_from_slice(&c[8..c.len()-1]);

    let mut new: Vec<_> = new.iter().map(|x| Myrgb(*x)).collect();

    // Darken the background color slightly.
    // if raw_colors[0][1] != "0":
    //     raw_colors[0] = util.darken_color(raw_colors[0], 0.40)
    //  Basically check for the red channel
    let c: Srgb<u8> = new[0].0.into_format();
    if c.red != 0 {
        new[0] = new[0].darken(0.40);
    }
    let b: Srgb<u8> = "#EEEEEE".parse().expect("Valid Color");
    new[7] = new[7].blend(b.into());
    new[8] = new[7].darken(0.30);
    new[15] = new[15].blend(b.into());

    Colors {
        background : new[0],
        foreground : new[15],
        cursor : new[15],

        color0 : new[0_],
        color1 : new[1_],
        color2 : new[2_],
        color3 : new[3_],
        color4 : new[4_],
        color5 : new[5_],
        color6 : new[6_],
        color7 : new[7_],
        color8 : new[8_],
        color9 : new[9_],
        color10: new[10],
        color11: new[11],
        color12: new[12],
        color13: new[13],
        color14: new[14],
        color15: new[15],
    }
}
