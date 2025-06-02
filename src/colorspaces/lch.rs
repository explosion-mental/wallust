//! # LCH
//! CIE L*C*h°, a polar version of CIE L*a*b*.
//! ref: <https://docs.rs/palette/latest/palette/lch/struct.Lch.html>
use super::*;

/// The LCH struct
#[derive(Debug)]
pub struct Lch;

pub type Spec = palette::Lch;

/// Simple shadow to avoid repetition
pub type Hist = Histo<Spec>;

/// Miminum Luminance (from L ab) required for a color to be accepted
pub const DARKEST: f32 = 4.5;

/// Maximuum Luminance (from L ab) required for a color to be accepted
pub const LIGHTEST: f32 = 95.5;

/// This is so there are more vivid colors!
pub const MIN_CHROMA: f32 = 10.0;

impl ColorTrait for Spec {}

impl Difference for Spec {
    /// You could use palette::color_difference::{EuclideanDistance, ImprovedCiede2000, ImprovedDeltaE, Ciede2000};
    fn col_diff(&self, a: &Self, threshold: u8) -> bool {
        use palette::color_difference::ImprovedCiede2000;
        self.improved_difference(*a) <= f32::from(threshold)
    }
}

impl BuildHisto<Spec> for Lch {

    /// This filter gets the average to remove extreme colors.
    /// TODO even another filter to avoid blank `black/white`.
    fn filter_cols(histo: Vec<Spec>) -> Vec<Spec> {
        let lights = histo.iter().map(|c| c.l).collect::<Vec<_>>();
        let darkest  = lights.iter().fold(f32::INFINITY, |a, &b| a.min(b)).max(DARKEST);
        let lightest = lights.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b)).min(LIGHTEST);

        // We don't care about mexchroma, but 0.0 to 1.0 chroma is grayscale like
        // we use lesschroma on monochromatic or similar imgs, so it doesn't error out
        let chromas = histo.iter().map(|c| c.chroma).collect::<Vec<_>>();
        let origch = util::avg(&chromas);
        let lessch  = chromas.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        let ch = if origch <= MIN_CHROMA { lessch } else { origch / 2.5 };

        let filt = |x: &Spec| (x.l >= darkest && x.l <= lightest) && x.chroma >= ch;

        histo.into_iter().filter(filt).collect()
    }

    fn sort_col(histo: Vec<Hist>, cs: &ColorOrder) -> Vec<Hist> {

        let mut histo = histo;
        use std::cmp::Ordering;

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
