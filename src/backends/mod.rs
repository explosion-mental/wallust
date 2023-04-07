//! Backends
//! A backend is like a filter
//! * There are multiple methods in which you can get the most relevant colors from an image; rather
//!   than hardcoding, give options
//! * TODO add Oklab method
//! On this file are usual helper functions
use std::path::PathBuf;
use std::fmt;

use crate::{Colors, Myrgb};

use image::io::Reader as ImageReader;
use anyhow::Result;
use serde::*;
use owo_colors::AnsiColors;

mod full;
mod resized;
mod wal;
use full::*;
use resized::*;
use wal::*;

/// This indicates what 'parser' method to use, defined in the config file
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Backend {
    Full,
    Resized,
    Wal,
}

pub fn main(f: &PathBuf, backend: &Backend) -> Result<Vec<u8>> {
    let method_to_use = match backend {
        Backend::Full => full,
        Backend::Resized => resized,
        Backend::Wal => wal,
    };
    method_to_use(f)
}

impl Backend {
    /// This assigns a colors for a backend, used when printing
    pub fn col(&self) -> AnsiColors {
        match self {
            Backend::Full => AnsiColors::Blue,
            Backend::Resized => AnsiColors::Cyan,
            Backend::Wal => AnsiColors::Red,
        }
    }
}

/// Add a simple `Display` for [`Backend`], used in main() to print which is in use
impl fmt::Display for Backend {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Full    => write!(f, "full"),
            Self::Resized => write!(f, "resized"),
            Self::Wal     => write!(f, "wal"),
        }
    }
}
