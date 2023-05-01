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
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Backend {
    Full,
    Resized,
    Wal,
    Thumb,
}

pub fn main(backend: &Backend) -> fn(&PathBuf) -> Result<Vec<u8>> {
    match backend {
        Backend::Full    => full::full,
        Backend::Resized => resized::resized,
        Backend::Wal     => wal::wal,
        Backend::Thumb => thumb::thumb,
    }
}

impl Backend {
    /// This assigns a colors for a backend, used when printing
    pub fn col(&self) -> AnsiColors {
        match self {
            Self::Full => AnsiColors::Blue,
            Self::Resized => AnsiColors::Cyan,
            Self::Wal => AnsiColors::Red,
            Self::Thumb => AnsiColors::Magenta,
        }
    }
}

/// Add a simple `Display` for [`Backend`], used in main() and part of the cache path.
impl fmt::Display for Backend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Full    => write!(f, "Full"),
            Self::Resized => write!(f, "Resized"),
            Self::Wal     => write!(f, "Wal"),
            Self::Thumb   => write!(f, "Thumb"),
        }
    }
}
