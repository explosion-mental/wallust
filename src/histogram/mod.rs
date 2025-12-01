//! Histogram
//!
//! This module purpuse is to gather the colors into a Histogram that can later be sampled.
//! There are two main methods, one that uses salience and other that is
//! TODO contrast

use palette::Srgb;

use crate::colors::Colors;
use crate::colorspaces::ColorOrder;
use crate::palettes::scheme_factory;
use crate::palettes::Scheme;

pub use self::luminance::Luminance;
use self::salience::Salience;

pub mod salience;
pub mod luminance;

/// TODO TRUNCATE?..
pub const MAX_COLS: u8 = 16;

pub enum DiffMode {
    DeltaE,
    ImprovedDeltaE,
}

pub enum Mode {
    Luminance,
    Salience,
}


pub trait Difference {
    fn diff(&self, a: &Self, threshold: f32, mode: &DiffMode) -> bool;
}


pub fn histo_factory(mode: &Mode, threshold: f32, ord: ColorOrder, skip: bool) -> Box<dyn Build> {
    match mode {
        Mode::Luminance => Box::new(Luminance::new(threshold, ord, DiffMode::DeltaE, skip)),
        Mode::Salience => Box::new(Salience::new(threshold, ord, DiffMode::DeltaE, skip)),
    }
}

pub trait Build
where
{
    // fn new(threshold: f32, ord: ColorOrder, mode: DiffMode, skip: bool) -> Self;

    /// This just convert readed bytes into a pre process histo
    /// Always runs, before the backend it should be known if the file is empty (no pixels/colors),
    /// later, in gather colors, we can analize if it sufice the palette.
    fn read_bytes(&mut self, bytes: &[u8]);

    /// Before dedup, after read. Filter could be useful here.
    fn post_read(&mut self);

    /// dedup
    fn dedup(&mut self);

    /// before truncating
    fn post_dedup(&mut self);

    /// less than 16 colors, depending on the configs.
    fn trunc(&mut self);

    /// After truncation, sorting goes here, but not exclusively
    fn post_trunc(&mut self);

    fn gen_palette(&mut self, scheme: Scheme) -> Colors {
        scheme_factory(scheme).colors()
    }

    // Helpers..
    fn to_luminance(self) -> Luminance;
    fn to_salience(self) -> Salience;

    // Alternative to the above
    fn dark(&self) -> Colors {
        todo!()
    }
}

/// Everything happens here.
pub fn gen_histo(bytes: &[u8], threshold: f32, ord: ColorOrder, skip: bool, mode: &Mode, diffmode: DiffMode, scheme: Scheme) -> Colors {
    // 1. choose type depending on mode. TODO another func to determine mode
    // should be inside colorspaces mod
    let mut histo = histo_factory(mode, threshold, ord, skip);

    // 2. transform to specs
    histo.read_bytes(bytes);

    // extra, post reading
    histo.post_read();

    // 2. dedup
    histo.dedup();

    // extra, another post procesing
    histo.post_dedup();

    // 3. Truncate
    histo.trunc();

    // extra, another post processing
    histo.post_trunc();

    // Final Step, generate Colors
    histo.gen_palette(scheme)
}
