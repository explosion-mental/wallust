//! # Filters
//! A filter is just a way to fill the [`Colors`] struct. A method to generate a scheme that makes
//! the most prominent colors make sense as a scheme/palette. The vector slice will always have at
//! least 6 colors, so don't fear on using `.expect()` with this certainty and avoiding boilerplate
//! code. The scenario in which an image has less than those colors is possible and already handled in
//! the [`colorspaces`] module, so don't bother with that.
//!
//! * TODO maybe on v3.0.0 change name to scheme, which sounds better.
//! * XXX would other filters need more than 6 (or even 8) colors? if so, change the return type to
//!   `Result<Colors>` or just fallback to a scheme
use std::fmt;

use owo_colors::AnsiColors;
use serde::{Serialize, Deserialize};

use crate::colors::{Colors, Myrgb};

/// re export to shorten from `Filters::Dark` to `Dark`
use self::Filters::*;

mod dark;
mod dark16;
mod light;
mod light16;

/// Corresponds to the modules inside this module and `filter` parameter in the config file.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Filters {
    /// Dark bg, light fg
    Dark,
    /// Dark bg and fg with some opaque variations, making 16 colors
    Dark16,
    /// Light bg, dark fg
    Light,
    /// Same as [`Light`] but with 8 color variations, making 16 in total
    Light16,
}

pub fn main(f: &Filters) -> fn(&[Myrgb]) -> Colors {
    match f {
        Dark    => dark::dark,
        Dark16  => dark16::dark16,
        Light   => light::light,
        Light16 => light16::light16,
    }
}

/// Use different sorting `sort_by` on different filters, which creates even more schemes.
pub fn sort_ord(f: &Filters) -> crate::colorspaces::ColorOrder {
    match f {
        Dark  | Dark16  => crate::colorspaces::ColorOrder::LightFirst,
        Light | Light16 => crate::colorspaces::ColorOrder::DarkFirst,
    }
}

impl Filters {
    /// Assign a color when printing in `main()`
    pub fn col(&self) -> AnsiColors {
        match self {
            Dark => AnsiColors::Blue,
            Dark16 => AnsiColors::Green,
            Light => AnsiColors::Yellow,
            Light16 => AnsiColors::Cyan,
        }
    }
}

/// Display what [`Filters`] is in use. Used in cache and main.
impl fmt::Display for Filters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Dark => write!(f, "Dark"),
            Dark16 => write!(f, "Dark16"),
            Light => write!(f, "Light"),
            Light16 => write!(f, "Light16"),
        }
    }
}
