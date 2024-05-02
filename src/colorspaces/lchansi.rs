//! # LCH
//! CIE L*C*h°, a polar version of CIE L*a*b*.
//! ref: <https://docs.rs/palette/latest/palette/lch/struct.Lch.html>
use palette::{GetHue, LabHue};

use super::*;

pub struct LchAnsi;

/// Shadow the colorspace type (Spectrum)
type Spec = palette::Lch;

/// Miminum Luminance (from L ab) required for a color to be accepted
pub const DARKEST: f32 = 4.5;

/// Maximuum Luminance (from L ab) required for a color to be accepted
pub const LIGHTEST: f32 = 95.5;

impl BuildColors for ColorHisto<Spec, LchAnsi> {
    type Color = Spec;
    fn filter_cols(a: Self::Color) -> bool { a.l >= DARKEST || a.l <= LIGHTEST }



    /// We change this in order to:
    ///  1. Follow ascii 8 bit colors
    ///  2. make sure we have MIN_COLS to not trigger a FallbackGenerator, hence assure that, for
    ///     example, color1 will always be greenish.

    /// Red     falls between 0   and 60  degrees.
    /// Yellow  falls between 61  and 120 degrees.
    /// Green   falls between 121 and 180 degrees.
    /// Cyan    falls between 181 and 240 degrees.
    /// Blue    falls between 241 and 300 degrees.
    /// Magenta falls between 301 and 360 degrees.
    fn gather_cols(colors: Vec<Self::Color>, _threshold: u8, _mix: bool) -> Self {
        let red     = &(0.0..60.0);
        let yellow  = &(61.0..120.0);
        let green   = &(121.0..180.0);
        let cyan    = &(181.0..240.0);
        let blue    = &(241.0..300.0);
        let magenta = &(301.0..360.0);

        let avg = |i: &[f32]| i.iter().sum::<f32>() / i.len() as f32;


        //TODO check lightness to not be black or white
        let col = |range: &std::ops::Range<f32>| -> Histo<Self::Color> {
            let mut hues = vec![];
            let mut lights = vec![];
            let mut chromes = vec![];

            for i in &colors {
                let hue = i.get_hue().into_inner();
                if range.contains(&hue) {
                    hues.push(hue);
                    lights.push(i.l);
                    chromes.push(i.chroma);
                }
            }


            let mut f = vec![];
            for i in range.start as usize..range.end as usize {
                f.push(i as f32);
            }

            let fallback = avg(&f);
            //println!("{fallback:?}");

            //artificially make "redish"
            let hue   = if hues.is_empty() { fallback } else { avg(&hues) };

            let c     = if chromes.is_empty() { 128.0 } else {
                let a = avg(&chromes);
                if a <= 64.0 {
                    a + 30.0
                } else if a > 120.0 {
                    a - 60.0
                } else {
                    a
                }
            };

            let     l = if lights.is_empty() { 80.0 } else {
                let a = avg(&lights);
                if a <= 10.0 {
                    a + 30.0
                } else if a > 90.0 {
                    a - 30.0
                } else {
                    a
                }
            };

            //println!("L {l} | c {c} | h {hue}");

            Histo { color: Spec::new(l, c, LabHue::new(hue)), count: 10000 }
        };

        // color1 red
        // color2 green
        // color3 yellow
        // color4 blue
        // color5 magenta
        // color6 cyan
        // color7 gray or dark white

        // color8 bright black or grey
        // and then it repats with bright variants..

        let histogram: Vec<Histo<Self::Color>> = vec![
            col(red),
            col(green),
            col(yellow),
            col(blue),
            col(magenta),
            col(cyan),
            Histo { color: Spec::new(80.0, 128.0, LabHue::new(360.0)), count: 10000 },
            Histo { color: Spec::new(80.0, 120.0, LabHue::new(200.0)), count: 10000 },
            Histo { color: Spec::new(10.0, 10.0, LabHue::new(200.0)), count: 10000 },
            Histo { color: Spec::new(20.0, 175.0, LabHue::new(10.0)), count: 10000 },
            //histogram.push( Histo { color: Spec::new(80.0, 128.0, LabHue::new(red)), count: 10000 } );
        ];

        //println!("{histogram:#?}");

        histogram.into()
    }

    fn color_generator(_histo: &[Histo<Self::Color>], _threshold: u8, _gen: &FallbackGenerator) -> Vec<Histo<Self::Color>> {
        // gather_colors SHOULD ALWAYS fill at least MIN_COLORS.
        unreachable!()
    }

    ///NO SORTING, since we set up everything in `gather_cols`
    fn sort_col(self, _cs: &ColorOrder) -> Self { self }

    /// no sorting here as well.
    fn sort_by_key_fn(_a: Histo<Self::Color>) -> impl Ord { }
}
