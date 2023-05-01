//! # Filters
//! A filter is just a way to fill the [`Colors`] struct. A method to generate a scheme that makes
//! the most prominent colors make sense as a scheme/palette. The vector slice will always have at
//! least 6 colors, so don't fear on using `.expect()` with this certainty and avoiding boilerplate
//! code. The scenario in which an image has less than those colors is possible and already handled in
//! the [`colorspaces`] module, so don't bother with that.
//! * TODO maybe on v3.0.0 change name to scheme, which sounds better.
//! * XXX would other filters need more than 6 (or even 8) colors? if so, change the return type to `Result<Colors>`
use std::fmt;

use owo_colors::AnsiColors;
use serde::{Serialize, Deserialize};

use crate::colors::{Colors, Myrgb};

mod dark;
mod dark16;
mod light;
mod light16;

/// Corresponds to the modules inside this module and `filter` parameter in the config file.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Filters {
    Dark,
    Dark16,
    Light,
    Light16,
}

pub fn main(f: &Filters) -> fn(&[Myrgb]) -> Colors {
    match f {
        Filters::Dark    => dark::dark,
        Filters::Dark16  => dark16::dark16,
        Filters::Light   => light::light,
        Filters::Light16 => light16::light16,
    }
}

impl Filters {
    /// Assign a color when printing in `main()`
    pub fn col(&self) -> AnsiColors {
        match self {
            Self::Dark => AnsiColors::Blue,
            Self::Dark16 => AnsiColors::Green,
            Self::Light => AnsiColors::Yellow,
            Self::Light16 => AnsiColors::Cyan,
        }
    }
}

/// Display what [`Filters`] is in use. Used in cache and main.
impl fmt::Display for Filters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Dark => write!(f, "Dark"),
            Self::Dark16 => write!(f, "Dark16"),
            Self::Light => write!(f, "Light"),
            Self::Light16 => write!(f, "Light16"),
        }
    }
}
