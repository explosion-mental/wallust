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


/// rename [`Filters`] so it's shorter to type
use self::Filters as F;

mod dark;
mod dark16;
mod harddark;
mod light;
mod light16;
mod softlight;
mod softdark;

/// Corresponds to the modules inside this module and `filter` parameter in the config file.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum Filters {
    /// Dark bg, light fg
    Dark,
    /// Dark bg and fg with some opaque variations, making 16 colors
    Dark16,
    #[serde(alias = "hard-dark")]
    /// Dark with hard hue colors
    HardDark,
    /// Light bg, dark fg
    Light,
    /// Same as [`Light`] but with 8 color variations, making 16 in total
    Light16,
    #[serde(alias = "soft-light")]
    /// Light with soft pastel colors
    SoftLight,
    #[serde(alias = "soft-dark")]
    /// Similar to [`Dark`] but with the colors inversed
    SoftDark,
}

pub fn main(f: &Filters) -> fn(&[Myrgb]) -> Colors {
    match f {
        F::Dark    => dark::dark,
        F::Dark16  => dark16::dark16,
        F::HardDark => harddark::harddark,
        F::Light   => light::light,
        F::Light16 => light16::light16,
        F::SoftLight => softlight::softlight,
        F::SoftDark => softdark::softdark,
    }
}

/// Use different sorting `sort_by` on different filters, which creates even more schemes.
pub fn sort_ord(f: &Filters) -> crate::colorspaces::ColorOrder {
    match f {
        F::SoftLight |
        F::Dark  | F::Dark16 |
        F::SoftDark
        => crate::colorspaces::ColorOrder::LightFirst,

        F::HardDark |
        F::Light | F::Light16 => crate::colorspaces::ColorOrder::DarkFirst,
    }
}

impl Filters {
    /// Assign a color when printing in `main()`
    pub fn col(&self) -> AnsiColors {
        match self {
            F::Dark => AnsiColors::Blue,
            F::Dark16 => AnsiColors::Green,
            F::HardDark => AnsiColors::Magenta,
            F::Light => AnsiColors::Yellow,
            F::Light16 => AnsiColors::Cyan,
            F::SoftLight => AnsiColors::BrightYellow,
            F::SoftDark => AnsiColors::BrightCyan,
        }
    }
}

/// Display what [`Filters`] is in use. Used in cache and main.
impl fmt::Display for Filters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            F::Dark => write!(f, "Dark"),
            F::Dark16 => write!(f, "Dark16"),
            F::HardDark => write!(f, "HardDark"),
            F::Light => write!(f, "Light"),
            F::Light16 => write!(f, "Light16"),
            F::SoftLight => write!(f, "SoftLight"),
            F::SoftDark => write!(f, "SoftDark"),
        }
    }
}
