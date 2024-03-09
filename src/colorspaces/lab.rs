//! #About LAB
//! > The lightness value, L*, also referred to as "Lstar," defines black at 0 and white at 100.
//! > The a* axis is relative to the green-red opponent colors, with negative values toward green
//! > and positive > values toward red.
//! > The b* axis represents the blue-yellow opponents, with negative numbers toward
//! > blue and positive toward yellow.
//! ref: <https://en.wikipedia.org/wiki/CIELAB_color_space>
use super::*;

/// Shadow the colorspace type (Spectrum)
type Spec = palette::Lab;

/// Miminum Luminance (from L ab) required for a color to be accepted
pub const DARKEST: f32 = 4.5;

/// Maximuum Luminance (from L ab) required for a color to be accepted
pub const LIGHTEST: f32 = 95.5;

impl ColorTrait for Spec {}

impl Difference for Spec {
    fn col_diff(&self, a: &Self, threshold: u8) -> bool {
        use palette::color_difference::ImprovedCiede2000;
        self.improved_difference(*a) <= threshold.into()
        // use palette::color_difference::{EuclideanDistance, ImprovedCiede2000, ImprovedDeltaE};
        //self.improved_difference(*a) <= 1.26 * f32::from(threshold).powf(0.55)
        // delta_1994(self, a) <= threshold.into()
    }
}

impl BuildColors for ColorHisto<Spec> {
    type Color = Spec;
    fn filter_cols(a: Self::Color) -> bool { a.l >= DARKEST || a.l <= LIGHTEST }

    fn sort_algo(cs: &ColorOrder, a: &Histo<Self::Color>, b: &Histo<Self::Color>) -> Ordering {
        match cs {
            ColorOrder::LightFirst => b.color.l.partial_cmp(&a.color.l).unwrap_or(std::cmp::Ordering::Equal),
            ColorOrder::DarkFirst  => a.color.l.partial_cmp(&b.color.l).unwrap_or(std::cmp::Ordering::Equal),
        }
    }

    fn sort_by_key_fn(a: Histo<Self::Color>) -> impl Ord {
        (a.color.l as u32, a.color.a as i32, a.color.b as i32)
    }
}

impl From<Spec> for Myrgb {
    fn from(lab: Spec) -> Self {
        let a: Srgb = lab.into_color();
        Self(a)
    }
}

impl From<Myrgb> for Spec {
    fn from(c: Myrgb) -> Self {
        c.0.into_color()
    }
}
