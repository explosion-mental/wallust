use std::cmp::Ordering;


use crate::histogram::Difference;
use crate::histogram::salience::Salience;
use crate::histogram::DiffMode;

use super::*;

// pub trait Dark: Build {
//     fn dark(self) -> Colors;
// }
//
// impl Dark for Luminance {
//     fn dark(self) -> Colors {
//         todo!("DARK LUMINANCE")
//     }
// }
//
// impl Dark for Salience {
//     fn dark(self) -> Colors {
//         generate(&self, &self.ord, &SamplingMode::Balanced)
//     }
// }
//

// const C0_MIN_SAL_BG: f32 = 1.5;
const C0_MIN_SAL_BG: f32 = 1.0;

/// This parser only needs 6 _ [0..=5]. Sorted by lowest salience first.
///
/// The whole point of this palette is that colors are sorted by salience in
/// accordance to the chosen background color. Even though a sort is called in
/// colorspace before the colors are passed here, we will sort again.
///
/// Two sorts are made. (1) a naive salience sort that does not take into
/// account hue to determine the least salient color to be set a bg, and (2) a
/// salience sort against the chose bg color.
pub fn generate(init: &Salience, ord: &ColorOrder, mode: &SamplingMode) -> Colors {
    // with colors finally sorted with salience against bg, operations onto
    // the first and last color will not alter salience ordering, so long as
    // the operations make sense (i.e. only make least salient less salient
    // and most salient more salient). you shouldn't mess with the hue.

    //let mut cols = cols;
    let ee = Myrgb(Srgb::<u8>::new(238, 238, 238).into_format()); //This is `#EEEEEE`

    // Always use col[0] for background. All of the previous logic on
    // colorspace saliencelch is there to ensure that col0 is the most
    // prominent color.

    // If we have more than 6 colors, we can have c[0] and c[1] be different.
    // Removed to shift all colors over for more elegant assignment later.
    // Now we have min 6 colors.
    //let mut cams: Vec<SalienceSpec> = cols.iter().map(|c| {c.into_format().into_color()}).collect();
    let mut cams: Vec<_> = init.histo.iter().map(|c| c.color).collect();

    let mut bg = if cams.len() > 6 { cams.remove(0) } else { cams[0] };
    bg = init.constrain_col_as_bg(bg, &cams);
    let res = init.constrain_col_against_cols(bg, &cams, &[init.bg_min_sal(), C0_MIN_SAL_BG]);
    let (bg, col0) = (res[0], res[1]);

    // Re-sort against the new finalized background.
    //
    // Redundant if we used salience backend because it should already be
    // sorted, but this allows it to work and make sense with other backends.
    // XXX no longer needed, salience works with salience.
    //cams.sort_by(|a, b| a.diff(&bg, init.threshold, &DiffMode::ImprovedDeltaE).partial_cmp(&b.diff(&bg, init.threshold, &DiffMode::DeltaE)).unwrap_or(Ordering::Equal));

    // If we still have more colors, turn cols[6] into fg, col7, col8, and col15
    let high_factor = 0.45;
    let high = if cams.len() > 6 {
        match mode {
            SamplingMode::Low       => init.inc_sal_l(cams[6], high_factor),
            SamplingMode::Balanced  => init.inc_sal_l(cams[*util::sample_center_idxs(&cams, 6).last().expect("Not Empty")], high_factor),
            _                       => init.inc_sal_l(*cams.last().expect("not empty"), high_factor)
        }
    } else {
        init.inc_sal_l(*cams.last().expect("not empty"), high_factor)
    };

    let (fg, col7, col8, col15) = match ord {
        ColorOrder::LightFirst => {
            let fg = init.inc_sal_l(high, 0.25);
            let col7 = high;
            let col8 = init.dec_sal_l(col7, 0.30);
            let col15 = ee.blend(Myrgb(init.to_rgb(high)));
            (fg, col7, col8, col15)
        },
        ColorOrder::DarkFirst => {
            let fg = high.darken(high_factor);
            let col7 = high.darken(high_factor);
            let col8 = col7.darken(0.30);
            let col15 = high.darken(0.85);
            (fg, col7, col8, Myrgb(init.to_rgb(col15)))
        }
    };

    let cols: Vec<_> = cams.into_iter().map(|c| init.to_rgb(c)).collect();

    // sample colors 0-5 that we will use
    let c: &[Srgb] = match mode {
        SamplingMode::Low           => &cols,
        SamplingMode::Balanced      => &util::sample_center(&cols, 6),
        SamplingMode::Distributed   => &util::sample_distributed(&cols, 6),
        SamplingMode::High          => &cols[cols.len() - 6..],
    };

    Colors {
        background : init.to_rgb(bg).into(),
        foreground : init.to_rgb(fg).into(),
        cursor : Myrgb(init.to_rgb(fg)).blend(c[4].into()),

        /* First row */
        color0 : init.to_rgb(col0).into(),
        color1 : c[0].into(),
        color2 : c[1].into(),
        color3 : c[2].into(),
        color4 : c[3].into(),
        color5 : c[4].into(),
        color6 : c[5].into(),
        color7 : init.to_rgb(col7).into(),

        /* Second row */
        color8 : init.to_rgb(col8).into(),
        color9 : c[0].into(),
        color10: c[1].into(),
        color11: c[2].into(),
        color12: c[3].into(),
        color13: c[4].into(),
        color14: c[5].into(),
        color15: col15.into(),
    }
}




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
