//! # LCH
//! CIE L*C*h°, a polar version of CIE L*a*b*.
//! ref: <https://docs.rs/palette/latest/palette/lch/struct.Lch.html>
use super::*;

use palette::IntoColor;
use palette::cast::ComponentsAs;

/// Shadow the colorspace type (Spectrum)
type Spec = palette::Lch;

/// Miminum Luminance (from L ab) required for a color to be accepted
pub const DARKEST: f32 = 4.5;

/// Maximuum Luminance (from L ab) required for a color to be accepted
pub const LIGHTEST: f32 = 95.5;

impl ColorTrait for Spec {}

impl Difference for Spec {
    //TODO see delta_e
    fn col_diff(&self, a: &Self, threshold: u8) -> bool {
        #[allow(unused)]
        use palette::color_difference::{EuclideanDistance, ImprovedCiede2000, ImprovedDeltaE};
        //ImprovedCiede2000::<Scalar = u8>::improved_difference(self, a);
        self.improved_difference(*a) <= 1.26 * f32::from(threshold).powf(0.55)
        // self.difference(*a) <= threshold.into()
        // self.improved_difference(*a) <= threshold.into()
        // delta_1994(self, a) <= threshold.into()
    }
}


impl BuildColors for ColorHisto<Spec> {
    type Color = Spec;
    fn read(bytes: &[u8]) -> Vec<Self::Color> {
        let s: &[Srgb<u8>] = bytes.components_as();
        s
            .iter()
            .map(|x| x.into_linear().into_color())
            .collect()
    }

    fn filter_cols(a: Self::Color) -> bool { a.l >= DARKEST || a.l <= LIGHTEST }

    fn sort_algo(cs: &ColorOrder, a: &Histo<Self::Color>, b: &Histo<Self::Color>) -> Ordering {
        match cs {
            ColorOrder::LightFirst => b.color.l.partial_cmp(&a.color.l).unwrap_or(std::cmp::Ordering::Equal),
            ColorOrder::DarkFirst  => a.color.l.partial_cmp(&b.color.l).unwrap_or(std::cmp::Ordering::Equal),
        }
    }

    fn sort_by_key_fn(a: Histo<Self::Color>) -> impl Ord {
        //TODO how to do .hue
        (a.color.chroma as i32, a.color.l as i32)
    }
}

impl From<Spec> for Myrgb {
    fn from(lab: Spec) -> Self {
        let a: Srgb = lab.into_color();
        let a: Srgb<u8> = a.into_format();
        Self(a.red, a.green, a.blue)
    }
}

impl From<Myrgb> for Spec {
    fn from(c: Myrgb) -> Self {
        Srgb::from([c.0, c.1, c.2]).into_linear().into_color()
    }
}
