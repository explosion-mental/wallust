//! # Colorspaces
//! This is just an interface to get the most (16) prominent colors, from darkest to lightest, as
//! an rgb, [`Myrgb`] wrapper type, value. Different ways of collecting these can be achieve, and
//! so this deserved it's own module.

//TODO finally understood how pywal does it. To get a good "uniform" palette, instead of sorting
//     with lightest or the top colors and the like, it should take into consideration the hue. If
//     you understand this module (me) then extrapolating to words from here would be something
//     like: 'making it the most hued colors acting as the `darkest`', so `DarkFirst` would get the
//     most hued colors first, or the 'hard' ones (uwu)

use std::cmp::Ordering;
use std::fmt;

// use crate::colors::Colors;
use crate::colors::Myrgb;

use anyhow::Result;
use serde::{Serialize, Deserialize};
use owo_colors::AnsiColors;
use itertools::Itertools;

mod lab;

const NOT_ENOUGH_COLS: &str =
"\
Not enough colors to create a scheme, even after trying to artificially generate new ones.
Try changing the threshold or the backend.
It may very well be that the image doesn't have enough colors.
Quitting...\
";

const ERR_TWO_COLS: &str = "Image should at least have two different pixel colors.";

/// Currently this works in function with the palettes methods, which currently only needs 6 colors.
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

/// rename [`ColorSpace`] so it's shorter to type
use self::ColorSpace as Cs;

/// Corresponds to the modules inside this module and `color_space` parameter in the config file.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy, Default, clap::ValueEnum)]
#[cfg_attr(feature = "doc" , derive(documented::Documented, documented::DocumentedFields))]
#[cfg_attr(feature = "iter", derive(strum::EnumIter))]
#[serde(rename_all = "lowercase")]
pub enum ColorSpace {
    /// Uses Cie L*a*b color space
    #[default]
    Lab,
    #[clap(alias = "lab-mixed", name = "labmixed")] //claps prefers this-name
    #[serde(alias = "lab-mixed")]
    /// Variant of `lab` that mixes the colors gathered, if not enough colors it fallbacks to usual
    /// lab (not recommended in small images)
    LabMixed,
    #[clap(alias = "lab-fast", name = "labfast")]
    #[serde(alias = "lab-fast")]
    /// Variant of `lab` that avoids floating arithmetic, thus, faster operations but not that much
    /// precise result. Images that work on lab/labmixed could not have enough colors for labfast.
    LabFast,
}

/// rename [`GenerateFallback`] so it's shorter to type
use self::FallbackGenerator as G;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy, Default, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum FallbackGenerator {
    /// uses [`interpolate`]
    #[default]
    Interpolate,
    /// uses [`complementary`]
    Complementary,
}

/// Simple Histogram
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Histo<T: ColorTrait> {
    /// SOME colorspace color
    color: T,
    /// number of times it has appeared
    count: usize,
}

pub struct Histograms<> {
}

pub trait ColorTrait {}
impl ColorTrait for ::lab::Lab {}

/// Type that stores and abstracts away the colorspace
/// 1. Get configs (threshold and mix)
/// 2. Get labs (from u8 -> Lab)
/// 3. return
/// TODO Find a way to make this `colorspace` agnostic, in the sense that it should not require to
///      store the `histo` per se, as this requires the [`Cols`] type to add a generic argument,
///      threshold and colorspace doesn't seem to be useful in the `palettes` stage.
#[derive(Debug, Clone, PartialEq)]
pub struct MyCol<T: ColorTrait> {
    /// The histogram
    pub histo: Vec<Histo<T>>,
    /// Histogram without any custom sorting, so it stays from most prominent color `[0]` to the least one `.last()`
    pub orig_histo: Vec<Histo<T>>,
}

impl<T: ColorTrait> From<Histo<T>> for Myrgb
    where Myrgb: From<T>
{
    fn from(h: Histo<T>) -> Self {
        h.color.into()
    }
}

impl FallbackGenerator {
    pub fn gen(&self) -> impl Fn(Myrgb, Myrgb, u8) -> Vec<Myrgb> {
        match self {
            G::Interpolate => interpolate,
            G::Complementary => complementary,
        }
    }
}

impl ColorSpace {
    pub fn color_constraint(&self) -> impl Fn(f32) -> bool {
        match self {
            Cs::Lab | Cs::LabMixed => |l| l >= lab::DARKEST || l <= lab::LIGHTEST,
            Cs::LabFast => |l| (l as u32) >= (lab::DARKEST as u32) || (l as u32) <= (lab::LIGHTEST as u32),
        }
    }
}

pub type Cols<T> = Vec<Histo<T>>;
#[derive(Debug, Clone, PartialEq)]
pub struct ColorHisto<T: ColorTrait>(Vec<Histo<T>>);

/// This trait is for creating a new `Col` type.
/// The Self parameter should always be an iteratable type
pub trait BuildColors: Sized {
    type Color: ColorTrait;
    type Histogram: Sized;
    fn read(bytes: &[u8]) -> Vec<Self::Color>;
    fn new_colors(bytes: &[u8], threshold: u8, c: &Cs) -> Self;
    fn color_generator(&self, threshold: u8, c: &Cs, gen: &G) -> Self;
    /// Simple Sort algo that determines how to order colors
    /// usecase: `histo.sort_by(|a, b| color_ord.sort_algo(a, b))`
    fn sort_algo(cs: &ColorOrder, a: &Self::Histogram, b: &Self::Histogram) -> Ordering;
    // fn to_colors(&self) -> Colors;
    fn to_myrgb(&self) -> Vec<Myrgb>;
    fn dedup_by_fn(a: &Self::Histogram, b: &Self::Histogram, threshold: u8) -> bool;
    fn sort_by_key_fn(a: Self::Histogram) -> impl Ord;
}

impl Cs {
    /// Assign a color for the ColorSpace
    pub fn col(&self) -> AnsiColors {
        match self {
            Cs::Lab => AnsiColors::Blue,
            Cs::LabMixed => AnsiColors::Green,
            Cs::LabFast => AnsiColors::Yellow,
        }
    }
}

impl G {
    pub fn col(&self) -> AnsiColors {
        match self {
            G::Interpolate => AnsiColors::Blue,
            G::Complementary => AnsiColors::Green,
        }
    }
}

impl fmt::Display for G {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            G::Interpolate => write!(f, "Interpolate"),
            G::Complementary => write!(f, "Complementary"),
        }
    }
}

/// Display what [`Cs`] is in use. Used in cache and main.
impl fmt::Display for Cs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cs::Lab => write!(f, "Lab"),
            Cs::LabMixed => write!(f, "LabMixed"),
            Cs::LabFast => write!(f, "LabFast"),
        }
    }
}
// pub fn main(c: Cs, cols: &[u8], threshold: u8, gen: &G, ord: &ColorOrder) -> Result<((Vec<Myrgb>, Vec<Myrgb>), bool)> {
//     match c {
//         Cs::Lab => histo::<::lab::Lab>(c, cols, threshold, gen, ord),
//         _ => todo!()
//     }
// }

pub fn main(c: Cs, cols: &[u8], threshold: u8, gen: &G, ord: &ColorOrder) -> Result<((Vec<Myrgb>, Vec<Myrgb>), bool)>
    // where T: BuildColors<Color = T, Histogram = Histo<T>> + Clone + ColorTrait,
{
    // This is to indicate if there were any warnings, since we can't print them directly
    let mut warn = false;

    let mut histo = ColorHisto::new_colors(cols, threshold, &c);

    // `interpolate()` requires two colors, else we can't attempt to generate colors at our own
    if histo.len() < 2 { anyhow::bail!(ERR_TWO_COLS); }

    // FORGET: testing this as much as I can, and `.dedup()`ing doesn't seem to remove "similar" colors.
    // dedup colors by
    // ---> this is wrong lmao, delta_e is da wae//cols.histo.dedup_by(|a, b| a.color == b.color);

    // The above is wrong, I've tested a lot and:
    // 1. using `dedup_by` without `sort_by_key` seems to not get much colors.
    // 2. obviously sorting without `dedup`ing won't do much.
    // 3. to get more colors `.truncate()` should accept `MAX_COLS`, however this used to get many
    //    similar colors, not resulting in an stable palette. By using these two methods below, we
    //    'asure' (lazyly) to have no duplicates, and thus, the benefit of 'more colors' won't
    //    imply 'bad scheme'.
    // histo.sort_by_key(|e| (e.color.l as u32, e.color.a as i32, e.color.b as i32));
    // histo.dedup_by(|a, b| lab::delta_e(a.color, b.color) <= threshold.into());
    // labs.sort_by_key(|e| (e.l.trunc() as u32, e.a.trunc() as i32, e.b.trunc() as i32));
    // labs.dedup_by(|a, b| lab::delta_e(*a, *b) <= threshold.into());
    // labs.dedup();
    histo.sort_by_key(|&a| ColorHisto::sort_by_key_fn(a));
    histo.dedup_by(|a, b| ColorHisto::dedup_by_fn(a, b, threshold));

    // sort vec by count, most used colors first
    histo.sort_by(|a, b| b.count.cmp(&a.count));

    // remove excess elements
    histo.truncate(MAX_COLS.into());

    // Artificially generate colors with linear interpolation in between the colors that we already
    // have. However even this can even fail and not generate enough different colors, so there is
    // another check below
    if histo.len() < MIN_COLS.into() {
        warn = true; // "artificially generation colors.."

        // `interpolate()`ion and `.append()` new colors to `
        //TODO give options on **how** to complete the colors:
        // - `interpolate()`ion, what's currently being used
        // - `.complementary()`, fill colors with it's complementary ones #13
        let mut new = ColorHisto::color_generator(&histo, threshold, &c, gen);

        histo.append(&mut new);

        // sort vec by count, most used colors first (if they are more than the MAX)
        histo.sort_by(|a, b| b.count.cmp(&a.count));

        // TODO Do another check on histo itself to update `.count` variables
        //       test if colors repetead again (on interpolation)

        // take the *necessary* most used colors
        histo.truncate(MAX_COLS.into());
    }

    // not enough colors, even after making new colors (if any)
    if histo.len() < MIN_COLS.into() { anyhow::bail!(NOT_ENOUGH_COLS); }

    // orig_histo will not be changed by `sort_colors`,
    // thus keeping the `top used colors` order in place
    let orig_histo = histo.clone();

    // custom sorting, checkout [`ColorOrder`] and [`sort_ord`]
    //histo = T::sort_algo(&Cs);
    histo.sort_by(|a, b| ColorHisto::sort_algo(&ord, &a, &b));

    Ok(
        ((histo.to_myrgb(), orig_histo.to_myrgb()), warn)
        )
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

fn complementary(color_a: Myrgb, color_b: Myrgb, _: u8) -> Vec<Myrgb> {
    vec![
        color_a.complementary(),
        color_b.complementary()
    ]
}
