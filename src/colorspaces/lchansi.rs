//! # LCH
//! CIE L*C*h°, a polar version of CIE L*a*b*.
//! ref: <https://docs.rs/palette/latest/palette/lch/struct.Lch.html>
use palette::{GetHue, LabHue};

use super::*;
use util::avg;

pub struct LchAnsi;

/// Shadow the colorspace type (Spectrum)
pub type Spec = palette::Lch;

/// Used for a better handle of colors and it's (perception) 'limits'
struct ColSettings {
    hue_start: f32,
    hue_end: f32,
    light_def: f32,
    chroma_def: f32,
}

impl BuildHisto<Spec> for LchAnsi {

    /// We don't care much about filter colors here. Use the old formula, since it's faster, to
    /// rapidly get the values. Since we modify them anyway.
    fn filter_cols(histo: Vec<Spec>) -> Vec<Spec> {
        let filt = |x: Spec| (x.l >= lch::DARKEST && x.l <= lch::LIGHTEST) &&  x.chroma > lch::MIN_CHROMA;
        histo.into_iter().filter(|&c| filt(c)).collect()
    }

    ///NO SORTING, since we set up everything in `gather_cols`
    fn sort_col(histo: Vec<Histo<Spec>>, _cs: &ColorOrder) -> Vec<Histo<Spec>> { histo }

    /// no sorting here as well.
    fn sort_by_key_fn(_a: Histo<Spec>) -> impl Ord { }

    /// We change this in order to:
    ///  1. Follow ascii 8 bit colors
    ///  2. make sure we have MIN_COLS to not trigger a FallbackGenerator, hence assure that, for
    ///     example, color1 will always be greenish.
    ///     Red     falls between 0   and 60  degrees.
    ///     Yellow  falls between 61  and 120 degrees.
    ///     Green   falls between 121 and 180 degrees.
    ///     Cyan    falls between 181 and 240 degrees.
    ///     Blue    falls between 241 and 300 degrees.
    ///     Magenta falls between 301 and 360 degrees.
    /// - Ref: <https://docs.rs/palette/latest/palette/lch/struct.Lch.html>
    // comments below are from the palette docs
    fn gather_cols(colors: Vec<Spec>, _threshold: u8, _mix: bool) -> Vec<Histo<Spec>> {

        let red     = ColSettings { hue_start:   0.0, hue_end:  60.0, light_def: 50.0, chroma_def: 181.0 };
        let yellow  = ColSettings { hue_start:  61.0, hue_end: 120.0, light_def: 80.0, chroma_def: 128.0 };
        let green   = ColSettings { hue_start: 121.0, hue_end: 180.0, light_def: 50.0, chroma_def: 128.0 };
        let cyan    = ColSettings { hue_start: 181.0, hue_end: 210.0, light_def: 80.0, chroma_def: 128.0 };
        let blue    = ColSettings { hue_start: 211.0, hue_end: 280.0, light_def: 40.0, chroma_def: 181.0 };
        let magenta = ColSettings { hue_start: 281.0, hue_end: 360.0, light_def: 70.0, chroma_def: 128.0 };

        let mut cols = colors.clone();

        let black = {
            //dummy
            let mut ret = Histo::new(Spec::new(0.0, 0.0, LabHue::new(0.0)), 777);

            let mut lights = vec![];
            let mut hues = vec![];
            let mut chromas = vec![];

            let dark = 5.0;

            for c in &cols {
                if c.l < dark {
                    ret = Histo::new_no_count(*c);
                    break;
                }
                lights.push(c.l);
                hues.push(c.hue.into_inner());
                chromas.push(c.chroma);
            }

            if ret.count != 777 { //dummy value gone
                //ret
                Histo::new_no_count(Spec::new(ret.color.l, 15.0, ret.color.hue))
            } else {
                let a = avg(&lights);
                let l = (7.0*dark + a) / 8.0;

                let avg_c = avg(&chromas);
                let chroma = (2.0*0.0 + avg_c) / 3.0;
                Histo::new_no_count(Spec::new(l, chroma, LabHue::new(avg(&hues))))
                // let r = Histo::new_no_count(Spec::new(l, chroma, LabHue::new(avg(&hues))));
                // println!("{r:?}");
                // r
            }
        };

        let gray = {
            //dummy
            let mut ret = Histo::new(Spec::new(0.0, 0.0, LabHue::new(0.0)), 777);

            let mut lights = vec![];
            let mut hues = vec![];
            let mut chromas = vec![];

            let lighty = 95.0;

            for c in &cols {
                if c.l > lighty {
                    ret = Histo::new_no_count(*c);
                    break;
                }
                lights.push(c.l);
                hues.push(c.hue.into_inner());
                chromas.push(c.chroma);
            }

            if ret.count != 777 { //dummy value gone
                //ret
                Histo::new_no_count(Spec::new(ret.color.l, 15.0, ret.color.hue))
            } else {
                // I'm very agressive with gray here, since it's more 'uncommon' than pitch black.
                let a = avg(&lights);
                let l = (4.0*lighty + a) / 5.0; //usually gets >80

                let avg_c = avg(&chromas);
                let chroma = (2.0*0.0 + avg_c) / 3.0;
                // let r = Histo::new_no_count(Spec::new(l, chroma, LabHue::new(avg(&hues))));
                //println!("{r:?}");
                //r
                Histo::new_no_count(Spec::new(l, chroma, LabHue::new(avg(&hues))))
            }
        };

        //color0 black
        // color1 red
        // color2 green
        // color3 yellow
        // color4 blue
        // color5 magenta
        // color6 cyan
        // color7 gray or dark white
        // color8 bright black or grey
        // and then it repats with bright variants..

        //XXX keep in mind that the order presented here is for reference
        //    since every palette, independently chooses the order. This is why
        //    there is a need for some information exchange between ColorSpaces <-> Palettes.
        let histogram = vec![
            black,
            get_colors(&mut cols, red),
            get_colors(&mut cols, green),
            get_colors(&mut cols, yellow),
            get_colors(&mut cols, blue),
            get_colors(&mut cols, magenta),
            get_colors(&mut cols, cyan),
            gray,
        ];

        assert!(histogram.len() >= MIN_COLS.into(), "Histogram has less colors than required.");

        //println!("{histogram:#?}");
        histogram
    }

    fn color_generator(_histo: &[Histo<Spec>], _threshold: u8, _gen: &FallbackGenerator) -> Vec<Histo<Spec>> {
        // gather_colors SHOULD ALWAYS fill at least MIN_COLORS.
        unreachable!()
    }
}


///TODO check lightness to not be black or white
fn get_colors(cols: &mut Vec<Spec>, color: ColSettings) -> Histo<Spec> {
    let mut hues = vec![];
    let mut lights = vec![];
    let mut chromes = vec![];

    // // get values that fit within the colors range
    // for i in 0..cols.len() {
    //     let hue = cols[i].get_hue().into_inner();
    //     if range.contains(&hue) {
    //         hues.push(hue);
    //         lights.push(cols[i].l);
    //         chromes.push(cols[i].chroma);
    //         rems.push(i);
    //     }
    //     println!("{i}")
    // }

    cols.retain(|c| {
        let hue = c.get_hue().into_inner();
        if (color.hue_start..color.hue_end).contains(&hue) {
            hues.push(hue);
            lights.push(c.l);
            chromes.push(c.chroma);
            true
        } else {
            false
        }
    });

    //artificially make the color in between
    let hue = if hues.is_empty() {
        let mut fallback = vec![];
        for i in color.hue_start as usize..color.hue_end as usize {
            fallback.push(i as f32);
        }
        let x = avg(&fallback);
        //get half (avg) of the hue
        let m = color.hue_start + color.hue_end / 2.0;
        //weighted ecuation
        (m + 2.0*x) / 3.0
    } else {
        avg(&hues)
    };

    // C* is the colorfulness of the color. It’s similar to saturation. 0.0 gives gray
    // scale colors, and numbers around 128-181 gives fully saturated colors. The upper
    // limit of 128 should include the whole L*a*b* space and some more.
    let chroma = if chromes.is_empty() { color.chroma_def } else {
        let a = avg(&chromes);
        (color.chroma_def + 2.0*a) / 3.0
            // if a <= 64.0 {
            //     a + 30.0
            // } else if a > 120.0 {
            //     a - 60.0
            // } else {
            //     a
            // }
    };

    // L* is the lightness of the color. 0.0 gives absolute black and 100.0 gives the
    // brightest white.
    let light = if lights.is_empty() { color.light_def } else {
        let a = avg(&lights);
        (color.light_def + 2.0*a) / 3.0
            // if a <= 10.0 {
            //     a + 30.0
            // } else if a > 90.0 {
            //     a - 30.0
            // } else {
            //     a
            // }
    };
    //println!("L {light} | c {chroma} | h {hue}");
    Histo::new_no_count(Spec::new(light, chroma, LabHue::new(hue)))
}
