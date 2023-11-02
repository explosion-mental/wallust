//! # dark16
use crate::filters::*;

/// **tldr; darkens the first row a bit.**
/// The first row (color 0 - 7) is darker, and the later (color 8 - 15) are left alone.
/// This is to make constranst between those (they got the same hue).
/// Sorted by [`LightFirst`]
/// Variation of dark with 16 colors. From <https://github.com/eylles/pywal16>
pub fn dark16(c: &[Myrgb]) -> Colors {
    let mut c = super::dark::dark(c);

    c.color1 = c.color1.darken(0.25);
    c.color2 = c.color2.darken(0.25);
    c.color3 = c.color3.darken(0.25);
    c.color4 = c.color4.darken(0.25);
    c.color5 = c.color5.darken(0.25);
    c.color6 = c.color6.darken(0.25);

    c
}
