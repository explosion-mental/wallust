//! Backends
//! * There are multiple methods in which you can get the most relevant colors from an image; rather
//!   than hardcoding, give options
//! * TODO add Oklab method
//! On this file are usual helper functions
use std::path::PathBuf;
use std::fmt;

use crate::{MyLab, Colors};

use image::io::Reader as ImageReader;
use anyhow::Result;
use lab::Lab;
use serde::*;

mod full;
mod resized;
use full::*;
use resized::*;

/// Simple Histogram
pub struct Histo {
    /// LAB colors
    pub color: Lab,
    /// number of times it has appeared
    pub count: usize,
}

impl Histo {
    /// Mix similar Lab colors, to catch most similars ones.
    pub fn mix(&mut self, new: Lab) {
        self.color.l = (self.color.l + new.l)  / 2.0;
        //self.color.a = (self.color.a + new.a).round()  / 2.0;
        //self.color.b = (self.color.b + new.b).round()  / 2.0;
    }
}

/// This indicates what 'parser' method to use, in the config file
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Backend {
    Full,
    Resized,
}

pub fn gen_colors(file: &PathBuf, backend: &Backend, threshold: u32) -> Result<Colors<MyLab>> {
    let method_to_use = match backend {
        Backend::Full => full,
        Backend::Resized => resized,
    };
    method_to_use(file, threshold)
}

/// Add a simple `Display` for [`Backend`], used in main() to print which is in use
impl fmt::Display for Backend {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Full    => write!(f, "full"),
            Self::Resized => write!(f, "resized"),
        }
    }
}

/// Threshold to accept the color difference
/// This is temporary, this constant should be auto to get the best result depending on the image
/// size (XXX maybe a threshold for image size then?)

/// determines whether a Lab color is present in our histogram, by using [`delta_e`] we compare if
/// colors are similar enough, using the [`Config.threshold`]
fn is_present(color: Lab, histogram: &mut Vec<Histo>, threshold: u32) -> bool {
    for e in histogram {
        // if any lab value is between a threshold, count it up
        if delta_e(color, e.color) < threshold {
            e.mix(color);
            e.count += 1;
            return true;
        }
    }
    false
}

fn gen_histogram(labs: Vec<Lab>, threshold: u32) -> Vec<Histo> {
    let mut histo: Vec<Histo> = vec![];

    for lab in labs {
        if is_present(lab, &mut histo, threshold) {
            continue;
        } else {
            histo.push(Histo { color: lab, count: 1 });
        }
    }

    // sort vec by count
    histo.sort_by(|a, b| b.count.cmp(&a.count));
    histo
}


/// Returns how much the colors differ
///
/// * TODO find out if the 2000 version worth
/// ref: <https://www.easyrgb.com/en/math.php>
#[inline]
pub fn delta_e(lab_0: Lab, lab_1: Lab) -> u32 {
    delta_2000(lab_0, lab_1).round() as u32
}

/// the 1994 simple euclidean formula
#[allow(dead_code)]
#[inline]
fn delta_1994(current: Lab, previous: Lab) -> f32 {
    (   ((previous.l - current.l).powf(2.0))
    +   ((previous.a - current.a).powf(2.0))
    +   ((previous.b - current.b).powf(2.0)) ).sqrt()
}

/// helper for the 2000 version
#[inline]
fn get_h_prime(a: f32, b: f32) -> f32 {
    let h_prime = b.atan2(a).to_degrees();
    if h_prime < 0.0 {
        h_prime + 360.0
    } else {
        h_prime
    }
}


/// the 2000 delta method, from <https://github.com/ryanobeirne/deltae>
#[inline]
fn delta_2000(lab_0: Lab, lab_1: Lab) -> f32 {
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
