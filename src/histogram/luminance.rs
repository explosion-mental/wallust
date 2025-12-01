//! Sort Histogram by prevalence
//!
//! No fallbacks, since our goal is to garantee a good enough contrast

use palette::cast::ComponentsAs;
use palette::{IntoColor, Lch, Srgb};

use crate::colorspaces::ColorOrder;

use super::{Build, DiffMode, Difference};

// pub struct Luminance;
type Spec = Lch;
type Specs = Vec<Lch>;

pub struct Histo {
    color: Spec,
    count: usize,
}

pub struct Luminance {
    histo: Vec<Histo>,
    threshold: f32,
    ord: ColorOrder,
    mode: DiffMode,
    /// Some backends can already process data (like convert and kmeans)
    skip: bool,
}

use palette::color_difference::{DeltaE, ImprovedCiede2000};

impl Difference for Spec {
    fn diff(&self, a: &Self, threshold: f32, mode: &DiffMode) -> bool {
        let ret = match mode {
            DiffMode::DeltaE => self.delta_e(*a),
            DiffMode::ImprovedDeltaE => self.improved_difference(*a),
        };

        ret <= threshold
    }
}

use itertools::Itertools;

/// another option: `fn ... (vec<>) -> vec<>` which removes the histogram from the type (Luminance)
/// and passes around as a value, maintaining configs as the 'owner' type (avoids &mut self)
impl Build for Luminance {
    fn new(threshold: f32, ord: ColorOrder, mode: DiffMode, skip: bool) -> Self {
        Self {
            histo: vec![],
            threshold, ord, mode, skip,
        }
    }

    fn read_bytes(&mut self, bytes: &[u8]) {
        let s: &[Srgb<u8>] = bytes.components_as();
        let colors = s
            .iter()
            .map(|x| x.into_linear().into_color())
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
            self.histo.push(Histo { color: c, count: 1 });
        }
    }

    fn post_read(&mut self) {
    }

    /// TODO how effective is this approach? I've tested this with lab previously, see
    /// colorspaces::dedup_cols
    fn dedup(&mut self) {
        self.histo.sort_by_key(|a| a.color.chroma as i32);
        self.histo
            .iter_mut()
            .dedup_by_with_count(|a, b| a.color.diff(&b.color, self.threshold, &self.mode))
            .for_each(|x| x.1.count += x.0);
    }

    fn post_dedup(&mut self) {

    }

    fn trunc(&mut self) {
        self.histo.truncate(16);
    }

    fn post_trunc(&mut self) {
    }
}
