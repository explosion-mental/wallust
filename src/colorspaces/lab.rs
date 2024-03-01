//! #About LAB
//! > The lightness value, L*, also referred to as "Lstar," defines black at 0 and white at 100.
//! > The a* axis is relative to the green-red opponent colors, with negative values toward green
//! > and positive > values toward red.
//! > The b* axis represents the blue-yellow opponents, with negative numbers toward
//! > blue and positive toward yellow.
//! ref: <https://en.wikipedia.org/wiki/CIELAB_color_space>
use std::cmp::Ordering;

use crate::colorspaces::*;

/// Shadow the colorspace type (Spectrum)
type Spec = palette::Lab;

/// shadow `Histo<Lab>` with Hist (since this module is all about LAB)
type Hist = Histo<Spec>;

/// Miminum Luminance (from L ab) required for a color to be accepted
pub const DARKEST: f32 = 4.5;

/// Maximuum Luminance (from L ab) required for a color to be accepted
pub const LIGHTEST: f32 = 95.5;

impl ColorTrait for Spec {}

use palette::color_difference::{EuclideanDistance, ImprovedCiede2000, ImprovedDeltaE};
use palette::{IntoColor, Srgb};

impl From<Spec> for Myrgb {
    fn from(lab: Spec) -> Self {
        let a: Srgb = lab.into_color();
        let a: Srgb<u8> = a.into_format();
        Self(a.red, a.green, a.blue)
    }
}

impl From<Myrgb> for Spec {
    fn from(c: Myrgb) -> Self {
        let a = Srgb::from([c.0, c.1, c.2]);
        let a: Srgb = a.into_format();
        a.into_color()
    }
}

impl From<Srgb<u8>> for Myrgb {
    fn from(c: Srgb<u8>) -> Self {
        Self(c.red, c.green, c.blue)
    }
}

impl From<Myrgb> for Srgb<u8> {
    fn from(c: Myrgb) -> Self {
        Self::new(c.0, c.1, c.2)
    }
}

/// Mixed all field of a LAB colorspace into one.
/// While the proper way to do that is by converting lab to rgb and then mixing rgb (.blend) and
/// then back to lab, I'm doing this hacky way in the meantime
fn mixed(color1: Spec, color2: Spec) -> Spec {
    let rgb1: Myrgb = color1.into();
    let rgb2: Myrgb = color2.into();
    let mut new: Spec = rgb1.blend(rgb2).into();

    if new.l > LIGHTEST {
        new.l = new.l - (LIGHTEST - new.l) - 1.0;
    } else if new.l < DARKEST {
        new.l = new.l + (DARKEST - new.l) + 1.0;
    }

    // new.l = ((color1.l + color2.l) / 2.0) - 1.0;
    // new.a = ((color1.a + color2.a) / 2.0) - 1.0;
    // new.b = ((color1.b + color2.b) / 2.0) - 1.0;
    new
}

impl Difference for Spec {
    //TODO see delta_e
    fn col_diff(&self, a: &Self, threshold: u8) -> bool {
        //self.improved_difference(*a) <= 1.26 * f32::from(threshold).powf(0.55)
        // self.improved_difference(*a) <= threshold.into()
        delta_1994(self, a) <= threshold.into()
    }
}

use palette::cast::ComponentsAs;

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
        (a.color.l as u32, a.color.a as i32, a.color.b as i32)
    }
}


//TODO
// pub fn histo<F>(cols: &[u8], threshold: u8, mix: bool, pred: F) -> Vec<Hist>
//     where F: Fn(f32) -> bool
// {
//     let mut labs = rgb_bytes_to_labs(cols);
//     labs.dedup();
//     // XXX using `delta_e` with `.dedup()` here, reduces the vector that littlel
//     // that the colors aren't the most prominent ones (for the most part).
//     // However, avoiding `.dedup()` and not calling it, also changes the result.
//     // After some testing I decided that the most 'plausible' colors would be
//     // the one that requires `.dedup()`.
//     //labs.dedup_by(|a, b| lab::delta_e(*a, *b) <= threshold.into());
//
//     gather_cols(labs, threshold, mix, &pred)
// }

/// Returns how much the colors differ
///
/// ref: <https://www.easyrgb.com/en/math.php>
/// NOTE: using `delta_1994()` instead of `delta_2000()` improves around 50% of of performance
/// (by criterion),
#[inline]
pub fn delta_e(lab_0: &Spec, lab_1: &Spec) -> u32 {
    // TODO properly analize the following
    // an improved version of delta_e.. This has a limit threshold of 8, with pretty good results.
    //https://github.com/Ogeon/palette/blob/c54efbd43c03267713da337bd72005c9d0390598/palette/src/lab.rs#L269
    //(1.26 * delta_1994(lab_0, lab_1).powf(0.55)).round() as u32
    //delta_2000(lab_0, lab_1) as u32
    delta_1994(lab_0, lab_1) as u32
}

/// the 1994 simple euclidean formula
#[allow(dead_code)]
#[inline]
fn delta_1994(current: &Spec, previous: &Spec) -> f32 {
    (   ((previous.l - current.l).powf(2.0))
    +   ((previous.a - current.a).powf(2.0))
    +   ((previous.b - current.b).powf(2.0)) ).sqrt()
}

/// the 2000 delta method, from <https://github.com/ryanobeirne/deltae>
#[allow(dead_code)]
#[inline]
fn delta_2000(lab_0: Spec, lab_1: Spec) -> f32 {

    let get_h_prime = |a: f32, b: f32| -> f32 {
        let h_prime = b.atan2(a).to_degrees();
        if h_prime < 0.0 {
            h_prime + 360.0
        } else {
            h_prime
        }
    };

    let chroma_0 = (lab_0.a.powi(2) + lab_0.b.powi(2)).sqrt();
    let chroma_1 = (lab_1.a.powi(2) + lab_1.b.powi(2)).sqrt();

    let c_bar = (chroma_0 + chroma_1) / 2.0;

    let g = 0.5 * (1.0 - ( c_bar.powi(7) / (c_bar.powi(7) + 25_f32.powi(7)) ).sqrt());

    let a_prime_0 = lab_0.a * (1.0 + g);
    let a_prime_1 = lab_1.a * (1.0 + g);

    let c_prime_0 = (a_prime_0.powi(2) + lab_0.b.powi(2)).sqrt();
    let c_prime_1 = (a_prime_1.powi(2) + lab_1.b.powi(2)).sqrt();

    let l_bar_prime = (lab_0.l + lab_1.l)/2.0;
    let c_bar_prime = (c_prime_0 + c_prime_1) / 2.0;

    let h_prime_0 = get_h_prime(a_prime_0, lab_0.b);
    let h_prime_1 = get_h_prime(a_prime_1, lab_1.b);

    let h_bar_prime = if (h_prime_0 - h_prime_1).abs() > 180.0 {
        if (h_prime_0 - h_prime_1) < 360.0 {
            (h_prime_0 + h_prime_1 + 360.0) / 2.0
        } else {
            (h_prime_0 + h_prime_1 - 360.0) / 2.0
        }
    } else {
        (h_prime_0 + h_prime_1) / 2.0
    };

    let t = 1.0 - 0.17 * ((      h_bar_prime - 30.0).to_radians()).cos()
                + 0.24 * ((2.0 * h_bar_prime       ).to_radians()).cos()
                + 0.32 * ((3.0 * h_bar_prime +  6.0).to_radians()).cos()
                - 0.20 * ((4.0 * h_bar_prime - 63.0).to_radians()).cos();

    let mut delta_h = h_prime_1 - h_prime_0;
    if delta_h > 180.0 && h_prime_1 <= h_prime_0 {
        delta_h += 360.0;
    } else if delta_h > 180.0 {
        delta_h -= 360.0;
    };

    let delta_l_prime = lab_1.l - lab_0.l;
    let delta_c_prime = c_prime_1 - c_prime_0;
    let delta_h_prime = 2.0 * (c_prime_0 * c_prime_1).sqrt() * (delta_h.to_radians() / 2.0).sin();

    let s_l = 1.0 + (
              (0.015 * (l_bar_prime - 50.0).powi(2))
            / (20.00 + (l_bar_prime - 50.0).powi(2)).sqrt()
        );
    let s_c = 1.0 + 0.045 * c_bar_prime;
    let s_h = 1.0 + 0.015 * c_bar_prime * t;

    let delta_theta = 30.0 * (-((h_bar_prime - 275.0)/25.0).powi(2)).exp();
    let r_c =  2.0 * (c_bar_prime.powi(7)/(c_bar_prime.powi(7) + 25_f32.powi(7))).sqrt();
    let r_t = -(r_c * (2.0 * delta_theta.to_radians()).sin());

    let k_l = 1.0;
    let k_c = 1.0;
    let k_h = 1.0;

    (
        (delta_l_prime/(k_l*s_l)).powi(2)
      + (delta_c_prime/(k_c*s_c)).powi(2)
      + (delta_h_prime/(k_h*s_h)).powi(2)
      + (r_t * (delta_c_prime/(k_c*s_c)) * (delta_h_prime/(k_h*s_h)))
    ).sqrt()
}
