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
mod dark;
mod lab;
use full::*;
use resized::*;
use wal::*;
use dark::*;
use self::lab::*;

/// This indicates what 'parser' method to use, defined in the config file
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Backend {
    Full,
    Resized,
    Wal,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Filter {
    Dark,
}

/// main fn that calls other methods, used in main.rs
pub fn gen_colors(file: &PathBuf, backend: &Backend, threshold: u32) -> Result<Colors<Myrgb>> {
    // read image
    let method_to_use = match backend {
        Backend::Full => full,
        Backend::Resized => resized,
        Backend::Wal => wal,
    };
    let rgbas = method_to_use(file)?;

    // get the top 8 most used colors, ordered from the lightess to the darkess. Different color
    // spaces could be used here.
    let histo = lab(&rgbas, threshold);

    // Apply a [`Filter`] that returns the [`Colors`] struct
    let colors = dark(histo);

    Ok(colors)
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

/// Simple Histogram
/// TODO think about a better generic way of storing (ColorSpace, count)
pub struct Histo {
    /// LAB colors - TODO allow other colorspaces
    pub color: ::lab::Lab,
    /// number of times it has appeared
    pub count: usize,
}
