//! # About LAB
//! > The lightness value, L*, also referred to as "Lstar," defines black at 0 and white at 100.
//! > The a* axis is relative to the green-red opponent colors, with negative values toward green
//! > and positive > values toward red.
//! > The b* axis represents the blue-yellow opponents, with negative numbers toward
//! > blue and positive toward yellow.
//! - ref: <https://en.wikipedia.org/wiki/CIELAB_color_space>
use super::*;

pub struct Lab;

/// Shadow the colorspace type (Spectrum)
pub type Spec = palette::Lab;

/// Miminum Luminance (from L ab) required for a color to be accepted
pub const DARKEST: f32 = 4.5;

/// Maximuum Luminance (from L ab) required for a color to be accepted
pub const LIGHTEST: f32 = 95.5;

impl ColorTrait for Spec {}

impl Difference for Spec {
    fn col_diff(&self, a: &Self, threshold: u8) -> bool {
        use palette::color_difference::ImprovedCiede2000;
        self.improved_difference(*a) <= threshold.into()
    }
}

impl BuildHisto<Spec> for Lab {
    fn filter_cols(histo: Vec<Spec>) -> Vec<Spec> {
        let lights = histo.iter().map(|c| c.l).collect::<Vec<_>>();
        let darkest  = lights.iter().fold(f32::INFINITY, |a, &b| a.min(b)).max(DARKEST);
        let lightest = lights.iter().fold(f32::INFINITY, |a, &b| a.max(b)).min(LIGHTEST);

        let filt = |x: Spec| x.l >= darkest && x.l <= lightest;

        histo.into_iter().filter(|&c| filt(c)).collect()

    }


    fn sort_col(histo: Vec<Histo<Spec>>, cs: &ColorOrder) -> Vec<Histo<Spec>> {
        let mut histo = histo;

        histo.sort_by(|a, b| match cs {
            ColorOrder::LightFirst => b.color.l.partial_cmp(&a.color.l).unwrap_or(std::cmp::Ordering::Equal),
            ColorOrder::DarkFirst  => a.color.l.partial_cmp(&b.color.l).unwrap_or(std::cmp::Ordering::Equal),
        });

        histo
    }

    fn sort_by_key_fn(a: Histo<Spec>) -> impl Ord {
        (a.color.l as u32, a.color.a as i32, a.color.b as i32)
    }
}

