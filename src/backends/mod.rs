//! # Backends
//! A backend is the **how** to read the image, and get rgb, as a `Vec<u8>`, from that image. This
//! is, all the colors present in the raw image file (so then it's used to find the most prominent
//! colors).
use std::path::Path;
use std::fmt;

use anyhow::Result;
use serde::{Serialize, Deserialize};
use owo_colors::AnsiColors;

/// rename [`Backend`] so it's shorter to type
use self::Backend as B;

mod full;
mod resized;
pub mod wal;
mod thumb;
mod fast_resize;
mod kmeans;

/// This indicates what 'parser' method to use, defined in the config file. Corresponds to the
/// modules inside this module
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy, Default, clap::ValueEnum)]
#[cfg_attr(feature = "doc" , derive(documented::Documented, documented::DocumentedFields))]
#[cfg_attr(feature = "iter", derive(strum::EnumIter))]
#[cfg_attr(feature = "schema" , derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum Backend {
    /// Read and return the whole image pixels (more precision, slower)
    Full,
    /// Resizes the image before parsing, mantaining it's aspect ratio
    Resized,
    /// Uses image magick `convert` to generate the colors, like pywal
    Wal,
    /// Faster algo hardcoded to 512x512 (no ratio respected)
    Thumb,
    #[clap(alias  = "fast-resize", name = "fastresize")] //claps prefers this-name
    #[serde(alias = "fast-resize")]
    /// A much faster resize algo that uses SIMD. For some reason it fails on some images where
    /// `resized` doesn't, for this reason it doesn't *replace* but rather it's a new option.
    #[default]
    FastResize,
    /// Kmeans is an algo that divides and picks pixels all around the image, giving a more
    /// diverse look.
    Kmeans,
}

impl Backend {
    /// match and return the proper backend
    pub fn main(&self) -> fn(&Path) -> Result<Vec<u8>> {
        match &self {
            B::Full    => full::full,
            B::Resized => resized::resized,
            B::Wal     => wal::wal,
            B::Thumb   => thumb::thumb,
            B::FastResize => fast_resize::fast_resize,
            B::Kmeans => kmeans::kmeans,
        }
    }
    /// This assigns a colors for a backend, used when printing
    pub fn col(&self) -> AnsiColors {
        match self {
            B::Full => AnsiColors::Blue,
            B::Resized => AnsiColors::Cyan,
            B::Wal => AnsiColors::Red,
            B::Thumb => AnsiColors::Magenta,
            B::FastResize => AnsiColors::Green,
            B::Kmeans => AnsiColors::BrightBlue,
        }
    }
}

/// Add a simple `Display` for [`Backend`], used in main() and part of the cache path.
impl fmt::Display for Backend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            B::Full    => write!(f, "Full"),
            B::Resized => write!(f, "Resized"),
            B::Wal     => write!(f, "Wal"),
            B::Thumb   => write!(f, "Thumb"),
            B::FastResize => write!(f, "FastResize"),
            B::Kmeans => write!(f, "Kmeans"),
        }
    }
}
