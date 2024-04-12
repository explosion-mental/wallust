//! # Colorspaces
//! This modules has the job of reducing all the bytes given from the `backend` to two (2) vectors:
//! 1. The first one is a sorted, see [`ColorOrder`], array.
//! 2. The second one is about preserving the most dominant color order in the vector, ensuring the
//!    first entry is the dominant (most repeated one).

use std::cmp::Ordering;
use std::fmt;
use std::ops::Deref;
use std::ops::DerefMut;

use crate::colors::Myrgb;
use crate::colors::Compl;

use palette::convert::FromColorUnclamped;
use palette::cast::ComponentsAs;
use palette::rgb::Rgb;
use palette::IntoColor;
use palette::Clamp;
use palette::Srgb;
use palette::Mix;
use serde::{Serialize, Deserialize};
use owo_colors::AnsiColors;
use itertools::Itertools;
use thiserror::Error;

mod lab;
mod lch;

/// Currently this works in function with the palettes methods, which currently only needs 6 colors.
/// Let's make sure the colorspace backend send at least these number of colors.
const MIN_COLS: u8 = 6;

/// The [`Colors`] struct only has capacity for 16 colors 0..=15. const is used in order to take
/// the top MAX_COLS lab colors.
const MAX_COLS: u8 = 16;

#[derive(Error, Debug)]
pub enum ColorSpaceError {
    #[error("\
Not enough colors to create a scheme, even after trying to artificially generate new ones.
Try changing the threshold or the backend.
It may very well be that the image doesn't have enough colors.
Quitting...\
    ")]
    NotEnough,
    #[error("Image should at least have two different pixel colors.")]
    TwoColors,
}




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
    /// CIE Lch, you can understand this color space like LAB but with chrome and hue added.
    /// Could help when sorting.
    Lch,
    /// CIE Lch, you can understand this color space like LAB but with chrome and hue added.
    /// Could help when sorting.
    #[clap(alias = "lch-mixed", name = "lchmixed")] //claps prefers this-name
    #[serde(alias = "lch-mixed")]
    LchMixed,
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

impl<T: ColorTrait> From<Histo<T>> for Myrgb
    where Myrgb: From<T>
{
    fn from(h: Histo<T>) -> Self {
        h.color.into()
    }
}

impl From<Srgb<u8>> for Myrgb {
    fn from(c: Srgb<u8>) -> Self {
        Self(c.into_format())
    }
}

impl From<Myrgb> for Srgb<u8> {
    fn from(c: Myrgb) -> Self {
        c.0.into_format()
    }
}

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

impl ColorSpace {
    /// Main function that matches agains the respective colorspace builder with BuildColors trait
    pub fn main(&self, bytes_rgb8: &[u8], threshold: u8, gen: &G, ord: &ColorOrder)
        -> Result<((Vec<Srgb>, Vec<Srgb>), bool), ColorSpaceError>
    {
        match self {
            Cs::Lab => main::<palette::Lab>(bytes_rgb8, threshold, gen, false, ord),
            Cs::LabMixed => main::<palette::Lab>(bytes_rgb8, threshold, gen, true, ord),

            Cs::Lch => main::<palette::Lch>(bytes_rgb8, threshold, gen, false, ord),
            Cs::LchMixed => main::<palette::Lch>(bytes_rgb8, threshold, gen, true, ord),
        }
    }
    /// Assign a color for the ColorSpace
    pub fn col(&self) -> AnsiColors {
        match self {
            Cs::Lab => AnsiColors::Blue,
            Cs::LabMixed => AnsiColors::Green,
            Cs::Lch => AnsiColors::Magenta,
            Cs::LchMixed => AnsiColors::Magenta,
        }
    }
    /// automatic threshold
    /// TODO needs more testing
    pub fn def_threshold(&self) -> u8 {
        match self {
            Cs::Lab | Cs::LabMixed => 17,
            Cs::Lch | Cs::LchMixed => 20,
        }
    }
}

impl<T: ColorTrait> Deref for ColorHisto<T> {
    type Target = Vec<Histo<T>>;
    fn deref(&self) -> &Vec<Histo<T>> { &self.0 }
}

impl<T: ColorTrait> DerefMut for ColorHisto<T> {
    fn deref_mut(&mut self) -> &mut Vec<Histo<T>> { &mut self.0 }
}

impl<T: ColorTrait> From<Vec<Histo<T>>> for ColorHisto<T> {
    fn from(c: Vec<Histo<T>>) -> Self { Self(c) }
}

// Implement into since Vec is a foreign type
impl<T: ColorTrait> Into<Vec<Histo<T>>> for ColorHisto<T> {
    fn into(self) -> Vec<Histo<T>> { self.0 }
}


impl<T: ColorTrait + Copy> From<ColorHisto<T>> for Vec<Myrgb>
    where
Myrgb: From<T>
{
    fn from(c: ColorHisto<T>) -> Self {
        c.0.iter().map(|x| x.color.into()).collect()
    }
}

impl<T: ColorTrait + Copy + IntoColor<Srgb>> From<ColorHisto<T>> for Vec<Srgb>
{
    fn from(c: ColorHisto<T>) -> Self {
        c.0.iter().map(|x| x.color.into_color()).collect()
    }
}

pub trait Difference {
    fn col_diff(&self, a: &Self, threshold: u8) -> bool;
}

/// Simple trait that groups all avaliable colorspaces
pub trait ColorTrait {}

/// Simple wrapper for a vector of histograms.
/// Abstracts away vector/slices operations by using Deref and DerefMut traits.
#[derive(Debug, Clone, PartialEq)]
pub struct ColorHisto<T: ColorTrait>(Vec<Histo<T>>);

/// This trait is for creating a new `ColorHisto` type.
/// The Self parameter should always be a wrapper like Color Histo.
/// The main logic of how these methods are used are in `main()`
pub trait BuildColors: Sized + From<Vec<Histo<Self::Color>>> + Into<Vec<Histo<Self::Color>>> {
    /// Colorspace to be used
    type Color: ColorTrait + Difference + Into<Myrgb> + From<Myrgb> + Copy + Mix<Scalar = f32> + IntoColor<Srgb>
        + FromColorUnclamped<Srgb>
        + Clamp
        + palette::convert::FromColorUnclamped<palette::rgb::Rgb<palette::encoding::Linear<palette::encoding::Srgb>>>;

    /// Function that read the image rgb8 bytes and converts them into it's colorspace
    fn read(bytes: &[u8]) -> Vec<Self::Color> {
        let s: &[Srgb<u8>] = bytes.components_as();
        s
            .iter()
            .map(|x| x.into_linear().into_color())
            .collect::<Vec<Self::Color>>()
    }

    /// What colors to avoid before adding. e.g. too dark/light
    fn filter_cols(a: Self::Color) -> bool;

    /// Simple Sort algo that determines how to order colors
    /// usecase: `histo.sort_by(|a, b| color_ord.sort_algo(a, b))`
    fn sort_algo(cs: &ColorOrder, a: &Histo<Self::Color>, b: &Histo<Self::Color>) -> Ordering;

    /// how to .sort_by_key, this is colorspace specific
    fn sort_by_key_fn(a: Histo<Self::Color>) -> impl Ord;

    /// Consumes self into a vec
    fn to_vec(self) -> Vec<Histo<Self::Color>> { self.into() }

    /// This function is used when the colors gathered by new_colors are not enough.
    /// See .gen()
    /// This is how we try to artificially generate colors when there are not at least [`MIN_COLS`].
    /// `pred` is for gather_cols() and `method` indicates how the colors are gonna be filled.
    /// This was called 'new_colors()' (generates a new Vec of Histograms)
    fn color_generator(histo: &[Histo<Self::Color>], threshold: u8, gen: &G) -> Vec<Histo<Self::Color>> {
        let mut new_cols = vec![];
        // try to generate new colors with interpolation in between the already gathered colors
        for comb in histo.iter().combinations(2) {
            let color_a: Srgb = comb[0].color.into_color();
            let color_b: Srgb = comb[1].color.into_color();

            let rgbs = gen.gen()(color_a, color_b, MAX_COLS)
                .iter().map(|&x| x.into_color()).collect();

            //similar to how it's done at the start of `lab()`
            // save the new colors, or discard them if similar enough
            // no more color mixing, we don't have much colors left.
            new_cols.append(&mut Self::gather_cols(rgbs, threshold, false).to_vec());

            let len = histo.len() + new_cols.len();

            if len >= MIN_COLS.into() { break; } //enough colors, stop interpolating
        }

        new_cols
    }

    /// This is a generic way of creating a histogram.
    fn gather_cols(colors: Vec<Self::Color>, threshold: u8, mix: bool) -> Self {
        let mut histogram: Vec<Histo<Self::Color>> = vec![];

        'outter: for c in colors {
            if Self::filter_cols(c) {
                // Check if whether the color is new or is already in the vec
                for hist in &mut histogram {
                    // if any color is between a threshold, count it up
                    if c.col_diff(&hist.color, threshold) {
                        if mix { hist.color = hist.color.mix(c, 0.5); }
                        hist.count += 1;
                        continue 'outter;
                    }
                }
                // if we reach here, the color hasn't been found in the histrogram,
                // so we found a new color.
                histogram.push(Histo { color: c, count: 1 });
            }
        }

        histogram.into()
    }
}



/// Basically returns a tuple with `((histogram, histogram_not_sorted), warn)`
/// `warn` is important for printing warnings, but it's only that, a warning.
/// Since we use [`FallbackGenerator`]s, maybe this should be split up in the future..
pub fn main<T>(bytes_rgb8: &[u8], threshold: u8, gen: &G, mix: bool, ord: &ColorOrder)
    -> Result<((Vec<Srgb>, Vec<Srgb>), bool), ColorSpaceError>
where
    ColorHisto<T>: BuildColors<Color = T> + Into<Vec<Myrgb>>,
    T: Copy + ColorTrait + Difference + FromColorUnclamped<Rgb> + Clamp,
    palette::rgb::Rgb: FromColorUnclamped<T>,
{
    // This is to indicate if there were any warnings, since we can't print them directly
    let mut warn = false;

    let color = ColorHisto::read(bytes_rgb8);

//     let mut labs = rgb_bytes_to_labs(cols);
//     labs.dedup();
//     // XXX using `delta_e` with `.dedup()` here, reduces the vector that littlel
//     // that the colors aren't the most prominent ones (for the most part).
//     // However, avoiding `.dedup()` and not calling it, also changes the result.
//     // After some testing I decided that the most 'plausible' colors would be
//     // the one that requires `.dedup()`.
//     //labs.dedup_by(|a, b| lab::delta_e(*a, *b) <= threshold.into());
//
//     gather_cols(labs, threshold, mix, &pred)

    let mut histo = ColorHisto::gather_cols(color, threshold, mix);

    // `interpolate()` requires two colors, else we can't attempt to generate colors at our own
    if histo.len() < 2 { return Err(ColorSpaceError::TwoColors) }

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
    histo.dedup_by(|a, b| a.color.col_diff(&b.color, threshold));

    // sort vec by count, most used colors first
    histo.sort_by(|a, b| b.count.cmp(&a.count));

    // remove excess elements
    histo.truncate(MAX_COLS.into());

    if histo.len() == 2 {
    // If the colors are exactly two, create a long interpolation from it.
        warn = true;
        let mut new = gen.gen()(histo[0].color.into_color(), histo[1].color.into_color(), MIN_COLS)
            .iter()
            .map(|&x| {
                let c: T = x.into_color();
                Histo { color: c, count: 1}
            })
            .collect::<Vec<Histo<T>>>();

        histo.append(&mut new);

        // sort vec by count, most used colors first (if they are more than the MAX)
        histo.sort_by(|a, b| b.count.cmp(&a.count));

        // take the *necessary* most used colors
        histo.truncate(MAX_COLS.into());

    } else if histo.len() < MIN_COLS.into() {
    // Artificially generate colors with linear interpolation in between the colors that we already
    // have. However even this can even fail and not generate enough different colors, so there is
    // another check below
        warn = true; // "artificially generation colors.."

        // fallback_generator
        // XXX Is this really necesary with the new "automatic handling of the threshold?"
        let mut new = ColorHisto::color_generator(&histo, threshold, gen);

        histo.append(&mut new);

        // sort vec by count, most used colors first (if they are more than the MAX)
        histo.sort_by(|a, b| b.count.cmp(&a.count));

        // take the *necessary* most used colors
        histo.truncate(MAX_COLS.into());
    }

    // not enough colors, even after making new colors (if any)
    if histo.len() < MIN_COLS.into() { return Err(ColorSpaceError::NotEnough) }

    // orig_histo will not be changed by `sort_colors`,
    // thus keeping the `top used colors` order in place
    let orig_histo = histo.clone();

    // custom sorting, checkout [`ColorOrder`] and [`sort_ord`]
    //histo = T::sort_algo(&Cs);
    histo.sort_by(|a, b| ColorHisto::sort_algo(ord, a, b));

    Ok(
        ((histo.into(), orig_histo.into()), warn)
    )
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
            Cs::Lch => write!(f, "Lch"),
            Cs::LchMixed => write!(f, "LchMixed"),
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
fn complementary(color_a: Srgb, color_b: Srgb, _: u8) -> Vec<Srgb> {
    vec![
        color_a.complementary(),
        color_b.complementary(),
    ]
}
