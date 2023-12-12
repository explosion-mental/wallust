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
use itertools::Itertools;

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

/// Currently this works in function with the filters methods, which currently only needs 6 colors.
/// Let's make sure the colorspace backend send at least these number of colors.
const MIN_COLS: u8 = 6;

/// The [`Colors`] struct only has capacity for 16 colors 0..=15. const is used in order to take
/// the top MAX_COLS lab colors.
const MAX_COLS: u8 = 16;

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
    #[clap(alias = "lab-mixed", name = "labmixed")] //claps prefers this-name
    #[serde(alias = "lab-mixed")]
    /// Same as `lab` but mixes the colors gathered
    LabMixed,
    #[clap(alias = "lab-fast", name = "labfast")]
    #[serde(alias = "lab-fast")]
    /// Avoids floating arithmetic, thus, faster operations
    LabFast,
}

/// Simple Histogram
#[derive(Debug, Copy, Clone, PartialEq)]
struct Histo<T> {
    /// SOME colorspace color
    color: T,
    /// number of times it has appeared
    count: usize,
}

#[allow(dead_code)]
/// Type that stores and abstracts away the colorspace
/// 1. Get configs (threshold and mix)
/// 2. Get labs (from u8 -> Lab)
/// 3. return
pub struct Cols {
    /// The histogram
    histo: Vec<Histo<::lab::Lab>>,
    /// explained in config.rs
    threshold: u8,
    /// whether to mix colors or not
    mix: bool,
    /// what are we using?
    c: ColorSpaces,
}


#[allow(dead_code)]
fn test() {
}

impl Cols {
    /// Creates a new [`Cols`]
    pub fn new(cols: &[u8], threshold: u8, c: &ColorSpaces) -> Self {
        let mix = match c {
            ColorSpaces::LabMixed => true,
            _ => false,
        };

        let histo = match c {
            ColorSpaces::Lab | ColorSpaces::LabMixed => lab::histo(cols, threshold, mix),
            ColorSpaces::LabFast => lab::histo_lazy(cols, threshold, mix),
        };

        Self {
            c: *c,
            histo,
            threshold,
            mix,
        }
    }

    /// Sort the colors, this depends on the colorspace being used
    pub fn sort_colors(&mut self, method: &ColorOrder) {
        match self.c {
            ColorSpaces::Lab | ColorSpaces::LabMixed | ColorSpaces::LabFast => lab::sort_colors(&mut self.histo, method),
        }
    }

    /// This function is called when the colors gathered are not enough. Usually implies calling
    /// [`interpolate`] function, however there could be other ways or simply do nothing (this will
    /// imply to quit the program, since later on the .len() it's evaluated and needs to be higher
    /// than [`MAX_COLS`])
    pub fn new_cols(&mut self) {
        match self.c {
            ColorSpaces::Lab | ColorSpaces::LabMixed => lab::new_cols(&mut self.histo, self.threshold),
            ColorSpaces::LabFast => lab::new_cols_lazy(&mut self.histo, self.threshold),
        }
    }

}

impl ColorSpaces {
    /// Assign a color for the ColorSpaces
    pub fn col(&self) -> AnsiColors {
        match self {
            C::Lab => AnsiColors::Blue,
            C::LabMixed => AnsiColors::Green,
            C::LabFast => AnsiColors::Yellow,
        }
    }
}

/// Display what [`ColorSpaces`] is in use. Used in cache and main.
impl fmt::Display for ColorSpaces {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            C::Lab => write!(f, "Lab"),
            C::LabMixed => write!(f, "LabMixed"),
            C::LabFast => write!(f, "LabFast"),
        }
    }
}

pub fn main(c: ColorSpaces, cols: &[u8], threshold: u8, sort_ord: ColorOrder) -> Result<(Rc<[Myrgb]>, bool)> {
    // This is to indicate if there were any warnings, since we can't print them directly
    let warn;

    let mut cols = Cols::new(cols, threshold, &c);

    if cols.histo.len() < 2 {
        anyhow::bail!(ERR_TWO_COLS);
    } else {
        // sort vec by count, most used colors first (if they are more than the MAX)
        if cols.histo.len() > MAX_COLS.into() {
            cols.histo.sort_by(|a, b| b.count.cmp(&a.count));
        }
        // take the *necessary* most used colors
        cols.histo.truncate(MAX_COLS.into());
    }

    // Artificially generate colors with linear interpolation in between the colors that we already
    // have. However even this can even fail and not generate enough different colors, so there is
    // another check below
    if cols.histo.len() < MIN_COLS.into() {
        warn = true;

        //new vector with new colors, later to be `.append()`ed
        cols.new_cols();
        cols.histo.truncate(MAX_COLS.into());
    } else {
        warn = false;
    }

    // not enough colors, even after making new colors (if any)
    if cols.histo.len() < MIN_COLS.into() {
        anyhow::bail!(NOT_ENOUGH_COLS);
    }

    // custom sorting, checkout [`ColorOrder`] and [`sort_ord`]
    cols.sort_colors(&sort_ord);

    let histo = cols.histo.iter().map(|x| x.color.into()).collect::<Rc<_>>();

    Ok((histo, warn))
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
