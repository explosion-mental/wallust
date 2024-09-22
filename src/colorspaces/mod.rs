//! # Colorspaces
//! This modules has the job of reducing all the bytes given from the `backend` to two (2) vectors:
//! 1. The first one is a sorted, see [`ColorOrder`], array.
//! 2. The second one is about preserving the most dominant color order in the vector, ensuring the
//!    first entry is the dominant (most repeated one).

use std::fmt;

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
pub const MIN_COLS: u8 = 6;

/// The [`Colors`] struct only has capacity for 16 colors 0..=15. const is used in order to take
/// the top MAX_COLS lab colors.
pub const MAX_COLS: u8 = 16;

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

pub fn run_dynamic<C: BuildHisto<U>, U: ColorTrait>(
    bytes: &[u8],
    _threshold: u8,
    gen: &G,
    mix: bool,
    ord: &ColorOrder,
    _dedup: bool, //TODO
) -> Option<(Vec<Srgb>, Vec<Srgb>, bool)> {

    //TODO one could use a binary tree split (and even maybe async) to find out the "best threshold"

    let mut fallback = false;
    let mut warn = false;
    let mut histo = vec![];

    // This is the max value a threshold can have.
    // => This has to be a hardcoded tested allround value to avoid going to inifinity.
    //let max_threshold = 30;
    let min_threshold = 2;
    let mut threshold = 20; //initial threshold

    use std::collections::HashMap;

    // The first element is going to be 0, this is to avoid `expect()` panicing
    // since, this hasmap will never be empty.
    // There can be more than one VALUE for the same KEY, given that different threshold could
    // generate the same lenght of colors:
    // - Key is the LEN of each threshold result
    // - Value is the threshold being used
    let mut hash: HashMap<usize, Vec<u8>> = HashMap::from([(0, vec![0])]);

    'running: loop {
        // read image
        let init = C::init(bytes, threshold, mix);

        // println!("INIT \n hash {hash:#?}\nth {threshold}");

        //TODO
        // run until max_threshold or break until [min_cols; x] U [x; max_cols] ?
        // meaning, threhols += 1 (start from 2) or threhols -= 1 (start from 30)
        // => Yes, run from 20 to 2, if that somehow fails.. run from 20 to 30.
        //    The passage from 20 to 30 is very unlikely to give good enough results tho... so maybe just fallback inmediatly?

        match init {
            // There are colors! This threshold works.
            Some(s) => {
                //histo = s;
                // println!("INITTT : {init:?}");
                let len = s.len();

                // enough colors, end
                if len >= MIN_COLS as usize
                && len <= MAX_COLS as usize //we can't use 200 colors...
                // if len >= MAX_COLS.into()
                // && len < (MAX_COLS * 2).into()
                {
                    histo = s;
                    break 'running
                }

                // store threshold with the LEN being the KEY
                hash.entry(len).or_default().push(threshold);

                if fallback { break 'running } // fallback activated below
            },
            // No colors.. Change threshold or end it here (fallback generator).
            None => {
                // max KEY
                let max = *hash.iter().max_by(|a, b| a.0.cmp(b.0)).expect("not empty").0;

                if (max - 3) < threshold.into() // at least 4 runs apart, without any len change before quitting
                && max < MIN_COLS.into()
                {
                    //not enough colors in the image, or the backend, etc..
                    return None;
                } else {
                    let possible_ths = hash.get(&max).expect("not empty");
                    threshold = possible_ths[possible_ths.len() / 2]; //median of thresholds
                    fallback = true;
                    // println!("FALLBACK! {possible_ths:?} | max {max} | threshold {threshold}")
                }
            },
        }

        if
            // histo.len() == MIN_COLS as usize // enough colors, we are done looping
            threshold == min_threshold // set a limit, don't go forever..
            // || fallback // we are done looping, bc scarse colors
        { break 'running }

        // inc threshold every loop [1; 50]
        threshold -= 1;
    }

    //if init.is_empty() { return None }
    //let mut init = init.expect("checked above");

    if histo.len() == 2 {
        warn = true;
        // println!("TWO COOOLORSS");
        histo = C::fallback_monochromatic(histo, gen);
    } else if fallback || histo.len() < MIN_COLS.into() {
        warn = true;
        histo = C::fallback(histo, threshold, gen);
    }

    let top  = C::sort_col(histo.clone(), ord);
    let top  = C::to_rgb(top);
    let orig = C::to_rgb(histo);

    Some( (top, orig, warn) )
}

pub fn run_once<C: BuildHisto<U>, U: ColorTrait>(
    bytes: &[u8],
    threshold: u8,
    gen: &G,
    mix: bool,
    ord: &ColorOrder,
    _dedup: bool, //TODO
) -> Option<(Vec<Srgb>, Vec<Srgb>, bool)> {

    let mut warn = false;

    let ret = match C::init(bytes, threshold, mix) {
        Some(s) => {
            if s.len() == 2 {
                warn = true;
                Some(C::fallback_monochromatic(s, gen))
            } else if s.len() < MIN_COLS.into() {
                warn = true;
                Some(C::fallback(s, threshold, gen))
            } else {
                // JUST WRITE HEREEEEEEEEEEEEEEEEEEEEEEE
                //eprilntln!("Something went Wrong!");
                warn = false;
                Some(s)
            }
        },
        None => None,
    };

    let ret = match ret {
        None => return None,
        Some(s) => s,
    };

    //TODO clone necesarry??
    let top  = C::sort_col(ret.clone(), ord);
    let top  = C::to_rgb(top);
    let orig = C::to_rgb(ret);

    Some( (top, orig, warn) )
}

impl ColorSpace {
    /// main function from ColorSpace, uses a respective dynamic or manual function
    pub fn run(&self, dynamic: bool, bytes_rgb8: &[u8], threshold: u8, gen: &G, ord: &ColorOrder) -> Option<(Vec<Srgb>, Vec<Srgb>, bool)> {
        match dynamic {
            true => self.run_dynamic(bytes_rgb8, threshold, gen, ord),
            false => self.run_once(bytes_rgb8, threshold, gen, ord),
        }
    }

    pub fn run_once(&self, bytes_rgb8: &[u8], threshold: u8 /* dummy */, gen: &G, ord: &ColorOrder) -> Option<(Vec<Srgb>, Vec<Srgb>, bool)> {
        let mix = self.mixed();
        let dedup = self.to_dedup();

        let f = match self {
            Cs::Lab => run_once::<lab::Lab, lab::Spec>,
            Cs::LabMixed => run_once::<lab::Lab, lab::Spec>,

            Cs::Lch => run_once::<lch::Lch, lch::Spec>,
            Cs::LchMixed => run_once::<lch::Lch, lch::Spec>,
            Cs::LchAnsi => run_once::<lchansi::LchAnsi, lch::Spec>,
        };

        f(bytes_rgb8, threshold, gen, mix, ord, dedup)

    }

    pub fn run_dynamic(&self, bytes_rgb8: &[u8], threshold: u8, gen: &G, ord: &ColorOrder) -> Option<(Vec<Srgb>, Vec<Srgb>, bool)> {
        let mix = self.mixed();
        let dedup = self.to_dedup();

        match self {
            Cs::Lab => run_dynamic::<lab::Lab, lab::Spec>(bytes_rgb8, threshold, gen, mix, ord, dedup),
            Cs::LabMixed => run_dynamic::<lab::Lab, lab::Spec>(bytes_rgb8, threshold, gen, mix, ord, dedup),

            Cs::Lch => run_dynamic::<lch::Lch, lch::Spec>(bytes_rgb8, threshold, gen, mix, ord, dedup),
            Cs::LchMixed => run_dynamic::<lch::Lch, lch::Spec>(bytes_rgb8, threshold, gen, mix, ord, dedup),
            Cs::LchAnsi => run_dynamic::<lchansi::LchAnsi, lch::Spec>(bytes_rgb8, threshold, gen, mix, ord, dedup),
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
    /// XXX not used anymore... since we search every value to get a good result.
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
        + std::fmt::Debug
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

    fn to_rgb(histo: Vec<Histo<C>>) -> Vec<Srgb> { histo.iter().map(|x| x.color.into_color()).collect() }
}

// /// Basically returns a tuple with `((histogram, histogram_not_sorted), warn)`
// /// `warn` is important for printing warnings, but it's only that, a warning.
// /// Since we use [`FallbackGenerator`]s, maybe this should be split up in the future..
// pub fn main<U, T: ColorTrait>(bytes_rgb8: &[u8], threshold: u8, gen: &G, mix: bool, ord: &ColorOrder, dedup: bool)
//     -> Result<((Vec<Srgb>, Vec<Srgb>), bool), ColorSpaceError>
// where
//     ColorHisto<T, U>: BuildColors<Color = T> + Into<Vec<Myrgb>>,
// {
//     // This is to indicate if there were any warnings, since we can't print them directly
//     let mut warn = false;
//
//     let color = ColorHisto::read(bytes_rgb8);
//
// //     let mut labs = rgb_bytes_to_labs(cols);
// //     labs.dedup();
// //     // XXX using `delta_e` with `.dedup()` here, reduces the vector that littlel
// //     // that the colors aren't the most prominent ones (for the most part).
// //     // However, avoiding `.dedup()` and not calling it, also changes the result.
// //     // After some testing I decided that the most 'plausible' colors would be
// //     // the one that requires `.dedup()`.
// //     //labs.dedup_by(|a, b| lab::delta_e(*a, *b) <= threshold.into());
// //
// //     gather_cols(labs, threshold, mix, &pred)
//
//     let mut histo = ColorHisto::gather_cols(color, threshold, mix);
//
//     // `interpolate()` requires two colors, else we can't attempt to generate colors at our own
//     if histo.len() < 2 { return Err(ColorSpaceError::TwoColors) }
//
//     if dedup {
//     // XXX I've tested a lot and: (requires more in depth findings)
//     // 1. using `dedup_by` without `sort_by_key` seems to not get much colors.
//     // 2. obviously sorting without `dedup`ing won't do much.
//     // 3. to get more colors `.truncate()` should accept `MAX_COLS`, however this used to get many
//     //    similar colors, not resulting in an stable palette. By using these two methods below, we
//     //    'asure' (lazyly) to have no duplicates, and thus, the benefit of 'more colors' won't
//     //    imply 'bad scheme'.
//     // histo.sort_by_key(|e| (e.color.l as u32, e.color.a as i32, e.color.b as i32));
//     // histo.dedup_by(|a, b| lab::delta_e(a.color, b.color) <= threshold.into());
//     // labs.sort_by_key(|e| (e.l.trunc() as u32, e.a.trunc() as i32, e.b.trunc() as i32));
//     // labs.dedup_by(|a, b| lab::delta_e(*a, *b) <= threshold.into());
//     // labs.dedup();
//     histo.sort_by_key(|&a| ColorHisto::sort_by_key_fn(a));
//     histo.dedup_by(|a, b| a.color.col_diff(&b.color, threshold));
//
//     // sort vec by count, most used colors first
//     histo.sort_by(|a, b| b.count.cmp(&a.count));
//
//     // remove excess elements
//     histo.truncate(MAX_COLS.into());
//     }
//
//     if histo.len() == 2 {
//     // If the colors are exactly two, create a long interpolation from it.
//         warn = true;
//         let mut new = gen.gen()(histo[0].color.into_color(), histo[1].color.into_color(), MIN_COLS)
//             .iter()
//             .map(|&x| {
//                 let c: T = x.into_color();
//                 Histo { color: c, count: 1 }
//             })
//             .collect::<Vec<Histo<T>>>();
//
//         histo.append(&mut new);
//
//         // sort vec by count, most used colors first (if they are more than the MAX)
//         histo.sort_by(|a, b| b.count.cmp(&a.count));
//
//         // take the *necessary* most used colors
//         histo.truncate(MAX_COLS.into());
//
//     } else if histo.len() < MIN_COLS.into() {
//         // Artificially generate colors with linear interpolation in between the colors that we already
//         // have. However even this can even fail and not generate enough different colors, so there is
//         // another check below
//
//         warn = true;
//
//         // fallback_generator
//         // XXX Is this really necesary with the new "automatic handling of the threshold?"
//         let mut new = ColorHisto::color_generator(&histo, threshold, gen);
//
//         histo.append(&mut new);
//
//         // sort vec by count, most used colors first (if they are more than the MAX)
//         histo.sort_by(|a, b| b.count.cmp(&a.count));
//
//         // take the *necessary* most used colors
//         histo.truncate(MAX_COLS.into());
//     }
//
//     // not enough colors, even after making new colors (if any)
//     if histo.len() < MIN_COLS.into() { return Err(ColorSpaceError::NotEnough) }
//
//     // TODO don't clone and sort here, since this is function runs multiple times (most of the time)
//     // orig_histo will not be changed by `sort_colors`,
//     // thus keeping the `top used colors` order in place
//     let orig_histo = histo.clone();
//
//     // custom sorting, checkout [`ColorOrder`] and [`sort_ord`]
//     //histo = T::sort_algo(&Cs);
//     //histo.sort_by(|a, b| ColorHisto::sort_algo(ord, a, b));
//     let histo = histo.sort_col(ord);
//
//     Ok(
//         ((histo.into(), ColorHisto(orig_histo, PhantomData::<lab::Lab>).into()), warn)
//     )
// }

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
