
use std::fmt;

use crate::colors::Compl;

use serde::{Serialize, Deserialize};
use palette::Mix;
use palette::Srgb;
use owo_colors::AnsiColors;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy, Default, clap::ValueEnum)]
#[cfg_attr(feature = "schema" , derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase")]
/// The FallbackGenerator are methods only activated when the colors aren't sufficient for creating
/// a scheme palette, so these methods make variants of the already gathered colros to create a
/// scheme.
pub enum FallbackGenerator {
    /// uses [`interpolate`]
    #[default]
    Interpolate,
    /// uses [`complementary`]
    Complementary,
}

use self::FallbackGenerator as G;

impl FallbackGenerator {
    pub fn gen(&self) -> impl Fn(Srgb, Srgb, u8) -> Vec<Srgb> {
        match self {
            G::Interpolate => interpolate,
            G::Complementary => complementary,
        }
    }

    pub fn col(&self) -> AnsiColors {
        match self {
            G::Interpolate => AnsiColors::Blue,
            G::Complementary => AnsiColors::Green,
        }
    }
}

/// Combines some colors to generate new ones
/// ref: <https://docs.rs/palette/latest/palette/trait.Mix.html>
/// This seems to be implemented in the palette crate for all colorspaces,
/// In that case, `complementary()` would be a generator that will need convertion.
fn interpolate(color_a: Srgb, color_b: Srgb, n: u8) -> Vec<Srgb> {
    let steps = 1.0 / f32::from(n);

    let mut v = vec![];
    let a = color_a.into_format();
    let b = color_b.into_format();

    for i in 1..=n {
        v.push(a.mix(b, steps * f32::from(i)))
    }
    v
}

//TODO implement triards, cuartets, quints
pub fn complementary(color_a: Srgb, color_b: Srgb, _: u8) -> Vec<Srgb> {
    vec![
        color_a.complementary(),
        color_b.complementary(),
    ]
}

impl fmt::Display for G {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            G::Interpolate => write!(f, "Interpolate"),
            G::Complementary => write!(f, "Complementary"),
        }
    }
}
