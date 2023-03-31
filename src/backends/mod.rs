//! Backends
//! * There are multiple methods in which you can get the most relevant colors from an image; rather
//!   than hardcoding, give options
//! * TODO add Oklab method
use std::path::PathBuf;
use std::fmt;

use crate::delta::delta_e;
use crate::{MyLab, Colors};

use image::io::Reader as ImageReader;
use anyhow::Result;
use lab::Lab;
use serde::*;

mod full;
mod resized;
pub use full::*;
pub use resized::*;

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
