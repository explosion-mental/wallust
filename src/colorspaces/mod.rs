//! # Colorspaces
//! This is just an interface to get the most (16) prominent colors, from darkest to lightest, as
//! an rgb, [`Myrgb`] wrapper type, value. Different ways of collecting these can be achieve, and
//! so this deserved it's own module.
use std::fmt;

use crate::colors::Myrgb;

use serde::{Serialize, Deserialize};
use owo_colors::AnsiColors;

mod lab;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ColorSpaces {
    Lab,
    LabMixed,
}

pub fn main(cols: &[u8], th: u32, cs: &ColorSpaces) -> Vec<Myrgb> {
    match cs {
        ColorSpaces::Lab => self::lab::lab(cols, th, false),
        ColorSpaces::LabMixed => self::lab::lab(cols, th, true),
    }
}

impl ColorSpaces {
    /// This assigns a colors for a backend, used when printing
    pub fn col(&self) -> AnsiColors {
        match self {
            Self::Lab => AnsiColors::Blue,
            Self::LabMixed => AnsiColors::Green,
        }
    }
}

/// Add a simple `Display` for [`Backend`], used in main() to print which is in use
impl fmt::Display for ColorSpaces {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Lab => write!(f, "Lab"),
            Self::LabMixed => write!(f, "LabMixed"),
        }
    }
}
