//! # Colorspaces
//! This is just an interface to get the most (16) prominent colors, from darkest to lightest, as
//! an rgb, [`Myrgb`] wrapper type, value. Different ways of collecting these can be achieve, and
//! so this deserved it's own module.
use std::fmt;
use std::rc::Rc;

use crate::colors::Myrgb;

use anyhow::Result;
use serde::{Serialize, Deserialize};
use owo_colors::AnsiColors;

/// rename [`ColorSpaces`] so it's shorter to type
use self::ColorSpaces as C;

mod lab;

const NOT_ENOUGH_COLS: &str =
"\
Not enough colors to create a scheme, even after trying to artificially generate new ones.
Try changing the threshold or the backend.
It may very well be that the image doesn't have enough colors.
Quitting...\
";


/// Enum to indicate how to sort the colors. This can allow you to choose which colors you would
/// like to use (e.g. light scheme or dark scheme), since you got them as the first colors.
/// Using these with [`full`] or [`resize`] backends, the LightFirst will give a more pastel
/// colors. While the DarkFrist will give you more heavy ones (more hue ones)
pub enum ColorOrder {
    /// `colors[0]` will be the lightest, and `colors.last()` will be the darkest
    LightFirst,
    /// `colors[0]` will be the darkest, and `colors.last()` will be the lightest
    DarkFirst,
}

/// Corresponds to the modules inside this module and `color_space` parameter in the config file.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum ColorSpaces {
    /// Uses Cie L*a*b color space
    Lab,
    /// Same as `lab` but mixes the colors gathered
    LabMixed,
}

pub fn main(c: ColorSpaces, cols: &[u8], th: u8, sort_ord: ColorOrder) -> Result<(Rc<[Myrgb]>, bool)> {
    match c {
        C::Lab      => lab::lab(cols, th, false, sort_ord),
        C::LabMixed => lab::lab(cols, th, true, sort_ord),
    }
}

impl ColorSpaces {
    /// Assign a color for the ColorSpaces
    pub fn col(&self) -> AnsiColors {
        match self {
            C::Lab => AnsiColors::Blue,
            C::LabMixed => AnsiColors::Green,
        }
    }
}

/// Display what [`ColorSpaces`] is in use. Used in cache and main.
impl fmt::Display for ColorSpaces {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            C::Lab => write!(f, "Lab"),
            C::LabMixed => write!(f, "LabMixed"),
        }
    }
}
