//! # LCH
//! CIE L*C*h°, a polar version of CIE L*a*b*.
//! ref: <https://docs.rs/palette/latest/palette/lch/struct.Lch.html>
use super::*;

#[derive(Debug)]
pub struct Lch;

/// Shadow the colorspace type (Spectrum)
pub type Spec = palette::Lch;

/// histo<spec>
pub type Hist = Histo<Spec>;

/// Miminum Luminance (from L ab) required for a color to be accepted
pub const DARKEST: f32 = 4.5;

/// Maximuum Luminance (from L ab) required for a color to be accepted
pub const LIGHTEST: f32 = 95.5;

/// This is so there are more vivid colors!
/// even another filter to avoid blank `black/white`.
const MIN_CHROMA: f32 = 4.5;
//TODO intelligent min chroma
// get the minimum value and discard every minimun value until MIN_COLS (or max_cols?)

impl ColorTrait for Spec {}

impl Difference for Spec {
    fn col_diff(&self, a: &Self, threshold: u8) -> bool {
        use palette::color_difference::ImprovedCiede2000;
        self.improved_difference(*a) <= f32::from(threshold)
        // use palette::color_difference::{EuclideanDistance, ImprovedCiede2000, ImprovedDeltaE, Ciede2000};
        // self.difference(*a) <= threshold.into()
        // self.improved_difference(*a) <= threshold.into()
        // delta_1994(self, ) <= threshold.into()
    }

    fn filter_cols(&self) -> bool { (self.l >= DARKEST && self.l <= LIGHTEST) &&  self.chroma > MIN_CHROMA }
}

impl BuildHisto<Spec> for Lch {
    fn sort_col(histo: Vec<Hist>, cs: &ColorOrder) -> Vec<Hist> {

        let mut histo = histo;
        use std::cmp::Ordering;
        // TODO use light or chrome/hue
        histo.sort_by(|a, b| match cs {
            // ColorOrder::LightFirst => b.color.l.partial_cmp(&a.color.l).unwrap_or(std::cmp::Ordering::Equal),
            // ColorOrder::DarkFirst  => a.color.l.partial_cmp(&b.color.l).unwrap_or(std::cmp::Ordering::Equal),

            // ColorOrder::LightFirst => a.color.chroma.partial_cmp(&b.color.chroma).unwrap_or(std::cmp::Ordering::Equal),
            // ColorOrder::DarkFirst  => b.color.chroma.partial_cmp(&a.color.chroma).unwrap_or(std::cmp::Ordering::Equal),

            // ColorOrder::LightFirst => b.color.hue.into_inner().partial_cmp(&a.color.hue.into_inner()).unwrap_or(std::cmp::Ordering::Equal),
            // ColorOrder::DarkFirst  => a.color.hue.into_inner().partial_cmp(&b.color.hue.into_inner()).unwrap_or(std::cmp::Ordering::Equal),

            ColorOrder::LightFirst => (b.color.l, a.color.chroma).partial_cmp(&(a.color.l, b.color.chroma)).unwrap_or(Ordering::Equal),
            ColorOrder::DarkFirst  => (a.color.l, b.color.chroma).partial_cmp(&(b.color.l, a.color.chroma)).unwrap_or(Ordering::Equal),
        });
        histo
    }

    fn sort_by_key_fn(a: Hist) -> impl Ord {
        // a.color.l.partial_cmp(&a.color.l).unwrap_or(std::cmp::Ordering::Equal)
        // (a.color.l as i32, a.color.hue.into_inner() as i32)
        a.color.chroma as i32
        // (a.color.l as u32, a.color.chroma as i32, a.color.hue.into_inner() as i32)
    }
}
