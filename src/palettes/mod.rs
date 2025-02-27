//! # Scheme palettes
//! A filter is just a way to fill the [`Colors`] struct. A method to generate a scheme that makes
//! the most prominent colors make sense as a scheme/palette. The vector slice will always have at
//! least 6 colors, so don't fear on using `.expect()` with this certainty and avoiding boilerplate
//! code. The scenario in which an image has less than those colors is possible and already handled in
//! the [`crate::colorspaces`] module, so don't bother with that.
//!
//! # Adding a new scheme palette
//! Have in mind these 3 rules
//!  1. The name of the palette should be as the filename and function name.
//!  2. Comments indicating the [`ColorOrder`], what does the palette do and how it does it, should
//!     be in a doc comment of the function itself.
//!  3. If it's a variation of an already existing palette, it should be indicated as a comment.
//!
//! * XXX would other palettes need more than 6 (or even 8) colors? if so, change the return type to
//!   `Result<Colors>` or just fallback to a scheme
use std::fmt;

use owo_colors::AnsiColors;
use serde::{Serialize, Deserialize};
use palette::Srgb;
use palette::{Darken, Lighten};

use crate::{
    colors::{
        Colors, Myrgb,
    },
    colorspaces::ColorOrder,
    //colorspaces::{BuildColors, ColorOrder},
};

/// rename [`Palette`] so it's shorter to type
use self::Palette as F;

// include!("dark.rs");
// include!("harddark.rs");
// include!("light.rs");
// include!("softdark.rs");
// include!("softlight.rs");
mod ansidark;
mod dark;
mod harddark;
mod light;
mod softdark;
mod softlight;

use ansidark::ansidark;
use dark::dark;
use harddark::harddark;
use light::light;
use softdark::softdark;
use softlight::softlight;

/// Corresponds to the modules inside this module and `palette` parameter in the config file.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy, Default, clap::ValueEnum)]
#[cfg_attr(feature = "doc" , derive(documented::Documented, documented::DocumentedFields))]
#[cfg_attr(feature = "iter", derive(strum::EnumIter))]
#[cfg_attr(feature = "schema" , derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum Palette {
    /// 8 dark colors, dark background and light contrast
    #[default]
    Dark,
    /// Same as `dark` but uses the 16 colors trick
    Dark16,
    /// This is a `dark` variant that changes all colors to it's complementary counterpart, giving
    /// the feeling of a 'new palette' but that still makes sense with the image provided.
    #[clap(alias  = "dark-comp", name = "darkcomp")]
    #[serde(alias = "dark-comp")]
    DarkComp,
    /// 16 variation of the dark complementary variant
    #[clap(alias  = "dark-comp16", name = "darkcomp16")]
    #[serde(alias = "dark-comp16")]
    DarkComp16,

    /// This is not a 'dark' variant, is a new palette that is meant to work with `lchansi`
    /// colorspace, which will maintain 'tty' like color order and only adjusting the colors
    /// acording to the theme. A possible solution for LS_COLORS and the like. Should workout with
    /// other colorspace, but the result may not be optimal.
    #[clap(alias  = "ansi-dark", name = "ansidark")]
    #[serde(alias = "ansi-dark")]
    AnsiDark,
    /// The ansidark palette with 16 color variation.
    #[clap(alias  = "ansi-dark16", name = "ansidark16")]
    #[serde(alias = "ansi-dark16")]
    AnsiDark16,

    /// Same as `dark` with hard hue colors
    #[clap(alias  = "hard-dark", name = "harddark")] //clap prefers this-name
    #[serde(alias = "hard-dark")]
    HardDark,
    /// Harddark with 16 color variation
    #[clap(alias  = "hard-dark16", name = "harddark16")]
    #[serde(alias = "hard-dark16")]
    HardDark16,
    /// complementary colors variation of harddark scheme
    #[clap(alias  = "hard-dark-comp", name = "harddarkcomp")]
    #[serde(alias = "hard-dark-comp")]
    HardDarkComp,
    /// complementary colors variation of harddark scheme
    #[clap(alias  = "hard-dark-comp16", name = "harddarkcomp16")]
    #[serde(alias = "hard-dark-comp16")]
    HardDarkComp16,

    /// Light bg, dark fg
    Light,
    /// Same as `light` but uses the 16 color trick
    Light16,
    /// complementary colors variation of light
    #[clap(alias  = "light-comp", name = "lightcomp")]
    #[serde(alias = "light-comp")]
    LightComp,
    /// complementary colors variation of light with the 16 color variation
    #[clap(alias  = "light-comp16", name = "lightcomp16")]
    #[serde(alias = "light-comp16")]
    LightComp16,

    /// Variant of softlight, uses the lightest colors and a dark background (could be
    /// interpreted as `dark` inversed)
    #[clap(alias  = "soft-dark", name = "softdark")]
    #[serde(alias = "soft-dark")]
    SoftDark,
    /// softdark with 16 color variation
    #[clap(alias  = "soft-dark16", name = "softdark16")]
    #[serde(alias = "soft-dark16")]
    SoftDark16,
    /// complementary variation for softdark
    #[clap(alias  = "soft-dark-comp", name = "softdarkcomp")]
    #[serde(alias = "soft-dark-comp")]
    SoftDarkComp,
    /// complementary variation for softdark with the 16 color variation
    #[clap(alias  = "soft-dark-comp16", name = "softdarkcomp16")]
    #[serde(alias = "soft-dark-comp16")]
    SoftDarkComp16,

    /// Light with soft pastel colors, counterpart of `harddark`
    #[clap(alias  = "soft-light", name = "softlight")]
    #[serde(alias = "soft-light")]
    SoftLight,
    /// softlight with 16 color variation
    #[clap(alias  = "soft-light16", name = "softlight16")]
    #[serde(alias = "soft-light16")]
    SoftLight16,
    /// softlight with complementary colors
    #[clap(alias  = "soft-light-comp", name = "softlightcomp")]
    #[serde(alias = "soft-light-comp")]
    SoftLightComp,
    /// softlight with complementary colors with 16 colors
    #[clap(alias  = "soft-light-comp16", name = "softlightcomp16")]
    #[serde(alias = "soft-light-comp16")]
    SoftLightComp16,
}

impl F {
    pub fn run(&self, c: Vec<Srgb>, orig: Vec<Srgb>) -> Colors {
        match self {
            F::Dark => dark(c, orig),
            F::Dark16 => dark(c, orig).to_16col(),
            F::DarkComp => dark(c, orig).to_comp(),
            F::DarkComp16 => dark(c, orig).to_comp().to_16col(),

            F::AnsiDark => ansidark(c, orig),
            F::AnsiDark16 => ansidark(c, orig).to_16col(),

            F::Light => light(c, orig),
            F::Light16 => light(c, orig).to_16col(),
            F::LightComp => light(c, orig).to_comp(),
            F::LightComp16 => light(c, orig).to_comp().to_16col(),

            F::HardDark => harddark(c, orig),
            F::HardDark16 => harddark(c, orig).to_16col(),
            F::HardDarkComp => harddark(c, orig).to_comp(),
            F::HardDarkComp16 => harddark(c, orig).to_comp().to_16col(),

            F::SoftDark => softdark(c, orig),
            F::SoftDark16 => softdark(c, orig).to_16col(),
            F::SoftDarkComp => softdark(c, orig).to_comp(),
            F::SoftDarkComp16 => softdark(c, orig).to_comp().to_16col(),

            F::SoftLight => softlight(c, orig),
            F::SoftLight16 => softlight(c, orig).to_16col(),
            F::SoftLightComp => softlight(c, orig).to_comp(),
            F::SoftLightComp16 => softlight(c, orig).to_comp().to_16col(),
        }
    }
    /// Use different sorting `sort_by` on different schemes palette, which creates even more schemes.
    pub fn sort_ord(&self) -> ColorOrder {
        match self {
              F::Dark  | F::Dark16 | F::DarkComp | F::DarkComp16
            | F::SoftDark | F::SoftDark16 | F::SoftDarkComp | F::SoftDarkComp16
            | F::SoftLight | F::SoftLight16 | F::SoftLightComp | F::SoftLightComp16
                => ColorOrder::LightFirst,

              F::Light | F::Light16 | F::LightComp | F::LightComp16
            | F::HardDark | F::HardDark16 | F::HardDarkComp | F::HardDarkComp16
            | F::AnsiDark | F::AnsiDark16
                => ColorOrder::DarkFirst,
        }
    }
    /// Assign a color when printing in `main()`
    pub fn col(&self) -> AnsiColors {
        match self {
            F::Dark => AnsiColors::Blue,
            F::Dark16 => AnsiColors::BrightBlue,
            F::DarkComp => AnsiColors::BrightBlue,
            F::DarkComp16 => AnsiColors::BrightBlue,

            F::AnsiDark => AnsiColors::Red,
            F::AnsiDark16 => AnsiColors::Red,

            F::HardDark => AnsiColors::Green,
            F::HardDark16 => AnsiColors::BrightGreen,
            F::HardDarkComp => AnsiColors::BrightGreen,
            F::HardDarkComp16 => AnsiColors::BrightGreen,

            F::Light => AnsiColors::Cyan,
            F::Light16 => AnsiColors::BrightCyan,
            F::LightComp => AnsiColors::BrightCyan,
            F::LightComp16 => AnsiColors::BrightCyan,

            F::SoftDark => AnsiColors::Magenta,
            F::SoftDark16 => AnsiColors::BrightMagenta,
            F::SoftDarkComp => AnsiColors::BrightMagenta,
            F::SoftDarkComp16 => AnsiColors::BrightMagenta,

            F::SoftLight => AnsiColors::Yellow,
            F::SoftLight16 => AnsiColors::BrightYellow,
            F::SoftLightComp => AnsiColors::BrightYellow,
            F::SoftLightComp16 => AnsiColors::BrightYellow,
        }
    }
}

/// Display what [`Palette`] is in use. Used in cache and main.
impl fmt::Display for F {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            F::Dark       => write!(f, "Dark"),
            F::Dark16     => write!(f, "Dark16"),
            F::DarkComp   => write!(f, "DarkComp"),
            F::DarkComp16 => write!(f, "DarkComp16"),

            F::AnsiDark => write!(f, "AnsiDark"),
            F::AnsiDark16 => write!(f, "AnsiDark16"),

            F::HardDark       => write!(f, "HardDark"),
            F::HardDark16     => write!(f, "HardDark16"),
            F::HardDarkComp   => write!(f, "HardDarkComp"),
            F::HardDarkComp16 => write!(f, "HardDarkComp16"),

            F::Light       => write!(f, "Light"),
            F::Light16     => write!(f, "Light16"),
            F::LightComp   => write!(f, "LightComp"),
            F::LightComp16 => write!(f, "LightComp16"),

            F::SoftDark       => write!(f, "SoftDark"),
            F::SoftDark16     => write!(f, "SoftDark16"),
            F::SoftDarkComp   => write!(f, "SoftDarkComp"),
            F::SoftDarkComp16 => write!(f, "SoftDarkComp16"),

            F::SoftLight       => write!(f, "SoftLight"),
            F::SoftLight16     => write!(f, "SoftLight16"),
            F::SoftLightComp   => write!(f, "SoftLightComp"),
            F::SoftLightComp16 => write!(f, "SoftLightComp16"),
        }
    }
}
