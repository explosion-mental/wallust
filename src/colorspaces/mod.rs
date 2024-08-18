//! # Colorspaces
//! This modules has the job of reducing all the bytes given from the `backend` to two (2) vectors:
//! 1. The first one is a sorted, see [`ColorOrder`], array.
//! 2. The second one is about preserving the most dominant color order in the vector, ensuring the
//!    first entry is the dominant (most repeated one).

use std::fmt;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;

use crate::colors::Myrgb;
use crate::colors::Compl;

use palette::convert::FromColorUnclamped;
use palette::cast::ComponentsAs;
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
mod lchansi;

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

    /// Variant of Lch which preserves 8 colors: black, red, green, yellow, blue, magenta, cyan and gray.
    /// This works best with 'darkansi' palette, allowing a constant color order.
    #[clap(alias = "lch-ansi", name = "lchansi")] //claps prefers this-name
    #[serde(alias = "lch-ansi")]
    LchAnsi,
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

impl<T: ColorTrait> Histo<T> {
    /// Creates a new histogram
    pub fn new(color: T, count: usize) -> Self {
        Self { color, count }
    }

    /// Creates a new histogram with a fixed count
    pub fn new_no_count(color: T) -> Self {
        Self { color, count: usize::MAX }
        // Self { color, count: 0 }
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
        // TODO reorganize this
        // * maybe assign variables inside the module to put it here, like `lchansi::LchAnsi::Mixed` and so on
        let mix = self.mixed();
        let dedup = self.to_dedup();

        match self {
            Cs::Lab => main::<lab::Lab, lab::Spec>(bytes_rgb8, threshold, gen, mix, ord, dedup),
            Cs::LabMixed => main::<lab::Lab, lab::Spec>(bytes_rgb8, threshold, gen, mix, ord, dedup),

            Cs::Lch => main::<lch::Lch, lch::Spec>(bytes_rgb8, threshold, gen, mix, ord, dedup),
            Cs::LchMixed => main::<lch::Lch, lch::Spec>(bytes_rgb8, threshold, gen, mix, ord, dedup),
            Cs::LchAnsi => main::<lchansi::LchAnsi, lch::Spec>(bytes_rgb8, threshold, gen, mix, ord, dedup),
        }
    }

    /// XXX just use matches!
    pub fn mixed(&self) -> bool {
        match self {
            Cs::LabMixed | Cs::LchMixed  => true,
            Cs::Lch | Cs::Lab | Cs::LchAnsi => false,
        }
    }

    /// Only LCHANSI requires to preserve it's order, no deduping!
    pub fn to_dedup(&self) -> bool {
        match self {
            Cs::LabMixed | Cs::LchMixed | Cs::Lch | Cs::Lab => true,
            Cs::LchAnsi => false,
        }
    }

    /// Assign a color for the ColorSpace
    pub fn col(&self) -> AnsiColors {
        match self {
            Cs::Lab => AnsiColors::Blue,
            Cs::LabMixed => AnsiColors::Green,
            Cs::Lch => AnsiColors::Magenta,
            Cs::LchMixed => AnsiColors::Magenta,
            Cs::LchAnsi => AnsiColors::Cyan,
        }
    }
    /// automatic threshold
    /// TODO needs more testing
    pub fn def_threshold(&self) -> u8 {
        match self {
            Cs::Lab | Cs::LabMixed => 17,
            Cs::Lch | Cs::LchMixed | Cs::LchAnsi => 20,
        }
    }
}

impl<T: ColorTrait> From<Histo<T>> for Myrgb {
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

impl<T: ColorTrait, U> Deref for ColorHisto<T, U> {
    type Target = Vec<Histo<T>>;
    fn deref(&self) -> &Vec<Histo<T>> { &self.0 }
}

impl<T: ColorTrait, U> DerefMut for ColorHisto<T, U> {
    fn deref_mut(&mut self) -> &mut Vec<Histo<T>> { &mut self.0 }
}

impl<T: ColorTrait, U> From<Vec<Histo<T>>> for ColorHisto<T, U> {
    fn from(c: Vec<Histo<T>>) -> Self { Self(c, PhantomData::<U>) }
}

impl<T: ColorTrait, U> From<ColorHisto<T, U>> for Vec<Histo<T>> {
    fn from(val: ColorHisto<T, U>) -> Self { val.0 }
}

impl<T: ColorTrait, U> From<ColorHisto<T, U>> for Vec<Myrgb> {
    fn from(c: ColorHisto<T, U>) -> Self { c.0.iter().map(|x| x.color.into()).collect() }
}

//impl<T: ColorTrait, U> From<ColorHisto<T, U>> for Vec<palette::rgb::Rgb> {
impl<T: ColorTrait, U> From<ColorHisto<T, U>> for Vec<Srgb> {
    fn from(c: ColorHisto<T, U>) -> Self { c.0.iter().map(|x| x.color.into_color()).collect() }
}

// impl<T: ColorTrait, U> Into<Vec<palette::rgb::Rgb>> for ColorHisto<T, U> {
//     fn into(self) -> Vec<Srgb> {
//         self.iter().map(|x| x.color.into_color()).collect()
//     }
//     //fn from(c: ColorHisto<T, U>) -> Self { c.0.iter().map(|x| x.color.into_color()).collect() }
// }

/// Method to use for color difference (deltaE)
pub trait Difference {
    fn col_diff(&self, a: &Self, threshold: u8) -> bool;
}

impl<T: ColorTrait> From<T> for Myrgb {
    fn from(lab: T) -> Self {
        let a: Srgb = lab.into_color();
        Self(a)
    }
}


/// Simple trait that groups all avaliable colorspaces
// TODO meassure the required traits.
pub trait ColorTrait:
        Copy
        + Difference
        + Into<Myrgb>
        + IntoColor<Srgb>
        + Mix<Scalar = f32>
        + FromColorUnclamped<Srgb>
        + Clamp
        + palette::convert::FromColorUnclamped<palette::rgb::Rgb<palette::encoding::Linear<palette::encoding::Srgb>>>
{}


pub trait BuildHisto<C: ColorTrait> {
    /// If this fails, then there are less than 2 colors.
    fn init(bytes: &[u8], threshold: u8, mix: bool) -> Option<Vec<Histo<C>>> {
        let b = Self::read(bytes);
        let ret = Self::gather_cols(b, threshold, mix);
        if ret.len() < 2 { None } else { Some(ret) }
    }

    /// If this fails, just quit. Here we try to artificially generate colors.
    fn fallback(histo: Vec<Histo<C>>, threshold: u8, gen: &G) -> Vec<Histo<C>> {
        let mut histo = histo;
        // Artificially generate colors with linear interpolation in between the colors that we already
        // have. However even this can even fail and not generate enough different colors, so there is
        // another check below

        // fallback_generator
        // XXX Is this really necesary with the new "automatic handling of the threshold?"
        let mut new = Self::color_generator(&histo, threshold, gen);

        histo.append(&mut new);

        // sort vec by count, most used colors first (if they are more than the MAX)
        histo.sort_by(|a, b| b.count.cmp(&a.count));

        // take the *necessary* most used colors
        histo.truncate(MAX_COLS.into());
        histo
    }

    /// No need for a threshold, since here we only got 2 colors.
    fn fallback_monochromatic(histo: Vec<Histo<C>>, gen: &G) -> Vec<Histo<C>> {
        let mut histo = histo;
        let mut new = gen.gen()(histo[0].color.into_color(), histo[1].color.into_color(), MIN_COLS)
            .iter()
            .map(|&x| {
                let c: C = x.into_color();
                Histo { color: c, count: 1 }
            })
            .collect::<Vec<Histo<C>>>();

        histo.append(&mut new);

        // sort vec by count, most used colors first (if they are more than the MAX)
        histo.sort_by(|a, b| b.count.cmp(&a.count));

        // take the *necessary* most used colors
        histo.truncate(MAX_COLS.into());
        histo
    }

    /// XXX I've tested a lot and: (requires more in depth findings)
    /// 1. using `dedup_by` without `sort_by_key` seems to not get much colors.
    /// 2. obviously sorting without `dedup`ing won't do much.
    /// 3. to get more colors `.truncate()` should accept `MAX_COLS`, however this used to get many
    ///    similar colors, not resulting in an stable palette. By using these two methods below, we
    ///    'asure' (lazyly) to have no duplicates, and thus, the benefit of 'more colors' won't
    ///    imply 'bad scheme'.
    fn dedup_cols(histo: Vec<Histo<C>>, threshold: u8) -> Vec<Histo<C>> {
        let mut histo = histo;
        // histo.sort_by_key(|e| (e.color.l as u32, e.color.a as i32, e.color.b as i32));
        // histo.dedup_by(|a, b| lab::delta_e(a.color, b.color) <= threshold.into());
        // labs.sort_by_key(|e| (e.l.trunc() as u32, e.a.trunc() as i32, e.b.trunc() as i32));
        // labs.dedup_by(|a, b| lab::delta_e(*a, *b) <= threshold.into());
        // labs.dedup();
        histo.sort_by_key(|&a| Self::sort_by_key_fn(a));
        histo.dedup_by(|a, b| a.color.col_diff(&b.color, threshold));

        // sort vec by count, most used colors first
        histo.sort_by(|a, b| b.count.cmp(&a.count));

        // remove excess elements
        histo.truncate(MAX_COLS.into());
        histo
    }

    /// Function that read the image rgb8 bytes and converts them into it's colorspace
    fn read(bytes: &[u8]) -> Vec<C> { read(bytes) }

    /// What colors to avoid before adding. e.g. too dark/light
    fn filter_cols(a: C) -> bool;

    /// Simple Sort algo that determines how to order colors
    /// usecase: `histo.sort_by(|a, b| color_ord.sort_algo(a, b))`
    fn sort_col(histo: Vec<Histo<C>>, cs: &ColorOrder) -> Vec<Histo<C>>;

    /// how to .sort_by_key, this is colorspace specific
    fn sort_by_key_fn(a: Histo<C>) -> impl Ord;

    // No need, we work directly with vecs
    // Consumes self into a vec
    //fn to_vec(self) -> Vec<Histo<C>> { self.into() }

    /// This function is used when the colors gathered by new_colors are not enough.
    /// See .gen()
    /// This is how we try to artificially generate colors when there are not at least [`MIN_COLS`].
    /// `pred` is for gather_cols() and `method` indicates how the colors are gonna be filled.
    /// This was called 'new_colors()' (generates a new Vec of Histograms)
    fn color_generator(histo: &[Histo<C>], threshold: u8, gen: &G) -> Vec<Histo<C>> {
        color_generator2::<C, Self>(histo, threshold, gen)
    }

    /// This is a generic way of creating a histogram.
    fn gather_cols(colors: Vec<C>, threshold: u8, mix: bool) -> Vec<Histo<C>> {
        gather_cols2::<C, Self>(colors, threshold, mix)
    }
}

/// Simple wrapper for a vector of histograms.
/// Abstracts away vector/slices operations by using Deref and DerefMut traits.
#[derive(Debug, Clone, PartialEq)]
pub struct ColorHisto<T: ColorTrait, U>(Vec<Histo<T>>, PhantomData<U>);

/// This trait is for creating a new `ColorHisto` type.
/// The Self parameter should always be a wrapper like Color Histo.
/// The main logic of how these methods are used are in `main()`
pub trait BuildColors: Sized + From<Vec<Histo<Self::Color>>> + Into<Vec<Histo<Self::Color>>> {
//pub trait BuildColors {
    /// Colorspace to be used
    type Color: ColorTrait;

    /// Function that read the image rgb8 bytes and converts them into it's colorspace
    fn read(bytes: &[u8]) -> Vec<Self::Color> { read(bytes) }

    /// What colors to avoid before adding. e.g. too dark/light
    fn filter_cols(a: Self::Color) -> bool;

    /// Simple Sort algo that determines how to order colors
    /// usecase: `histo.sort_by(|a, b| color_ord.sort_algo(a, b))`
    fn sort_col(self, cs: &ColorOrder) -> Self;

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
        color_generator::<Self::Color, Self>(histo, threshold, gen)
    }

    /// This is a generic way of creating a histogram.
    fn gather_cols(colors: Vec<Self::Color>, threshold: u8, mix: bool) -> Self {
        gather_cols(colors, threshold, mix)
    }
}

/// Basically returns a tuple with `((histogram, histogram_not_sorted), warn)`
/// `warn` is important for printing warnings, but it's only that, a warning.
/// Since we use [`FallbackGenerator`]s, maybe this should be split up in the future..
pub fn main<U, T: ColorTrait>(bytes_rgb8: &[u8], threshold: u8, gen: &G, mix: bool, ord: &ColorOrder, dedup: bool)
    -> Result<((Vec<Srgb>, Vec<Srgb>), bool), ColorSpaceError>
where
    ColorHisto<T, U>: BuildColors<Color = T> + Into<Vec<Myrgb>>,
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

    if dedup {
    // XXX I've tested a lot and: (requires more in depth findings)
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
    }

    if histo.len() == 2 {
    // If the colors are exactly two, create a long interpolation from it.
        warn = true;
        let mut new = gen.gen()(histo[0].color.into_color(), histo[1].color.into_color(), MIN_COLS)
            .iter()
            .map(|&x| {
                let c: T = x.into_color();
                Histo { color: c, count: 1 }
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

        warn = true;

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

    // TODO don't clone and sort here, since this is function runs multiple times (most of the time)
    // orig_histo will not be changed by `sort_colors`,
    // thus keeping the `top used colors` order in place
    let orig_histo = histo.clone();

    // custom sorting, checkout [`ColorOrder`] and [`sort_ord`]
    //histo = T::sort_algo(&Cs);
    //histo.sort_by(|a, b| ColorHisto::sort_algo(ord, a, b));
    let histo = histo.sort_col(ord);

    Ok(
        ((histo.into(), ColorHisto(orig_histo, PhantomData::<lab::Lab>).into()), warn)
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
            Cs::LchAnsi => write!(f, "LchAnsi"),
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


/* generic impl */


/// Function that read the image rgb8 bytes and converts them into it's colorspace
fn read<T: ColorTrait>(bytes: &[u8]) -> Vec<T> {
    let s: &[Srgb<u8>] = bytes.components_as();
    s
        .iter()
        .map(|x| x.into_linear().into_color())
        .collect::<Vec<T>>()
}



/// This function is used when the colors gathered by new_colors are not enough.
/// See .gen()
/// This is how we try to artificially generate colors when there are not at least [`MIN_COLS`].
/// `pred` is for gather_cols() and `method` indicates how the colors are gonna be filled.
/// This was called 'new_colors()' (generates a new Vec of Histograms)
fn color_generator<T: ColorTrait, U: BuildColors<Color = T>> (histo: &[Histo<T>], threshold: u8, gen: &G) -> Vec<Histo<T>>
{
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
        new_cols.append(&mut gather_cols::<T, U>(rgbs, threshold, false).to_vec());

        let len = histo.len() + new_cols.len();

        if len >= MIN_COLS.into() { break; } //enough colors, stop interpolating
    }

    new_cols
}

fn color_generator2<T: ColorTrait, U: BuildHisto<T> + ?Sized> (histo: &[Histo<T>], threshold: u8, gen: &G) -> Vec<Histo<T>>
{
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
        new_cols.append(&mut gather_cols2::<T, U>(rgbs, threshold, false));

        let len = histo.len() + new_cols.len();

        if len >= MIN_COLS.into() { break; } //enough colors, stop interpolating
    }

    new_cols
}


fn gather_cols2<T: ColorTrait, U: BuildHisto<T> + ?Sized>(colors: Vec<T>, threshold: u8, mix: bool) -> Vec<Histo<T>> {
    let mut histogram: Vec<Histo<T>> = vec![];

    'outter: for c in colors {
        if U::filter_cols(c) {
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

/// This is a generic way of creating a histogram.
fn gather_cols<T: ColorTrait, U: BuildColors<Color = T>>(colors: Vec<T>, threshold: u8, mix: bool) -> U
{
    let mut histogram: Vec<Histo<T>> = vec![];

    'outter: for c in colors {
        if U::filter_cols(c) {
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
