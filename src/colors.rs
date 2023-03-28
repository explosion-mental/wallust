//! Colors logic, structs and methods
//! * TODO force 16 colors. maybe use `--theme`s, like `wal`, as backup colors
//! * TODO generate background and foreground colors, in relation to black and white
use std::fmt;
use crate::backends::Histo;

use lab::Lab;
use owo_colors::*;

/// Generic type used for TinyTemplate and to store the actual colors.
/// Colors are ordered by the most used to the least used.
#[derive(serde::Serialize)]
pub struct Colors<T> {
    background: T,
    foreground: T,
    color0: T,
    color1: T,
    color2: T,
    color3: T,
    color4: T,
    color5: T,
    color6: T,
    color7: T,
    color8: T,
    color9: T,
    color10: T,
    color11: T,
    color12: T,
    color13: T,
    color14: T,
    color15: T,
}

/// newtype trick to add methods
pub struct MyLab(Lab);

/// Display the hex color when displaying Lab
impl fmt::Display for MyLab {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a = self.to_rgb();
        write!(f, "#{:02X}{:02X}{:02X}", a[0], a[1], a[2])
    }
}

/// Methods for wrapper [`MyLab`] type (wraps [`Lab`])
impl MyLab {
    pub fn to_rgb(&self) -> [u8; 3] {
        self.0.to_rgb()
    }
    /*
    pub fn l(&self) -> f32 {
        self.0.l
    }
    pub fn a(&self) -> f32 {
        self.0.a
    }
    pub fn b(&self) -> f32 {
        self.0.b
    }
    */
    pub fn col(&self) -> Rgb {
        let a = self.to_rgb();
        Rgb(a[0], a[1], a[2])
    }
}

/// Methods for Colors when it's type uses MyLab
impl Colors<MyLab> {
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

/// From implementation trait for the [`Colors`] with [`MyLab`] type to a String for TinyTemplate
/// to use
impl From<&Colors<MyLab>> for Colors<String> {
    fn from(c: &Colors<MyLab>) -> Self {
        Self {
            background : c.background.to_string(),
            foreground : c.foreground.to_string(),
            color0  : c.color0.to_string(),
            color1  : c.color0.to_string(),
            color2  : c.color0.to_string(),
            color3  : c.color0.to_string(),
            color4  : c.color0.to_string(),
            color5  : c.color0.to_string(),
            color6  : c.color0.to_string(),
            color7  : c.color0.to_string(),
            color8  : c.color0.to_string(),
            color9  : c.color0.to_string(),
            color10 : c.color0.to_string(),
            color11 : c.color0.to_string(),
            color12 : c.color0.to_string(),
            color13 : c.color0.to_string(),
            color14 : c.color0.to_string(),
            color15 : c.color0.to_string(),
        }
    }
}

/// From implementation trait for the vec of [`Histo`] generated in main() to [`Colors`]
impl From<&Vec<Histo>> for Colors<MyLab> {
    fn from(histo: &Vec<Histo>) -> Self {
        // control the light value, for bg and fg
        let light = |lab: Lab, amount| {
            Lab {
                l: amount,
                a: lab.a,
                b: lab.b
            }
        };
        Self {
            background : light(histo[0].color, 0.0).into(),
            foreground : light(histo[0].color, 100.0).into(),
            color0 : histo[ 0].color.into(),
            color1 : histo[ 1].color.into(),
            color2 : histo[ 2].color.into(),
            color3 : histo[ 3].color.into(),
            color4 : histo[ 4].color.into(),
            color5 : histo[ 5].color.into(),
            color6 : histo[ 6].color.into(),
            color7 : histo[ 7].color.into(),
            color8 : histo[ 8].color.into(),
            color9 : histo[ 9].color.into(),
            color10: histo[10].color.into(),
            color11: histo[11].color.into(),
            color12: histo[12].color.into(),
            color13: histo[13].color.into(),
            color14: histo[14].color.into(),
            color15: histo[15].color.into(),
        }
    }
}

impl From<Lab> for MyLab {
    fn from(lab: Lab) -> Self {
        Self(lab)
    }
}

