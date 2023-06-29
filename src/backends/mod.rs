//! # Backends
//! A backend is the **how** to read the image, and get rgb, as a `Vec<u8>`, from that image. This
//! is, all the colors present in the raw image file (so then it's used to find the most prominent
//! colors).
use std::path::PathBuf;
use std::fmt;

use image::io::Reader as ImageReader;
use anyhow::Result;
use serde::{Serialize, Deserialize};
use owo_colors::AnsiColors;

mod full;
mod resized;
mod wal;
mod thumb;

/// This indicates what 'parser' method to use, defined in the config file.
/// Corresponds to the modules inside this module
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum Backend {
    /// Read and return the whole image pixels
    Full,
    /// Resize it, then get read the image
    Resized,
    /// Uses image magick to generate the colors, like pywal
    Wal,
    /// Faster algo than the `resized` module, hardcoded to 512x512
    Thumb,
}

/// re export to shorten from `Backend::Full` to `Full`
use self::Backend::*;

pub fn main(backend: &Backend) -> fn(&PathBuf) -> Result<Vec<u8>> {
    match backend {
        Full    => full::full,
        Resized => resized::resized,
        Wal     => wal::wal,
        Thumb   => thumb::thumb,
    }
}

impl Backend {
    /// This assigns a colors for a backend, used when printing
    pub fn col(&self) -> AnsiColors {
        match self {
            Full => AnsiColors::Blue,
            Resized => AnsiColors::Cyan,
            Wal => AnsiColors::Red,
            Thumb => AnsiColors::Magenta,
        }
    }
}

/// Add a simple `Display` for [`Backend`], used in main() and part of the cache path.
impl fmt::Display for Backend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Full    => write!(f, "Full"),
            Resized => write!(f, "Resized"),
            Wal     => write!(f, "Wal"),
            Thumb   => write!(f, "Thumb"),
        }
    }
}
