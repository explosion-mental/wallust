//! # saliencedark
//! Alternative method for generating colors, sorted by salience (attention)
use super::*;
use palette::IntoColor;
use std::cmp::Ordering;

use crate::colorspaces::salience::{
    Spec as SalienceSpec,
    Salient,
    CAM16_VIEW,
    init_view,
    get_bg_min_sal,
    constrain_col_as_bg,
    constrain_col_against_cols,
    get_dec_sal_l_fn,
    get_inc_sal_l_fn,
};

// const C0_MIN_SAL_BG: f32 = 1.5;
const C0_MIN_SAL_BG: f32 = 1.0;

/// Generate a palette from colors, centered on median attention
pub fn saliencedarkhigh(cols: Vec<Srgb>, org: Vec<Srgb>) -> Colors {
    generate(cols, org, &ColorOrder::LightFirst, &SamplingMode::High)
}

/// Generate a palette with colors of most attention
pub fn saliencedarkbalanced(cols: Vec<Srgb>, org: Vec<Srgb>) -> Colors {
    generate(cols, org, &ColorOrder::LightFirst, &SamplingMode::Balanced)
}

/// Generate a palette from colors, evenly distributing attention
pub fn saliencedarkdistributed(cols: Vec<Srgb>, org: Vec<Srgb>) -> Colors {
    generate(cols, org, &ColorOrder::LightFirst, &SamplingMode::Distributed)
}

/// Generate a palette with colors of least attention
pub fn saliencedarklow(cols: Vec<Srgb>, org: Vec<Srgb>) -> Colors {
    generate(cols, org, &ColorOrder::LightFirst, &SamplingMode::Low)
}

/// Generate a palette from colors, centered on median attention
pub fn saliencelighthigh(cols: Vec<Srgb>, _org: Vec<Srgb>) -> Colors {
    generate(cols, _org, &ColorOrder::DarkFirst, &SamplingMode::High)
}

/// Generate a palette with colors of most attention
pub fn saliencelightbalanced(cols: Vec<Srgb>, _org: Vec<Srgb>) -> Colors {
    generate(cols, _org, &ColorOrder::DarkFirst, &SamplingMode::Balanced)
}

/// Generate a palette from colors, evenly distributing attention
pub fn saliencelightdistributed(cols: Vec<Srgb>, _org: Vec<Srgb>) -> Colors {
    generate(cols, _org, &ColorOrder::DarkFirst, &SamplingMode::Distributed)
}

/// Generate a palette with colors of least attention
pub fn saliencelightlow(cols: Vec<Srgb>, _org: Vec<Srgb>) -> Colors {
    generate(cols, _org, &ColorOrder::DarkFirst, &SamplingMode::Low)
}

/// This parser only needs 6 _ [0..=5]. Sorted by lowest salience first.
///
/// The whole point of this palette is that colors are sorted by salience in
/// accordance to the chosen background color. Even though a sort is called in
/// colorspace before the colors are passed here, we will sort again.
///
/// Two sorts are made. (1) a naive salience sort that does not take into
/// account hue to determine the least salient color to be set a bg, and (2) a
/// salience sort against the chose bg color.
fn generate(cols: Vec<Srgb>, _orig: Vec<Srgb>, ord: &ColorOrder, mode: &SamplingMode) -> Colors {
    // with colors finally sorted with salience against bg, operations onto
    // the first and last color will not alter salience ordering, so long as
    // the operations make sense (i.e. only make least salient less salient
    // and most salient more salient). you shouldn't mess with the hue.

    // Initialized colorspace view according to dark/light theme
    if CAM16_VIEW.set(init_view(ord)).is_err() {}; // ignore if already set

    let mut cols = cols;
    let ee = Myrgb(Srgb::<u8>::new(238, 238, 238).into_format()); //This is `#EEEEEE`
    let dec_sal_l = get_dec_sal_l_fn(ord);
    let inc_sal_l = get_inc_sal_l_fn(ord);

    // Always use col[0] for background. All of the previous logic on
    // colorspace saliencelch is there to ensure that col0 is the most
    // prominent color.

    // If we have more than 6 colors, we can have c[0] and c[1] be different.
    // Removed to shift all colors over for more elegant assignment later.
    // Now we have min 6 colors.
    let mut cams: Vec<SalienceSpec> = cols.iter().map(|c| {c.into_format().into_color()}).collect();

    let mut bg = if cams.len() > 6 {cams.remove(0)} else {cams[0]};
    bg = constrain_col_as_bg(bg, &cams, ord);
    let res = constrain_col_against_cols(bg, &cams, ord, &[get_bg_min_sal(ord), C0_MIN_SAL_BG]);
    let (bg, col0) = (res[0], res[1]);

    // Re-sort against the new finalized background.
    //
    // Redundant if we used salience backend because it should already be
    // sorted, but this allows it to work and make sense with other backends.
    cams.sort_by(|a, b| a.improved_salience(&bg).partial_cmp(&b.improved_salience(&bg)).unwrap_or(Ordering::Equal));

    // If we still have more colors, turn cols[6] into fg, col7, col8, and col15
    let high_factor = 0.45;
    let high = if cams.len() > 6 {
        match mode {
            SamplingMode::Low       => inc_sal_l(cams[6], high_factor),
            SamplingMode::Balanced  => inc_sal_l(cams[*util::sample_center_idxs(&cams, 6).last().unwrap()], high_factor),
            _                       => inc_sal_l(*cams.last().expect("not empty"), high_factor)
        }
    }
    else {
        inc_sal_l(*cams.last().expect("not empty"), high_factor)
    };

    let (fg, col7, col8, col15): (SalienceSpec, SalienceSpec, SalienceSpec, Myrgb) = match ord {
        ColorOrder::LightFirst => {
            let fg = inc_sal_l(high, 0.25);
            let col7 = high;
            let col8 = dec_sal_l(col7, 0.30);
            let col15 = ee.blend(high.into());
            (fg, col7, col8, col15)
        },
        ColorOrder::DarkFirst => {
            let fg = high.darken(high_factor);
            let col7 = high.darken(high_factor);
            let col8 = col7.darken(0.30);
            let col15 = high.darken(0.85).into();
            (fg, col7, col8, col15)
        }
    };

    cols = cams.into_iter().map(|c: SalienceSpec| c.into_color()).collect();

    // sample colors 0-5 that we will use
    let c: &[Srgb] = match mode {
        SamplingMode::Low           => &cols,
        SamplingMode::Balanced      => &util::sample_center(&cols, 6),
        SamplingMode::Distributed   => &util::sample_distributed(&cols, 6),
        SamplingMode::High          => &cols[cols.len() - 6..],
    };

    Colors {
        background : bg.into(),
        foreground : fg.into(),
        cursor : Myrgb(fg.into_color()).blend(c[4].into()),

        /* First row */
        color0 : col0.into(),
        color1 : c[0].into(),
        color2 : c[1].into(),
        color3 : c[2].into(),
        color4 : c[3].into(),
        color5 : c[4].into(),
        color6 : c[5].into(),
        color7 : col7.into(),

        /* Second row */
        color8 : col8.into(),
        color9 : c[0].into(),
        color10: c[1].into(),
        color11: c[2].into(),
        color12: c[3].into(),
        color13: c[4].into(),
        color14: c[5].into(),
        color15: col15.into(),
    }
}
