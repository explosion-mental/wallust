//! # Filters
//! A filter is just a way to fill the [`Colors`] struct. A method to generate a scheme that makes
//! the most prominent colors make sense as a scheme/palette. You _should_ get 16 colors returned
//! by [`ColorSpaces`], but the scenario in which an image has less than those colors is possible
//! so it is needed to handle that event, in case you need some amount of colors.
//! TODO improve API: instead of defininr `Colors` in each module on here, just adjust the values (like c.color0.darken() etc)
use std::fmt;

use owo_colors::AnsiColors;
use serde::{Serialize, Deserialize};

use crate::colors::{Colors, Myrgb};

pub mod dark;
pub mod dark16;
pub mod light;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Filters {
    Dark,
    Dark16,
    Light,
}

impl Filters {
    /// This assigns a colors for a backend, used when printing
    pub fn col(&self) -> AnsiColors {
        match self {
            Self::Dark => AnsiColors::Blue,
            Self::Dark16 => AnsiColors::Green,
            Self::Light => AnsiColors::Yellow,
        }
    }
}

/// Add a simple `Display` for [`Backend`], used in main() to print which is in use
impl fmt::Display for Filters {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Dark => write!(f, "Dark"),
            Self::Dark16 => write!(f, "Dark16"),
            Self::Light => write!(f, "Light"),
        }
    }
}
