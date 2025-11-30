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

/// The Salience struct
#[derive(Debug)]
pub struct Salience;

// pub type Spec = palette::cam16::Cam16Jmh<f32>;
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Spec(Cam16UcsJmh<f32>);

/// Simple shadow to avoid repetition
pub type Hist = Histo<Spec>;

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

/// Bake viewing conditions for Cam16 so it doesn't re-compute constants every
/// time a color is converted to and from the space (which happens a lot).
/// ~10% faster
pub static CAM16_VIEW: OnceLock<BakedParameters<StaticWp<D65>, f32>> = OnceLock::new();

impl BuildHisto<Spec> for Salience {
    /// Like provided implementation, but also sets CAM16_VIEW
    fn init(bytes: &[u8], threshold: u8, mix: bool, ord: &ColorOrder) -> Option<Vec<Histo<Spec>>> {
        if CAM16_VIEW.set(init_view(ord)).is_err() {}; // ignore if already set

        let b = Self::read(bytes);
        let ret = Self::gather_cols(b, threshold, mix);
        if ret.len() < 2 { None } else { Some(ret) }
    }

    /// Once deuduped, we want to sort the colors in a way that such that after
    /// truncating, we keep the colors we want. What colors do we want?
    ///
    /// We'll define promimancy as the count of a color in relation to the
    /// image size. A color is prominent if it appears so-so many times, but
    /// after some count, beyond prominent has little-to-no meaning.
    ///
    /// We also use saliency against static colors DARKEST_COL or LIGHTEST_COL
    /// depending on context. Because we don't know our final background yet,
    /// we will only take into account l and c.
    ///
    /// Finally, we take the lowest scored color PROMINENT color and have it
    /// as the last element AFTER truncation. We will use this as the
    /// background color on palette generation.
    ///
    /// Later, we can include the saliency map and include those values into
    /// the score.
    fn process_deduped(histo: Vec<Histo<Spec>>, ord: &ColorOrder, _bytes: &[u8])-> Vec<Histo<Spec>> {
        // Make the most prominent (most count) color as the background
        // i.e. if blue self.themed wallpaper, then blue theme!
        //
        // We do special handling later to ensure that even after truncating,
        // this color is registered as the lowest, which will then be picked
        // by the palette for background processing.
        //
        // DO NOT TRANSFORM THE BACKGROUND HERE!
        let mut histo = histo;
        let (idx, _) = histo.iter().enumerate().max_by_key(|(_, item)| item.count).unwrap();
        let max_histo = histo.remove(idx);
        let max_histo_og = max_histo;

        let bg = max_histo.color;
        let cams: Vec<Spec> = histo.iter().map(|h| h.color).collect();
        let bg = constrain_col_as_bg(bg, &cams, ord);
        let res = constrain_col_against_cols(bg, &cams, ord, &[get_bg_min_sal(ord)]);
        let bg = res[0];

        histo = sort_histogram_by_score(histo, bg);

        // Re-insert the ORIGINAL background color as the least salient color
        // before truncating in the next step (clip_cols)
        //
        // MAX_COLS-1 because we removed the color for sort calcs earlier
        let max_cols = (MAX_COLS-1) as usize;
        let len = histo.len();
        let last = if len > max_cols {max_cols} else {len};
        histo.insert(last, max_histo_og);

        histo
    }

    fn filter_cols(histo: Vec<Spec>) -> Vec<Spec> {
        // We can't use only salience as a filter because there are scenarios
        // where colors will be of equal brightness but of high differences in
        // colorfulness.
        //
        // Colorfulness at low lightness is a bit wonky, as it is only
        // salient-accuracte for specific hues

        let lights = histo.iter().map(|c| c.lightness).collect::<Vec<_>>();
        let darkest  = lights.iter().fold(f32::INFINITY, |a, &b| a.min(b)).max(DARKEST);
        let lightest = lights.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b)).min(LIGHTEST);
        //
        // We don't care about mexchroma, but 0.0 to 1.0 chroma is grayscale like
        // we use lesschroma on monochromatic or similar imgs, so it doesn't error out
        let colorfulnesses = histo.iter().map(|c| c.colorfulness).collect::<Vec<_>>();
        let origcl = util::avg(&colorfulnesses);
        let lesscl  = colorfulnesses.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        let ch = if origcl <= MIN_COLORFULNESS { lesscl } else { origcl / 2.5 };
        //
        let filt = |x: &Spec| {
               x.lightness >= darkest
            && x.lightness <= lightest
            && x.colorfulness >= ch
            && x.improved_salience_naive(&ColorOrder::LightFirst) > COL_DARK_MIN_SAL
            && x.improved_salience_naive(&ColorOrder::DarkFirst)  > COL_LIGHT_MIN_SAL
        };

        histo.into_iter().filter(filt).collect()
    }

    fn clip_cols(histo: Vec<Histo<Spec>>, ord: &ColorOrder) -> Vec<Histo<Spec>> {
        let mut histo = histo;

        // remove anything just a bit brighter/darker than the finalized bg
        let bg_histo = if histo.len() > MAX_COLS.into() {histo.remove(MAX_COLS as usize - 1)} else {histo.pop().expect("not empty")};
        let bg = bg_histo.color;
        let cams: Vec<Spec> = histo.iter().map(|h| h.color).collect();
        let bg = constrain_col_as_bg(bg, &cams, ord);

        let lt_mod = match ord {
            ColorOrder::LightFirst => BG_DARK_ENFORCE_L_DELTA,
            ColorOrder::DarkFirst  => -BG_LIGHT_ENFORCE_L_DELTA,
        };
        let l_threshold = bg.lightness + lt_mod;

        let filt = |h: &Histo<Spec>| match ord {
            ColorOrder::LightFirst => h.color.lightness > l_threshold,
            ColorOrder::DarkFirst  => h.color.lightness < l_threshold
        };

        let mut histo: Vec<Histo<Spec>> = histo.into_iter().filter(filt).collect();

        // Re-insert the ORIGINAL background color as the least salient color
        // before truncating in the next step (clip_cols)
        //
        // MAX_COLS-1 because we removed the color for sort calcs earlier
        let max_cols = (MAX_COLS-1) as usize;
        let len = histo.len();
        let last = if len > max_cols {max_cols} else {len};
        histo.insert(last, bg_histo);


        // remove excess elements
        histo.truncate(MAX_COLS.into());
        histo
    }

    fn sort_col(histo: Vec<Hist>, ord: &ColorOrder) -> Vec<Hist> {
        // after clip_cols
        let mut histo = histo;

        // reserve original bg
        let histo_bg = histo.pop().expect("not empty");
        let histo_bg_og = histo_bg;
        let bg = histo_bg.color;
        let cols: Vec<Spec> = histo.iter().map(|h| h.color).collect();

        // generate bg
        let bg = constrain_col_as_bg(bg, &cols, ord);
        let res = constrain_col_against_cols(bg, &cols, ord, &[get_bg_min_sal(ord)]);
        let bg = *res.first().expect("not empty");

        // and sort
        histo.sort_by(|a, b| a.color.improved_salience(&bg).partial_cmp(&b.color.improved_salience(&bg)).unwrap_or(Ordering::Equal));

        // add back in original bg
        histo.insert(0, histo_bg_og);

        histo
    }

    /// Clone what lch.rs has, maybe consider experimenting with DeltaE?
    fn sort_by_key_fn(a: Hist) -> impl Ord {
        a.color.colorfulness as i32
    }
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

    // let mut view = Parameters::default_static_wp(80.0); // <- adapting luminance
    // view.background_luminance = 0.5;
    // view.surround = Surround::Dim;

    view
}

/// Get Parameters viewing environment for light themes
pub fn get_light_view() -> Parameters<StaticWp<D65>, f32> {
    // Assume light theme user is in a well-lit room with a bright monitor?
    // See `get_dark_view()` for details on Parameters.

    // let mut view = Parameters::default_static_wp(40.0); // <- adapting luminance
    // view.background_luminance = 0.5;
    // view.surround = Surround::Average;

    let mut view = Parameters::default_static_wp(500.0); // <- adapting luminance
    view.background_luminance = 0.8;
    view.surround = Surround::Average;

    view
}

/// Deref Spec to use as Cam16UcsJmh without needing to access inner type
/// For trait implementations, you should still explicitly access inner type
impl Deref for Spec {
    type Target = Cam16UcsJmh<f32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Spec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Spec {
    pub fn new<H: Into<Cam16Hue<f32>>>(lightness: f32, colorfulness: f32, hue: H) -> Self {
        Spec(Cam16UcsJmh::<f32>::new(lightness, colorfulness, hue.into()))
    }

    pub const fn new_const(lightness: f32, colorfulness: f32, hue: Cam16Hue<f32>) -> Self {
        Self(Cam16UcsJmh::<f32>::new_const(lightness, colorfulness, hue))
    }
}

impl ColorTrait for Spec { }

impl Difference for Spec {
    fn col_diff(&self, &a: &Self, threshold: u8) -> bool {
        // Workaround for generating more colors than usual given the usual
        // thresholding values in other colorspaces so sampling works better.
        // Also because Cam16UcsJmh's ImproveDeltaE scales slightly differently.
        let t_sclr = 0.80;
        self.improved_delta_e(a) <= f32::from(threshold) * t_sclr
    }
}

impl DeltaE for Spec {
    type Scalar = f32;

    fn delta_e(self, other: Self) -> Self::Scalar {
        self.0.delta_e(other.0)
    }
}

impl ImprovedDeltaE for Spec {
    fn improved_delta_e(self, other: Self) -> Self::Scalar {
        self.0.improved_delta_e(other.0)
    }
}

pub trait DeltaENaive {
    fn delta_e_naive(self, other: Self) -> f32;
}

impl DeltaENaive for Spec {
    fn delta_e_naive(self, other: Self) -> f32 {
        // Jmh delta, just no h.
        let dl = self.lightness - other.lightness;
        let dm = self.colorfulness - other.colorfulness;
        (dl.powi(2) + dm.powi(2)).powf(0.5)
    }
}

pub trait ImprovedDeltaENaive {
    fn improved_delta_e_naive(self, other: Self) -> f32;
}

impl ImprovedDeltaENaive for Spec {
    fn improved_delta_e_naive(self, other: Self) -> f32 {
        // Jmh improved delta, just no h.
        let dl = (self.lightness - other.lightness).powi(2);
        let dm = (self.colorfulness - other.colorfulness).powi(2);

        // new scalar to roughly match ~1 JND
        // not proved, ai guessed ~1.5-1.55 around there
        1.55 * (dl + dm).powf(0.63 * 0.5)
    }
}

impl IntoColor<Srgb> for Spec {
    fn into_color(self) -> Srgb {
        let view = CAM16_VIEW.get().expect("is set");
        let cam16: Cam16Jmh<f32> = self.0.into_color();
        let xyz = cam16.into_xyz(*view);
        Srgb::from_color(xyz)
    }
}

impl FromColorUnclamped<Srgb> for Spec {
    fn from_color_unclamped(val: Srgb) -> Self {
        // CAM16-UCS from sRGB, or most other color spaces:
        let rgb = val;
        let view = CAM16_VIEW.get().expect("is set");
        let cam16 = Cam16::from_xyz(rgb.into_color(), *view);
        let ucs_from_rgb = Cam16UcsJmh::from_color(cam16);
        Spec(ucs_from_rgb)
    }
}

impl Mix for Spec {
    type Scalar = f32;

    fn mix(self, other: Self, factor: Self::Scalar) -> Self {
        Spec(self.0.mix(other.0, factor))
    }
}

impl Clamp for Spec {
    fn clamp(self) -> Self {
        Spec(self.0.clamp())
    }
}

impl FromColorUnclamped<Rgb<Linear<SrgbEnc>>> for Spec {
    fn from_color_unclamped(val: Rgb<Linear<SrgbEnc>>) -> Self {
        let view = CAM16_VIEW.get().expect("is set");
        let cam16 = Cam16::from_xyz(val.into_color(), *view);
        let ucs_from_rgb = Cam16UcsJmh::from_color(cam16);
        Spec(ucs_from_rgb)
    }
}

impl Lighten for Spec {
    type Scalar = f32;

    fn lighten(self, factor: Self::Scalar) -> Self {
        Self(self.0.lighten(factor))
    }

    fn lighten_fixed(self, amount: Self::Scalar) -> Self {
        Self(self.0.lighten_fixed(amount))
    }
}

impl Desaturate for Spec {
    type Scalar = f32;

    fn desaturate(self, factor: Self::Scalar) -> Self {
        Self(self.0.desaturate(factor))
    }

    fn desaturate_fixed(self, amount: Self::Scalar) -> Self {
        Self(self.0.desaturate_fixed(amount))
    }
}

pub fn init_view(ord: &ColorOrder) -> BakedParameters<StaticWp<D65>, f32> {
    let view = match ord {
        ColorOrder::LightFirst => get_dark_view(),
        ColorOrder::DarkFirst => get_light_view()
    };
    view.into()
}

/// Given an object and context of its own type, compares how much it stands
/// out (salience).
///
/// DeltaE scales salience linearly to reality.
///
/// However, this does not take into account the psychological compression.
/// Higher salience actually have diminishing effects. In other words, once
/// something is different, there isn't much beyond being different.
///
/// ImprovedDeltaE takes compression into account to shape the scaling so
/// salience maps linearly to human perception.
///
/// salience and improve_salience use the appropriate DeltaE.
///
/// _naive does NOT take into account hue in salinece calculations.
pub trait Salient {
    fn salience(&self, o: &Self) -> f32;
    fn salience_naive(&self, ord: &ColorOrder) -> f32;
    fn improved_salience(&self, o: &Self) -> f32;
    fn improved_salience_naive(&self, ord: &ColorOrder) -> f32;
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

impl Salient for Spec {
    /// Calculate the physiucal salience between self and another color.
    fn salience(&self, &o: &Self) -> f32 {
        let s = Cam16UcsJab::from_color_unclamped(self.0);
        let o = Cam16UcsJab::from_color_unclamped(o.0);

        let mut dl = s.lightness - o.lightness;
        let mut da = s.a - o.a;
        let mut db = s.b - o.b;

        let (wl, wc) = get_sal_weights();
        dl *= wl;
        da *= wc;
        db *= wc;

        (dl.powi(2) + da.powi(2) + db.powi(2)).sqrt()
    }

    /// Naive does not take into account the hue of the color.
    fn salience_naive(&self, ord: &ColorOrder) -> f32 {
        // Expect calculations to underrepresent, especially in cases where
        // the only difference is hue.
        let s = self.0;
        let o = get_bg_naive(ord);

        let mut dl = s.lightness - o.lightness;
        let mut dc = s.colorfulness - o.colorfulness;

        let (wl, wc) = get_sal_weights();
        dl *= wl;
        dc *= wc;

        (dl.powi(2) + dc.powi(2)).sqrt()
    }

    /// Calculate the perceptual salience between self and another color,
    /// power-scaled to contrast perception.
    ///
    // Coefficients from "Power functions improving the performance of
    // color-difference formulas" by Huang et al.
    // https://opg.optica.org/oe/fulltext.cfm?uri=oe-23-1-597&id=307643
    fn improved_salience(&self, &o: &Self) -> f32 {
        // 1.41 * self.salience(&o).powf(0.63) // already sqrt
        1.67 * self.salience(&o).powf(0.64) // already sqrt
    }

    /// Naive does not take into account the hue of the color.
    fn improved_salience_naive(&self, ord: &ColorOrder) -> f32 {
        // 1.55 * self.salience_naive(ord).powf(0.64) // already sqrt
        1.79 * self.salience_naive(ord).powf(0.64) // already sqrt
    }
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
    cols.sort_by(|a, b| a.improved_salience(&context).partial_cmp(&b.improved_salience(&context)).unwrap_or(Ordering::Equal));
    let least_sal = *cols.first().expect("not empty");

    // Define how we sort
    let dec_sal_l = get_dec_sal_l_fn(ord);
    let mut mod_and_sort = |mut ctx, mut other: Spec, t: f32, is_initial_bg: bool| -> Spec {
        let dl_enforce = match ord {
            ColorOrder::LightFirst => BG_DARK_ENFORCE_L_DELTA,
            ColorOrder::DarkFirst  => BG_LIGHT_ENFORCE_L_DELTA,
        };
        while other.improved_salience(&ctx) < t || ((ctx.lightness - other.lightness).abs() < dl_enforce && is_initial_bg) {
            // ctx = ctx.desaturate(0.05); // prefer darken over desaturate
            ctx = dec_sal_l(ctx, 0.05);
            ctx = ctx.clamp();

            if ctx.lightness < BG_DARK_L_SOFTMIN || ctx.lightness > BG_LIGHT_L_SOFTMAX {break};

            // This continuous sort needs to be done if salience calculations
            // differ would result in different sorting as a result of ctx alterations.
            cols.sort_by(|a, b| a.improved_salience(&ctx).partial_cmp(&b.improved_salience(&ctx)).unwrap_or(Ordering::Equal));
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
