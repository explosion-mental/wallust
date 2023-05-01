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

/// This indicates what 'parser' method to use, defined in the config file
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Backend {
    Full,
    Resized,
    Wal,
}
pub fn main(backend: &Backend) -> fn(&PathBuf) -> Result<Vec<u8>> {
    match backend {
        Backend::Full    => full::full,
        Backend::Resized => resized::resized,
        Backend::Wal     => wal::wal,
    }
}

impl Backend {
    /// This assigns a colors for a backend, used when printing
    pub fn col(&self) -> AnsiColors {
        match self {
            Self::Full => AnsiColors::Blue,
            Self::Resized => AnsiColors::Cyan,
            Self::Wal => AnsiColors::Red,
        }
    }
}

/// Add a simple `Display` for [`Backend`], used in main() to print which is in use
impl fmt::Display for Backend {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Full    => write!(f, "Full"),
            Self::Resized => write!(f, "Resized"),
            Self::Wal     => write!(f, "Wal"),
        }
    }
}
