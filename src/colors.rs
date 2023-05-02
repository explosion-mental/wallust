//! # Colors logic
//! Here [`Colors`] and [`Myrgb`] types are defined. These are simple enough used by backends,
//! colorspace and filters modules as a reference, rather than to keep using `Vec<u8>`. This way
//! the base has more structure (also because it's only 16 colors).
use std::fmt;
use std::fs::File;
use std::io::Write;

use owo_colors::{OwoColorize, Rgb};
use serde::{Serialize, Deserialize};

/// This is how the scheme it's organized
#[derive(Serialize, Deserialize)]
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

/// Type that every backend should return
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Myrgb(pub u8, pub u8, pub u8);

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
        "E ".color(self.color0 .col()).bold().blink(),
        "N ".color(self.color1 .col()).bold().blink(),
        "J ".color(self.color2 .col()).bold().blink(),
        "O ".color(self.color3 .col()).bold().blink(),
        "Y ".color(self.color4 .col()).bold().blink(),
        "T ".color(self.color5 .col()).bold().blink(),
        "H ".color(self.color6 .col()).bold().blink(),
        "E ".color(self.color7 .col()).bold().blink(),
        "P ".color(self.color15.col()).bold().blink(),
        "A ".color(self.color14.col()).bold().blink(),
        "L ".color(self.color13.col()).bold().blink(),
        "E ".color(self.color12.col()).bold().blink(),
        "T ".color(self.color11.col()).bold().blink(),
        "T ".color(self.color10.col()).bold().blink(),
        "E ".color(self.color9 .col()).bold().blink(),
        "! ".color(self.color8 .col()).bold().blink(),
        );
    }

    /// Sets terminal color sequences
    /// ref: <https://github.com/dylanaraps/pywal/blob/master/pywal/sequences.py>
    /// ## Special colors.
    /// Source: https://goo.gl/KcoQgP
    /// 10 = foreground, 11 = background, 12 = cursor foreground
    /// 13 = mouse foreground, 708 = background border color.
    pub fn sequences(&self) -> anyhow::Result<()> {
        let seq_file = shellexpand::tilde("~/.cache/wallust/sequences");
        let sequences = format!(
"\x1B]4;0;{col0}\x1B\\\
\x1B]4;1;{col1}\x1B\\\
\x1B]4;2;{col2}\x1B\\\
\x1B]4;3;{col3}\x1B\\\
\x1B]4;4;{col4}\x1B\\\
\x1B]4;5;{col5}\x1B\\\
\x1B]4;6;{col6}\x1B\\\
\x1B]4;7;{col7}\x1B\\\
\x1B]4;8;{col8}\x1B\\\
\x1B]4;9;{col9}\x1B\\\
\x1B]4;10;{col10}\x1B\\\
\x1B]4;11;{col11}\x1B\\\
\x1B]4;12;{col12}\x1B\\\
\x1B]4;13;{col13}\x1B\\\
\x1B]4;14;{col14}\x1B\\\
\x1B]4;15;{col15}\x1B\\\
\x1B]10;{fg}\x1B\\\
\x1B]11;{bg}\x1B\\\
\x1B]12;{cursor}\x1B\\\
\x1B]13;{fg}\x1B\\\
\x1B]17;{fg}\x1B\\\
\x1B]19;{bg}\x1B\\\
\x1B]4;232;{bg}\x1B\\\
\x1B]4;256;{fg}\x1B\\\
\x1B]4;257;{bg}\x1B\\\
",
        bg = self.background,
        fg = self.foreground,
        cursor = self.foreground,
        col0  = self.color0,
        col1  = self.color1,
        col2  = self.color2,
        col3  = self.color3,
        col4  = self.color4,
        col5  = self.color5,
        col6  = self.color6,
        col7  = self.color7,
        col8  = self.color8,
        col9  = self.color9,
        col10 = self.color10,
        col11 = self.color11,
        col12 = self.color12,
        col13 = self.color13,
        col14 = self.color14,
        col15 = self.color15,
        );

        for entry in glob::glob("/dev/pts/[0-9]*").expect("glob pattern is ok") {
            match entry {
                Ok(path) => {
                    File::create(&path)?
                        .write_all(sequences.as_bytes())?;
                    },
                Err(e) => {
                    anyhow::bail!("Error while sending sequences to terminals:\n{e}")
                },
            };
        }

        File::create(seq_file.as_ref())?
            .write_all(sequences.as_bytes())?;

        Ok(())
    }
}
