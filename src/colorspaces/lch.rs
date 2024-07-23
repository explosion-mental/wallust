//! # LCH
//! CIE L*C*h°, a polar version of CIE L*a*b*.
//! ref: <https://docs.rs/palette/latest/palette/lch/struct.Lch.html>
use super::*;

pub struct Lch;

/// Shadow the colorspace type (Spectrum)
type Spec = palette::Lch;

/// Miminum Luminance (from L ab) required for a color to be accepted
pub const DARKEST: f32 = 4.5;

/// Maximuum Luminance (from L ab) required for a color to be accepted
pub const LIGHTEST: f32 = 95.5;

impl ColorTrait for Spec {}

impl Difference for Spec {
    fn col_diff(&self, a: &Self, threshold: u8) -> bool {
        use palette::color_difference::ImprovedCiede2000;
        self.improved_difference(*a) <= f32::from(threshold)
        // use palette::color_difference::{EuclideanDistance, ImprovedCiede2000, ImprovedDeltaE, Ciede2000};
        // self.difference(*a) <= threshold.into()
        // self.improved_difference(*a) <= threshold.into()
        // delta_1994(self, a) <= threshold.into()
    }
}

impl BuildColors for ColorHisto<Spec, Lch> {
    type Color = Spec;
    fn filter_cols(a: Self::Color) -> bool { a.l >= DARKEST || a.l <= LIGHTEST }

    fn sort_col(self, cs: &ColorOrder) -> Self {
        let mut new = self; //take ownership

        // TODO use light or chrome/hue
        new.sort_by(|a, b| match cs {
            // ColorOrder::LightFirst => b.color.l.partial_cmp(&a.color.l).unwrap_or(std::cmp::Ordering::Equal),
            // ColorOrder::DarkFirst  => a.color.l.partial_cmp(&b.color.l).unwrap_or(std::cmp::Ordering::Equal),

            ColorOrder::LightFirst => a.color.chroma.partial_cmp(&b.color.chroma).unwrap_or(std::cmp::Ordering::Equal),
            ColorOrder::DarkFirst  => b.color.chroma.partial_cmp(&a.color.chroma).unwrap_or(std::cmp::Ordering::Equal),

            // ColorOrder::LightFirst => b.color.hue.into_inner().partial_cmp(&a.color.hue.into_inner()).unwrap_or(std::cmp::Ordering::Equal),
            // ColorOrder::DarkFirst  => a.color.hue.into_inner().partial_cmp(&b.color.hue.into_inner()).unwrap_or(std::cmp::Ordering::Equal),

            // ColorOrder::LightFirst => (b.color.l, b.color.chroma).partial_cmp(&(a.color.l, a.color.chroma)).unwrap_or(Ordering::Equal),
            // ColorOrder::DarkFirst  => (a.color.l, a.color.chroma).partial_cmp(&(b.color.l, b.color.chroma)).unwrap_or(Ordering::Equal),
        });
        new
    }

    fn sort_by_key_fn(a: Histo<Self::Color>) -> impl Ord {
        // a.color.l.partial_cmp(&a.color.l).unwrap_or(std::cmp::Ordering::Equal)
        // (a.color.l as i32, a.color.hue.into_inner() as i32)
        a.color.chroma as i32
        // (a.color.l as u32, a.color.chroma as i32, a.color.hue.into_inner() as i32)
    }
}
