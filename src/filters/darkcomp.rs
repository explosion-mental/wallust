//! # dark16
use crate::filters::*;

/// **tldr; darkens the first row a bit.**
/// The first row (color 0 - 7) is darker, and the later (color 8 - 15) are left alone.
/// This is to make constranst between those (they got the same hue).
/// Sorted by [`LightFirst`]
/// Variation of dark with 16 colors. From <https://github.com/eylles/pywal16>
pub fn darkcomp(c: Cols) -> Colors {
    let mut r = super::dark::dark(c);

    r.color9  = r.color1.saturate(0.3).complementary();
    r.color10 = r.color2.saturate(0.3).complementary();
    r.color11 = r.color3.saturate(0.3).complementary();
    r.color12 = r.color4.saturate(0.3).complementary();
    r.color13 = r.color5.saturate(0.3).complementary();
    r.color14 = r.color6.saturate(0.3).complementary();

    r
}
