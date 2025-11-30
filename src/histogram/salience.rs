//! # Salience
//! Salience using Cam16UcsJmh with DeltaE and ImprovedDeltaE.
//! Color Picker: https://apps.colorjs.io/picker/cam16-jmh
//!
//! Instead of luminance and chroma, Cam16UcsJmh uses lightness and
//! colorfulness.
//!
//! For other color spaces, light and color are mapped to physics rather than
//! perception (luminance and chroma). This results in a perceptual coupling
//! of light and color; if force a color into high/low luminance, the viewed
//! color is not accurate to human intentions. Lightness and colorfulness
//! instead maps linearly to perception, decoupling light and color.
//!
//! Cam16UcsJmh's components result in a much more elegant salience
//! calculations, being the euclidian distance of all components. However,
//! for saliency mapping, lightness still dominates, so not all components
//! can be weighted equally.
//!
//! Cam16 provides Parameters which allows us to set viewing conditions. This
//! will adjust colors to appear as they would under said viewing conditions.
//! May be unwanted for our use case.
//!
//! ref: <https://docs.rs/palette/0.7.6/palette/cam16/struct.Cam16UcsJmh.html>
#![allow(unused_imports)]

use palette::IntoColor;
use palette::cast::ComponentsAs;
use palette::convert::IntoColorUnclamped;

use crate::colorspaces::ColorOrder;

use super::DiffMode;


use super::*;
use std::sync::OnceLock;
use std::cmp::Ordering;
use std::ops::{Deref, DerefMut};

use palette::{
    Lighten,
    Darken,
    Desaturate,
    Mix,
    Clamp,
    FromColor,
    color_difference::{DeltaE, ImprovedDeltaE},
    cam16::{
        Cam16,
        Cam16Jmh,
        Cam16UcsJab,
        Cam16UcsJmh,
        BakedParameters,
        Parameters,
        StaticWp,
        Surround,
    },
    hues::Cam16Hue,
    convert::FromColorUnclamped,
    white_point::D65,
    rgb::Rgb,
    encoding::{
        Srgb as SrgbEnc,
        Linear
    }
};


/// Our working types
type Spec = Cam16UcsJmh<f32>;
type Specs = Vec<Spec>;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Histo {
    color: Spec,
    count: u32,
    score: f32,
}

pub struct Salience {
    histo: Vec<Histo>,
    threshold: f32,
    ord: ColorOrder,
    mode: DiffMode,
    skip: bool,
    view: OnceLock<BakedParameters<StaticWp<D65>, f32>>,
}

/// Constraints for a color to be accepted and gathered
/// This will not alter the background, only the colors selected
pub const DARKEST: f32 = 5.0;
pub const LIGHTEST: f32 = 95.0;
pub const MIN_COLORFULNESS: f32 = 10.0;
pub const COL_DARK_MIN_SAL: f32 = 7.5;
pub const COL_LIGHT_MIN_SAL: f32 = 15.0;
pub const BG_DARK_ENFORCE_L_DELTA: f32 = 5.0;
pub const BG_LIGHT_ENFORCE_L_DELTA: f32 = 10.0;

// Constraints for background generation
// This will alter only the background (probably)
pub const BG_DARK_MIN_SAL: f32 = 7.5;
pub const BG_LIGHT_MIN_SAL: f32 = 15.0;
pub const BG_DARK_L_SOFTMIN: f32 = 2.0;
pub const BG_DARK_L_SOFTMAX: f32 = 5.0;
pub const BG_LIGHT_L_SOFTMIN: f32 = 90.0;
pub const BG_LIGHT_L_SOFTMAX: f32 = 95.0;
pub const BG_COLORFULNESS_MIN: f32 = 2.5;
pub const BG_COLORFULNESS_MAX: f32 = 7.5;

/// Hue definitions for color temperature
pub const WARM: f32 = 30.0;
pub const COOL: f32 = 210.0;

// Upper bounds in colorspace components.
pub const L_BND: f32 = 100.0;
pub const C_BND: f32 = 128.0;

/// Default bg's for dark/light
pub const DARKEST_COL: Spec = Spec::new_const(DARKEST, MIN_COLORFULNESS, Cam16Hue::new(210.0));
pub const LIGHTEST_COL: Spec = Spec::new_const(LIGHTEST, MIN_COLORFULNESS, Cam16Hue::new(30.0));


// pub static CAM16_VIEW: OnceLock<BakedParameters<StaticWp<D65>, f32>> = OnceLock::new();

impl Difference for Spec {
    fn diff(&self, a: &Self, threshold: f32, mode: &DiffMode) -> bool {
        let ret = match mode {
            DiffMode::DeltaE => self.delta_e(*a),
            DiffMode::ImprovedDeltaE => self.improved_delta_e(*a),
        };

        ret <= threshold
    }
}

fn improved_salience(a: &Spec, b: &Spec) -> f32 {
    1.67 * salience(a, b).powf(0.64)
}


/// Calculate the physiucal salience between self and another color.
fn salience(a: &Spec, b: &Spec) -> f32 {
    let a = Cam16UcsJab::from_color_unclamped(*a);
    let b = Cam16UcsJab::from_color_unclamped(*b);

    let mut dl = a.lightness - b.lightness;
    let mut da = a.a - b.a;
    let mut db = a.b - b.b;

    let (wl, wc) = get_sal_weights();
    dl *= wl;
    da *= wc;
    db *= wc;

    (dl.powi(2) + da.powi(2) + db.powi(2)).sqrt()
}

impl Build for Salience {
    fn new(threshold: f32, ord: ColorOrder, mode: DiffMode, skip: bool) -> Self {
        Self {
            histo: vec![],
            view: OnceLock::new(),
            threshold, ord, mode, skip,
        }
    }

    fn read_bytes(&mut self, bytes: &[u8]) {
        if self.view.set(init_view(&self.ord)).is_err() {}; // ignore if already set
        let s: &[Srgb<u8>] = bytes.components_as();
        let colors = s
            .iter()
            .map(|x| {
                let to = Cam16::from_xyz(x.into_linear().into_color(), *self.view.get().expect("SHOULD BE SET"));
                Cam16UcsJmh::from_color(to)
            })
            .collect::<Specs>();


        // gather
        'outter: for c in colors {
            // Check if whether the color is new or is already in the vec
            for hist in &mut self.histo {
                // if any color is between a threshold, count it up
                if c.diff(&hist.color, self.threshold, &self.mode) {
                    // if mix { hist.color = hist.color.mix(c, 0.5); } // XXX Mix???
                    hist.count += 1;
                    continue 'outter;
                }
            }
            // if we reach here, the color hasn't been found in the histrogram,
            // so we found a new color.
            self.histo.push(Histo { color: c, count: 1, score: 0.0 });
        }
    }

    /// TODO how effective is this approach? I've tested this with lab previously, see
    /// colorspaces::dedup_cols
    fn dedup(&mut self) {
        // self.histo.sort_by_key(|a| a.color.chroma as i32);
        // self.histo
        //     .iter_mut()
        //     .dedup_by_with_count(|a, b| a.color.diff(&b.color, self.threshold, &self.mode))
        //     .for_each(|x| x.1.count += x.0);
    }

    fn post_processing(&mut self) {
        // Make the most prominent (most count) color as the background
        // i.e. if blue themed wallpaper, then blue theme!
        //
        // We do special handling later to ensure that even after truncating,
        // this color is registered as the lowest, which will then be picked
        // by the palette for background processing.
        //
        // DO NOT TRANSFORM THE BACKGROUND HERE!
        let (idx, _) = self.histo.iter().enumerate().max_by_key(|(_, item)| item.count).unwrap();
        let max_histo = self.histo.remove(idx);
        let max_histo_og = max_histo;

        let bg = max_histo.color;
        let cams: Vec<Spec> = self.histo.iter().map(|h| h.color).collect();
        let bg = constrain_col_as_bg(bg, &cams, &self.ord);
        let res = constrain_col_against_cols(bg, &cams, &self.ord, &[get_bg_min_sal(&self.ord)]);
        let bg = res[0];

        sort_histogram_by_score(self, bg); //in place change

        // Re-insert the ORIGINAL background color as the least salient color
        // before truncating in the next step (clip_cols)
        //
        // MAX_COLS-1 because we removed the color for sort calcs earlier
        let max_cols = (MAX_COLS-1) as usize;
        let len = self.histo.len();
        let last = if len > max_cols {max_cols} else {len};
        self.histo.insert(last, max_histo_og);
    }

    fn trunc(&mut self) {
        self.histo.truncate(16);
    }
}

pub fn init_view(ord: &ColorOrder) -> BakedParameters<StaticWp<D65>, f32> {
    let view = match ord {
        ColorOrder::LightFirst => get_dark_view(),
        ColorOrder::DarkFirst => get_light_view()
    };
    view.into()
}



/// Return a histogram sorted asc on score (count and salience) based on bg
pub fn sort_histogram_by_score(histo: &mut Salience, bg: Spec) {
    let histoscore = &mut histo.histo;
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
        let score_log_scaled = (hs.count as f32).powf(1.0/cnt_sclr);
        let col = hs.color;
        hs.score = score_log_scaled * (1.0 + improved_salience(&col, &bg)*sal_factor);
    });
    histoscore.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Greater));
}


/// Turn the color into a sane background given a collection of planned colors to use
pub fn constrain_col_as_bg(col: Spec, cams: &[Spec], ord: &ColorOrder) -> Spec {
    let mut bg = col;
    let least_sal: &Spec = cams.first().expect("not empty");
    bg.lightness = match ord {
        ColorOrder::LightFirst => least_sal.lightness.max(bg.lightness).clamp(BG_DARK_L_SOFTMIN, BG_DARK_L_SOFTMAX),
        ColorOrder::DarkFirst  => least_sal.lightness.min(bg.lightness).clamp(BG_LIGHT_L_SOFTMIN, BG_LIGHT_L_SOFTMAX)
    };
    bg.colorfulness = least_sal.colorfulness.min(bg.colorfulness).clamp(BG_COLORFULNESS_MIN, BG_COLORFULNESS_MAX);
    bg.clamp()
}

/// Return a color that is like `col` but is constrained to the minimum
/// salient and delta lightness `cols`.
///
/// You can use the returned color as the "true" background, and then sort
/// a collection of colors against said color against salience. Sometimes
/// sorting may change as you alter the background, so this is to /finalize/
/// the background and then sort.
///
/// If you're using Vec<Histo> or Vec<Histoscore>, map it flat as Vec<Spec>,
/// then use the `bg` returned here to sort your Vec.
///
/// For thresholds, pass in a size `n` vector to get a size `n` vector back,
/// whose first element satisfies against the least salient colors in `cols`,
/// of `thresholds[0]`, and whose `i`th element satisfies `thresholds[i]` for
/// return vector [i-1].
///
/// In other words, [0.5, 0.1] means...
///     the least salient color is at least 0.5 salient against ret[0]
///     ret[1] is at least 0.1 salient against ret[0]
///     ...
///     ret[n] is at least thresholds[n] salient against ret[n-1]
///
/// When we reach pure black, all remaining elements will be black.
pub fn constrain_col_against_cols(context: Spec, cols: &[Spec], ord: &ColorOrder, thresholds: &[f32]) -> Vec<Spec> {
    if thresholds.is_empty() { return vec!(cols[0]) }

    let mut thresholds = thresholds.to_vec();
    thresholds.reverse();
    let mut cols = cols.to_vec();

    // Initial sorting against context to get least salient color
    cols.sort_by(|a, b| improved_salience(&a, &context).partial_cmp(&improved_salience(&b, &context)).unwrap_or(Ordering::Equal));
    let least_sal = *cols.first().expect("not empty");

    // Define how we sort
    let dec_sal_l = get_dec_sal_l_fn(ord);
    let mut mod_and_sort = |mut ctx, mut other: Spec, t: f32, is_initial_bg: bool| -> Spec {
        let dl_enforce = match ord {
            ColorOrder::LightFirst => BG_DARK_ENFORCE_L_DELTA,
            ColorOrder::DarkFirst  => BG_LIGHT_ENFORCE_L_DELTA,
        };
        while improved_salience(&other, &ctx) < t || ((ctx.lightness - other.lightness).abs() < dl_enforce && is_initial_bg) {
            // ctx = ctx.desaturate(0.05); // prefer darken over desaturate
            ctx = dec_sal_l(ctx, 0.05);
            ctx = ctx.clamp();

            if ctx.lightness < BG_DARK_L_SOFTMIN || ctx.lightness > BG_LIGHT_L_SOFTMAX {break};

            // This continuous sort needs to be done if salience calculations
            // differ would result in different sorting as a result of ctx alterations.
            cols.sort_by(|a, b| improved_salience(&a, &ctx).partial_cmp(&improved_salience(&b, &ctx)).unwrap_or(Ordering::Equal));
            if is_initial_bg { other = *cols.first().expect("not empty") };
        };
        ctx
    };

    let mut ret: Vec<Spec> = Vec::new();
    let mut threshold = thresholds.pop().expect("not empty");

    // Tune ctx against lowest salient color (or ctx) and continuously sort
    let context = mod_and_sort(context, least_sal, threshold, true);
    ret.push(context);
    while !thresholds.is_empty() {
        threshold = thresholds.pop().expect("not empty");
        ret.push(mod_and_sort(context, context, threshold, false))
    }

    ret.reverse();

    ret
}


/// Get Parameters viewing environment for dark themes
pub fn get_dark_view() -> Parameters<StaticWp<D65>, f32> {
    // Assume dark theme user is in a dimly-lit room with a no-so-blinding monitor?
    //
    // Adapting Luminance: overall room brightness
    //      Night Dark room                5 ====> 20
    //      Dimly lit room/movie theater  20 ====> 50
    //      Office/typical indoor         80 ===> 200
    //      Bright office/sunny indoor   200 ===> 500
    //      Outdoor shade/cloudy day     500 ==> 1000
    //      Direct sunlight             2000 => 10000
    // Background Luminance: background
    //      % to full white, best guess per theme
    // Surround: ambient lighting at edges of FOV

    let mut view = Parameters::default_static_wp(140.0); // <- adapting luminance
    view.background_luminance = 0.2;
    view.surround = Surround::Average;
    view
}

/// Get Parameters viewing environment for light themes
pub fn get_light_view() -> Parameters<StaticWp<D65>, f32> {
    let mut view = Parameters::default_static_wp(500.0); // <- adapting luminance
    view.background_luminance = 0.8;
    view.surround = Surround::Average;

    view
}


pub fn get_bg_naive(ord: &ColorOrder) -> Spec {
    match ord {
        ColorOrder::LightFirst => DARKEST_COL,
        ColorOrder::DarkFirst  => LIGHTEST_COL,
    }
}

pub fn get_lightness_bound(ord: &ColorOrder) -> f32 {
    match ord {
        ColorOrder::LightFirst => DARKEST,
        ColorOrder::DarkFirst  => LIGHTEST,
    }
}

pub fn get_bg_min_sal(ord: &ColorOrder) -> f32 {
    match ord {
        ColorOrder::LightFirst => BG_DARK_MIN_SAL,
        ColorOrder::DarkFirst  => BG_LIGHT_MIN_SAL,
    }
}


fn get_sal_weights() -> (f32, f32) {
    // lightness:color ratio

    // // 1:16
    // let wl = 1.0;
    // let wc = 8.0;

    // // 1:8
    // let wl = 1.0;
    // let wc = 4.0;

    // // 1:6
    // let wl = 1.0;
    // let wc = 3.0;

    // // 1:5
    // let wl = 1.0;
    // let wc = 2.5;

    // // 1:4
    // let wl = 1.0;
    // let wc = 2.0;

    // 1:3
    let wl: f32 = 1.0;   // weight on light
    let wc: f32 = 1.50;  // weight on color (a and b)

    // // 1:2
    // let wl: f32 = 1.0;   // weight on light
    // let wc: f32 = 1.00;  // weight on color (a and b)

    // // 1:1
    // let wl: f32 = 1.5;   // weight on light
    // let wc: f32 = 0.75;  // weight on color (a and b)

    // // 2:1
    // let wl: f32 = 1.2;   // weight on light
    // let wc: f32 = 0.50;  // weight on color (a and b)


    let norm = normalize_to_sum(&[wl, wc, wc], 3.0);
    (norm[0], norm[1])
}


fn normalize_to_sum(slice: &[f32], target_sum: f32) -> Vec<f32> {
    let current_sum: f32 = slice.iter().sum();
    if current_sum == 0.0 {
        // Distribute target_sum evenly if original sum is 0
        let n = slice.len() as f32;
        return vec![target_sum / n; slice.len()];
    }

    slice.iter().map(|&x| x * target_sum / current_sum).collect()
}

pub fn get_dec_sal_l_fn(ord: &ColorOrder) -> fn(Spec, f32) -> Spec {
    match ord {
        ColorOrder::LightFirst => Spec::darken,
        ColorOrder::DarkFirst  => Spec::lighten,
    }
}

pub fn get_inc_sal_l_fn(ord: &ColorOrder) -> fn(Spec, f32) -> Spec {
    match ord {
        ColorOrder::LightFirst => Spec::lighten,
        ColorOrder::DarkFirst  => Spec::darken,
    }
}
