//! Histogram
//!
//! This module purpuse is to gather the colors into a Histogram that can later be sampled.
//! There are two main methods, one that uses salience and other that is
//! TODO contrast

use palette::Srgb;

use crate::colorspaces::ColorOrder;

use self::luminance::Luminance;

mod salience;
mod luminance;


// pub enum Histos {
//     Salience(Vec<salience::Histo>),
//     Luminance(Vec<luminance::Histo>),
// }
//
// /// element for different sortings?
// pub struct Histogram {
//     histo: Histos,
//     threshold: u8,
//     ord: ColorOrder,
// }

pub enum DiffMode {
    DeltaE,
    ImprovedDeltaE,
}

pub trait Difference {
    fn diff(&self, a: &Self, threshold: f32, mode: &DiffMode) -> bool;
}

pub trait Build
where
    Self: Sized
{
    fn new(threshold: f32, ord: ColorOrder, mode: DiffMode, skip: bool) -> Self;

    /// This just convert readed bytes into a pre process histo
    /// Always runs, before the backend it should be known if the file is empty (no pixels/colors),
    /// later, in gather colors, we can analize if it sufice the palette.
    fn read_bytes(&mut self, bytes: &[u8]);

    /// dedup
    fn dedup(&mut self);

    /// before truncating
    fn post_processing(&mut self);

    /// less than 16 colors, depending on the configs.
    fn trunc(&mut self);
}

pub enum Mode {
    Luminance,
    Salience,
}

/// WE NEED A RETURN TYPE!! (js return Srgb for now)
pub fn gen_histo(bytes: &[u8], threshold: f32, ord: ColorOrder, skip: bool, mode: &Mode, diffmode: DiffMode) -> Vec<Srgb> {


    // 1. choose type depending on mode. TODO another func to determine mode
    // should be inside colorspaces mod
    let mut histo = match mode {
        Mode::Luminance => Luminance::new(threshold, ord, diffmode, skip),
        Mode::Salience => todo!(),
    };

    // 2. transform to specs
    histo.read_bytes(bytes);

    // 3. dedup
    histo.dedup();

    //4. post procesing
    histo.post_processing();

    todo!();
}











