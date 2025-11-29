//! # Colorspaces
//! This modules has the job of reducing all the bytes given from the `backend` to two (2) vectors:
//! 1. The first one is a sorted, see [`ColorOrder`], array.
//! 2. The second one is about preserving the most dominant color order in the vector, ensuring the
//!    first entry is the dominant (most repeated one).

use std::fmt;

use crate::colors::Myrgb;

use palette::convert::FromColorUnclamped;
use palette::cast::ComponentsAs;
use palette::IntoColor;
use palette::Clamp;
use palette::Srgb;
use palette::Mix;
use serde::{Serialize, Deserialize};
use owo_colors::AnsiColors;
use itertools::Itertools;
pub use fallback_generator::FallbackGenerator;

use salience::Salient;

mod lab;
mod lch;
pub mod salience;
mod lchansi;
mod util;
mod fallback_generator;

use fallback_generator::FallbackGenerator as G;
/// Currently this works in function with the palettes methods, which currently only needs 6 colors.
/// Let's make sure the colorspace backend send at least these number of colors.
pub const MIN_COLS: u8 = 6;

/// The [`Colors`] struct only has capacity for 16 colors 0..=15. const is used in order to take
/// the top MAX_COLS lab colors.
pub const MAX_COLS: u8 = 16;

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
#[cfg_attr(feature = "schema" , derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum ColorSpace {
    /// Uses Cie L*a*b color space
    Lab,
    #[clap(alias = "lab-mixed", name = "labmixed")] //claps prefers this-name
    #[serde(alias = "lab-mixed")]
    /// Variant of `lab` that mixes the colors gathered, if not enough colors it fallbacks to usual
    /// lab (not recommended in small images)
    LabMixed,
    /// CIE Lch, you can understand this color space like LAB but with chrome and hue added.
    /// Could help when sorting.
    #[default]
    Lch,
    /// CIE Lch variant that mixed on every similar color.
    #[clap(alias = "lch-mixed", name = "lchmixed")] //claps prefers this-name
    #[serde(alias = "lch-mixed")]
    LchMixed,

    /// Differentiates colors by visual **salience**. Salience refers to how much something (a
    /// color) pops out from a context (the background). Currently based on CIE Lch.
    #[clap(alias = "salience", name = "salience")] //claps prefers this-name
    #[serde(alias = "salience")]
    Salience,

    /// Variant of Lch which preserves 8 colors: black, red, green, yellow, blue, magenta, cyan and gray.
    /// This works best with 'darkansi' palette, allowing a constant color order.
    #[clap(alias = "lch-ansi", name = "lchansi")] //claps prefers this-name
    #[serde(alias = "lch-ansi")]
    LchAnsi,
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
    pub fn new(color: T, count: usize) -> Self { Self { color, count } }

    /// Creates a new histogram with a fixed count
    pub fn new_no_count(color: T) -> Self { Self { color, count: usize::MAX } }
}

impl<T: ColorTrait> From<HistoScore<T>> for Histo<T> {
    fn from(hs: HistoScore<T>) -> Self {
        Self { color: hs.histo.color, count: hs.histo.count }
    }
}

/// Histogram wrapper with Scoring
/// Used on salience
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HistoScore<T: ColorTrait> {
    histo: Histo<T>,
    score: f32,
}

impl<T: ColorTrait> HistoScore<T> {
    /// Creates a new histoscore
    pub fn new(histo:Histo<T>, score: f32) -> Self { Self { histo, score } }
}

impl<T: ColorTrait> From<Histo<T>> for HistoScore<T> {
    fn from(h: Histo<T>) -> Self {
        Self { histo: h, score: 0.0 }
    }
}

/// Return a histogram sorted asc on score (count and salience) based on bg
pub fn sort_histogram_by_score<T: ColorTrait + Salient>(histo: Vec<Histo<T>>, bg: T) -> Vec<Histo<T>> {
    let mut histoscore: Vec<HistoScore<T>> = histo.into_iter().map(HistoScore::from).collect();

    // Some kind of constant I pulled from thin air to be used in a formula.
    //
    // The formula is that given colors A and B, where B is half as
    // salient as A, we need 2^cnt_sclr count for B to overpower A.
    let cnt_sclr = 4.0;

    // Factor to adjust salience to score
    // Salience seems to generally scale from 0-100, with slightly preceptable
    // at 5. In other words, with a black background, a pure white color would
    // yield salience 100.
    let sal_factor = 1.0 / 20.0 * 1.5;

    histoscore.iter_mut().for_each(|hs| {
        let score_log_scaled = (hs.histo.count as f32).powf(1.0/cnt_sclr);
        let col = hs.histo.color;
        hs.score = score_log_scaled * (1.0 + col.improved_salience(&bg)*sal_factor);
    });
    histoscore.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Greater));

    histoscore.into_iter().map(Histo::from).collect()
}

/// This a multithreaded function to look up for the best threshold that has the best palette color generation.
pub fn run_dynamic<C: BuildHisto<U>, U: ColorTrait + std::marker::Send> (
    bytes: &[u8],
    _threshold: u8,
    gen: &G,
    mix: bool,
    ord: &ColorOrder,
    dedup: bool,
) -> Option<(Vec<Srgb>, Vec<Srgb>, bool)> {

    use std::thread;
    use std::collections::HashMap;
    use std::sync::mpsc;

    // primitives, can be used around threads
    let mut warn = false;
    let mut fallback = false;
    let mut threshold = 20; //initial threshold

    let (txfinal, rxfinal) = mpsc::channel();

    thread::scope(|s| {
        let mut histo = vec![];

        // => This has to be a hardcoded tested allround value to avoid going to inifinity.
        // let max_threshold = 30;
        let min_threshold = 2;

        // The first element is going to be 0, this is to avoid `expect()` panicing
        // since, this hasmap will never be empty.
        // There can be more than one VALUE for the same KEY, given that different threshold could
        // generate the same lenght of colors:
        // - Key is the LEN of each threshold result
        // - Value is the threshold being used
        let mut hash: HashMap<usize, Vec<u8>> = HashMap::from([(0, vec![0])]);


        // start from the middle, and then search either upper or lower values, as a simple bin tree
        let idx = [14, 16, 13, 17, 12, 18, 11, 19, 10, 20, 9, 21, 8, 22, 7, 23, 6, 24, 5, 25, 4,
        26, 3, 27, 2, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44];

        // println!("\n\n{idx:?}\n\n");

    'running: for i in 0..idx.len() {

        /* spawn threads */
        // we go with a step of 2 because we are spawing 2 threads per loop
        // we go from 30 (threshold) to 2 (the minimun
        let th1 = idx[i];
        let myread1 = s.spawn(move || C::init(bytes, th1, mix, ord));

        let th2 = idx[i+1];
        let myread2 = s.spawn(move || C::init(bytes, th2, mix, ord));

        let th3 = idx[i+2];
        let myread3 = s.spawn(move || C::init(bytes, th3, mix, ord));


        // since we go by a step of 2, check the threads that already have runed
        for (storage, th) in [(myread1, th1), (myread2, th2), (myread3, th3)] {

            threshold = th;
            // wait for the threads
        match storage.join().expect("Waiting for the thread.") {
            // There are colors! This threshold works.
            Some(s) => {

                let s = if dedup {
                    let s = C::dedup_cols(s, threshold, ord);
                    let s = C::process_deduped(s, ord, bytes);
                    C::clip_cols(s, ord)
                } else { s };
                let len = s.len();
                // enough colors, end
                // we handle here before deduping in case is not needed
                if len >= MIN_COLS as usize
                && len <= MAX_COLS as usize //we can't use 200 colors...
                // if len >= MAX_COLS.into()
                // && len < (MAX_COLS * 2).into()
                {
                    histo = s;
                    break 'running
                }


                let len = s.len();

                if len >= MIN_COLS as usize && len <= MAX_COLS as usize {
                    histo = s;
                    break 'running;
                }

                // store threshold with the LEN being the KEY
                hash.entry(len).or_default().push(threshold);
            },
            // No colors.. Change threshold or end it here (fallback generator).
            None => 'nocolor: {
                // What to do here?
                // Some images like lower thresholds...
                // Given that the mayority of images work well with highet and 15+ thresholds, and plateu at ~20,

                // max KEY, meaning the most length
                let max = *hash.iter().max_by(|a, b| a.0.cmp(b.0)).expect("not empty").0;

                // continue trying if max doesn't comply with at least two colors
                if max < 2 { break 'nocolor }

                // We are done, fallback methods require at least 2 colors.
                if threshold == 2 && max < 2 { return }

                if threshold < 10  // one digit threshold
                && max < MIN_COLS.into()
                {
                    let possible_ths = hash.get(&max).expect("not empty");
                    let median = possible_ths[possible_ths.len() / 2]; //median of thresholds
                    threshold = median;
                    fallback = true;
                }
            },
        }

        if threshold == min_threshold // set a limit, don't go forever..
        { break 'running }

        }
    }

    txfinal.send(histo).expect("Sending message MPSC");
    });

    let mut histo = rxfinal.recv().expect("Receiving message MPSC");

    let len = histo.len();

    if len < 2 { return None }

    if len == 2 {
        warn = true;
        histo = C::fallback_monochromatic(histo, gen);
    } else if fallback || len < MIN_COLS.into() {
        warn = true;
        histo = C::fallback(histo, threshold, gen);
    }

    let orig = C::to_rgb(&histo);
    let top  = C::sort_col(histo, ord);
    let top  = C::to_rgb(&top);

    Some( (top, orig, warn) )
}

pub fn run_once<C: BuildHisto<U>, U: ColorTrait>(
    bytes: &[u8],
    threshold: u8,
    gen: &G,
    mix: bool,
    ord: &ColorOrder,
    dedup: bool,
) -> Option<(Vec<Srgb>, Vec<Srgb>, bool)> {

    let mut warn = false;

    let ret = match C::init(bytes, threshold, mix, ord) {
        Some(s) => {
            let s = if dedup {
                let s = C::dedup_cols(s, threshold, ord);
                let s = C::process_deduped(s, ord, bytes);
                C::clip_cols(s, ord)
            } else {
                s
            };


            let len = s.len();

            if len == 2 { //exactly two colors
                warn = true;
                Some(C::fallback_monochromatic(s, gen))
            } else if len < MIN_COLS.into() { // less than MIN_COLs, requires fallback
                warn = true;
                Some(C::fallback(s, threshold, gen))
            } else if len < 2 { // one color?
                warn = true;
                None
            } else { //edge case is more or eq than MIN_COLS
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

    let orig = C::to_rgb(&ret);
    let top  = C::sort_col(ret, ord);
    let top  = C::to_rgb(&top);

    Some( (top, orig, warn) )
}

impl ColorSpace {
    /// main function from ColorSpace, uses a respective dynamic or manual function
    pub fn run(&self, dynamic: bool, bytes_rgb8: &[u8], threshold: u8, gen: &G, ord: &ColorOrder) -> Option<(Vec<Srgb>, Vec<Srgb>, bool)> {
        match dynamic {
            true  => self.run_dynamic(bytes_rgb8, threshold, gen, ord),
            false => self.run_once   (bytes_rgb8, threshold, gen, ord),
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
            Cs::Salience => run_once::<salience::Salience, salience::Spec>,
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
            Cs::Salience => run_dynamic::<salience::Salience, salience::Spec>(bytes_rgb8, threshold, gen, mix, ord, dedup),
        }
    }

    /// XXX just use matches!
    pub fn mixed(&self) -> bool {
        match self {
            Cs::LabMixed | Cs::LchMixed  => true,
            Cs::Lch | Cs::Lab | Cs::Salience | Cs::LchAnsi => false,
        }
    }

    /// Only LCHANSI requires to preserve it's order, no deduping!
    pub fn to_dedup(&self) -> bool {
        match self {
            Cs::LabMixed | Cs::LchMixed | Cs::Lch | Cs::Salience | Cs::Lab => true,
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
            Cs::Salience => AnsiColors::White,
            Cs::LchAnsi => AnsiColors::Cyan,
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
    fn init(bytes: &[u8], threshold: u8, mix: bool, _cs: &ColorOrder) -> Option<Vec<Histo<C>>> {
        let b = Self::read(bytes);
        //let b = Self::additional(self, b);
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
    fn dedup_cols(histo: Vec<Histo<C>>, threshold: u8, _cs: &ColorOrder) -> Vec<Histo<C>> {
        let mut histo = histo;

        // histo.sort_by_key(|e| (e.color.l as u32, e.color.a as i32, e.color.b as i32));
        // histo.dedup_by(|a, b| lab::delta_e(a.color, b.color) <= threshold.into());
        // labs.sort_by_key(|e| (e.l.trunc() as u32, e.a.trunc() as i32, e.b.trunc() as i32));
        // labs.dedup_by(|a, b| lab::delta_e(*a, *b) <= threshold.into());
        // labs.dedup();
        // dedup_by_with_count() allows us to store how many times the colors are dup
        histo.sort_by_key(|&a| Self::sort_by_key_fn(a));
        //histo.dedup_by(|a, b| a.color.col_diff(&b.color, threshold));
        histo
            .iter_mut()
            .dedup_by_with_count(|a, b| a.color.col_diff(&b.color, threshold))
            .for_each(|x| x.1.count += x.0);

        histo
    }

    /// Intermediary between dedup and truncating colors, further process colors based on provided context.
    /// Currently only has an effect on Salience colorspace.
    fn process_deduped(histo: Vec<Histo<C>>, _ord: &ColorOrder, _bytes: &[u8])-> Vec<Histo<C>> {
        let mut histo = histo;

        // sort vec by count, most used colors first
        histo.sort_by(|a, b| b.count.cmp(&a.count));

        histo
    }

    /// Originally, truncating colors were couple with deduplicating, but
    /// sometimes we want to further process colors before truncating. We
    /// separate the two to allow for another step in the pipeline.
    fn clip_cols(histo: Vec<Histo<C>>, _ord: &ColorOrder)-> Vec<Histo<C>> {
        let mut histo = histo;

        // remove excess elements
        histo.truncate(MAX_COLS.into());
        histo
    }

    /// Function that read the image rgb8 bytes and converts them into it's colorspace
    fn read(bytes: &[u8]) -> Vec<C> { read(bytes) }

    // What colors to avoid before adding. e.g. too dark/light
    fn filter_cols(histo: Vec<C>) -> Vec<C>;

    /// Simple Sort algo that determines how to order colors
    /// usecase: `histo.sort_by(|a, b| color_ord.sort_algo(a, b))`
    fn sort_col(histo: Vec<Histo<C>>, cs: &ColorOrder) -> Vec<Histo<C>>;

    /// how to .sort_by_key, this is colorspace specific
    fn sort_by_key_fn(a: Histo<C>) -> impl Ord;

    /// This function is used when the colors gathered by new_colors are not enough.
    /// See .gen()
    /// This is how we try to artificially generate colors when there are not at least [`MIN_COLS`].
    /// `pred` is for gather_cols() and `method` indicates how the colors are gonna be filled.
    /// This was called 'new_colors()' (generates a new Vec of Histograms)
    fn color_generator(histo: &[Histo<C>], threshold: u8, gen: &G) -> Vec<Histo<C>> {
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
            new_cols.append(&mut Self::gather_cols(rgbs, threshold, false));

            let len = histo.len() + new_cols.len();

            if len >= MIN_COLS.into() { break; } //enough colors, stop interpolating
        }

        new_cols
    }

    /// This is a generic way of creating a histogram.
    fn gather_cols(colors: Vec<C>, threshold: u8, mix: bool) -> Vec<Histo<C>> {
        let mut histogram: Vec<Histo<C>> = vec![];
        let colors: Vec<C> = Self::filter_cols(colors);

        'outter: for c in colors {
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

        histogram.into()
    }

    fn to_rgb(histo: &[Histo<C>]) -> Vec<Srgb> { histo.iter().map(|x| x.color.into_color()).collect() }
}

/// Display what [`Cs`] is in use. Used in cache and main.
impl fmt::Display for Cs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cs::Lab => write!(f, "Lab"),
            Cs::LabMixed => write!(f, "LabMixed"),
            Cs::Lch => write!(f, "Lch"),
            Cs::LchMixed => write!(f, "LchMixed"),
            Cs::Salience => write!(f, "Salience"),
            Cs::LchAnsi => write!(f, "LchAnsi"),
        }
    }
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
