//! # Colors logic
//! Since most libraries offer conversion to hex (as `#EEEEEE`), the struct will contain rgb values
//! * TODO force 16 colors. maybe use `--theme`s, like `wal`, as backup colors
//! Module about the [`Colors`] struct type, how to construct it and uses for it
//! * TODO generate background and foreground colors, in relation to black and white
//! #About LAB
//! > The lightness value, L*, also referred to as "Lstar," defines black at 0 and white at 100.
//! > The a* axis is relative to the green-red opponent colors, with negative values toward green
//! > and positive > values toward red.
//! > The b* axis represents the blue-yellow opponents, with negative numbers toward
//! > blue and positive toward yellow.
//! ref: <https://en.wikipedia.org/wiki/CIELAB_color_space>
use std::fmt;

use lab::Lab;
use owo_colors::*;
use serde::{Serialize, Deserialize};

/// Generic type used for TinyTemplate and to store the actual colors.
/// Colors are ordered by the most used to the least used.
#[derive(Serialize, Deserialize)]
pub struct Colors<T> {
    pub background: T,
    pub foreground: T,
    pub color0: T,
    pub color1: T,
    pub color2: T,
    pub color3: T,
    pub color4: T,
    pub color5: T,
    pub color6: T,
    pub color7: T,
    pub color8: T,
    pub color9: T,
    pub color10: T,
    pub color11: T,
    pub color12: T,
    pub color13: T,
    pub color14: T,
    pub color15: T,
}

/// Type that every backend should return
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Myrgb(u8, u8, u8);

/// Display the hex color when displaying Lab
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
}

/// print colors as `var.print()`
impl Colors<Myrgb> {
    pub fn print(&self) {
        print!(
"
{}{}{}{}{}{}{}{}
{}{}{}{}{}{}{}{}

background: {}
foreground: {}
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
        "    ".on_color(self.background.col()),
        "    ".on_color(self.foreground.col()),
    );
    }
}

/// From implementation trait for the [`Colors`] with [`Myrgb`] type to a String for TinyTemplate
/// to use
impl From<&Colors<Myrgb>> for Colors<String> {
    fn from(c: &Colors<Myrgb>) -> Self {
        Self {
            background : c.background.to_string(),
            foreground : c.foreground.to_string(),
            color0  : c.color0.to_string(),
            color1  : c.color1.to_string(),
            color2  : c.color2.to_string(),
            color3  : c.color3.to_string(),
            color4  : c.color4.to_string(),
            color5  : c.color5.to_string(),
            color6  : c.color6.to_string(),
            color7  : c.color7.to_string(),
            color8  : c.color8.to_string(),
            color9  : c.color9.to_string(),
            color10 : c.color10.to_string(),
            color11 : c.color11.to_string(),
            color12 : c.color12.to_string(),
            color13 : c.color13.to_string(),
            color14 : c.color14.to_string(),
            color15 : c.color15.to_string(),
        }
    }
}

/// Creates a new darker Lab color derive from the given one, changes values by amount, but
/// prioritizes l(ight) value
fn darken(lab: Lab, amount: f32) -> Lab {
    Lab {
        l: lab.l * (1.0 - amount),
        a: lab.a * (1.0 - amount - 0.15),
        b: lab.b * (1.0 - amount - 0.15),
    }
}

/// Creates a new lighter Lab color derive from the given one, changes values by amount, but
/// prioritizes l(ight) value
fn lighten(lab: Lab, amount: f32) -> Lab {
    Lab {
        l: lab.l + (100.0 - lab.l) * amount,
        a: lab.a + (100.0 - lab.a) * (amount - 0.15),
        b: lab.b + (100.0 - lab.b) * (amount - 0.15),
    }
}

impl From<Lab> for Myrgb {
    fn from(lab: Lab) -> Self {
        let a = lab.to_rgb();
        Self(a[0], a[1], a[2])
    }
}

