//! # dark16
use super::*;

/// **tldr; darkens the first row a bit.**
/// The first row (color 0 - 7) is darker, and the later (color 8 - 15) are left alone.
/// This is to make constranst between those (they got the same hue).
/// Sorted by [`LightFirst`]
/// Variation of dark with 16 colors. From <https://github.com/eylles/pywal16>
pub fn dark16(c: Cols) -> Colors {
    super::dark::dark(c).to_16col()
}
