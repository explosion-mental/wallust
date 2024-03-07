//! # Colors logic
//! Here [`Colors`] and [`Myrgb`] types are defined. These are simple enough used by backends,
//! colorspace and palettes modules as a reference, rather than to keep using `Vec<u8>`. This way
//! the base has more structure (also because it's only 16 colors).
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use anyhow::Result;
use num_traits::Pow;
use owo_colors::{OwoColorize, Rgb};
use palette::GetHue;
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use palette::{Hsv, Srgb, IntoColor};

use crate::args::Sequences;
/// This is how the scheme it's organized, the `cursor` field it's the same as the foreground (only
/// put to be compatible with pywal)
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Colors {
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
pub struct Myrgb(pub u8, pub u8, pub u8);

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
                if value.len() != 7 || !value.starts_with('#') {
                    return Err(Error::invalid_value(
                        serde::de::Unexpected::Str(value),
                        &self,
                    ));
                }

                let r = u8::from_str_radix(&value[1..3], 16).map_err(Error::custom)?;
                let g = u8::from_str_radix(&value[3..5], 16).map_err(Error::custom)?;
                let b = u8::from_str_radix(&value[5..7], 16).map_err(Error::custom)?;

                Ok(Myrgb(r, g, b))
            }
        }

        deserializer.deserialize_str(RgbVisitor)
    }
}


/// Display [`Myrgb`] like hex (e.g. `(238, 238, 238)` as `#EEEEEE`)
impl fmt::Display for Myrgb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.0, self.1, self.2)
    }
}

/// methods for [`Myrgb`] darken and lighten are basically from pywal `util.py` (just 'type safe' :p)
impl Myrgb {
    /// to owo [`Rgb`]
    pub fn col(&self) -> Rgb {
        Rgb(self.0, self.1, self.2)
    }

    /// darkens rgb by amount (lossy)
    pub fn darken(&self, amount: f32) -> Self {
        Self(
            (f32::from(self.0) * (1.0 - amount)) as u8,
            (f32::from(self.1) * (1.0 - amount)) as u8,
            (f32::from(self.2) * (1.0 - amount)) as u8,
        )
    }

    /// ligthen rgb by amount (lossy)
    pub fn lighten(&self, amount: f32) -> Self {
        Self(
            (f32::from(self.0) + f32::from(255 - self.0) * amount) as u8,
            (f32::from(self.1) + f32::from(255 - self.1) * amount) as u8,
            (f32::from(self.2) + f32::from(255 - self.2) * amount) as u8,
        )
    }

    /// Mix with other [`Myrgb`]
    pub fn blend(&self, other: Self) -> Self {
        Self(
            (0.5 * f32::from(self.0) + 0.5 * f32::from(other.0)) as u8,
            (0.5 * f32::from(self.1) + 0.5 * f32::from(other.1)) as u8,
            (0.5 * f32::from(self.2) + 0.5 * f32::from(other.2)) as u8,

        )
    }

    /// This outputs `235,235,235` as r,g,b
    pub fn rgb(&self) -> String {
        format!("{},{},{}", self.0, self.1, self.2)
    }

    /// .rgba output `235,235,235,1.0`
    pub fn rgba(&self) -> String {
        let alpha = 1.0; //see top of the file for alpha explanation
        format!("rgba({},{},{},{alpha})", self.0, self.1, self.2)
    }

    /// xrgba outputs `ee/ee/ee/ff` as r/g/b/alpha in hex but using `/` as a separator
    pub fn xrgba(&self) -> String {
        format!("{:02x}/{:02x}/{:02x}/ff", self.0, self.1, self.2)
    }

    /// - xrgba outputs `ee/ee/ee/ff` as r/g/b/alpha in hex but using `/` as a separator
    /// - xrgba but without alpha
    /// - alpha is a variable itself, not contained in Colors. so it could be formatted standalone.
    /// (e.g. `{{color0 | xrgb}}{{"/"}}{{alpha_hex}}` )
    pub fn xrgb(&self) -> String {
        format!("{:02x}/{:02x}/{:02x}", self.0, self.1, self.2)
    }

    /// This only "strips" the `#` from the usual output, leaving the following: `EEEEEE`
    pub fn strip(&self) -> String {
        format!("{:02X}{:02X}{:02X}", self.0, self.1, self.2)
    }

    // Red green and blue values as u8s
    // XXX maybe also make red green and blue for hex values?
    pub fn red  (&self) -> String { format!("{}", self.0) }
    pub fn green(&self) -> String { format!("{}", self.1) }
    pub fn blue (&self) -> String { format!("{}", self.2) }

    /// private fn that returns sequences
    /// "Convert a hex color to a text color sequence"
    fn set_color(&self, index: u32) -> String {
        if cfg!(target_os = "macos") && index < 20 {
            return format!("\x1B]P%1x{self}\x1B\\");
        }

        format!("\x1B]4;{index};{self}\x1B\\")
    }

    /// Convert a hex color to a special sequence.
    /// Currently no alpha is supported. The sequence below is only supported by urxvt, by pywal
    fn set_special(&self, index: u32, iterm_name: &str) -> String {
        //let alpha = 100;

        if cfg!(target_os = "macos") && !iterm_name.is_empty() {
            return format!("\x1B]P{iterm_name}{}\x1B\\", self.strip());
        }

        // if (11..=708).contains(&index) && alpha != 100 {
        //     return format!("\x1B]{index};[{alpha}]{self}\x1B\\");
        // }

        format!("\x1B]{index};{self}\x1B\\")
    }

    fn luminance(&self) -> f32 {
        const GAMMA: f32 = 2.4;
        const RED: f32 = 0.2126;
        const GREEN: f32 = 0.7152;
        const BLUE: f32 = 0.0722;

        let a = [
            f32::from(self.0), //r
            f32::from(self.1), //g
            f32::from(self.2), //b
        ].map(|mut x| {
            x /= 255.0;
            if x <= 0.03928 {
                x / 12.92
            } else {
                (x + 0.055 / 1.055).pow(GAMMA)
            }
        });

        a[0] * RED   +
        a[1] * GREEN +
        a[2] * BLUE
    }

    /// saturate the current color by `amount`, which should be between [0.0, 1.0] (inclusive)
    pub fn saturate(&self, amount: f32) -> Self {
        use palette::Saturate;

        //initial
        let a: Srgb<f32> = Srgb::from([self.0, self.1, self.2]).into_format();
        let a: Hsv = a.into_color();
        // saturate is not implemented for rgb
        let rgb: Srgb<f32> = a.saturate(amount).into_color();
        let rgb: Srgb<u8> = rgb.into_format();

        Self(rgb.red, rgb.green, rgb.blue)
    }

    /// Get the complementary color of a color.
    /// Rather than doing something generic like
    /// <https://stackoverflow.com/questions/9577590/formula-to-find-the-split-complementaries-of-a-color#12014465>
    /// I decided to go to the exact oposite.
    /// # Reminder, Hue value from HSV:
    /// Red     falls between 0   and 60  degrees.
    /// Yellow  falls between 61  and 120 degrees.
    /// Green   falls between 121 and 180 degrees.
    /// Cyan    falls between 181 and 240 degrees.
    /// Blue    falls between 241 and 300 degrees.
    /// Magenta falls between 301 and 360 degrees.
    pub fn complementary(&self) -> Self {
        use palette::ShiftHue;
        //initial
        let a: Srgb<f32> = Srgb::from([self.0, self.1, self.2]).into_format();
        let hsv: Hsv = a.into_color();

        // saturate is not implemented for rgb
        let check = hsv.get_hue().into_positive_degrees() as u32;
        let sum = match check {
            0..=60    => 180,
            61..=120  => 240,
            121..=180 => 180,
            181..=240 => -240,
            241..=300 => -420,
            301..=360 => -540,
            _ => 180,
        };

        let rgb: Srgb<f32> = hsv.shift_hue(sum as f32).into_color();
        let rgb: Srgb<u8> = rgb.into_format();

        Self(rgb.red, rgb.green, rgb.blue)
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
        "    ".on_color(self.color0.col()),
        "    ".on_color(self.color1.col()),
        "    ".on_color(self.color2.col()),
        "    ".on_color(self.color3.col()),
        "    ".on_color(self.color4.col()),
        "    ".on_color(self.color5.col()),
        "    ".on_color(self.color6.col()),
        "    ".on_color(self.color7.col()),
        "    ".on_color(self.color8.col()),
        "    ".on_color(self.color9.col()),
        "    ".on_color(self.color10.col()),
        "    ".on_color(self.color11.col()),
        "    ".on_color(self.color12.col()),
        "    ".on_color(self.color13.col()),
        "    ".on_color(self.color14.col()),
        "    ".on_color(self.color15.col()),
        );
    }

    /// Fancy `enjoy the palette!` message
    pub fn done(&self) {
        let space = "  ".strikethrough();
        print!(
"
{}{}{}{}{}{space}{}{}{}{space}{}{}{}{}{}{}{}{}
",
        "E ".color(self.color15.col()).bold().blink(),
        "N ".color(self.color14.col()).bold().blink(),
        "J ".color(self.color13.col()).bold().blink(),
        "O ".color(self.color12.col()).bold().blink(),
        "Y ".color(self.color11.col()).bold().blink(),
        "T ".color(self.color10.col()).bold().blink(),
        "H ".color(self.color9 .col()).bold().blink(),
        "E ".color(self.color8 .col()).bold().blink(),
        "P ".color(self.color7 .col()).bold().blink(),
        "A ".color(self.color6 .col()).bold().blink(),
        "L ".color(self.color5 .col()).bold().blink(),
        "E ".color(self.color4 .col()).bold().blink(),
        "T ".color(self.color3 .col()).bold().blink(),
        "T ".color(self.color2 .col()).bold().blink(),
        "E ".color(self.color1 .col()).bold().blink(),
        "! ".color(self.foreground.col()).bold().blink(),
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
    pub fn contrast_well(a: Myrgb, b: Myrgb) -> bool {
        /// Currently the ratio is hardcoded to `4.5`, standard, but could be decreased at `3.0`
        /// for bigger fonts (tested), but a more allround solution is `4.5` tho.
        const RATIO: f32 = 4.5;

        contrast(a, b) >= RATIO
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
            c.background.set_special(11, "h"),
            c.background.set_special(19, "m"),
            c.background.set_color(232),
            c.background.set_color(257),
            c.background.set_special(708, ""),
        ];


        let fg = [
            //foreground is between 21..=23
            c.foreground.set_special(10, "g"),
            c.foreground.set_special(17, "k"),
            c.foreground.set_color(256),
        ];

        let cursor = [
            //cursor is between 24..=len()
            c.foreground.set_special(12, "l"), //cursor
            c.foreground.set_special(13, "j"), //mouse
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
        return windows_term(self);

        #[cfg(target_family = "unix")]
        return unix_term(self, _cache_path, _ignore);
    }
}

/// returns the ratio between the two colors
fn contrast(a: Myrgb, b: Myrgb) -> f32 {
        let lum1 = a.luminance();
        let lum2 = b.luminance();

        let brightest = f32::max(lum1, lum2);
        let darkest   = f32::min(lum1, lum2);

        (brightest + 0.05) / (darkest + 0.05)
}

/// Set iTerm2 tab/window color
/// `\a` is BELL in octal escape byte, `\x07` in hex
#[cfg(target_os = "macos")]
fn set_iterm_tab_color(c: &Colors) -> String {
    let col = c.background.rgb();
    format!(
"\x1B]6;1;bg;red;brightness;{col}\x07\\\
\x1B]6;1;bg;green;brightness;{col}\x07\\\
\x1B]6;1;bg;blue;brightness;{col}\x07\\\
"
    )
}

/// Uses terminal sequences to update terminal colors
/// ref: <https://github.com/dylanaraps/pywal/blob/master/pywal/sequences.py>
/// ## Special colors.
/// Source: <https://goo.gl/KcoQgP>
/// 10 = foreground, 11 = background, 12 = cursor foreground, 13 = mouse foreground,
/// 708 = background border color.
/// ## Format
/// Escape sequences is "\033]4;%s;%s\033\\" but hex, note the escaped backslash at the end.
/// A triple `\\\` is needed to remove the new line and print a single `\`
#[cfg(target_family = "unix")]
fn unix_term(c: &Colors, cache_path: &Path, remove: Option<&[Sequences]>) -> Result<()> {
    let seq_file = cache_path.join("wallust/sequences");

    let sequences = c.to_seq(remove);

    // set iterm on mac
    #[cfg(target_os = "macos")]
    let sequences = sequences + &set_iterm_tab_color(c);

    #[cfg(target_os = "macos")]
    let tty_pattern = "/dev/ttys00[0-9]*";

    #[cfg(not(target_os = "macos"))]
    let tty_pattern = "/dev/pts/[0-9]*";

    // set custom devices on bsd
    #[cfg(target_os = "openbsd")]
    let devices = openbsd_ttys()?;

    // usually at /dev/pts/*
    #[cfg(not(target_os = "openbsd"))]
    let devices = glob::glob(tty_pattern).expect("glob pattern is ok");

    for entry in devices {
        match entry {
            Ok(path) => {
                match File::create(&path) {
                    Ok(o) => o,
                    Err(e) => { //ignore errors, but report them
                        eprintln!("[{w}] Couldn't write to {p}: {e}", p = path.display(), w = "W".red().bold());
                        continue;
                    },
                }.write_all(sequences.as_bytes())?
            },
            Err(e) => anyhow::bail!("Error while sending sequences to terminals:\n{e}"),
        };
    }

    File::create(seq_file)?
        .write_all(sequences.as_bytes())?;

    Ok(())
}

/// Sets terminal colors on OpenBSD.
/// Calls `ps -o tty | sed -e 1d -e s#^#/dev/# | sort | uniq`
/// ref: <https://github.com/dylanaraps/pywal/pull/510>
#[cfg(target_os = "openbsd")]
use std::path::PathBuf;
#[cfg(target_os = "openbsd")]
fn openbsd_ttys() -> Result<Vec<Result<PathBuf>>> {
    use itertools::Itertools;
    use std::process::{Command, Stdio};
    use std::str;

    let ps = Command::new("ps").arg("-o").arg("tty")
        .stdout(Stdio::piped())       // of which we will pipe the output.
        .spawn()?;

    let ps_out = match ps.stdout {
        Some(s) => s,
        //return empty vec, to avoid quitting on an error.
        None => return Ok(vec![]),
    };

    let sed = Command::new("sed").args(["-e", "1d", "-e", "s#^#/dev/#"])
        .stdin(Stdio::from(ps_out)) // Pipe through.
        .stdout(Stdio::piped())
        .spawn()?;

    let sort = Command::new("sort")
        .stdin(Stdio::from(sed.stdout.expect("should be filled"))) // Pipe through.
        .stdout(Stdio::piped())
        .spawn()?;

    let mut paths = vec![];

    let output = sort.wait_with_output()?;

    //add every line
    for line in str::from_utf8(&output.stdout)?.lines().unique() {
        let p = PathBuf::try_from(line).map_err(anyhow::Error::from);
        paths.push(p);
    }

    Ok(paths)
}

const SCHEME_NAME: &str = "wallust";

/// searches for `settings.json` file to change the scheme in windows cli
#[cfg(target_family = "windows")]
fn windows_term(cols: &Colors) -> Result<()> {
    let Some(dir) = dirs::data_local_dir() else {
        anyhow::bail!("Couldn't get %LOCALAPPDATA%, quitting..");
    };

    let stable  = dir.join("Packages/Microsoft.WindowsTerminal_8wekyb3d8bbwe/LocalState/settings.json");
    let preview = dir.join("Packages/Microsoft.WindowsTerminalPreview_8wekyb3d8bbwe/LocalState/settings.json");
    let unpkg   = dir.join("Microsoft/WindowsTerminal/settings.json");

    let files = [stable, preview, unpkg];

    for i in files {
        let content = match std::fs::read_to_string(&i) {
            Ok(o) => o,
            Err(_) => continue,
        };

        let mut settings_json = match serde_json::from_str::<WinTerm>(&content) {
            Ok(o) => o,
            Err(e) => {
                eprintln!("[{w}] Deserializing json failed {p}: {e}", p = i.display(), w = "W".red().bold());
                continue;
            }
        };

        let mut found = false;

        for (i, s) in settings_json.schemes.iter().enumerate() {
            if s.name == SCHEME_NAME {
                settings_json.schemes[i] = cols.into();
                found = true;
                break; //only do this once, it should only be one "wal" scheme anyway
            }
        }

        // a "wallust" scheme wasn't found, append it.
        if found == false {
            settings_json.schemes.push(cols.into());
        }

        let new_json = match serde_json::to_string_pretty(&settings_json) {
            Ok(o) => o,
            Err(e) => {
                eprintln!("[{w}] Writing json failed {p}: {e}", p = i.display(), w = "W".red().bold());
                continue;
            }
        };

        File::create(&i)?
            .write_all(new_json.as_bytes())?
    }

    Ok(())

}

impl From<&Colors> for WinScheme {
    fn from(c: &Colors) -> Self {
        Self {
            name                : SCHEME_NAME.to_string(),
            cursor_color        : c.color8 .to_string(),
            selection_background: c.color15.to_string(),
            foreground          : c.foreground.to_string(),
            background          : c.background.to_string(),
            black               : c.color0 .to_string(),
            blue                : c.color4 .to_string(),
            cyan                : c.color5 .to_string(),
            green               : c.color1 .to_string(),
            purple              : c.color2 .to_string(),
            red                 : c.color3 .to_string(),
            white               : c.color15.to_string(),
            yellow              : c.color6 .to_string(),
            bright_black        : c.color8 .to_string(),
            bright_blue         : c.color12.to_string(),
            bright_cyan         : c.color13.to_string(),
            bright_green        : c.color9 .to_string(),
            bright_purple       : c.color10.to_string(),
            bright_red          : c.color11.to_string(),
            bright_white        : c.color7 .to_string(),
            bright_yellow       : c.color14.to_string(),
        }
    }
}

use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WinTerm {
    #[serde(rename = "$help")]
    pub help: String,
    #[serde(rename = "$schema")]
    pub schema: String,
    pub actions: Value,
    pub copy_formatting: String,
    pub copy_on_select: bool,
    pub default_profile: String,
    pub new_tab_menu: Value,
    pub profiles: Value,
    pub themes: Value,
    /// This is the only field we need
    pub schemes: Vec<WinScheme>,
}

/// a WindowsTerminal Scheme
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WinScheme {
    pub name: String,
    pub cursor_color: String,
    pub selection_background: String,
    pub background: String,
    pub foreground: String,
    pub black: String,
    pub blue: String,
    pub cyan: String,
    pub green: String,
    pub purple: String,
    pub red: String,
    pub white: String,
    pub yellow: String,
    pub bright_black: String,
    pub bright_blue: String,
    pub bright_cyan: String,
    pub bright_green: String,
    pub bright_purple: String,
    pub bright_red: String,
    pub bright_white: String,
    pub bright_yellow: String,
}

pub trait HexConversion {
    fn decode_hex(&self) -> Result<Vec<u8>>;
}

/// Simple hex decode from string
/// * input `#EEEEEE` or `EEEEEE`
/// * output `[238, 238, 238]`
/// ref: <https://stackoverflow.com/a/52992629>
/// # Example
/// ```
/// # use wallust::colors::HexConversion;
/// let gray    = "#EEEEEE".decode_hex().unwrap();
/// let no_hash = "EE0000".decode_hex().unwrap();
/// assert_eq!(vec![238, 238, 238], gray);
/// assert_eq!(vec![238, 0, 0], no_hash);
/// ```
///
/// # Errors
/// ```
/// # use wallust::colors::HexConversion;
/// let wrong_letter   = "#EEEEEG".decode_hex().unwrap_err();
/// let unneeded_chars = "##EEEEEE".decode_hex().unwrap_err();
/// assert_eq!("invalid digit found in string", wrong_letter.to_string());
/// assert_eq!("Error decoding hex, OddLength", unneeded_chars.to_string());
/// ```
impl HexConversion for &str {
    fn decode_hex(&self) -> Result<Vec<u8>> {
        let s = if &self[..1] == "#" { &self[1..] } else { self };
        let len = s.len();

        if len % 2 != 0 {
            anyhow::bail!("Error decoding hex, OddLength");
        } else {
            (0..len)
                .step_by(2)
                .map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.into()))
                .collect()
        }
    }
}

/// From a vec to Myrgb
impl From<Vec<u8>> for Myrgb {
    fn from(v: Vec<u8>) -> Myrgb {
        Myrgb(v[0], v[1], v[2])
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
