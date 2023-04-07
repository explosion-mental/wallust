//! Filters
use std::fmt;

use owo_colors::AnsiColors;
use serde::*;

use crate::{Colors, Myrgb};


mod dark;
use dark::*;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Filters {
    Dark,
}

pub fn main(histo: Vec<Myrgb>, filter: &Filters) -> Colors {
    let method_to_use = match filter {
        Filters::Dark => dark,
    };
    method_to_use(histo)
}

impl Filters {
    /// This assigns a colors for a backend, used when printing
    pub fn col(&self) -> AnsiColors {
        match self {
            Self::Dark => AnsiColors::Blue,
        }
    }
}

/// Add a simple `Display` for [`Backend`], used in main() to print which is in use
impl fmt::Display for Filters {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Dark => write!(f, "dark"),
        }
    }
}
