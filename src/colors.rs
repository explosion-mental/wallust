//! Colors logic, structs and methods
use std::fmt;

use lab::Lab;
use owo_colors::*;

#[derive(serde::Serialize)]
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

/// newtype trick to add methods
pub struct MyLab(Lab);

/// Display the hex color when displaying Lab
impl fmt::Display for MyLab {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a = self.to_rgb();
        write!(f, "#{:02X}{:02X}{:02X}", a[0], a[1], a[2])
    }
}

impl MyLab {
    pub fn to_rgb(&self) -> [u8; 3] {
        self.0.to_rgb()
    }
    pub fn l(&self) -> f32 {
        self.0.l
    }
    pub fn a(&self) -> f32 {
        self.0.a
    }
    pub fn b(&self) -> f32 {
        self.0.b
    }
    /// return the Rgb type value rather than alloc to_string
    pub fn col(&self) -> String {
        let a = self.to_rgb();
        "    ".on_color(Rgb(a[0], a[1], a[2])).to_string()
    }
}

impl Colors<String> {
    fn from(c: Colors<MyLab>) -> Self {
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

impl Colors<MyLab> {
    pub fn print(&self) {
        print!(
"
{}{}{}{}{}{}{}{}
{}{}{}{}{}{}{}{}

background: {}
foreground: {}
",
        self.color0.col(),
        self.color1.col(),
        self.color2.col(),
        self.color3.col(),
        self.color4.col(),
        self.color5.col(),
        self.color6.col(),
        self.color7.col(),
        self.color8.col(),
        self.color9.col(),
        self.color10.col(),
        self.color11.col(),
        self.color12.col(),
        self.color13.col(),
        self.color14.col(),
        self.color15.col(),
        self.background.col(),
        self.foreground.col(),
    );
    }

    pub fn from(histo: &[Histo]) -> Self {
        // control the light value, for bg and fg
        let light = |lab: Lab, amount| {
            Lab {
                l: amount,
                a: lab.a,
                b: lab.b
            }
        };
        Self {
            background : MyLab(light(histo[0].color, 0.0)),
            foreground : MyLab(light(histo[0].color, 100.0)),
            color0 : MyLab(histo[00].color),
            color1 : MyLab(histo[01].color),
            color2 : MyLab(histo[02].color),
            color3 : MyLab(histo[03].color),
            color4 : MyLab(histo[04].color),
            color5 : MyLab(histo[05].color),
            color6 : MyLab(histo[06].color),
            color7 : MyLab(histo[07].color),
            color8 : MyLab(histo[08].color),
            color9 : MyLab(histo[09].color),
            color10: MyLab(histo[10].color),
            color11: MyLab(histo[11].color),
            color12: MyLab(histo[12].color),
            color13: MyLab(histo[13].color),
            color14: MyLab(histo[14].color),
            color15: MyLab(histo[15].color),
        }
    }
}

/// Simple Histogram
pub struct Histo {
    /// LAB colors
    pub color: Lab,
    /// number of times it has appeared
    pub count: usize,
}

impl Histo {
    pub fn mix(&mut self, new: Lab) {
        self.color.l = (new.l + self.color.l) / 2.0;
        self.color.a = (new.a + self.color.a) / 2.0;
        self.color.b = (new.b + self.color.b) / 2.0;
    }

    pub fn print_cols(&self) -> String {
        let a = self.color.to_rgb();
        format!("{} x {}\t\t{}", "    ".on_color(Rgb(a[0], a[1], a[2])), self.count, self)
    }

    //TODO compare light value between the darkest color, and use it as a background. If it isn't
    //dark enough, alter it artificially
    pub fn background(&self) -> Self {
        Self {
            color: Lab {
                l: 0.0,
                a: self.color.a,
                b: self.color.b,
            },
            count: 1,
        }
    }

    //TODO same as background
    pub fn foreground(&self) -> Self {
        Self {
            color: Lab {
                l: 100.0,
                a: self.color.a,
                b: self.color.b,
            },
            count: 1,
        }
    }
}

/// Display the hex color when formatting [`Histo`]
impl fmt::Display for Histo {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a = self.color.to_rgb();
        write!(f, "#{:02X}{:02X}{:02X}", a[0], a[1], a[2])
    }
}


