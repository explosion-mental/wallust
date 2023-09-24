//! #About LAB
//! > The lightness value, L*, also referred to as "Lstar," defines black at 0 and white at 100.
//! > The a* axis is relative to the green-red opponent colors, with negative values toward green
//! > and positive > values toward red.
//! > The b* axis represents the blue-yellow opponents, with negative numbers toward
//! > blue and positive toward yellow.
//! ref: <https://en.wikipedia.org/wiki/CIELAB_color_space>
use crate::colorspaces::*;

use ::lab::rgb_bytes_to_labs;
use ::lab::Lab;

/// Shadow the colorspace type (Spectrum)
type Spec = Lab;

/// shadow `Histo<Lab>` with Hist (since this module is all about LAB)
type Hist = Histo<Spec>;


impl Hist {
    /// Mix similar Lab colors, to catch most similars ones.
    /// NOTE: This reduces color quantity
    fn mix(&mut self, new: Spec) {
        self.color.l = self.color.l * 0.5 + new.l * 0.5;
        //self.color.a = self.color.a * 0.5 + new.a * 0.5;
        //self.color.b = self.color.b * 0.5 + new.b * 0.5;
    }
}

impl From<Spec> for Myrgb {
    fn from(lab: Spec) -> Self {
        let a = lab.to_rgb();
        Self(a[0], a[1], a[2])
    }
}

impl From<Myrgb> for Spec {
    fn from(c: Myrgb) -> Self {
        Lab::from_rgb(&[c.0, c.1, c.2])
    }
}

/// determines whether a Lab color is present in our histogram, by using [`delta_e`] we compare if
/// colors are similar enough, using the [`Config.threshold`]
fn is_present(color: Spec, histogram: &mut [Hist], threshold: u8, mix: bool) -> bool {
    for e in histogram {
        // if any lab value is between a threshold, count it up
        if delta_e(color, e.color) < threshold.into() {
            if mix { e.mix(color); }
            e.count += 1;
            return true;
        }
    }
    false
}

/// This doesn't `Histo.mix()`, so no need for mutability
fn is_present_no_mut(color: Spec, histogram: &[Hist], threshold: u8) -> bool {
    histogram.iter().any(|&x| delta_e(color, x.color) < threshold.into())
}

/// ColorSpaces for the [`Lab`] with floating numbers (more precise)
impl CSpaces for Cols<Spec, f32> {
    fn new(cols: &[u8], threshold: u8, mix: bool) -> Self {
        let darkest_lab = f32::from(threshold) * 0.3;
        let lightest_lab = 100.0 - darkest_lab;
        let mut histo: Vec<Hist> = vec![];
        let mut labs = rgb_bytes_to_labs(cols);
        labs.dedup();

        for lab in labs {
            if lab.l <  darkest_lab //ignore really dark colors
            || lab.l > lightest_lab //ignore really light colors
            || is_present(lab, &mut histo, threshold, mix) {
                continue;
            } else {
                histo.push(Histo { color: lab, count: 1 });
            }
        }
        //histo
        Self { histo, threshold,
            darkest: darkest_lab,
            lightest: lightest_lab,
        }
    }
    fn sort_colors(&mut self, method: &ColorOrder) {
        self.histo.sort_by(|a, b|
            match method {
                ColorOrder::LightFirst => b.color.l.partial_cmp(&a.color.l).unwrap_or(std::cmp::Ordering::Equal),
                ColorOrder::DarkFirst  => a.color.l.partial_cmp(&b.color.l).unwrap_or(std::cmp::Ordering::Equal),
            }
        );
    }
    fn new_cols(&mut self) {
        let threshold = self.threshold;
        let histo = &self.histo;

        let darkest_lab = self.darkest;
        let lightest_lab = self.lightest;

        let mut new_cols = vec![];
        // try to generate new colors with interpolation in between the already gathered colors
        for comb in histo.iter().combinations(2) {
            let color_a: Myrgb = comb[0].color.into();
            let color_b: Myrgb = comb[1].color.into();

            let new = interpolate(color_a, color_b, MAX_COLS);

            //similar to how it's done at the start of `lab()`
            // save the new colors, or discard them if similar enough
            for i in new {
                let lab: Spec = i.into();
                if lab.l <  darkest_lab
                || lab.l > lightest_lab
                || is_present_no_mut(lab, &histo, threshold) {
                    continue;
                } else {
                    new_cols.push(Histo { color: lab, count: 1 });
                }
            }

            let len = histo.len() + new_cols.len();

            if len >= MIN_COLS.into() { break; } //enough colors, stop interpolating
        }


        //join `new_cols` to histo
        self.histo.extend(new_cols);
    }
}

/// ColorSpaces for the [`Lab`] with unsigned integers (faster but more margin of error)
impl CSpaces for Cols<Spec, u32> {
    fn new(cols: &[u8], threshold: u8, mix: bool) -> Self {
        let darkest_lab = f32::from(threshold) * 0.3;
        let lightest_lab = (100.0 - darkest_lab) as u32;
        let darkest_lab = darkest_lab as u32;

        let mut histo: Vec<Hist> = vec![];
        let mut labs = rgb_bytes_to_labs(cols);
        labs.dedup();

        for lab in labs {
            if (lab.l as u32) <  darkest_lab //ignore really dark colors
            || (lab.l as u32) > lightest_lab //ignore really light colors
            || is_present(lab, &mut histo, threshold, mix) {
                continue;
            } else {
                histo.push(Histo { color: lab, count: 1 });
            }
        }
        //histo
        Self { histo, threshold,
            darkest: darkest_lab,
            lightest: lightest_lab,
        }
    }
    fn sort_colors(&mut self, method: &ColorOrder) {
        self.histo.sort_by(|a, b|
            match method {
                ColorOrder::LightFirst => b.color.l.partial_cmp(&a.color.l).unwrap_or(std::cmp::Ordering::Equal),
                ColorOrder::DarkFirst  => a.color.l.partial_cmp(&b.color.l).unwrap_or(std::cmp::Ordering::Equal),
            }
        );
    }
    fn new_cols(&mut self) {
        let threshold = self.threshold;
        let histo = &self.histo;

        let darkest_lab = self.darkest;
        let lightest_lab = self.lightest;

        let mut new_cols = vec![];
        // try to generate new colors with interpolation in between the already gathered colors
        for comb in histo.iter().combinations(2) {
            let color_a: Myrgb = comb[0].color.into();
            let color_b: Myrgb = comb[1].color.into();

            let new = interpolate(color_a, color_b, MAX_COLS);

            //similar to how it's done at the start of `lab()`
            // save the new colors, or discard them if similar enough
            for i in new {
                let lab: Spec = i.into();
                //ignore really dark/light colors
                if (lab.l as u32) < darkest_lab
                || (lab.l as u32) > lightest_lab
                || is_present_no_mut(lab, &histo, threshold) {
                    continue;
                } else {
                    new_cols.push(Histo { color: lab, count: 1 });
                }
            }

            let len = histo.len() + new_cols.len();

            if len >= MIN_COLS.into() { break; } //enough colors, stop interpolating
        }

        //join `new_cols` to histo
        self.histo.extend(new_cols);
    }
}

/// Returns how much the colors differ
///
/// ref: <https://www.easyrgb.com/en/math.php>
/// NOTE: using `delta_1994()` instead of `delta_2000()` improves around 50% of of performance
/// (by criterion),
#[inline]
fn delta_e(lab_0: Spec, lab_1: Spec) -> u32 {
    //delta_2000(lab_0, lab_1) as u32
    delta_1994(lab_0, lab_1) as u32
}

/// the 1994 simple euclidean formula
#[allow(dead_code)]
#[inline]
fn delta_1994(current: Spec, previous: Spec) -> f32 {
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
