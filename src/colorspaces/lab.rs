//! #About LAB
//! > The lightness value, L*, also referred to as "Lstar," defines black at 0 and white at 100.
//! > The a* axis is relative to the green-red opponent colors, with negative values toward green
//! > and positive > values toward red.
//! > The b* axis represents the blue-yellow opponents, with negative numbers toward
//! > blue and positive toward yellow.
//! ref: <https://en.wikipedia.org/wiki/CIELAB_color_space>
//! * TODO find out if the 2000 version worth
use crate::colorspaces::*;

use ::lab::rgb_bytes_to_labs;
use ::lab::Lab;
use itertools::Itertools;

/// Currently this works in function with the filters methods, which currently only needs 6 colors.
/// Let's make sure the colorspace backend send at least these number of colors.
const MIN_COLS: u8 = 6;

/// The [`Colors`] struct only has capacity for 16 colors 0..=15. const is used in order to take
/// the top MAX_COLS lab colors.
const MAX_COLS: u8 = 16;

pub fn lab(cols: &[u8], threshold: u8, mix: bool, sort: ColorOrder) -> Result<(Vec<Myrgb>, bool)> {
    let mut labs = rgb_bytes_to_labs(cols);
    labs.dedup();

    // This is to indicate if there were any warnings, since we can't print them directly
    let mut warn = false;

    let mut histo: Vec<Histo> = vec![];

    for lab in labs {
        if lab.l < 1.0 || lab.l > 99.0 { continue; } //ignore really dark/light colors
        if is_present(lab, &mut histo, threshold, mix) {
            continue;
        } else {
            histo.push(Histo { color: lab, count: 1 });
        }
    }

    // sort vec by count, most used colors
    histo.sort_by(|a, b| b.count.cmp(&a.count));

    // take the *necessary* most used colors
    let mut histo: Vec<Histo> = histo.into_iter().take(MAX_COLS.into()).collect();

    if histo.len() < 2 {
        anyhow::bail!("Image should at least have two different pixel colors.");
    }

    // Artificially generate colors with linear interpolation in between the colors that we already
    // have. However even this can even fail and not generate enough different colors, so there is
    // another check below
    if histo.len() < MIN_COLS.into() {
        warn = true;
        // copy the vector and combine over it.
        let combination = histo.clone().into_iter().combinations(2);
        // try to generate new colors with interpolation in between the already gathered colors
        for comb in combination {
            let color_a: Myrgb = comb[0].color.into();
            let color_b: Myrgb = comb[1].color.into();

            interpolate(&mut histo, color_a, color_b, MAX_COLS, threshold);
            if histo.len() >= MIN_COLS.into() { break; } //enough colors, stop interpolating
        }
        // take again, just to be sure
        histo = histo.into_iter().take(MAX_COLS.into()).collect();
    }

    // not enough colors, even after making new colors (if any)
    if histo.len() < MIN_COLS.into() {
        anyhow::bail!(NOT_ENOUGH_COLS);
    }

    // custom sorting, checkout [`ColorOrder`] and [`sort_ord`]
    // TODO: read more about partial_cmp and float arithmetic
    // XXX: inverting these will create a pseudo light scheme
    histo.sort_by(|a, b|
        match &sort {
            ColorOrder::LightFirst => b.color.l.partial_cmp(&a.color.l).unwrap(),
            ColorOrder::DarkFirst  => a.color.l.partial_cmp(&b.color.l).unwrap(),
        }
    );

    let histo: Vec<Myrgb> = histo.iter().map(|x| x.color.into()).collect();

    Ok((histo, warn))
}

/// Combines some colors to generate new ones
/// Using something similar to <https://github.com/ndavd/colinterp>
/// I didn't find anything about interpolating CIE L,a*b* colors, only RGB ones, so I'm accepting
/// converting into and from just for this operation (which should not overhead the program since
/// at max is only 5 values in combination)
/// This goes like this: `lab -> rgb -> interpolation -> lab -> sort_by -> rgb`
fn interpolate(histo: &mut Vec<Histo>, color_a: Myrgb, color_b: Myrgb, n: u8, threshold: u8) {
    //return (endValue - startValue) * stepNumber / lastStepNumber + startValue;
    let mut palette: Vec<Myrgb> = vec![];

    // cast to i16 to not overflow u8
    let jump_r = (f32::from(color_b.0 as i16 - color_a.0 as i16)) / (f32::from(n) - 1.0);
    let jump_g = (f32::from(color_b.1 as i16 - color_a.1 as i16)) / (f32::from(n) - 1.0);
    let jump_b = (f32::from(color_b.2 as i16 - color_a.2 as i16)) / (f32::from(n) - 1.0);

    let mut curr_r = f32::from(color_a.0);
    let mut curr_g = f32::from(color_a.1);
    let mut curr_b = f32::from(color_a.2);

    for _ in 0..n {
        let r = curr_r.round() as u8;
        let g = curr_g.round() as u8;
        let b = curr_b.round() as u8;
        palette.push(Myrgb(r, g, b));
        curr_r += jump_r;
        curr_g += jump_g;
        curr_b += jump_b;

    }

    //similar to how it's done at the start of `lab()`
    for c in palette {
        let lab = Lab::from_rgb(&[c.0, c.1, c.2]);
        if lab.l < 1.0 || lab.l > 99.0 { continue; } //ignore really dark/light colors
        // We can't mix colors given that there aren't many.
        if is_present(lab, histo, threshold, false) {
            continue;
        } else {
            histo.push(Histo { color: lab, count: 1 });
        }
    }
}

/// Simple Histogram
/// TODO think about a better generic way of storing (ColorSpace, count)
#[derive(Debug, Copy, Clone)]
struct Histo {
    /// LAB colors
    color: ::lab::Lab,
    /// number of times it has appeared
    count: usize,
}

impl Histo {
    /// Mix similar Lab colors, to catch most similars ones.
    /// NOTE: This reduces color quantity
    fn mix(&mut self, new: Lab) {
        self.color.l = self.color.l * 0.5 + new.l * 0.5;
        //self.color.a = self.color.a * 0.5 + new.a * 0.5;
        //self.color.b = self.color.b * 0.5 + new.b * 0.5;
    }
}

impl From<Lab> for Myrgb {
    fn from(lab: Lab) -> Self {
        let a = lab.to_rgb();
        Self(a[0], a[1], a[2])
    }
}


/// determines whether a Lab color is present in our histogram, by using [`delta_e`] we compare if
/// colors are similar enough, using the [`Config.threshold`]
fn is_present(color: Lab, histogram: &mut Vec<Histo>, threshold: u8, mix: bool) -> bool {
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

/// Returns how much the colors differ
///
/// ref: <https://www.easyrgb.com/en/math.php>
#[inline]
fn delta_e(lab_0: Lab, lab_1: Lab) -> u32 {
    delta_2000(lab_0, lab_1) as u32
}

/// the 1994 simple euclidean formula
#[allow(dead_code)]
#[inline]
fn delta_1994(current: Lab, previous: Lab) -> f32 {
    (   ((previous.l - current.l).powf(2.0))
    +   ((previous.a - current.a).powf(2.0))
    +   ((previous.b - current.b).powf(2.0)) ).sqrt()
}

/// the 2000 delta method, from <https://github.com/ryanobeirne/deltae>
#[allow(dead_code)]
#[inline]
fn delta_2000(lab_0: Lab, lab_1: Lab) -> f32 {

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
