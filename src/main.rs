//use std::collections::HashMap;
//use std::io::Cursor;
use std::fmt;

//use colorsys::{ColorAlpha, Hsl, Rgb};
use clap::Parser;
use image::io::Reader as ImageReader;
use lab::Lab;
use owo_colors::*;
use anyhow::Result;

mod args;
mod config;
mod delta;
use delta::delta_e;
use args::Cli;
use config::*;
//TODO handle errors
//TODO generate background and foreground colors, in relation to black and white
//XXX BTree?
//XXX generate an actual scheme, rather than listing colors¿

/// Simple Histogram
pub struct Histo {
    /// LAB colors
    pub color: Lab,
    /// number of times it has appeared
    pub count: usize,
}

impl Histo {
    pub fn darken(&mut self, amount: f32) {
        let lightness = self.color.l * (1.0 - amount);
        self.color.l = lightness;
    }

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


/// Threshold to accept the color difference
/// This is temporary, this constant should be auto to get the best result depending on the image
/// size (XXX maybe a threshold for image size then?)
const TH: u32 = 20;

/// #About LAB
/// > The lightness value, L*, also referred to as "Lstar," defines black at 0 and white at 100.
/// > The a* axis is relative to the green-red opponent colors, with negative values toward green
/// > and positive > values toward red.
/// > The b* axis represents the blue-yellow opponents, with negative numbers toward
/// > blue and positive toward yellow.
/// ref: <https://en.wikipedia.org/wiki/CIELAB_color_space>

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Init image, then convert it into rgb and finally to LAB
    let img = ImageReader::open(cli.file)?.decode()?.to_rgba8();
    let labs = lab::rgb_bytes_to_labs(img.as_raw());

    let mut histo: Vec<Histo> = vec![];

    for lab in labs {
        if is_present(lab, &mut histo) {
            continue;
        } else {
            histo.push(Histo { color: lab, count: 1 });
        }

        //let a = lab.to_rgb();
        //print!("{}   ", "COLOR".on_color(Rgb(a[0], a[1], a[2])));
    }

    // sort vec by count
    histo.sort_by(|a, b| b.count.cmp(&a.count));

    //darken the Lab color. Maybe apply these in [`is_present`] ?
    //for i in &mut histo {
    //    i.darken(0.5);
    //}

    let conf = parse_conf()?;
    match conf.entry {
        None => (),
        Some(s) => config::write_template(s, &histo)?,
    };
    //TODO force 16 colors. maybe use `--theme`s, like `wal`, as backup colors
    //for i in histo {
    // only print the top 16 colors
    println!("background:{}", histo[0].background().print_cols());
    println!("foreground:{}", histo[0].foreground().print_cols());

    for (i, color) in histo.iter().take(16).enumerate() {
        let space = if i < 10 { "    " } else { "   " };
        println!("color{}:{}{}", i, space, color.print_cols());
    }

    Ok(())
}

/// determines whether a Lab color is present in our histogram, by using [`delta_e`] we compare if
/// colors are similar enough, using the [`TH`] (threshold)
fn is_present(color: Lab, histogram: &mut Vec<Histo>) -> bool {
    for e in histogram {
        // if any lab value is between a threshold, count it up
        if delta_e(color, e.color) < TH {
            e.mix(color);
            e.count += 1;
            return true;
        }
    }
    false
}
