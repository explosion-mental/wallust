//! # Filters
//! A filter is just a way to fill the [`Colors`] struct. A method to generate a scheme that makes
//! the most prominent colors make sense as a scheme/palette. The vector slice will always have at
//! least 6 colors, so don't fear on using `.expect()` with this certainty and avoiding boilerplate
//! code. The scenario in which an image has less than those colors is possible and already handled in
//! the [`crate::colorspaces`] module, so don't bother with that.
//!
//! # Adding a new filter
//! To integrate a new filter you have in mind, there are X rules:
//!  1. The name of the filter should be as the filename and function name.
//!  2. Comments indicating the [`ColorOrder`], what does the filter do and how it does it, should
//!     be in a doc comment of the function itself.
//!  3. If it's a variation of an already existing filter, it should be indicated as a comment.
//!
//! * TODO maybe on v3.0.0 change name to scheme, which sounds better.
//! * XXX would other filters need more than 6 (or even 8) colors? if so, change the return type to
//!   `Result<Colors>` or just fallback to a scheme
use std::fmt;

use owo_colors::AnsiColors;
use serde::{Serialize, Deserialize};

use crate::{
    colors::{
        Colors, Myrgb
    },
    colorspaces::{Cols, ColorOrder}
};

/// rename [`Filters`] so it's shorter to type
use self::Filters as F;

mod dark;
mod dark16;
mod harddark;
mod harddark16;
mod light;
mod light16;
mod softdark;
mod softdark16;
mod softlight;
mod softlight16;
mod darkcomp;

/// Corresponds to the modules inside this module and `filter` parameter in the config file.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy, Default, clap::ValueEnum)]
#[cfg_attr(feature = "makeconfig", derive(documented::Documented, documented::DocumentedFields, strum::EnumIter))]
#[serde(rename_all = "lowercase")]
pub enum Filters {
    /// 8 dark colors, dark background and light contrast
    #[default]
    Dark,
    /// Same as `dark` but uses the 16 colors trick
    Dark16,
    /// Same as `dark` with hard hue colors
    #[clap(alias  = "hard-dark", name = "harddark")] //clap prefers this-name
    #[serde(alias = "hard-dark")]
    HardDark,
    /// Harddark with 16 color variation
    #[clap(alias  = "hard-dark16", name = "harddark16")] //clap prefers this-name
    #[serde(alias = "hard-dark16")]
    HardDark16,
    /// Light bg, dark fg
    Light,
    /// Same as `light` but uses the 16 color trick
    Light16,
    /// Variant of softlight, uses the lightest colors and a dark background (could be interpreted as `dark` inversed)
    #[clap(alias  = "soft-dark", name = "softdark")]
    #[serde(alias = "soft-dark")]
    SoftDark,
    /// softdark with 16 color variation
    #[clap(alias  = "soft-dark16", name = "softdark16")]
    #[serde(alias = "soft-dark16")]
    SoftDark16,
    /// Light with soft pastel colors, counterpart of `harddark`
    #[clap(alias  = "soft-light", name = "softlight")]
    #[serde(alias = "soft-light")]
    SoftLight,
    /// softlight with 16 color variation
    #[clap(alias  = "soft-light16", name = "softlight16")]
    #[serde(alias = "soft-light16")]
    SoftLight16,
    /// Dark but colors from 9 to 14 are complementary to 1 to 7
    #[clap(alias  = "dark-comp", name = "darkcomp")]
    #[serde(alias = "dark-comp")]
    DarkComp,
}

pub fn main(f: &Filters) -> fn(Cols) -> Colors {
    match f {
        F::Dark    => dark::dark,
        F::Dark16  => dark16::dark16,
        F::HardDark => harddark::harddark,
        F::HardDark16 => harddark16::harddark16,
        F::Light   => light::light,
        F::Light16 => light16::light16,
        F::SoftDark => softdark::softdark,
        F::SoftDark16 => softdark16::softdark16,
        F::SoftLight => softlight::softlight,
        F::SoftLight16 => softlight16::softlight16,
        F::DarkComp => darkcomp::darkcomp,
    }
}

/// Use different sorting `sort_by` on different filters, which creates even more schemes.
pub fn sort_ord(f: &Filters) -> ColorOrder {
    match f {
          F::Dark  | F::Dark16
        | F::SoftDark | F::SoftDark16
        | F::SoftLight | F::SoftLight16
        | F::DarkComp
        => ColorOrder::LightFirst,

          F::HardDark | F::HardDark16
        | F::Light | F::Light16
        => ColorOrder::DarkFirst,
    }
}

impl Filters {
    /// Assign a color when printing in `main()`
    pub fn col(&self) -> AnsiColors {
        match self {
            F::Dark => AnsiColors::Blue,
            F::Dark16 => AnsiColors::Green,
            F::HardDark => AnsiColors::Magenta,
            F::HardDark16 => AnsiColors::Magenta,
            F::Light => AnsiColors::Yellow,
            F::Light16 => AnsiColors::Cyan,
            F::SoftDark => AnsiColors::BrightCyan,
            F::SoftDark16 => AnsiColors::BrightCyan,
            F::SoftLight => AnsiColors::BrightYellow,
            F::SoftLight16 => AnsiColors::BrightYellow,
            F::DarkComp => AnsiColors::BrightYellow,
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
            F::HardDark16 => write!(f, "HardDark16"),
            F::Light => write!(f, "Light"),
            F::Light16 => write!(f, "Light16"),
            F::SoftDark => write!(f, "SoftDark"),
            F::SoftDark16 => write!(f, "SoftDark16"),
            F::SoftLight => write!(f, "SoftLight"),
            F::SoftLight16 => write!(f, "SoftLight16"),
            F::DarkComp => write!(f, "SoftLight16"),
        }
    }
}
