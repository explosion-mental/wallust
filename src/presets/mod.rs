//! # Presets
//! This new feature contains presets of detemmined "schemes", which are a set of a chosen backend,
//! colorspace and palette, or even a custom one, that allows it to be consistant, in the sense
//! that it could help maintain a "style".
mod pywal;

use std::fmt;
use palette::Srgb;
use serde::Deserialize;
use crate::colors::Colors;
use crate::backends::Backend;
use anyhow::Result;
use std::path::Path;


/// Presets overwrite, ignore, any backend, colorspace or palette defined. They take preference,
/// that is why they are optional.
#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Preset {
    #[default]
    Pywal,
}

// XXX enum with Skip?

impl Preset {
    /// Whether a preset has a corresponding backend (meaning same cache file) or is actually a
    /// new/modified backend, hence, none of the Backend matches.
    // pub fn has_backend(&self) -> Option<Backend> {
    //     match self {
    //         Preset::Pywal => Some(Backend::Wal),
    //         // _ => None,
    //     }
    // }
    pub fn backend(&self, p: &Path) -> Result<Vec<u8>> {
        match self {
            Preset::Pywal => Backend::Wal.main()(p),
        }
    }

    pub fn cs(&self, rgb8s: Vec<u8>) -> Vec<Srgb> {
        match self {
            Preset::Pywal => pywal::cs(rgb8s),
        }
    }

    pub fn palette(&self, cols: Vec<Srgb>) -> Colors {
        match self {
            Preset::Pywal => pywal::palette(cols),
        }
    }
}

impl fmt::Display for Preset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Preset::Pywal => write!(f, "pywal"),
        }
    }
}
