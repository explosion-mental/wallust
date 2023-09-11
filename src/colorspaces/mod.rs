//! # Colorspaces
//! This is just an interface to get the most (16) prominent colors, from darkest to lightest, as
//! an rgb, [`Myrgb`] wrapper type, value. Different ways of collecting these can be achieve, and
//! so this deserved it's own module.
use std::fmt;
use std::rc::Rc;

use crate::colors::Myrgb;

use anyhow::Result;
use serde::{Serialize, Deserialize};
use owo_colors::AnsiColors;

/// rename [`ColorSpaces`] so it's shorter to type
use self::ColorSpaces as C;

mod lab;

const NOT_ENOUGH_COLS: &str =
"\
Not enough colors to create a scheme, even after trying to artificially generate new ones.
Try changing the threshold or the backend.
It may very well be that the image doesn't have enough colors.
Quitting...\
";

const ERR_TWO_COLS: &str = "Image should at least have two different pixel colors.";

/// Enum to indicate how to sort the colors. This can allow you to choose which colors you would
/// like to use (e.g. light scheme or dark scheme), since you got them as the first colors.
/// Using these with [`full`] or [`resize`] backends, the LightFirst will give a more pastel
/// colors. While the DarkFrist will give you more heavy ones (more hue ones)
pub enum ColorOrder {
    /// `colors[0]` will be the lightest, and `colors.last()` will be the darkest
    LightFirst,
    /// `colors[0]` will be the darkest, and `colors.last()` will be the lightest
    DarkFirst,
}

/// Corresponds to the modules inside this module and `color_space` parameter in the config file.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum ColorSpaces {
    /// Uses Cie L*a*b color space
    Lab,
    /// Same as `lab` but mixes the colors gathered
    LabMixed,
}

/// Simple Histogram
/// TODO think about a better generic way of storing (ColorSpace, count)
#[derive(Debug, Copy, Clone, PartialEq)]
struct Histo<T> {
    /// SOME colorspace color
    color: T,
    /// number of times it has appeared
    count: usize,
}

pub fn main(c: ColorSpaces, cols: &[u8], th: u8, sort_ord: ColorOrder) -> Result<(Rc<[Myrgb]>, bool)> {
    match c {
        C::Lab      => lab::lab(cols, th, false, sort_ord),
        C::LabMixed => lab::lab(cols, th, true, sort_ord),
    }
}

/// Combines some colors to generate new ones
/// Using something similar to <https://github.com/ndavd/colinterp>
/// I didn't find anything about interpolating CIE L,a*b* colors, only RGB ones, so I'm accepting
/// converting into and from just for this operation (which should not overhead the program since
/// at max is only 5 values in combination)
/// This goes like this: `lab -> rgb -> interpolation -> lab -> sort_by -> rgb`
/// `n` is the number of jumps, colors to generate (or at least to aim for that)
/// Since all of these operation are in RGB colorspace, is a tool for all.
fn interpolate(color_a: Myrgb, color_b: Myrgb, n: u8) -> Vec<Myrgb> {
    //return (endValue - startValue) * stepNumber / lastStepNumber + startValue;
    let mut palette: Vec<Myrgb> = vec![];

    // cast to i16 to not overflow u8
    let jump_r = (f32::from(color_b.0 as i16 - color_a.0 as i16)) / (f32::from(n) - 1.0);
    let jump_g = (f32::from(color_b.1 as i16 - color_a.1 as i16)) / (f32::from(n) - 1.0);
    let jump_b = (f32::from(color_b.2 as i16 - color_a.2 as i16)) / (f32::from(n) - 1.0);

    let mut curr_r = f32::from(color_a.0);
    let mut curr_g = f32::from(color_a.1);
    let mut curr_b = f32::from(color_a.2);

    for _ in 0..n {
        let r = curr_r.round() as u8;
        let g = curr_g.round() as u8;
        let b = curr_b.round() as u8;
        palette.push(Myrgb(r, g, b));
        curr_r += jump_r;
        curr_g += jump_g;
        curr_b += jump_b;
    }

    palette
}

/// Trait to encapsulate common colorspaces operations, this should be applied to `[Histo<T>]`
trait ColSpace {
    /// sort a vec of Hist
    fn sort_cols(&mut self, method: &ColorOrder);
}

impl ColorSpaces {
    /// Assign a color for the ColorSpaces
    pub fn col(&self) -> AnsiColors {
        match self {
            C::Lab => AnsiColors::Blue,
            C::LabMixed => AnsiColors::Green,
        }
    }
}

/// Display what [`ColorSpaces`] is in use. Used in cache and main.
impl fmt::Display for ColorSpaces {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            C::Lab => write!(f, "Lab"),
            C::LabMixed => write!(f, "LabMixed"),
        }
    }
}
