//! # Colors logic
//! Here [`Colors`] and [`Myrgb`] types are defined. These are simple enough used by backends,
//! colorspace and palettes modules as a reference, rather than to keep using `Vec<u8>`. This way
//! the base has more structure (also because it's only 16 colors).
use std::fmt;
use std::path::Path;

use anyhow::Result;
use owo_colors::{OwoColorize, Rgb};
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use palette::{
    color_theory::Complementary,
    convert::FromColorUnclamped,
    Hsv, Srgb, IntoColor,
};

use crate::args::Sequences;
use crate::sequences;
/// This is how the scheme it's organized, the `cursor` field it's the same as the foreground (only
/// put to be compatible with pywal)
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Colors {
    pub cursor: Myrgb,
    pub background: Myrgb,
    pub foreground: Myrgb,
    pub color0 : Myrgb,
    pub color1 : Myrgb,
    pub color2 : Myrgb,
    pub color3 : Myrgb,
    pub color4 : Myrgb,
    pub color5 : Myrgb,
    pub color6 : Myrgb,
    pub color7 : Myrgb,
    pub color8 : Myrgb,
    pub color9 : Myrgb,
    pub color10: Myrgb,
    pub color11: Myrgb,
    pub color12: Myrgb,
    pub color13: Myrgb,
    pub color14: Myrgb,
    pub color15: Myrgb,
}

/// Custom RGB type wrapper that works for compatibility (either by working with other crates,
/// since most of them include their own `RGB` type) and by including methods for convertion and
/// modification to the color. Every backend should return `Myrgb`.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Myrgb(pub Srgb);

impl Serialize for Myrgb {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}
use serde::de::{Visitor, Error};
impl<'de> Deserialize<'de> for Myrgb {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RgbVisitor;

        impl<'de> Visitor<'de> for RgbVisitor {
            type Value = Myrgb;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing an RGB value")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                // don't error out on RRGGBBAA values, just ignore the alpha
                let value = if value.len() == 8 || value.len() == 9 { &value[..value.len() - 2] } else { value };

                let s: Srgb<u8> = value.parse()
                    .map_err(Error::custom)?;

                Ok(Myrgb(s.into_format()))
            }
        }

        deserializer.deserialize_str(RgbVisitor)
    }
}


/// Display [`Myrgb`] like hex (e.g. `(238, 238, 238)` as `#EEEEEE`)
impl fmt::Display for Myrgb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (r, g, b) = self.0.into_format::<u8>().into_components();
        write!(f, "#{r:02X}{g:02X}{b:02X}", )
    }
}

pub trait Compl: palette::Clamp + Sized + FromColorUnclamped<Hsv>
where
    Hsv: FromColorUnclamped<Self>,
{
    fn complementary(self) -> Self {
        let hsv: Hsv = self.into_color();
        hsv
            .complementary()
            .into_color()
    }
}

impl Compl for Srgb {}
impl Compl for palette::Srgba {}

/// My blending, not sure what technical name it has (TODO)
/// gathered this from pywal.
pub fn blend(a: Srgb, b: Srgb) -> Srgb {
    Srgb::new(
        0.5 * a.red   + 0.5 * b.red,
        0.5 * a.green + 0.5 * b.green,
        0.5 * a.blue  + 0.5 * b.blue,
    )
}

use palette::Srgba;
pub fn blend_alpha(a: Srgba, b: Srgba) -> Srgba {
    Srgba::new(
        0.5 * a.red   + 0.5 * b.red,
        0.5 * a.green + 0.5 * b.green,
        0.5 * a.blue  + 0.5 * b.blue,
        0.5 * a.alpha + 0.5 * b.alpha,
    )
}

//default way of representing an SRGB for the palette crate
pub trait SrgbString {
    fn strsrgb(&self) -> String;
    fn striped(&self) -> String;
    fn owo_col(&self) -> Rgb;
}

/// Display [`Myrgb`] like hex (e.g. `(238, 238, 238)` as `#EEEEEE`)
impl SrgbString for Srgb {
    fn strsrgb(&self) -> String {
        let (r, g, b) = self.into_format::<u8>().into_components();
        format!("#{r:02X}{g:02X}{b:02X}")
    }

    fn striped(&self) -> String {
        let (r, g, b) = self.into_format::<u8>().into_components();
        format!("{r:02X}{g:02X}{b:02X}")
    }
    fn owo_col(&self) -> Rgb {
        let (r, g, b) = self.into_format::<u8>().into_components();
        Rgb(r, g, b)
    }
}


/// methods for [`Myrgb`] darken and lighten are basically from pywal `util.py` (just 'type safe' :p)
impl Myrgb {
    /// to owo [`Rgb`]
    pub fn owo_col(&self) -> Rgb {
        let (r, g, b) = self.0.into_format::<u8>().into_components();
        Rgb(r, g, b)
    }

    pub fn to_rgb8(self) -> (u8, u8, u8) {
        self.0.into_format::<u8>().into_components()
    }

    /// private fn that returns sequences
    /// "Convert a hex color to a text color sequence"
    fn set_color(&self, index: u32) -> String {
        sequences::set_color(&self.0, index)
    }

    /// Convert a hex color to a special sequence.
    /// Currently no alpha is supported. The sequence below is only supported by urxvt, by pywal
    fn set_special(&self, index: u32) -> String {
        sequences::set_special(&self.0, index)
    }

    /// darkens rgb by amount (lossy)
    pub fn darken(&self, amount: f32) -> Self {
        use palette::Darken;
        Self(self.0.darken(amount))
    }

    /// ligthen rgb by amount (lossy)
    pub fn lighten(&self, amount: f32) -> Self {
        use palette::Lighten;
        Self(self.0.lighten(amount))
    }

    /// see blend from above the file.
    pub fn blend(&self, other: Self) -> Self {
        let me = self.0;
        let other = other.0;
        let new = Srgb::new(
            0.5 * me.red   + 0.5 * other.red,
            0.5 * me.green + 0.5 * other.green,
            0.5 * me.blue  + 0.5 * other.blue,
        );
        Self(new)
    }

    /// saturate the current color by `amount`, which should be between [0.0, 1.0] (inclusive)
    pub fn saturate(&self, amount: f32) -> Self {
        use palette::Saturate;

        //initial
        let a: Hsv = self.0.into_color();
        // saturate is not implemented for rgb
        let rgb: Srgb<f32> = a.saturate(amount).into_color();

        Self(rgb)
    }

    /// Get the complementary color of a color.
    /// Ref:
    /// https://docs.rs/palette/latest/palette/color_theory/trait.Complementary.html
    pub fn complementary(&self) -> Self {
        Self(
            self.0.complementary()
        )
    }
}

impl Colors {
    /// Print the scheme out
    pub fn print(&self) {
        print!(
"
{}{}{}{}{}{}{}{}
{}{}{}{}{}{}{}{}

",
        "    ".on_color(self.color0 .owo_col()),
        "    ".on_color(self.color1 .owo_col()),
        "    ".on_color(self.color2 .owo_col()),
        "    ".on_color(self.color3 .owo_col()),
        "    ".on_color(self.color4 .owo_col()),
        "    ".on_color(self.color5 .owo_col()),
        "    ".on_color(self.color6 .owo_col()),
        "    ".on_color(self.color7 .owo_col()),
        "    ".on_color(self.color8 .owo_col()),
        "    ".on_color(self.color9 .owo_col()),
        "    ".on_color(self.color10.owo_col()),
        "    ".on_color(self.color11.owo_col()),
        "    ".on_color(self.color12.owo_col()),
        "    ".on_color(self.color13.owo_col()),
        "    ".on_color(self.color14.owo_col()),
        "    ".on_color(self.color15.owo_col()),
        );
    }

    /// Fancy `enjoy the palette!` message
    pub fn done(&self) {
        let space = "  ".strikethrough();
        print!(
"
{}{}{}{}{}{space}{}{}{}{space}{}{}{}{}{}{}{}{}
",
        "E ".color(self.color15.owo_col()).bold().blink(),
        "N ".color(self.color14.owo_col()).bold().blink(),
        "J ".color(self.color13.owo_col()).bold().blink(),
        "O ".color(self.color12.owo_col()).bold().blink(),
        "Y ".color(self.color11.owo_col()).bold().blink(),
        "T ".color(self.color10.owo_col()).bold().blink(),
        "H ".color(self.color9 .owo_col()).bold().blink(),
        "E ".color(self.color8 .owo_col()).bold().blink(),
        "P ".color(self.color7 .owo_col()).bold().blink(),
        "A ".color(self.color6 .owo_col()).bold().blink(),
        "L ".color(self.color5 .owo_col()).bold().blink(),
        "E ".color(self.color4 .owo_col()).bold().blink(),
        "T ".color(self.color3 .owo_col()).bold().blink(),
        "T ".color(self.color2 .owo_col()).bold().blink(),
        "E ".color(self.color1 .owo_col()).bold().blink(),
        "! ".color(self.foreground.owo_col()).bold().blink(),
        );
    }

    /// A simple variation that follows the steps below, making the 'ilusion' of "more colors"
    /// * ref1: <https://github.com/dylanaraps/pywal/pull/662>
    /// * ref2: <https://github.com/eylles/pywal16>
    pub fn to_16col(self) -> Self {
        let c = self;
        Self {
            color1: c.color1.darken(0.25),
            color2: c.color2.darken(0.25),
            color3: c.color3.darken(0.25),
            color4: c.color4.darken(0.25),
            color5: c.color5.darken(0.25),
            color6: c.color6.darken(0.25),
            ..c
        }
    }

    /// 'complementary' colors variation.
    /// This variations changes all the colors to it's complementary counterpart.
    pub fn to_comp(self) -> Self {
        let c = self;

        // This version 'flips' to complementary color the second row colors from 9 to 14.
        // Self {
        //     color9  : c.color1.saturate(0.3).complementary(),
        //     color10 : c.color2.saturate(0.3).complementary(),
        //     color11 : c.color3.saturate(0.3).complementary(),
        //     color12 : c.color4.saturate(0.3).complementary(),
        //     color13 : c.color5.saturate(0.3).complementary(),
        //     color14 : c.color6.saturate(0.3).complementary(),
        //     ..c
        // }

        // This version completely flips the whole palette to it's complementary one (allowing to
        // work also with 16 color variation). One annoyance could be sorting, since luminace/hue
        // won't be the same after flipping.
        Self {
            color1  : c.color1.saturate(0.3).complementary(),
            color9  : c.color1.saturate(0.3).complementary(),

            color2  : c.color2.saturate(0.3).complementary(),
            color10 : c.color2.saturate(0.3).complementary(),

            color3  : c.color3.saturate(0.3).complementary(),
            color11 : c.color3.saturate(0.3).complementary(),

            color4  : c.color4.saturate(0.3).complementary(),
            color12 : c.color4.saturate(0.3).complementary(),

            color5  : c.color5.saturate(0.3).complementary(),
            color13 : c.color5.saturate(0.3).complementary(),

            color6  : c.color6.saturate(0.3).complementary(),
            color14 : c.color6.saturate(0.3).complementary(),
            ..c
        }
    }

    /// amount is between 0. and 1
    pub fn saturate_colors(&mut self, amount: f32) {
        if amount > 1.0 && amount.is_sign_negative() { return }
        [
            //&mut self.color0,
            &mut self.color1,
            &mut self.color2,
            &mut self.color3,
            &mut self.color4,
            &mut self.color5,
            &mut self.color6,
            //&mut self.color7,
            //&mut self.color8,
            &mut self.color9,
            &mut self.color10,
            &mut self.color11,
            &mut self.color12,
            &mut self.color13,
            &mut self.color14,
            //&mut self.color15,
        ].map(|i| *i = i.saturate(amount));
    }

    /// Checks whether the foregound and backgroudnd of `[Colors]` contrast good enough.
    /// * from: <https://stackoverflow.com/questions/9733288/how-to-programmatically-calculate-the-contrast-ratio-between-two-colors#9733420>
    /// * updated to: <https://docs.rs/palette/latest/palette/color_difference/trait.Wcag21RelativeContrast.html>
    pub fn contrast_well(a: Myrgb, b: Myrgb) -> bool {
        use palette::color_difference::Wcag21RelativeContrast;

        a.0.has_min_contrast_text(b.0)
    }

    /// Checks the contrast for all colors, pywal seems to ignore color0, color7, color8 and
    /// color15, mainly because or they are too bright or to dark.
    pub fn check_contrast_all(&mut self) {
        let a = [
            //&mut self.color0,
            &mut self.color1,
            &mut self.color2,
            &mut self.color3,
            &mut self.color4,
            &mut self.color5,
            &mut self.color6,
            //&mut self.color7,
            //&mut self.color8,
            &mut self.color9,
            &mut self.color10,
            &mut self.color11,
            &mut self.color12,
            &mut self.color13,
            &mut self.color14,
            //&mut self.color15,
        ];

        let mut i: u32 = 0;
        let mut bg_already_dark = false;

        // 1. loop until it's a good contrast
        // 2. at max, 10 iteration should be good enough, since it will probably cap out to
        //    white/black (avoiding infinite loops; which shouldn't, and hasn't, happen anyway)
        while !Self::contrast_well(self.background, self.foreground) && i < 10 {
            self.background = self.background.darken(0.15);
            self.foreground = self.foreground.lighten(0.15);
            bg_already_dark = true;
            i += 1;
        }

        // do the same with all other colors, except the mentioned above
        // max 5 iteration, otherwise the color usually loses it's saturation
        for col in a {
            i = 0;
            while !Self::contrast_well(self.background, *col) && i < 5 {
                if !bg_already_dark {
                    self.background = self.background.darken(0.15);
                    bg_already_dark = true;
                }
                *col = col.lighten(0.05);
                i += 1;
            }
        }
    }

    /// Return the colors into sequences.
    pub fn to_seq(&self, remove: Option<&[Sequences]>) -> String {
        let c = self;

        let cols = [
            // colors from 0-15
            c.color0 .set_color(0 ),
            c.color1 .set_color(1 ),
            c.color2 .set_color(2 ),
            c.color3 .set_color(3 ),
            c.color4 .set_color(4 ),
            c.color5 .set_color(5 ),
            c.color6 .set_color(6 ),
            c.color7 .set_color(7 ),
            c.color8 .set_color(8 ),
            c.color9 .set_color(9 ),
            c.color10.set_color(10),
            c.color11.set_color(11),
            c.color12.set_color(12),
            c.color13.set_color(13),
            c.color14.set_color(14),
            c.color15.set_color(15),
        ];

        let bg = [
            // special colors, see above the fn
            //backgroud is between 16..=20
            c.background.set_special(11),
            c.background.set_special(19),
            c.background.set_color(232),
            c.background.set_color(257),
            c.background.set_special(708),
        ];


        let fg = [
            //foreground is between 21..=23
            c.foreground.set_special(10),
            c.foreground.set_special(17),
            c.foreground.set_color(256),
        ];

        let cursor = [
            //cursor is between 24..=len()
            c.cursor.set_special(12), //cursor
            c.cursor.set_special(13), //mouse
        ];

        let arr;

        if let Some(seqs) = remove {
            use crate::args::Sequences as Seq;
            use std::collections::HashMap;
            let bg = bg.join("");
            let fg = fg.join("");
            let cursor = cursor.join("");

            let mut h = HashMap::from([
                (Seq::Color0     , &cols[0 ]),
                (Seq::Color1     , &cols[1 ]),
                (Seq::Color2     , &cols[2 ]),
                (Seq::Color3     , &cols[3 ]),
                (Seq::Color4     , &cols[4 ]),
                (Seq::Color5     , &cols[5 ]),
                (Seq::Color6     , &cols[6 ]),
                (Seq::Color7     , &cols[7 ]),
                (Seq::Color8     , &cols[8 ]),
                (Seq::Color9     , &cols[9 ]),
                (Seq::Color10    , &cols[10]),
                (Seq::Color11    , &cols[11]),
                (Seq::Color12    , &cols[12]),
                (Seq::Color13    , &cols[13]),
                (Seq::Color14    , &cols[14]),
                (Seq::Color15    , &cols[15]),
                (Seq::Background , &bg      ),
                (Seq::Foreground , &fg      ),
                (Seq::Cursor     , &cursor  ),
            ]);

            for i in seqs {
                h.remove(i);
            }

            arr = h
                .into_values()
                .map(|x| x.to_owned())
                .collect();
        } else {
            arr = [ cols.join(""), bg.join(""), fg.join(""), cursor.join("")].join("");
        }

        arr
    }

    /// # Sets terminal colors
    /// ANSI escape codes tables and helpful guidelines:
    /// <https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797>
    /// As well as support for iTerm2 (macOS) and windows terminal, depending on the OS.
    pub fn sequences(&self, _cache_path: &Path, _ignore: Option<&[Sequences]>) -> anyhow::Result<()> {
        #[cfg(target_family = "windows")]
        return sequences::windows_term(self);

        #[cfg(target_family = "unix")]
        return sequences::unix_term(self, _cache_path, _ignore);
    }
}

impl From<Srgb> for Myrgb {
    fn from(v: Srgb) -> Myrgb {
        Myrgb(v)
    }
}

impl From<&Srgb> for Myrgb {
    fn from(v: &Srgb) -> Myrgb {
        Myrgb(*v)
    }
}

/// Dummy type to iterate over [`Colors`]
pub struct ColorsIntoIter {
    pub me: Colors,
    pub index: usize,
}

/// Make [`Colors`] possible to `.iter()` into it.
/// The order of the index is simple and will always be:
/// * 0-15 => colors from 0 to 15
/// * 16 => background
/// * 17 => foreground
impl IntoIterator for Colors {
    type Item = Myrgb;
    type IntoIter = ColorsIntoIter;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            me: self,
            index: 0,
        }
    }
}

impl Iterator for ColorsIntoIter {
    type Item = Myrgb;
    fn next(&mut self) -> Option<Myrgb> {
        let result = match self.index {
            0  => self.me.color0,
            1  => self.me.color1,
            2  => self.me.color2,
            3  => self.me.color3,
            4  => self.me.color4,
            5  => self.me.color5,
            6  => self.me.color6,
            7  => self.me.color7,
            8  => self.me.color8,
            9  => self.me.color9,
            10 => self.me.color10,
            11 => self.me.color11,
            12 => self.me.color12,
            13 => self.me.color13,
            14 => self.me.color14,
            15 => self.me.color15,
            16 => self.me.background,
            17 => self.me.foreground,
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}
